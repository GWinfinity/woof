# Woof 🐕

**⚡ 极速 Go 语言 Linter & Formatter —— 比传统工具快 10-100 倍**

[![Crates.io](https://img.shields.io/crates/v/woofmt)](https://crates.io/crates/woofmt)
[![Docs.rs](https://docs.rs/woofmt/badge.svg)](https://docs.rs/woofmt)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](LICENSE)

Woof 是用 Rust 编写的极速 Go 代码质量工具，将 Python 生态中 Ruff 的体验带到 Go 世界。从零开始设计，追求极致性能。

---

## 🚀 极致性能

### 速度对比

| 场景 | Woof | golangci-lint | staticcheck | 领先倍数 |
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

## 📊 性能详情

### 冷启动 vs 热运行

| 指标 | 冷启动 | 热运行 | 加速比 |
|------|--------|--------|--------|
| 单文件检查 | 14ms | **2ms** | **7.5x** |
| 内存占用 | 3.5MB | 3.5MB | - |
| 二进制大小 | **2MB** | - | - |

### 并行扩展性

```
CPU 核心 │ 耗时    │ 加速比 │ 效率
─────────┼─────────┼────────┼────────
   1     │ 6,211ms │ 1.00x  │ 100%
   2     │ 3,488ms │ 1.78x  │  89%  
   4     │ 2,573ms │ 2.41x  │  60%
   8     │ 2,130ms │ 2.91x  │  36%
  12     │ 1,749ms │ 3.55x  │  30%
```

*测试项目：500 文件，~50,000 行 Go 代码*

### 内存效率

| 项目规模 | 优化前 | 优化后 | 降低比例 |
|----------|--------|--------|----------|
| 100 文件 | 50MB | 20MB | **-60%** |
| 1,000 文件 | 300MB | 120MB | **-60%** |
| 10,000 文件 | 2GB | 800MB | **-60%** |
| 增量检查 | 300MB | 25MB | **-92%** |

---

## ✨ 功能特性

| 特性 | 描述 |
|------|------|
| 🔍 **116+ Lint 规则** | E/F/B/I/UP/SIM/S/D/P/C/SEC 完整规则体系 |
| 🔧 **自动修复** | 一键修复常见问题，`--fix` 即刻生效 |
| 🎨 **智能格式化** | Opinionated Go 代码格式化 |
| 📦 **单二进制** | 2MB 单文件，零依赖 |
| 🔌 **GitHub Actions** | 原生支持 annotations |
| 📊 **多格式输出** | text / json / github |
| ⚙️ **TOML 配置** | 灵活可配置的 `woof.toml` |

---

## 📦 安装

### 从 crates.io

```bash
cargo install woofmt
```

### 从源码

```bash
git clone https://github.com/GWinfinity/woof.git
cd woof
cargo install --path . --release
```

### 预编译二进制

```bash
# Linux x86_64
curl -L https://github.com/GWinfinity/woof/releases/latest/download/woof-linux-amd64 -o woof
chmod +x woof
sudo mv woof /usr/local/bin/
```

---

## 🚀 快速开始

```bash
# 检查当前目录
woof check .

# 检查并自动修复
woof check . --fix

# 格式化代码
woof format .

# 检查格式化（CI 模式）
woof format . --check

# 查看所有规则
woof rules

# 使用指定线程数
woof check . --threads 8

# JSON 输出
woof check . --format json

# 初始化配置
woof init
```

---

## 🎯 Lint 规则体系

### 规则前缀

| 前缀 | 含义 | 数量 |
|------|------|------|
| **E** | 代码风格与语法错误 | 20 |
| **F** | 逻辑错误与运行时问题 | 18 |
| **B** | 代码质量与反模式 | 15 |
| **I** | 导入排序与分组 | 8 |
| **SA** | Staticcheck 核心规则 | 23 |
| **GEN** | Go 1.18+ 泛型检查 | 3 |
| **FUZZ** | Fuzzing 测试规范 | 3 |
| **WS** | Workspace 配置 | 4 |
| **C** | 并发问题检测 | 5 |
| **SEC** | 安全问题 | 5 |
| ... | 更多 | **116+** |

### 示例规则

```go
// SA1019: 使用了已弃用的函数
ioutil.ReadFile("file.txt")  // 应使用 os.ReadFile

// SA5000: 向 nil map 赋值
var m map[string]int
m["key"] = 1  // panic!

// SA2000: 错误的 WaitGroup 使用
go func() {
    wg.Add(1)  // 应在 goroutine 外调用
    defer wg.Done()
}()

// GEN002: 类型参数遮蔽内置类型
func process[int any](x int) {}  // int 遮蔽了内置类型
```

---

## ⚙️ 配置示例

创建 `woof.toml`：

```toml
[global]
target_go_version = "1.21"
respect_gitignore = true
exclude = ["vendor/", "*.gen.go", "*_test.go"]

[linter]
select = ["E", "F", "SA", "C"]
ignore = ["E101", "SA5008"]

[linter.rules.line-too-long]
enabled = true
severity = "error"
options = { max_length = 120 }

[formatter]
use_tabs = true
tab_width = 4
line_length = 120
simplify = true

[memory]
parser_pool_size = 8
ast_cache_size = 1000
batch_size = 100
```

### Go 版本控制

通过 `target_go_version` 控制升级规则的启用：

| 目标版本 | 启用的升级规则 |
|----------|----------------|
| `1.21` (默认) | 无，适用于 Go 1.21 及以下项目 |
| `1.22` | UP1221-UP1225 (整数 range, math/rand/v2 等) |
| `1.23` | UP122x + UP1231-UP1235 (slices/maps 迭代器, unique 包) |
| `1.24` | UP12xx + UP1241-UP1244 (rand.Text, slog.DiscardHandler 等) |
| `1.25` | 全部 (json/v2, synctest, 容器感知 GOMAXPROCS 等) |

**示例**：为 Go 1.25 项目启用所有升级建议
```toml
[global]
target_go_version = "1.25"
```

---

## 🏗️ 架构亮点

```
┌─────────────────────────────────────────────────────────────┐
│                    Woof 高性能架构                           │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │
│  │ Parser Pool │    │  AST Cache  │    │ Arena Alloc │     │
│  │  (8 items)  │    │ (LRU: 1000) │    │ (per thread)│     │
│  └──────┬──────┘    └──────┬──────┘    └──────┬──────┘     │
│         │                  │                  │             │
│         ▼                  ▼                  ▼             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │         Parallel Linting (Rayon)                    │   │
│  │  ┌─────┐ ┌─────┐ ┌─────┐        ┌─────┐            │   │
│  │  │Worker│ │Worker│ │Worker│  ...  │Worker│            │   │
│  │  │  1   │ │  2   │ │  3   │       │  N   │            │   │
│  │  └──┬──┘ └──┬──┘ └──┬──┘        └──┬──┘            │   │
│  └─────┼───────┼───────┼──────────────┼───────────────┘   │
│        └───────┴───────┴──────┬───────┘                   │
│                               ▼                            │
│                    Lock-Free Result Channel                 │
│                               │                            │
│                        ┌──────┴──────┐                     │
│                        │ Sort & Output │                    │
│                        └─────────────┘                     │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 核心技术

| 技术 | 用途 | 效果 |
|------|------|------|
| **Tree-sitter** | Go 代码解析 | 精确、快速 |
| **Rayon** | 数据并行 | 自动负载均衡 |
| **Bumpalo** | Arena 内存分配 | 减少碎片 20% |
| **Crossbeam** | 无锁通道 | 消除同步瓶颈 |
| **Memmap2** | 零拷贝 I/O | 大文件优化 |

---

## 🔬 基准测试

### 测试方法

```bash
# 克隆测试仓库
git clone https://github.com/kubernetes/kubernetes.git /tmp/k8s
cd /home/mey/woof

# 运行对比测试
./benchmark/run_parallel_comparison.sh /tmp/k8s

# 可视化分析
python3 benchmark/visualize_speedup.py /tmp/k8s
```

### 实际项目测试

| 项目 | 文件数 | 代码行数 | Woof | golangci-lint | 领先 |
|------|--------|----------|------|---------------|------|
| Kubernetes | 6,000+ | 2M+ | 1.2s | ~45s | **37x** |
| etcd | 800+ | 300K | 180ms | ~8s | **44x** |
| Gin | 100+ | 50K | 25ms | ~1.5s | **60x** |

---

## 💡 使用场景

### IDE 集成（2ms 响应）

```
用户保存文件 → woof lint → 显示结果
总延迟: ~5ms (含 IDE 开销)
体验: ✅ 即时反馈，无感知延迟
```

### Git Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit
woof check . --fix
woof format . --check
```

### CI/CD 管道

```yaml
# .github/workflows/lint.yml
- name: Lint with Woof
  uses: GWinfinity/woof-action@v1
  with:
    args: 'check . --format github'
```

### 大规模代码库

```bash
# 256 核服务器
woof check . --threads 64
# 处理 10,000+ 文件仅需数秒
```

---

## 📚 文档

- [API 文档](https://docs.rs/woofmt)
- [配置参考](docs/CONFIGURATION.md)
- [规则目录](docs/RULES_CATALOG.md)
- [性能报告](docs/PARALLEL_SPEEDUP_SUMMARY.md)
- [架构设计](docs/SCALING_ROADMAP.md)

---

## 🤝 贡献

欢迎贡献！请查看 [CONTRIBUTING.md](CONTRIBUTING.md)。

```bash
# 开发环境
git clone https://github.com/GWinfinity/woof.git
cd woof
cargo test
cargo bench
```

---

## 📄 许可证

Apache License 2.0 © GWinfinity

---

**Made with ❤️ and 🦀 Rust**

> *"Woof 让 Go 代码检查快到忘记它存在。"*
