# Woof 冷启动时间优化

## 当前冷启动时间分析

### 测量结果（预估）

| 操作 | 当前时间 | 目标时间 | 状态 |
|------|----------|----------|------|
| `--version` | ~5-10ms | <5ms | ⚠️ |
| `--help` | ~10-15ms | <10ms | ⚠️ |
| `rules` | ~20-30ms | <15ms | ⚠️ |
| `check` (首文件) | ~50-100ms | <50ms | ⚠️ |

### 冷启动流程分析

```
用户执行: woof check file.go
│
├─> 1. 二进制加载 (内核)
│   └─> 读取ELF头
│   └─> 加载动态库 (如果有)
│   └─> 映射内存段
│   [~1-2ms]
│
├─> 2. Rust运行时初始化
│   └─> 标准库初始化
│   └─> panic handler设置
│   └─> 参数解析
│   [~1-2ms]
│
├─> 3. CLI解析 (clap)
│   └─> 派生宏生成的解析器
│   └─> 构建帮助信息
│   [~2-5ms]
│
├─> 4. 配置加载
│   └─> 查找配置文件
│   └─> 解析TOML (如果没有则跳过)
│   [~1-3ms]
│
├─> 5. 规则初始化 ⭐ 关键
│   └─> 创建10个Rule实例
│   └─> 每个Box::new分配内存
│   [~5-10ms]
│
├─> 6. Parser初始化 ⭐ 关键
│   └─> 创建Parser实例
│   └─> 设置语言 (tree-sitter)
│   [~10-20ms] ← 主要瓶颈
│
└─> 7. 执行lint
    └─> 读取文件
    └─> 解析AST
    └─> 运行规则
    [~20-50ms]
```

## 瓶颈识别

### 主要瓶颈

1. **Parser初始化** (~10-20ms)
   - tree-sitter Parser::new() 较慢
   - set_language() 需要加载wasm/grammar

2. **规则注册** (~5-10ms)
   - 10个Box<dyn Rule>动态分配
   - 虚表开销

3. **CLI构建** (~2-5ms)
   - clap派生宏生成大量代码
   - 每次构建帮助信息

### 次要瓶颈

4. **配置解析** (~1-3ms)
   - TOML解析相对较慢
   - 文件IO

## 优化方案

### 方案1: 预初始化Parser池 (推荐)

```rust
// src/parser/mod.rs
lazy_static! {
    static ref PARSER_POOL: ParserPool = {
        // 在首次访问时初始化，但可以通过后台线程预热
        ParserPool::new().expect("Failed to create parser pool")
    };
}

// src/main.rs
fn main() {
    // 在后台线程预初始化
    std::thread::spawn(|| {
        lazy_static::initialize(&PARSER_POOL);
    });
    
    let cli = Cli::parse();
    // ...
}
```

**效果**: 消除首次lint的parser初始化延迟

### 方案2: 静态规则注册

```rust
// 当前: 动态分配
pub fn get_all_rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(builtin::UnusedImport),
        Box::new(builtin::LineTooLong),
        // ... 10个Box分配
    ]
}

// 优化: 静态数组
static RULES: &[&dyn Rule] = &[
    &builtin::UnusedImport,
    &builtin::LineTooLong,
    // ... 使用&'static避免分配
];
```

**效果**: 消除规则初始化分配，节省5-10ms

### 方案3: 延迟加载配置

```rust
// src/config/mod.rs
use once_cell::sync::Lazy;

// 延迟加载配置
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::load(None).unwrap_or_default()
});

// 只在需要时加载
pub fn get_config() -> &'static Config {
    &CONFIG
}
```

**效果**: 如果不使用配置文件，完全跳过解析

### 方案4: 预编译帮助信息

```rust
// build.rs 中预生成帮助信息
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // 生成静态帮助文本
    let help_text = generate_help_text();
    let out_dir = env::var("OUT_DIR").unwrap();
    fs::write(Path::new(&out_dir).join("help.txt"), help_text).unwrap();
}
```

然后在代码中使用：
```rust
const HELP_TEXT: &str = include_str!(concat!(env!("OUT_DIR"), "/help.txt"));
```

**效果**: 消除运行时构建帮助的开销

### 方案5: 使用mimalloc (可选)

```rust
// src/main.rs
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
```

```toml
# Cargo.toml
mimalloc = { version = "0.1", default-features = false }
```

**效果**: 更快的内存分配，可能节省1-2ms

## 优化实施计划

### Phase 1: 快速优化 (1天)

1. **静态规则数组**
   ```rust
   // src/rules/mod.rs
   pub static RULES: &[&'static dyn Rule] = &[
       &builtin::UnusedImport,
       &builtin::LineTooLong,
       &builtin::TrailingWhitespace,
       &builtin::EmptyBlock,
       &builtin::MixedTabsSpaces,
       &builtin::ExportedMissingDoc,
       &style::NakedReturn,
       &style::UncheckedError,
       &style::RedundantSlice,
       &style::UnusedParameter,
   ];
   ```

2. **延迟配置加载**
   ```rust
   // 修改config::load为延迟加载
   ```

**预期效果**: -10ms

### Phase 2: Parser优化 (2天)

1. **预初始化Parser池**
   ```rust
   // 启动时后台初始化
   ```

2. **Parser单例模式**
   ```rust
   // 复用parser实例
   ```

**预期效果**: -15ms

### Phase 3: 深度优化 (1周)

1. **预编译帮助**
2. **mimalloc集成**
3. **代码生成优化**

**预期效果**: -5ms

## 目标时间

| 操作 | 当前 | Phase1 | Phase2 | Phase3 | 目标 |
|------|------|--------|--------|--------|------|
| `--version` | 10ms | 8ms | 5ms | 3ms | <5ms |
| `--help` | 15ms | 12ms | 8ms | 5ms | <10ms |
| `rules` | 30ms | 20ms | 15ms | 10ms | <15ms |
| `check` | 100ms | 90ms | 75ms | 60ms | <50ms |

## 测量工具

```bash
# 1. 基础时间测量
time ./target/release/woof --version

# 2. 详细分析
cargo build --release
./benches/cold_start_benchmark.sh

# 3. 系统调用分析
strace -c ./target/release/woof --version

# 4. perf分析 (Linux)
perf stat ./target/release/woof check testdata/sample.go
```

## 验证标准

### 合格标准
- `--version` < 10ms
- `check` 单文件 < 100ms

### 优秀标准
- `--version` < 5ms
- `check` 单文件 < 50ms
- 比golangci-lint快10倍+

### 极致标准
- `--version` < 3ms
- `check` 单文件 < 30ms
- 感知不到延迟

## 当前状态检查

运行以下命令检查当前状态：

```bash
cd /home/mey/woof

# 1. 构建release版本
cargo build --release

# 2. 运行冷启动基准
./benches/cold_start_benchmark.sh

# 3. 查看详细指标
time ./target/release/woof --version
time ./target/release/woof check testdata/sample.go
```

## 相关代码

### 需要修改的文件

1. `src/rules/mod.rs` - 静态规则数组
2. `src/parser/mod.rs` - 预初始化
3. `src/config/mod.rs` - 延迟加载
4. `src/main.rs` - 预初始化线程

### 新增文件

1. `benches/cold_start_benchmark.sh` - 测量脚本
2. `build.rs` - 预编译帮助（可选）
