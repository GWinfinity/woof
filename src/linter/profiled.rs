//! Profiled linter with per-rule timing
//!
//! This module provides profiling capabilities for linting operations,
//! tracking the execution time of each individual rule.

use crate::config::Config;
use crate::logger::{LinterPerfTracker, LogLevel, PerfStats, RulePerfStat};
use crate::rules::get_enabled_rules;
use crate::Diagnostic;
use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tree_sitter::Parser;

/// Lint a file with per-rule profiling
pub fn lint_file_profiled<P: AsRef<Path>>(
    path: P,
    config: &Config,
) -> Result<(Vec<Diagnostic>, Option<PerfStats>)> {
    let path = path.as_ref();
    let source = std::fs::read_to_string(path)?;
    let file_path = path.to_string_lossy().to_string();

    // Track overall file linting time
    let _file_timer = if crate::logger::is_enabled(LogLevel::Perf) {
        Some(crate::logger::PerfTimer::new(format!(
            "lint_file:{}",
            file_path
        )))
    } else {
        None
    };

    let mut parser = Parser::new();
    let language = tree_sitter_go::LANGUAGE.into();
    parser.set_language(&language)?;

    let tree = parser
        .parse(&source, None)
        .ok_or_else(|| anyhow::anyhow!("Failed to parse {}", path.display()))?;

    let root = tree.root_node();
    let rules = get_enabled_rules(config);

    let mut diagnostics = Vec::new();
    let mut timings: HashMap<String, (String, Duration)> = HashMap::new();

    // Run all rules with profiling
    for rule in &rules {
        let code = rule.code().to_string();
        let name = rule.name().to_string();

        let tracker = LinterPerfTracker::start(&code, &name);
        let rule_diagnostics = rule.check(root, &source, &file_path);
        let elapsed = tracker.stop();

        timings.insert(code, (name, elapsed));
        diagnostics.extend(rule_diagnostics);
    }

    // Build performance stats
    let stats = if crate::logger::is_enabled(LogLevel::Perf) {
        Some(build_perf_stats(timings))
    } else {
        None
    };

    Ok((diagnostics, stats))
}

/// Build performance statistics from timings
fn build_perf_stats(timings: HashMap<String, (String, Duration)>) -> PerfStats {
    let total_time_us: u64 = timings.values().map(|(_, d)| d.as_micros() as u64).sum();

    let rule_times: Vec<RulePerfStat> = timings
        .into_iter()
        .map(|(code, (name, duration))| {
            let time_us = duration.as_micros() as u64;
            let percentage = if total_time_us > 0 {
                (time_us as f64 / total_time_us as f64) * 100.0
            } else {
                0.0
            };

            RulePerfStat {
                code,
                name,
                time_us,
                percentage,
            }
        })
        .collect();

    PerfStats {
        total_rules: rule_times.len(),
        total_time_ms: total_time_us / 1000,
        rule_times,
    }
}

/// Lint a directory with profiling and aggregate stats
pub fn lint_path_profiled<P: AsRef<Path>>(
    path: P,
    config: &Config,
) -> Result<(Vec<Diagnostic>, Option<PerfStats>)> {
    let path = path.as_ref();
    let mut all_diagnostics = Vec::new();
    let mut all_timings: HashMap<String, (String, Duration)> = HashMap::new();

    let overall_timer = if crate::logger::is_enabled(LogLevel::Perf) {
        Some(crate::logger::PerfTimer::new(format!(
            "lint_path:{}",
            path.display()
        )))
    } else {
        None
    };

    if path.is_file() {
        if is_go_file(path) {
            let (diags, stats) = lint_file_profiled(path, config)?;
            all_diagnostics.extend(diags);

            if let Some(s) = stats {
                for stat in s.rule_times {
                    let entry = all_timings
                        .entry(stat.code)
                        .or_insert((stat.name, Duration::ZERO));
                    entry.1 += Duration::from_micros(stat.time_us);
                }
            }
        }
    } else if path.is_dir() {
        let files = collect_go_files(path, config)?;

        // Process files in parallel
        use rayon::prelude::*;

        let results: Vec<_> = files
            .par_iter()
            .filter_map(|file| lint_file_profiled(file, config).ok())
            .collect();

        for (diags, stats) in results {
            all_diagnostics.extend(diags);

            if let Some(s) = stats {
                for stat in s.rule_times {
                    let entry = all_timings
                        .entry(stat.code)
                        .or_insert((stat.name, Duration::ZERO));
                    entry.1 += Duration::from_micros(stat.time_us);
                }
            }
        }
    }

    // Sort diagnostics
    all_diagnostics
        .sort_by(|a, b| (&a.file_path, a.line, a.column).cmp(&(&b.file_path, b.line, b.column)));

    // Build aggregate stats
    let stats = if crate::logger::is_enabled(LogLevel::Perf) {
        let total_time_us: u64 = all_timings
            .values()
            .map(|(_, d)| d.as_micros() as u64)
            .sum();

        let rule_times: Vec<RulePerfStat> = all_timings
            .into_iter()
            .map(|(code, (name, duration))| {
                let time_us = duration.as_micros() as u64;
                let percentage = if total_time_us > 0 {
                    (time_us as f64 / total_time_us as f64) * 100.0
                } else {
                    0.0
                };

                RulePerfStat {
                    code,
                    name,
                    time_us,
                    percentage,
                }
            })
            .collect();

        let perf_stats = PerfStats {
            total_rules: rule_times.len(),
            total_time_ms: total_time_us / 1000,
            rule_times,
        };

        // Drop overall timer to print its result
        drop(overall_timer);

        // Print aggregate stats
        perf_stats.print();

        Some(perf_stats)
    } else {
        None
    };

    Ok((all_diagnostics, stats))
}

/// Check if a file is a Go source file
fn is_go_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e == "go")
        .unwrap_or(false)
}

/// Collect all Go files in a directory
fn collect_go_files<P: AsRef<Path>>(path: P, config: &Config) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    let walker = if config.global.respect_gitignore {
        ignore::WalkBuilder::new(path)
            .standard_filters(true)
            .build()
    } else {
        ignore::WalkBuilder::new(path)
            .standard_filters(false)
            .build()
    };

    for entry in walker {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && is_go_file(path)
            && !should_exclude(path, config) {
                files.push(path.to_path_buf());
            }
    }

    Ok(files)
}

/// Check if a file should be excluded
fn should_exclude(path: &Path, config: &Config) -> bool {
    let path_str = path.to_string_lossy();

    for pattern in &config.global.exclude {
        if globset::Glob::new(pattern)
            .ok()
            .and_then(|g| g.compile_matcher().is_match(&*path_str).then_some(()))
            .is_some()
        {
            return true;
        }
    }

    false
}
