//! Parser pool and AST caching for efficient memory usage

use anyhow::Result;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tree_sitter::{Language, Parser, Tree};

/// Thread-safe parser pool to reuse parser instances
pub struct ParserPool {
    parsers: Mutex<Vec<Parser>>,
    language: Language,
}

impl ParserPool {
    pub fn new() -> Result<Self> {
        let language = tree_sitter_go::LANGUAGE.into();
        
        // Pre-initialize parsers for each thread
        let num_threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        
        let mut parsers = Vec::with_capacity(num_threads);
        for _ in 0..num_threads {
            let mut parser = Parser::new();
            parser.set_language(&language)?;
            parsers.push(parser);
        }

        Ok(Self {
            parsers: Mutex::new(parsers),
            language,
        })
    }

    /// Acquire a parser from the pool
    pub fn acquire(&self) -> PooledParser<'_> {
        let mut parsers = self.parsers.lock().unwrap();
        let parser = parsers.pop().unwrap_or_else(|| {
            let mut p = Parser::new();
            p.set_language(&self.language).unwrap();
            p
        });

        PooledParser {
            parser: Some(parser),
            pool: &self.parsers,
        }
    }
}

/// A parser that returns to the pool when dropped
pub struct PooledParser<'a> {
    parser: Option<Parser>,
    pool: &'a Mutex<Vec<Parser>>,
}

impl<'a> PooledParser<'a> {
    /// Parse source code into AST
    pub fn parse(&mut self, source: &str, old_tree: Option<&Tree>) -> Option<Tree> {
        self.parser.as_mut()?.parse(source, old_tree)
    }
}

impl<'a> Drop for PooledParser<'a> {
    fn drop(&mut self) {
        if let Some(parser) = self.parser.take() {
            let mut pool = self.pool.lock().unwrap();
            pool.push(parser);
        }
    }
}

/// Cached AST with source code for incremental parsing
#[derive(Clone)]
pub struct CachedAst {
    pub tree: Tree,
    pub source: Arc<String>,
    pub file_path: Arc<String>,
    pub version: u64,
}

impl CachedAst {
    pub fn new(tree: Tree, source: String, file_path: String) -> Self {
        Self {
            tree,
            source: Arc::new(source),
            file_path: Arc::new(file_path),
            version: 1,
        }
    }

    /// Get root node
    pub fn root_node(&self) -> tree_sitter::Node<'_> {
        self.tree.root_node()
    }
}

/// AST cache with LRU eviction
pub struct AstCache {
    cache: Mutex<lru::LruCache<String, CachedAst>>,
    max_size: usize,
}

impl AstCache {
    pub fn new(max_entries: usize) -> Self {
        Self {
            cache: Mutex::new(lru::LruCache::new(
                std::num::NonZeroUsize::new(max_entries).unwrap()
            )),
            max_size: max_entries,
        }
    }

    /// Get AST from cache
    pub fn get(&self, path: &str) -> Option<CachedAst> {
        let mut cache = self.cache.lock().unwrap();
        cache.get(path).cloned()
    }

    /// Put AST into cache
    pub fn put(&self, path: String, ast: CachedAst) {
        let mut cache = self.cache.lock().unwrap();
        cache.put(path, ast);
    }

    /// Clear the cache
    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let cache = self.cache.lock().unwrap();
        CacheStats {
            entries: cache.len(),
            max_entries: self.max_size,
        }
    }
}

#[derive(Debug)]
pub struct CacheStats {
    pub entries: usize,
    pub max_entries: usize,
}

/// Streaming parser for large files
pub struct StreamingParser;

impl StreamingParser {
    /// Parse a file in chunks if it's too large
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<(Tree, String)> {
        let path = path.as_ref();
        let source = std::fs::read_to_string(path)?;
        
        // For now, use standard parsing
        // TODO: Implement actual streaming for files > 1MB
        let mut parser = Parser::new();
        let language = tree_sitter_go::LANGUAGE.into();
        parser.set_language(&language)?;
        
        let tree = parser.parse(&source, None)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse {}", path.display()))?;
        
        Ok((tree, source))
    }
}

/// Memory-efficient file source that can use memory mapping
pub enum FileSource {
    String(String),
    Mmap(memmap2::Mmap),
}

impl FileSource {
    /// Load file with automatic strategy selection
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let metadata = std::fs::metadata(path)?;
        
        // Use memory mapping for files > 10MB
        if metadata.len() > 10 * 1024 * 1024 {
            let file = std::fs::File::open(path)?;
            let mmap = unsafe { memmap2::Mmap::map(&file)? };
            Ok(Self::Mmap(mmap))
        } else {
            Ok(Self::String(std::fs::read_to_string(path)?))
        }
    }

    /// Get as string slice
    pub fn as_str(&self) -> Result<&str> {
        match self {
            Self::String(s) => Ok(s.as_str()),
            Self::Mmap(m) => std::str::from_utf8(m).map_err(|e| e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_pool() {
        let pool = ParserPool::new().unwrap();
        
        {
            let mut parser = pool.acquire();
            let tree = parser.parse("package main\nfunc main() {}", None);
            assert!(tree.is_some());
        } // Returns to pool
        
        {
            let mut parser = pool.acquire();
            let tree = parser.parse("package test\nvar x = 1", None);
            assert!(tree.is_some());
        }
    }

    #[test]
    fn test_ast_cache() {
        let cache = AstCache::new(100);
        
        let source = "package main\nfunc main() {}".to_string();
        let mut parser = Parser::new();
        let language = tree_sitter_go::LANGUAGE.into();
        parser.set_language(&language).unwrap();
        let tree = parser.parse(&source, None).unwrap();
        
        let ast = CachedAst::new(tree, source, "test.go".to_string());
        cache.put("test.go".to_string(), ast.clone());
        
        let cached = cache.get("test.go");
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().file_path.as_ref(), "test.go");
    }
}
