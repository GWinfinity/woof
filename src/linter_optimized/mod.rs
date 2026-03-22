//! Memory-optimized linter with shared AST and streaming support

use crate::config::Config;
use crate::rules::{get_enabled_rules, Rule};
use crate::Diagnostic;
use crate::parser::{AstCache, CachedAst, ParserPool};
use anyhow::Result;
use std::path::{Path, PathBuf};


// Global parser pool (shared across all threads)
lazy_static::lazy_static! {
    static ref PARSER_POOL: ParserPool = ParserPool::new().expect("Failed to create parser pool");
    static ref AST_CACHE: AstCache = AstCache::new(1000);
}

/// Lint a file or directory with memory optimization
pub fn lint_path_optimized<P: AsRef<Path>>(path: P, config: &Config) -> Result<Vec<Diagnostic>> {
    let path = path.as_ref();
    let mut all_diagnostics = Vec::new();

    if path.is_file() {
        if is_go_file(path) {
            let diags = lint_file_optimized(path, config)?;
            all_diagnostics.extend(diags);
        }
    } else if path.is_dir() {
        let files = collect_go_files(path, config)?;
        
        // Process files with controlled parallelism and memory
        use rayon::prelude::*;
        
        // Limit memory usage by processing in batches
        const BATCH_SIZE: usize = 100;
        
        for batch in files.chunks(BATCH_SIZE) {
            let results: Vec<_> = batch
                .par_iter()
                .filter_map(|file| lint_file_optimized(file, config).ok())
                .collect();
            
            for diags in results {
                all_diagnostics.extend(diags);
            }
            
            // Clear cache after each batch to control memory
            if files.len() > BATCH_SIZE {
                AST_CACHE.clear();
            }
        }
    }

    // Sort diagnostics by file and line
    all_diagnostics.sort_by(|a, b| {
        (&a.file_path, a.line, a.column).cmp(&(&b.file_path, b.line, b.column))
    });

    Ok(all_diagnostics)
}

/// Lint a single Go file with memory optimization
pub fn lint_file_optimized<P: AsRef<Path>>(path: P, config: &Config) -> Result<Vec<Diagnostic>> {
    let path = path.as_ref();
    let file_path = path.to_string_lossy().to_string();
    
    // Try to get from cache first
    if let Some(cached) = AST_CACHE.get(&file_path) {
        return lint_cached_ast(&cached, config);
    }
    
    // Parse the file
    let source = std::fs::read_to_string(path)?;
    let mut parser = PARSER_POOL.acquire();
    
    let tree = parser.parse(&source, None).ok_or_else(|| {
        anyhow::anyhow!("Failed to parse {}", path.display())
    })?;
    
    // Cache the AST for potential reuse
    let cached = CachedAst::new(tree, source, file_path.clone());
    AST_CACHE.put(file_path.clone(), cached.clone());
    
    lint_cached_ast(&cached, config)
}

/// Lint a cached AST (shared across multiple rules)
fn lint_cached_ast(ast: &CachedAst, config: &Config) -> Result<Vec<Diagnostic>> {
    let root = ast.root_node();
    let rules = get_enabled_rules(config);
    let file_path = ast.file_path.as_ref();
    let source = ast.source.as_ref();
    
    let mut diagnostics = Vec::new();
    
    // Run node-level rules via visitor
    let node_rules: Vec<Box<dyn Rule>> = rules
        .into_iter()
        .filter(|r| !is_file_level_rule(r.code()))
        .collect();
    
    let mut visitor = crate::linter::visitor::Visitor::new(&node_rules, source, file_path);
    visitor.visit(root, &mut diagnostics);

    // Run file-level rules once per file
    let file_rules: Vec<Box<dyn Rule>> = get_enabled_rules(config)
        .into_iter()
        .filter(|r| is_file_level_rule(r.code()))
        .collect();
    
    for rule in file_rules {
        let file_diagnostics = rule.check(root, source, file_path);
        diagnostics.extend(file_diagnostics);
    }

    Ok(diagnostics)
}

fn is_file_level_rule(code: &str) -> bool {
    matches!(code, "E101" | "E201" | "E401")
}

fn is_go_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e == "go")
        .unwrap_or(false)
}

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
