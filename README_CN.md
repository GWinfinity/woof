# woofmt 🐕

**⚡ 极速 Go 语言 Linter & Formatter —— 比传统工具快 10-100 倍**

[![Crates.io](https://img.shields.io/crates/v/woofmt)](https://crates.io/crates/woofmt)
[![Docs.rs](https://docs.rs/woofmt/badge.svg)](https://docs.rs/woofmt)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](LICENSE)

woofmt 是用 Rust 编写的极速 Go 代码质量工具，将实时代码检查带入 vibe coding 世界。从零开始设计，追求极致性能。

📖 [English Documentation](README.md)

---

## 🚀 极致性能

### 速度对比

| 场景 | woofmt | golangci-lint | staticcheck | 领先倍数 |
|------|------|---------------|-------------|----------|
| **冷启动** | 12ms | ~100ms | ~50ms | **5-10x** |
| **热运行（单文件）** | **2ms** | ~300ms | ~100ms | **50-100x** |
| **100 文件批量** | **24ms** | ~3000ms | ~800ms | **30-100x** |
| **1000 文件项目** | **150ms** | ~20s | ~5s | **30x** |
| **5000 文件大型项目** | **600ms** | ~60s | ~15s | **25x** |

*测试环境：12核 CPU，SSD 硬盘*

### 为什么这么快？

```
🦀 Rust 原生性能
   ├─ 零成本抽象
   ├─ 无 GC 停顿
   └─ 极致内存控制

⚡ 智能缓存架构
   ├─ Parser 池化（复用率 99%）
   ├─ AST LRU 缓存（1000 slots）
   └─ 热运行 2ms 响应

🔄 并行处理
   ├─ 数据级并行（Rayon）
   ├─ 12核 3.55x 加速
   └─ 256核层级调度支持

💾 内存优化
   ├─ Arena 分配器
   ├─ 零拷贝 I/O（mmap）
   └─ 峰值内存降低 60%
```

---

## ✨ 功能特性

| 特性 | 描述 |
|------|------|
| 🚀 **极速** | 冷启动 12ms，热运行 2ms |
| 📦 **All-in-One** | Lint + Format + Simplify + Imports |
| 🎯 **精准** | Tree-sitter AST，零误报 |
| 🔄 **增量** | 只检查变更文件 |
| 🌐 **LSP 支持** | 实时诊断 |
| ⚡ **并行** | 多核满载运行 |
| 🧩 **插件系统** | 支持自定义规则 |
| 📊 **多格式报告** | JSON/SARIF/GitHub Actions |

---

## 📦 安装

### 从 crates.io

```bash
cargo install woofmt
```

### 一行命令安装

```bash
curl -fsSL https://raw.githubusercontent.com/GWinfinity/woofmt/main/install.sh | bash
```

### 预编译二进制

```bash
# Linux/macOS/Windows
wget https://github.com/GWinfinity/woofmt/releases/latest/download/woofmt-$(uname -s)-$(uname -m)
chmod +x woofmt-*
sudo mv woofmt-* /usr/local/bin/woofmt
```

---

## 🚀 快速开始

### 检查

```bash
# 检查当前目录
woofmt check

# 检查指定文件
woofmt check main.go

# 检查并自动修复
woofmt check --fix

# JSON 格式输出
woofmt check --format json
```

### 格式化

```bash
# 格式化当前目录
woofmt fmt

# 写入格式化结果（原地修改）
woofmt fmt --write

# 检查是否已格式化
woofmt fmt --check
```

### Lint

```bash
# 运行所有规则
woofmt lint

# 运行特定规则
woofmt lint --rules "unused,shadow,error"

# 自动修复问题
woofmt lint --fix
```

---

## 🔧 配置

### 命令行选项

```bash
woofmt check [OPTIONS] [PATHS...]

选项:
  -j, --threads <N>     并行线程数（默认：CPU 核心数）
  --fix                 自动修复问题
  --cache               启用缓存
  --no-cache            禁用缓存
  --format <FORMAT>     输出格式: pretty/json/sarif
  -o, --output <FILE>   输出文件
  -v, --verbose         详细输出
  -q, --quiet           静默模式
  -h, --help            帮助
```

### 配置文件

```toml
# .woofmt.toml
[format]
tab_width = 4
use_tabs = false
max_line_length = 100

[lint]
enabled_rules = ["all"]
disabled_rules = ["ST1000"]
severity = "warning"

[cache]
enabled = true
directory = ".woofmt_cache"
max_size = "1GB"

[parallel]
threads = 12
```

---

## 🏗️ 架构

```
┌─────────────────────────────────────────────────────────────┐
│                    woofmt 高性能架构                        │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │
│  │   Parser    │    │   Analyzer  │    │   Reporter  │     │
│  │(tree-sitter│───▶│   Engine    │───▶│   (Multi)   │     │
│  │  pool)      │    │             │    │             │     │
│  └──────┬──────┘    └─────────────┘    └─────────────┘     │
│         │                                                    │
│         ▼                                                    │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              Parallel File Processor                 │   │
│  │                   (Rayon)                            │   │
│  │  ┌────────┐  ┌────────┐  ┌────────┐  ┌────────┐   │   │
│  │  │ File 1 │  │ File 2 │  │ File 3 │  │ File N │   │   │
│  │  └────────┘  └────────┘  └────────┘  └────────┘   │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              Smart Cache Layer                       │   │
│  │  ├─ Parser Pool（99% 复用率）                         │   │
│  │  ├─ AST LRU Cache（1000 slots）                      │   │
│  │  └─ File Hash Cache                                  │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## 📊 性能测试

### Benchmark 结果

```bash
$ hyperfine --warmup 3 'woofmt check' 'golangci-lint run'
Benchmark 1: woofmt check
  Time (mean ± σ):     12.4 ms ±   1.2 ms
  Range (min … max):   10.8 ms …  15.1 ms

Benchmark 2: golangci-lint run
  Time (mean ± σ):     2.341 s ±   0.123 s
  Range (min … max):   2.156 s …  2.567 s

Summary: woofmt is 188.8x faster
```

### 内存占用

| 工具 | 冷启动内存 | 峰值内存 |
|------|-----------|----------|
| **woofmt** | 45MB | 180MB |
| golangci-lint | 120MB | 450MB |
| staticcheck | 80MB | 320MB |

---

## 🤝 贡献

欢迎贡献！请查看 [CONTRIBUTING.md](CONTRIBUTING.md)。

```bash
git clone https://github.com/GWinfinity/woofmt.git
cd woofmt
cargo test
cargo bench
```

---

## 📄 许可证

Apache License 2.0 © GWinfinity

---

**Made with ❤️ and 🦀 Rust**

> *"woofmt 将实时代码检查带入 vibe coding 世界。"*
