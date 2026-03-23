//! Code formatter for Go source code.
//!
//! This module provides opinionated formatting for Go code with support for:
//! - In-place formatting
//! - Check-only mode for CI
//! - String-based formatting (no file I/O)
//! - Parallel processing for directories
//!
//! # Example
//!
//! ```no_run
//! use woofmt::formatter::{format_path, format_to_string};
//! use woofmt::config::Config;
//!
//! let config = Config::default();
//!
//! // Format a directory
//! let result = format_path("./src", false, &config).unwrap();
//! println!("Formatted {} files", result.files_formatted);
//!
//! // Format a string
//! let source = "package main\n\nfunc main() {\n  fmt.Println(\"hello\")\n}";
//! let formatted = format_to_string(source, &config).unwrap();
//! ```

use crate::config::Config;
use crate::FormatResult;
use anyhow::Result;
use std::path::{Path, PathBuf};

mod engine;
mod printer;

/// Format a file or directory
pub fn format_path<P: AsRef<Path>>(
    path: P,
    check_only: bool,
    config: &Config,
) -> Result<FormatResult> {
    let path = path.as_ref();
    let mut files_checked = 0;
    let mut files_formatted = 0;

    if path.is_file() {
        if is_go_file(path) {
            files_checked += 1;
            if format_file(path, check_only, config)? {
                files_formatted += 1;
            }
        }
    } else if path.is_dir() {
        let files = collect_go_files(path, config)?;

        use rayon::prelude::*;
        let results: Vec<_> = files
            .par_iter()
            .map(|file| format_file(file, check_only, config).map(|changed| (file, changed)))
            .collect::<Result<Vec<_>>>()?;

        for (_file, changed) in results {
            files_checked += 1;
            if changed {
                files_formatted += 1;
            }
        }
    }

    Ok(FormatResult {
        files_checked,
        files_formatted,
        unchanged: files_formatted == 0,
    })
}

/// Format a single Go file
/// Returns true if the file was changed
fn format_file<P: AsRef<Path>>(path: P, check_only: bool, config: &Config) -> Result<bool> {
    let path = path.as_ref();
    let source = std::fs::read_to_string(path)?;
    let formatted = format_source(&source, config)?;

    if source != formatted {
        if !check_only {
            std::fs::write(path, formatted)?;
        }
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Format Go source code
pub fn format_source(source: &str, config: &Config) -> Result<String> {
    let mut parser = tree_sitter::Parser::new();
    let language = tree_sitter_go::LANGUAGE.into();
    parser.set_language(&language)?;

    let tree = parser.parse(source, None).ok_or_else(|| {
        anyhow::anyhow!("Failed to parse source code")
    })?;

    let formatter = engine::Formatter::new(config);
    formatter.format(tree.root_node(), source)
}

/// Format Go source code and return result as string (for stdout mode)
pub fn format_to_string(source: &str, config: &Config) -> Result<String> {
    format_source(source, config)
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

        if path.is_file() && is_go_file(path) {
            if !should_exclude(path, config) {
                files.push(path.to_path_buf());
            }
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
