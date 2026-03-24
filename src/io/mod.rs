//! Zero-copy I/O and shared memory for efficient file processing
//!
//! Features:
//! - Memory-mapped file I/O (mmap)
//! - Shared file content cache (inode-based)
//! - Copy-on-write AST sharing
//! - Zero-copy string slices

use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

/// File identifier for cache lookup (inode + modification time)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileId {
    pub inode: u64,
    pub device: u64,
    pub mtime: SystemTime,
    pub size: u64,
}

impl FileId {
    /// Create FileId from file metadata
    #[cfg(unix)]
    pub fn from_metadata(metadata: &std::fs::Metadata) -> Self {
        Self {
            inode: metadata.ino(),
            device: metadata.dev(),
            mtime: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
            size: metadata.len(),
        }
    }

    /// Create FileId from file metadata (Windows version)
    #[cfg(windows)]
    pub fn from_metadata(metadata: &std::fs::Metadata) -> Self {
        // On Windows, we use file creation time and size as a fallback
        // since there's no inode concept
        Self {
            inode: 0,
            device: 0,
            mtime: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
            size: metadata.len(),
        }
    }

    /// Create FileId from path
    #[cfg(unix)]
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let metadata = std::fs::metadata(path)?;
        Ok(Self::from_metadata(&metadata))
    }

    /// Create FileId from path (Windows version)
    #[cfg(windows)]
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        use std::collections::hash_map::DefaultHasher;

        let metadata = std::fs::metadata(&path)?;
        let mut file_id = Self::from_metadata(&metadata);

        // On Windows, hash the file path to create a pseudo-inode
        let path_str = path.as_ref().to_string_lossy();
        let mut hasher = DefaultHasher::new();
        path_str.hash(&mut hasher);
        file_id.inode = hasher.finish();

        Ok(file_id)
    }
}

/// Zero-copy file content using memory mapping or shared reference
pub enum ZeroCopySource {
    /// Memory-mapped file (zero-copy from kernel)
    Mmap(Arc<memmap2::Mmap>),
    /// Shared string (for small files or already-loaded content)
    Shared(Arc<String>),
    /// Static reference (for embedded/test content)
    Static(&'static str),
}

impl ZeroCopySource {
    /// Open file with zero-copy strategy
    ///
    /// For files > threshold: use mmap
    /// For files <= threshold: read into shared string
    pub fn open<P: AsRef<Path>>(path: P, mmap_threshold: usize) -> Result<Self> {
        let path = path.as_ref();
        let metadata = std::fs::metadata(path)?;
        let file_size = metadata.len() as usize;

        if file_size > mmap_threshold {
            // Use memory mapping for large files
            let file = File::open(path)?;
            let mmap = unsafe { memmap2::Mmap::map(&file)? };
            Ok(Self::Mmap(Arc::new(mmap)))
        } else {
            // Read small files into memory
            let content = std::fs::read_to_string(path)?;
            Ok(Self::Shared(Arc::new(content)))
        }
    }

    /// Create from string (for testing or stdin)
    pub fn from_string(s: String) -> Self {
        Self::Shared(Arc::new(s))
    }

    /// Get content as string slice
    ///
    /// For mmap: validates UTF-8 (may fail for binary files)
    /// For shared: direct reference
    pub fn as_str(&self) -> Result<&str> {
        match self {
            Self::Mmap(mmap) => std::str::from_utf8(mmap)
                .map_err(|e| anyhow::anyhow!("Invalid UTF-8 in mmap: {}", e)),
            Self::Shared(s) => Ok(s.as_str()),
            Self::Static(s) => Ok(s),
        }
    }

    /// Get content as bytes
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Mmap(mmap) => mmap.as_ref(),
            Self::Shared(s) => s.as_bytes(),
            Self::Static(s) => s.as_bytes(),
        }
    }

    /// Get length in bytes
    pub fn len(&self) -> usize {
        self.as_bytes().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clone the handle (increments refcount, no data copy)
    pub fn clone_handle(&self) -> Self {
        match self {
            Self::Mmap(mmap) => Self::Mmap(Arc::clone(mmap)),
            Self::Shared(s) => Self::Shared(Arc::clone(s)),
            Self::Static(s) => Self::Static(s),
        }
    }
}

/// Shared file content cache with inode-based lookup
pub struct SharedFileCache {
    cache: RwLock<HashMap<FileId, Arc<ZeroCopySource>>>,
    mmap_threshold: usize,
    hits: AtomicUsize,
    misses: AtomicUsize,
}

impl SharedFileCache {
    /// Create new cache with specified mmap threshold
    pub fn new(mmap_threshold: usize) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            mmap_threshold,
            hits: AtomicUsize::new(0),
            misses: AtomicUsize::new(0),
        }
    }

    /// Get or load file content
    ///
    /// If file is in cache and not modified, returns cached reference.
    /// Otherwise, loads file and caches it.
    pub fn get_or_load<P: AsRef<Path>>(&self, path: P) -> Result<Arc<ZeroCopySource>> {
        let path = path.as_ref();
        let file_id = FileId::from_path(path)?;

        // Try read lock first
        {
            let cache = self.cache.read().unwrap();
            if let Some(content) = cache.get(&file_id) {
                self.hits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                return Ok(Arc::clone(content));
            }
        }

        // Cache miss - load file
        self.misses
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let content = Arc::new(ZeroCopySource::open(path, self.mmap_threshold)?);

        // Insert with write lock
        {
            let mut cache = self.cache.write().unwrap();
            cache.insert(file_id, Arc::clone(&content));
        }

        Ok(content)
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let hits = self.hits.load(std::sync::atomic::Ordering::Relaxed);
        let misses = self.misses.load(std::sync::atomic::Ordering::Relaxed);
        let total = hits + misses;
        let hit_rate = if total > 0 {
            (hits as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        let entries = self.cache.read().unwrap().len();

        CacheStats {
            entries,
            hits,
            misses,
            hit_rate,
        }
    }

    /// Clear the cache
    pub fn clear(&self) {
        let mut cache = self.cache.write().unwrap();
        cache.clear();
        self.hits.store(0, std::sync::atomic::Ordering::Relaxed);
        self.misses.store(0, std::sync::atomic::Ordering::Relaxed);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CacheStats {
    pub entries: usize,
    pub hits: usize,
    pub misses: usize,
    pub hit_rate: f64,
}

impl std::fmt::Display for CacheStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cache: {} entries, {} hits, {} misses, {:.1}% hit rate",
            self.entries, self.hits, self.misses, self.hit_rate
        )
    }
}

// Global shared file cache (lazy initialized)
lazy_static::lazy_static! {
    static ref GLOBAL_FILE_CACHE: SharedFileCache = SharedFileCache::new(
        // Use mmap for files > 1MB
        1024 * 1024
    );
}

/// Get global file cache
pub fn global_file_cache() -> &'static SharedFileCache {
    &GLOBAL_FILE_CACHE
}

/// Zero-copy file reader that uses the global cache
pub struct ZeroCopyFileReader;

impl ZeroCopyFileReader {
    /// Read file with automatic caching
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Arc<ZeroCopySource>> {
        global_file_cache().get_or_load(path)
    }

    /// Read file without caching (for one-time access)
    pub fn read_uncached<P: AsRef<Path>>(path: P) -> Result<ZeroCopySource> {
        ZeroCopySource::open(path, 1024 * 1024) // 1MB threshold
    }

    /// Get cache statistics
    pub fn cache_stats() -> CacheStats {
        global_file_cache().stats()
    }
}

/// Memory pool for reusing buffers
pub struct BufferPool {
    buffers: RwLock<Vec<Vec<u8>>>,
    max_size: usize,
    max_buffers: usize,
}

impl BufferPool {
    /// Create new buffer pool
    pub fn new(max_size: usize, max_buffers: usize) -> Self {
        Self {
            buffers: RwLock::new(Vec::with_capacity(max_buffers)),
            max_size,
            max_buffers,
        }
    }

    /// Acquire a buffer from the pool
    pub fn acquire(&self) -> Vec<u8> {
        let mut buffers = self.buffers.write().unwrap();
        buffers
            .pop()
            .unwrap_or_else(|| Vec::with_capacity(self.max_size))
    }

    /// Return a buffer to the pool
    pub fn release(&self, mut buffer: Vec<u8>) {
        if buffer.capacity() <= self.max_size {
            buffer.clear();
            let mut buffers = self.buffers.write().unwrap();
            if buffers.len() < self.max_buffers {
                buffers.push(buffer);
            }
        }
        // Otherwise, drop the buffer (too large)
    }
}

/// Fast file collection with parallel directory traversal
pub fn collect_files_parallel<P: AsRef<Path>>(root: P, pattern: &str) -> Result<Vec<PathBuf>> {
    use rayon::prelude::*;

    let root = root.as_ref();

    // Single-threaded walk (I/O bound)
    let entries: Vec<_> = ignore::WalkBuilder::new(root)
        .standard_filters(true)
        .build()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .map(|e| e.path().to_path_buf())
        .collect();

    // Parallel pattern matching
    let files: Vec<_> = entries
        .par_iter()
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.ends_with(pattern))
                .unwrap_or(false)
        })
        .cloned()
        .collect();

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_file_id() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp, "test content").unwrap();

        let id1 = FileId::from_path(tmp.path()).unwrap();
        let id2 = FileId::from_path(tmp.path()).unwrap();

        assert_eq!(id1, id2);
    }

    #[test]
    fn test_zero_copy_source() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp, "hello world").unwrap();

        let source = ZeroCopySource::open(tmp.path(), 1024).unwrap();
        assert_eq!(source.as_str().unwrap().trim(), "hello world");
    }

    #[test]
    fn test_shared_cache() {
        let cache = SharedFileCache::new(1024);

        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp, "cached content").unwrap();

        // First access - miss
        let content1 = cache.get_or_load(tmp.path()).unwrap();

        // Second access - hit
        let content2 = cache.get_or_load(tmp.path()).unwrap();

        assert_eq!(content1.as_str().unwrap(), content2.as_str().unwrap());

        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
    }

    #[test]
    fn test_buffer_pool() {
        let pool = BufferPool::new(1024, 10);

        let buf = pool.acquire();
        assert!(buf.capacity() <= 1024);

        pool.release(buf);

        let buf2 = pool.acquire();
        assert!(buf2.capacity() <= 1024);
    }
}
