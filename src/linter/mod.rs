pub mod massive_parallel;
pub mod parallel;
pub mod profiled;
pub mod visitor;

use crate::config::Config;
use crate::Diagnostic;
use std::path::Path;
use anyhow::Result;

/// Main lint entry point - chooses appropriate implementation
pub fn lint_path<P: AsRef<Path>>(path: P, config: &Config) -> Result<Vec<Diagnostic>> {
    // For now, use the optimized linter
    crate::linter_optimized::lint_path_optimized(path, config)
}

/// Check if we should use massive parallel mode
pub fn should_use_massive_parallel(config: &Config) -> bool {
    let threads = config.global.parallelism.num_threads
        .max(std::env::var("WOOF_THREADS").ok().and_then(|s| s.parse().ok()).unwrap_or(0))
        .max(num_cpus::get());
    
    threads >= 64
}
