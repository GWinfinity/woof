//! Woof - An extremely fast Go linter and code formatter written in Rust
//!
//! This library provides functionality for linting and formatting Go code.

pub mod arena;
pub mod cli;
pub mod config;
pub mod formatter;
pub mod io;  // Zero-copy I/O
pub mod linter;
pub mod linter_optimized;
pub mod linter_profiled {
    pub use super::linter::profiled::*;
}
pub mod linter_parallel {
    pub use super::linter::parallel::*;
}
pub mod logger;
pub mod parser;
pub mod rules;

// Re-export logging macros
#[doc(hidden)]
pub use logger::{log_level, is_enabled, LogLevel};

use anyhow::Result;
use std::path::Path;

/// Represents a diagnostic message from linting
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub file_path: String,
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub code: String,
    pub severity: Severity,
    pub fix: Option<Fix>,
}

/// Severity level of a diagnostic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
            Severity::Info => write!(f, "info"),
        }
    }
}

/// A fix suggestion for a diagnostic
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fix {
    pub description: String,
    pub replacement: String,
    pub start_byte: usize,
    pub end_byte: usize,
}

/// Run the linter on a file or directory
pub fn lint_path<P: AsRef<Path>>(path: P, config: &config::Config) -> Result<Vec<Diagnostic>> {
    linter::lint_path(path, config)
}

/// Format a file or directory
pub fn format_path<P: AsRef<Path>>(path: P, check_only: bool, config: &config::Config) -> Result<FormatResult> {
    formatter::format_path(path, check_only, config)
}

/// Format Go source code string
pub fn format_to_string(source: &str, config: &config::Config) -> Result<String> {
    formatter::format_to_string(source, config)
}

/// Result of a format operation
#[derive(Debug, Clone)]
pub struct FormatResult {
    pub files_checked: usize,
    pub files_formatted: usize,
    pub unchanged: bool,
}
