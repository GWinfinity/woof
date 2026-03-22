# Woof 生产化路线图

要将 woof 从原型工具提升为生产级代码审查工具，需要满足以下条件：

## 1. 规则覆盖 (Rule Coverage) 📋

### 当前状态
- 10条基础规则
- 覆盖：未使用导入、空代码块、行长度等

### 生产要求
目标：**50+ 条规则**，覆盖以下类别：

```
E类 - 错误 (Errors)
├── E001: unused-import ✅
├── E002: unused-variable ⬜
├── E003: unused-parameter ⬜
├── E004: unreachable-code ⬜
├── E005: shadow-variable ⬜
├── E006: nil-pointer-deref ⬜
├── E007: divide-by-zero ⬜
├── E008: integer-overflow ⬜
├── E009: slice-bounds ⬜
├── E010: map-iteration-copy ⬜
└── ... (15条)

S类 - 风格 (Style)
├── S001: naked-return ✅
├── S002: unchecked-error ✅
├── S003: redundant-slice ✅
├── S004: unused-parameter ✅
├── S005: snake-case ⬜
├── S006: stuttering ⬜
├── S007: var-naming ⬜
├── S008: package-comments ⬜
├── S009: receiver-naming ⬜
├── S010: error-naming ⬜
└── ... (15条)

P类 - 性能 (Performance)
├── P001: slice-preallocation ⬜
├── P002: string-concatenation ⬜
├── P003: unnecessary-allocation ⬜
├── P004: range-loop-copy ⬜
├── P005: sync-pool-usage ⬜
└── ... (10条)

C类 - 并发 (Concurrency)
├── C001: race-condition ⬜
├── C002: mutex-copy ✅ (基础)
├── C003: channel-close ⬜
├── C004: goroutine-leak ⬜
├── C005: defer-in-loop ⬜
├── C006: context-propagation ⬜
└── ... (10条)

S类 - 安全 (Security)
├── SEC001: sql-injection ⬜
├── SEC002: command-injection ⬜
├── SEC003: path-traversal ⬜
├── SEC004: insecure-random ⬜
├── SEC005: hardcoded-credentials ⬜
└── ... (10条)
```

## 2. 类型系统支持 (Type Checking) 🔍

### 问题
当前woof仅使用tree-sitter进行AST解析，**没有类型信息**。

### 生产要求

#### 2.1 集成 go/types
```rust
// 伪代码示例
use go_analysis::{TypeChecker, Package};

pub struct TypeAwareLinter {
    ast: tree_sitter::Tree,
    types: go_analysis::TypeInfo,  // 类型信息
}

impl TypeAwareLinter {
    fn check_nil_pointer(&self, node: Node) -> Vec<Diagnostic> {
        // 需要类型信息来判断是否为nilable类型
        let expr_type = self.types.type_of(node);
        if expr_type.is_nilable() {
            // 检查是否可能nil解引用
        }
    }
}
```

#### 2.2 需要类型信息的规则
- [ ] nil指针解引用检测
- [ ] 类型断言检查
- [ ] 接口实现验证
- [ ] 未使用返回值（按类型过滤）
- [ ] 错误类型检查

#### 2.3 技术方案
选项A：调用 `go/types` 通过CGO
- 优点：准确，标准库
- 缺点：引入CGO，编译复杂

选项B：使用 `go/analysis` 框架
- 优点：社区标准，Pass机制
- 缺点：需要Go运行时

选项C：自研轻量级类型推断
- 优点：纯Rust，保持速度
- 缺点：工作量大，可能不完整

**推荐：选项B**，初期通过go/analysis bridge，长期考虑选项C

## 3. 准确率优化 (Accuracy) 🎯

### 3.1 减少误报 (False Positives)

#### 当前问题示例
```go
// channel_close - 误报
ch := make(chan int)
close(ch)  // 如果ch不用于接收，这是合法的
```

#### 解决方案
- [ ] 实现控制流分析 (CFG)
- [ ] 添加抑制注释支持：`// nolint:woof` 或 `// nolint:E001`
- [ ] 跨文件分析（针对exported符号）

#### 抑制注释实现
```rust
// 在rule检查前检查注释
fn check_with_suppression(&self, node: Node, source: &str) -> Vec<Diagnostic> {
    let line = get_line(source, node.start_byte());
    if has_nolint_comment(source, line, self.code()) {
        return vec![];
    }
    self.check(node, source)
}
```

### 3.2 减少漏报 (False Negatives)

#### 跨过程分析
```go
func helper() error { return nil }

func main() {
    helper()  // 错误被忽略 - 需要跨过程分析检测
}
```

#### 解决方案
- [ ] 构建调用图 (Call Graph)
- [ ] 基本的进程间分析

## 4. 配置系统 (Configuration) ⚙️

### 4.1 配置格式扩展

```toml
# woof.toml - 生产级配置
[global]
target-go-version = "1.21"
respect-gitignore = true
modules-download-mode = "readonly"  # 新增

[linter]
# 规则启用/禁用
select = ["E", "S", "P"]
ignore = ["E101", "S001"]

# 严重性覆盖
[linter.severity]
E001 = "error"      # 将警告提升为错误
E101 = "info"       # 将警告降级为信息

# 规则特定配置
[linter.rules.line-too-long]
max-length = 100
ignore-comments = true
ignore-urls = true

[linter.rules.cyclomatic-complexity]
max-complexity = 10

# 排除模式
[linter.exclude]
patterns = [
    "vendor/",
    "*.gen.go",
    "*_test.go",
]
# 按规则排除
[[linter.exclude.rules]]
rule = "D001"
patterns = ["internal/", "cmd/"]

# 格式化配置
[formatter]
use-tabs = true
tab-width = 4
line-length = 100
simplify = true

# 新增：导入格式化
[formatter.imports]
local-prefixes = ["github.com/myorg"]
section-separator = true
```

### 4.2 golangci-lint 配置兼容
支持读取 `.golangci.yml` 并映射到woof规则：

```yaml
# .golangci.yml - woof应该能读取
linters:
  enable:
    - errcheck
    - gosimple
    - govet
    - staticcheck
  disable:
    - typecheck

linters-settings:
  lll:
    line-length: 120
  
issues:
  exclude-rules:
    - path: _test\.go
      linters:
        - gocyclo
```

## 5. 性能优化 (Performance) ⚡

### 5.1 保持速度的同时增加功能

| 功能 | 目标时间 | 优化策略 |
|------|----------|----------|
| 单文件 (<1000行) | <10ms | AST缓存 |
| 中型项目 (100文件) | <100ms | 并行处理 ✅ |
| 大型项目 (1000文件) | <500ms | 增量检查 |
| 巨型项目 (10000文件) | <2s | 分布式/缓存 |

### 5.2 增量检查
```rust
pub struct IncrementalLinter {
    cache: FileCache,  // 文件哈希缓存
}

impl IncrementalLinter {
    fn lint(&self, files: &[Path]) -> Vec<Diagnostic> {
        let changed_files: Vec<_> = files
            .iter()
            .filter(|f| self.cache.is_changed(f))
            .collect();
        
        // 只检查变更的文件
        self.lint_files(&changed_files)
    }
}
```

### 5.3 内存优化
- 流式解析大文件
- 诊断结果流式输出
- AST内存池复用

## 6. 集成能力 (Integration) 🔌

### 6.1 IDE 集成

#### LSP (Language Server Protocol)
```rust
// 实现LSP服务
pub struct WoofLanguageServer {
    linter: Linter,
    formatter: Formatter,
}

impl LanguageServer for WoofLanguageServer {
    fn on_did_save(&self, params: DidSaveTextDocumentParams) {
        let diagnostics = self.linter.lint_file(&params.text_document.uri);
        self.publish_diagnostics(params.text_document.uri, diagnostics);
    }
}
```

#### 支持的IDE
- [ ] VS Code (插件)
- [ ] GoLand (外部工具)
- [ ] Vim/Neovim (LSP)
- [ ] Emacs (LSP)

### 6.2 CI/CD 集成

#### GitHub Actions
```yaml
# .github/workflows/woof.yml
name: Lint
on: [push, pull_request]
jobs:
  woof:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Run woof
        uses: woof-action@v1
        with:
          version: latest
          args: --format=github
```

#### 输出格式
- [x] 文本 ✅
- [x] JSON ✅
- [x] GitHub Actions ✅
- [ ] Checkstyle XML
- [ ] JUnit XML
- [ ] SARIF (GitHub Security)

### 6.3 Pre-commit Hook
```yaml
# .pre-commit-hooks.yml
- repo: https://github.com/yourusername/woof
  rev: v1.0.0
  hooks:
    - id: woof
      args: ['--fix']
```

## 7. 测试与可靠性 (Testing) ✅

### 7.1 测试覆盖率目标
- 单元测试：>80%
- 集成测试：覆盖所有规则
- 回归测试：真实项目测试

### 7.2 真实项目测试
在以下项目验证woof：
- [ ] Kubernetes
- [ ] Docker
- [ ] Prometheus
- [ ] etcd
- [ ] Grafana

### 7.3 基准测试
持续跟踪性能：
```rust
// benches/linter_bench.rs
fn benchmark_large_project(c: &mut Criterion) {
    c.bench_function("lint_kubernetes", |b| {
        b.iter(|| lint_project("./kubernetes"))
    });
}
```

## 8. 文档与用户体验 (Documentation) 📚

### 8.1 文档要求
- [ ] 完整规则文档（每个规则带示例）
- [ ] 配置选项参考
- [ ] 迁移指南（从golangci-lint）
- [ ] 性能调优指南
- [ ] 贡献者文档

### 8.2 错误信息改进
```
# 当前
test.go:10:5: warning [E001] Unused import

# 改进后
test.go:10:5: warning [E001] Import 'fmt' is unused
  |
8 | import (
9 |     "fmt"      // <- help: Remove this import
10|     "os"
11| )
  |
  = note: 'fmt' was not used in this file
  = help: Run `woof check --fix` to auto-fix
```

## 9. 发布与维护 (Distribution) 📦

### 9.1 分发渠道
- [ ] GitHub Releases (多平台二进制)
- [ ] Homebrew: `brew install woof`
- [ ] AUR (Arch Linux)
- [ ] Scoop (Windows)
- [ ] Docker Image
- [ ] Go Install: `go install github.com/.../cmd/woof`

### 9.2 版本策略
遵循语义化版本 (SemVer)：
- 新增规则 -> Minor版本
- Bug修复 -> Patch版本
- 破坏性变更 -> Major版本

### 9.3 签名与校验
- GPG签名发布包
- Checksum文件
- SLSA合规 (供应链安全)

## 10. 社区与生态 (Community) 🌐

### 10.1 开源治理
- [ ] 明确的开源协议 (MIT/Apache-2.0)
- [ ] 行为准则 (Code of Conduct)
- [ ] 贡献指南 (CONTRIBUTING.md)
- [ ] Issue模板
- [ ] PR审查流程

### 10.2 插件系统（长期）
允许第三方规则：
```rust
// 插件接口
trait PluginRule: Rule {
    fn name(&self) -> &str;
    fn init(&mut self, config: &Config);
}

// 通过WASM加载插件
wasmtime::Module::from_file(&engine, "custom_rule.wasm");
```

## 实施路线图

### Phase 1: MVP+ (当前 -> 3个月)
- [ ] 实现20条核心规则
- [ ] 添加抑制注释支持
- [ ] 改进错误信息
- [ ] 基础IDE支持 (LSP)

### Phase 2: 生产就绪 (3-6个月)
- [ ] 集成 go/analysis
- [ ] 50条规则
- [ ] 完整配置系统
- [ ] CI/CD集成
- [ ] 真实项目测试

### Phase 3: 竞争 (6-12个月)
- [ ] 100+ 规则
- [ ] 高级分析 (SSA)
- [ ] 插件系统
- [ ] 企业支持

## 关键指标 (KPIs)

| 指标 | 当前 | 6个月目标 | 12个月目标 |
|------|------|----------|-----------|
| 规则数量 | 10 | 50 | 100+ |
| 误报率 | 20% | <5% | <2% |
| 大型项目时间 | N/A | <1s | <500ms |
| 测试覆盖率 | 60% | 80% | 90% |
| 真实项目测试 | 0 | 5 | 20+ |

## 结论

woof要达到生产级别，**最关键的是**：
1. **类型系统支持** - 这是准确性的基础
2. **规则覆盖度** - 至少50条核心规则
3. **误报控制** - 通过抑制注释和更智能的分析
4. **生态系统** - IDE、CI/CD集成

保持**速度优势**的同时增加这些功能，是woof的核心竞争力。
