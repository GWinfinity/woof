# Woof 零拷贝 I/O 和共享内存优化

## 概述

Woof 实现了多种零拷贝和共享内存技术，以最小化磁盘 I/O 开销：

1. **内存映射文件 (mmap)** - 大文件零拷贝读取
2. **基于 inode 的文件缓存** - 避免重复读取相同文件
3. **共享内存引用** - Arc 共享减少内存复制
4. **Buffer Pool** - 缓冲区复用

## 实现模块

### `src/io/mod.rs`

核心零拷贝 I/O 模块，提供：

### ZeroCopySource

```rust
pub enum ZeroCopySource {
    Mmap(Arc<memmap2::Mmap>),      // 内存映射（大文件）
    Shared(Arc<String>),           // 共享字符串（小文件）
    Static(&'static str),          // 静态引用
}
```

**使用策略：**
- 文件 > 1MB：使用 `mmap`，避免加载到用户空间
- 文件 <= 1MB：使用 `read_to_string`，更快的随机访问

### SharedFileCache

基于 inode 的全局文件缓存：

```rust
lazy_static! {
    static ref GLOBAL_FILE_CACHE: SharedFileCache;
}

// 自动缓存相同文件（通过 inode + mtime 识别）
let content = ZeroCopyFileReader::read("file.go")?;
```

**缓存命中率：**
- 重复检查同一文件：100% 命中
- 未修改的依赖文件：100% 命中
- 缓存自动失效（基于 mtime）

### FileId

文件唯一标识符（基于 inode）：

```rust
pub struct FileId {
    pub inode: u64,
    pub device: u64,
    pub mtime: SystemTime,
    pub size: u64,
}
```

避免通过路径比较文件，使用 inode 更可靠。

## 性能对比

### 测试场景：500 个 Go 文件

| 模式 | 读取方式 | 内存使用 | 速度 |
|------|----------|----------|------|
| 传统 | read_to_string | 高（多份拷贝） | 基准 |
| 零拷贝 | mmap | 低（共享页） | +10-20% |
| 共享缓存 | Arc<String> | 极低（单份共享） | +20-30% |

### 内存映射优势

**大文件（>1MB）：**
```
传统: read_to_string
  - 内核 → 用户空间拷贝
  - 每个进程独立内存

mmap: 
  - 内核页直接映射到用户空间
  - 多个进程共享物理页
  - 延迟加载（只读实际访问部分）
```

## 使用方法

### 自动模式（默认）

```bash
# 自动使用零拷贝 I/O
woof check .
```

### 查看缓存统计

```rust
use woof::io::ZeroCopyFileReader;

let stats = ZeroCopyFileReader::cache_stats();
println!("{}", stats);
// Cache: 150 entries, 230 hits, 50 misses, 82.1% hit rate
```

### 手动控制

```rust
use woof::io::{ZeroCopySource, SharedFileCache};

// 创建自定义缓存
let cache = SharedFileCache::new(1024 * 1024); // 1MB mmap 阈值

// 读取文件
let content = cache.get_or_load("file.go")?;

// 零拷贝获取字符串
let source = content.as_str()?;
```

## 架构设计

```
┌─────────────────────────────────────────────────────┐
│                  Parallel Linter                     │
├─────────────────────────────────────────────────────┤
│  ┌──────────────┐    ┌─────────────────────────┐   │
│  │ Thread Pool  │    │    File Collector       │   │
│  │  (Rayon)     │    │   (Parallel WalkDir)    │   │
│  └──────┬───────┘    └────────────┬────────────┘   │
│         │                         │                 │
│         ▼                         ▼                 │
│  ┌──────────────┐    ┌─────────────────────────┐   │
│  │ Zero-Copy    │    │  SharedFileCache        │   │
│  │ File Reader  │◄───┤  (inode-based lookup)   │   │
│  └──────┬───────┘    └─────────────────────────┘   │
│         │                                           │
│         ▼                                           │
│  ┌──────────────┐    ┌─────────────────────────┐   │
│  │ Thread-Local │    │  Tree-sitter Parser     │   │
│  │ Parser Cache │───►│  (zero-copy source)     │   │
│  └──────────────┘    └─────────────────────────┘   │
└─────────────────────────────────────────────────────┘
```

## 优化策略

### 小文件优化 (< 1MB)

```rust
// 直接读入内存，更快解析
ZeroCopySource::Shared(Arc::new(content))
```

### 大文件优化 (> 1MB)

```rust
// 使用 mmap，延迟加载
ZeroCopySource::Mmap(Arc::new(mmap))
```

### 增量检查优化

```rust
// 基于 inode + mtime 的缓存键
let file_id = FileId::from_path(path)?;

// 缓存自动处理：
// - 相同 inode + 未修改 → 命中缓存
// - 相同 inode + 已修改 → 重新加载
// - 不同 inode → 新缓存条目
```

## 技术细节

### 内存安全

```rust
// mmap 使用 unsafe，但封装在安全接口中
let mmap = unsafe { memmap2::Mmap::map(&file)? };

// UTF-8 验证在 as_str() 时进行
pub fn as_str(&self) -> Result<&str> {
    std::str::from_utf8(mmap)
        .map_err(|e| anyhow::anyhow!("Invalid UTF-8: {}", e))
}
```

### 线程安全

```rust
// Arc 保证线程安全共享
pub struct SharedFileCache {
    cache: RwLock<HashMap<FileId, Arc<ZeroCopySource>>>,
}

// 读多写少场景优化
// - 读：共享锁 (多个读者)
// - 写：独占锁 (缓存未命中)
```

## 配置选项

```toml
[global.io]
# mmap 阈值 (bytes)
mmap_threshold = 1048576  # 1MB

# 最大缓存条目数
max_cache_entries = 1000

# 启用缓冲区池
buffer_pool = true
```

## 未来优化

- [ ] **异步 I/O** (io_uring on Linux)
- [ ] **预读取** (Sequential file prefetching)
- [ ] **压缩缓存** (Large file compression)
- [ ] **跨进程共享** (Shared memory between woof instances)
