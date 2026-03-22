//! Memory usage benchmark for woof

use std::time::Instant;

fn main() {
    println!("Woof Memory Usage Benchmark");
    println!("============================\n");
    
    let args: Vec<String> = std::env::args().collect();
    let target = args.get(1).map(|s| s.as_str()).unwrap_or("benchmark/scenarios");
    
    println!("Target: {}", target);
    
    // Measure baseline memory
    let baseline = get_memory_usage();
    println!("Baseline memory: {} MB", baseline);
    
    // Run optimized linter
    let config = woof::config::Config::default();
    
    let start = Instant::now();
    let before_mem = get_memory_usage();
    
    let result = woof::linter_optimized::lint_path_optimized(target, &config);
    
    let after_mem = get_memory_usage();
    let duration = start.elapsed();
    
    match result {
        Ok(diagnostics) => {
            println!("\nResults:");
            println!("  Files processed: {}", diagnostics.len());
            println!("  Time: {:?}", duration);
            println!("  Memory before: {} MB", before_mem);
            println!("  Memory after: {} MB", after_mem);
            println!("  Memory delta: {} MB", after_mem - before_mem);
            
            // Count issues by severity
            let mut errors = 0;
            let mut warnings = 0;
            let mut infos = 0;
            
            for diag in &diagnostics {
                match diag.severity {
                    woof::Severity::Error => errors += 1,
                    woof::Severity::Warning => warnings += 1,
                    woof::Severity::Info => infos += 1,
                }
            }
            
            println!("  Issues: {} errors, {} warnings, {} infos", errors, warnings, infos);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

fn get_memory_usage() -> usize {
    // Read from /proc/self/status on Linux
    #[cfg(target_os = "linux")]
    {
        if let Ok(contents) = std::fs::read_to_string("/proc/self/status") {
            for line in contents.lines() {
                if line.starts_with("VmRSS:") {
                    let parts: Vec<_> = line.split_whitespace().collect();
                    if let Some(kb_str) = parts.get(1) {
                        if let Ok(kb) = kb_str.parse::<usize>() {
                            return kb / 1024; // Convert to MB
                        }
                    }
                }
            }
        }
    }
    0
}
