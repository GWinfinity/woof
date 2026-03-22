//! Arena-based memory management for AST nodes
//! 
//! This module provides efficient memory allocation for AST nodes
//! by using arena allocators and object pooling.

use std::sync::{Arc, Mutex};

/// Thread-local arena for AST allocation
pub struct AstArena {
    bump: bumpalo::Bump,
}

impl AstArena {
    pub fn new() -> Self {
        Self {
            bump: bumpalo::Bump::new(),
        }
    }

    /// Allocate an object in the arena
    pub fn alloc<T>(&self, val: T) -> &mut T {
        self.bump.alloc(val)
    }

    /// Allocate a slice in the arena
    pub fn alloc_slice<T>(&self, slice: &[T]) -> &mut [T]
    where
        T: Copy,
    {
        self.bump.alloc_slice_copy(slice)
    }

    /// Reset the arena (keeps allocated memory for reuse)
    pub fn reset(&mut self) {
        self.bump.reset();
    }

    /// Get memory usage statistics
    pub fn allocated_bytes(&self) -> usize {
        self.bump.allocated_bytes()
    }
}

impl Default for AstArena {
    fn default() -> Self {
        Self::new()
    }
}

/// Shared AST storage with reference counting
pub struct SharedAst<T> {
    inner: Arc<T>,
}

impl<T> SharedAst<T> {
    pub fn new(data: T) -> Self {
        Self {
            inner: Arc::new(data),
        }
    }

    pub fn clone_ref(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> std::ops::Deref for SharedAst<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Clone for SharedAst<T> {
    fn clone(&self) -> Self {
        self.clone_ref()
    }
}

/// Memory pool for reusable objects
pub struct ObjectPool<T> {
    pool: Mutex<Vec<T>>,
    create: Box<dyn Fn() -> T + Send + Sync>,
    reset: Box<dyn Fn(&mut T) + Send + Sync>,
}

impl<T> ObjectPool<T> {
    pub fn new<F, R>(create: F, reset: R) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
        R: Fn(&mut T) + Send + Sync + 'static,
    {
        Self {
            pool: Mutex::new(Vec::new()),
            create: Box::new(create),
            reset: Box::new(reset),
        }
    }

    /// Get an object from the pool
    pub fn acquire(&self) -> PooledObject<'_, T> {
        let mut pool = self.pool.lock().unwrap();
        let mut obj = pool.pop().unwrap_or_else(|| (self.create)());
        (self.reset)(&mut obj);
        
        PooledObject {
            obj: Some(obj),
            pool: &self.pool,
        }
    }


}

/// A pooled object that returns to the pool when dropped
pub struct PooledObject<'a, T> {
    obj: Option<T>,
    pool: &'a Mutex<Vec<T>>,
}

impl<'a, T> std::ops::Deref for PooledObject<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.obj.as_ref().unwrap()
    }
}

impl<'a, T> std::ops::DerefMut for PooledObject<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.obj.as_mut().unwrap()
    }
}

impl<'a, T> Drop for PooledObject<'a, T> {
    fn drop(&mut self) {
        if let Some(obj) = self.obj.take() {
            let mut pool = self.pool.lock().unwrap();
            pool.push(obj);
        }
    }
}

/// Global memory statistics
#[derive(Debug, Default)]
pub struct MemoryStats {
    pub ast_bytes: usize,
    pub source_bytes: usize,
    pub diagnostic_count: usize,
    pub pool_hits: usize,
    pub pool_misses: usize,
}

/// Thread-local memory context
pub struct MemoryContext {
    arena: AstArena,
    stats: MemoryStats,
}

impl MemoryContext {
    pub fn new() -> Self {
        Self {
            arena: AstArena::new(),
            stats: MemoryStats::default(),
        }
    }

    pub fn arena(&self) -> &AstArena {
        &self.arena
    }

    pub fn stats(&self) -> &MemoryStats {
        &self.stats
    }

    pub fn reset(&mut self) {
        self.arena.reset();
        self.stats = MemoryStats::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arena_allocation() {
        let arena = AstArena::new();
        let nums = arena.alloc_slice(&[1, 2, 3, 4, 5]);
        assert_eq!(nums, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_object_pool() {
        let pool = ObjectPool::new(
            || Vec::<i32>::with_capacity(100),
            |v| v.clear(),
        );

        {
            let mut obj = pool.acquire();
            obj.push(1);
            obj.push(2);
            assert_eq!(obj.len(), 2);
        } // Returns to pool here

        {
            let obj = pool.acquire();
            assert!(obj.is_empty()); // Reset was called
        }
    }
}
