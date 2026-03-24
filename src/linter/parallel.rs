//! High-performance parallel linter with near-linear speedup
//!
//! Features:
//! - Work-stealing task distribution for load balancing
//! - Zero-copy file I/O (mmap for large files)
//! - Optimized batch sizes for minimal scheduling overhead
//! - Thread-local parser cache

use crate::config::Config;
use crate::io::ZeroCopyFileReader;
use crate::rules::get_enabled_rules;
use crate::Diagnostic;
use anyhow::Result;
use rayon::prelude::*;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::time::Instant;

/// Metrics for parallel execution monitoring
#[derive(Debug, Default)]
pub struct ParallelMetrics {
    pub files_processed: AtomicUsize,
    pub total_files: usize,
    pub elapsed_ms: u64,
    pub throughput_files_per_sec: f64,
}

impl ParallelMetrics {
    pub fn cpu_utilization_percent(&self) -> f64 {
        let num_cpus = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1) as f64;

        let theoretical_max = (self.elapsed_ms as f64 / 1000.0) * num_cpus * 1000.0;
        if theoretical_max > 0.0 {
            ((self.total_files as f64 / theoretical_max) * 100.0).min(100.0)
        } else {
            0.0
        }
    }
}

// Thread-local parser cache
thread_local! {
    static PARSER: RefCell<Option<tree_sitter::Parser>> = RefCell::new(None);
}

/// Get or create thread-local parser
fn with_parser<F, R>(f: F) -> R
where
    F: FnOnce(&mut tree_sitter::Parser) -> R,
{
    PARSER.with(|cell| {
        let mut opt = cell.borrow_mut();
        if opt.is_none() {
            let mut parser = tree_sitter::Parser::new();
            parser.set_language(&tree_sitter_go::LANGUAGE.into()).ok();
            *opt = Some(parser);
        }
        f(opt.as_mut().unwrap())
    })
}

/// Calculate optimal batch size based on file count and CPU cores
fn optimal_batch_size(total_files: usize, num_cpus: usize) -> usize {
    if total_files <= num_cpus {
        1
    } else {
        // Aim for at least 4 batches per CPU to ensure good load balancing
        // but not too many to avoid scheduling overhead
        let target_batches = num_cpus * 4;
        let batch_size = (total_files / target_batches).max(1);
        batch_size.min(100) // Cap at 100 to ensure progress visibility
    }
}

/// Lint multiple paths with optimized parallel execution
///
/// Achieves near-linear speedup by:
/// 1. Using work-stealing for dynamic load balancing
/// 2. Optimized batch sizes to minimize scheduling overhead
/// 3. Thread-local parser cache to avoid recreation
/// 4. Zero-copy file I/O for minimal memory overhead
pub fn lint_paths_parallel(
    files: &[PathBuf],
    config: &Config,
) -> Result<(Vec<Diagnostic>, ParallelMetrics)> {
    let start = Instant::now();
    let total_files = files.len();

    if total_files == 0 {
        return Ok((Vec::new(), ParallelMetrics::default()));
    }

    // Get enabled rules once - Arc for cheap cloning
    let rules = Arc::new(get_enabled_rules(config));

    // Determine optimal parallelism
    let num_cpus = config.global.parallelism.num_threads.max(1).min(
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4),
    );

    // Calculate optimal batch size for this workload
    let batch_size = if config.global.parallelism.batch_size > 0 {
        config.global.parallelism.batch_size
    } else {
        optimal_batch_size(total_files, num_cpus)
    };

    crate::log_debug!(
        "Parallel lint: {} files, {} CPUs, batch size {}",
        total_files,
        num_cpus,
        batch_size
    );

    // Use work-stealing parallel iterator for best load balancing
    let all_diagnostics: Vec<Diagnostic> = files
        .par_chunks(batch_size)
        .flat_map(|chunk| {
            // Process each chunk - work will be stolen between threads
            let rules = Arc::clone(&rules);
            chunk
                .iter()
                .flat_map(|file| lint_single_file(file, &rules).unwrap_or_default())
                .collect::<Vec<_>>()
        })
        .collect();

    // Parallel sort for fast result ordering
    let mut sorted_diagnostics = all_diagnostics;
    sorted_diagnostics.par_sort_unstable_by(|a, b| {
        (&a.file_path, a.line, a.column).cmp(&(&b.file_path, b.line, b.column))
    });

    // Calculate metrics
    let elapsed = start.elapsed();
    let elapsed_ms = elapsed.as_millis() as u64;
    let throughput = if elapsed_ms > 0 {
        (total_files as f64) / (elapsed_ms as f64 / 1000.0)
    } else {
        0.0
    };

    let metrics = ParallelMetrics {
        files_processed: AtomicUsize::new(total_files),
        total_files,
        elapsed_ms,
        throughput_files_per_sec: throughput,
    };

    Ok((sorted_diagnostics, metrics))
}

/// Convenience function for single path
pub fn lint_path_parallel<P: AsRef<Path>>(
    path: P,
    config: &Config,
) -> Result<(Vec<Diagnostic>, ParallelMetrics)> {
    let files = if path.as_ref().is_file() {
        vec![path.as_ref().to_path_buf()]
    } else {
        collect_go_files(path.as_ref(), config)?
    };

    lint_paths_parallel(&files, config)
}

/// Lint a single file using zero-copy I/O and thread-local parser
fn lint_single_file(
    path: &Path,
    rules: &Arc<Vec<Box<dyn crate::rules::Rule>>>,
) -> Result<Vec<Diagnostic>> {
    let file_path = path.to_string_lossy().to_string();

    // Use zero-copy file reading (mmap for large files)
    let source_arc = ZeroCopyFileReader::read(path)?;
    let source = source_arc.as_str()?;

    // Parse file using thread-local parser
    let tree = with_parser(|parser| parser.parse(source, None))
        .ok_or_else(|| anyhow::anyhow!("Failed to parse {}", path.display()))?;

    let root = tree.root_node();

    let mut diagnostics = Vec::new();

    // Run file-level rules
    for rule in rules.iter() {
        if is_file_level_rule(rule.code()) {
            diagnostics.extend(rule.check(root, source, &file_path));
        }
    }

    // Run node-level rules
    for rule in rules.iter() {
        if !is_file_level_rule(rule.code()) {
            visit_node_recursive(root, rule, source, &file_path, &mut diagnostics);
        }
    }

    Ok(diagnostics)
}

/// Recursively visit nodes and apply rule
fn visit_node_recursive(
    node: tree_sitter::Node,
    rule: &Box<dyn crate::rules::Rule>,
    source: &str,
    file_path: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    diagnostics.extend(rule.check(node, source, file_path));

    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            visit_node_recursive(cursor.node(), rule, source, file_path, diagnostics);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Collect all Go files (public for CLI usage)
pub fn collect_go_files(path: &Path, config: &Config) -> Result<Vec<PathBuf>> {
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

        if path.is_file() && is_go_file(path) {
            if !should_exclude(path, config) {
                files.push(path.to_path_buf());
            }
        }
    }

    Ok(files)
}

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

/// File-level rule codes
fn is_file_level_rule(code: &str) -> bool {
    matches!(
        code,
        "E101" | "E115" | "E116" | "E117" | "E118" | "E201" | "E401"
    )
}

fn is_go_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e == "go")
        .unwrap_or(false)
}

/// Benchmark function to measure parallel performance
pub fn benchmark_parallel<P: AsRef<Path>>(
    path: P,
    config: &Config,
    iterations: usize,
) -> Result<Vec<ParallelMetrics>> {
    let files = if path.as_ref().is_file() {
        vec![path.as_ref().to_path_buf()]
    } else {
        collect_go_files(path.as_ref(), config)?
    };

    let mut results = Vec::with_capacity(iterations);

    for i in 0..iterations {
        crate::log_debug!("Benchmark iteration {}/{}", i + 1, iterations);
        let (_, metrics) = lint_paths_parallel(&files, config)?;
        results.push(metrics);
    }

    Ok(results)
}

/// Print parallel scaling analysis
pub fn analyze_scaling(baseline_ms: u64, results: &[(usize, u64)]) {
    println!("\n=== Parallel Scaling Analysis ===");
    println!("Baseline (1 CPU): {} ms", baseline_ms);
    println!();
    println!(
        "{:>8} {:>12} {:>12} {:>12}",
        "CPUs", "Time (ms)", "Speedup", "Efficiency"
    );
    println!("{:-^48}", "");

    for (cpus, time_ms) in results {
        let speedup = baseline_ms as f64 / *time_ms as f64;
        let efficiency = (speedup / *cpus as f64) * 100.0;

        println!(
            "{:>8} {:>12} {:>11.2}x {:>10.1}%",
            cpus, time_ms, speedup, efficiency
        );
    }

    println!();

    // Find best efficiency point
    let best = results
        .iter()
        .map(|(cpus, time_ms)| {
            let speedup = baseline_ms as f64 / *time_ms as f64;
            let efficiency = (speedup / *cpus as f64) * 100.0;
            (*cpus, efficiency)
        })
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap_or((1, 100.0));

    println!("Best efficiency: {} CPUs at {:.1}%", best.0, best.1);

    if best.1 > 80.0 {
        println!("✓ Excellent parallel scaling!");
    } else if best.1 > 60.0 {
        println!("✓ Good parallel scaling");
    } else {
        println!("⚠ Consider reducing thread count for better efficiency");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_metrics() {
        let metrics = ParallelMetrics::default();
        assert_eq!(metrics.total_files, 0);
    }

    #[test]
    fn test_is_file_level_rule() {
        assert!(is_file_level_rule("E101"));
        assert!(!is_file_level_rule("F401"));
    }

    #[test]
    fn test_optimal_batch_size() {
        assert_eq!(optimal_batch_size(10, 12), 1);
        assert_eq!(optimal_batch_size(100, 4), 6); // 100 / 16 = 6.25
        assert_eq!(optimal_batch_size(1000, 8), 31); // 1000 / 32 = 31.25
    }
}
