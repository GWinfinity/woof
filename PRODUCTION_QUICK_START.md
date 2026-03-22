# Woof 生产化快速参考

## 一句话总结

要让woof达到生产级别，关键是：**抑制注释 + 20条规则 + 类型检查 + IDE集成**。

## 关键差距（按重要性排序）

```
🔴 阻塞级（没有这些无法生产使用）
├── 抑制注释 (// nolint)           - 1天
├── 修复重复诊断                   - 1天
└── 准确率调优                     - 3天

🟠 核心级（达到基本可用）
├── 20条核心规则                   - 2周
├── 错误信息改进                   - 3天
├── CI输出格式 (SARIF)            - 3天
└── 配置文件完善                   - 2天

🟡 重要级（达到有竞争力）
├── 类型检查集成                   - 3周
├── LSP/IDE支持                   - 2周
├── 安装包分发                     - 1周
└── 50条规则                       - 4周

🟢 优化级（达到行业领先）
├── 增量检查                       - 2周
├── 100+规则                       - 8周
├── SSA分析                        - 6周
└── 插件系统                       - 8周
```

## 最低生产配置（6周计划）

### 第1周：阻塞修复
```bash
# 实现抑制注释
// nolint          # 跳过所有
// nolint:E001     # 跳过E001
// nolint:E001,E002 # 跳过多个
```

### 第2-3周：核心规则（20条）
```rust
// 优先级最高的规则
E001: unused-import        ✅ 已有
E002: unused-variable      ⬜ 新增
E003: unreachable-code     ⬜ 新增
E004: shadow-variable      ⬜ 新增
E005: nil-pointer-deref    ⬜ 新增

S001: naked-return         ✅ 已有
S002: unchecked-error      ✅ 已有
S003: redundant-slice      ✅ 已有
S004: snake-case           ⬜ 新增
S005: stuttering           ⬜ 新增

// ... 再加10条
```

### 第4周：输出与集成
```bash
# 支持的输出格式
woof check .                    # 文本
woof check . --format json     # JSON
woof check . --format github   # GitHub Actions
woof check . --format sarif    # SARIF (新增)
```

### 第5-6周：分发
- GitHub Releases (Linux/Mac/Windows)
- Homebrew: `brew install woof`
- GitHub Action

## 类型检查方案对比

| 方案 | 难度 | 准确性 | 维护成本 | 推荐度 |
|------|------|--------|----------|--------|
| A: CGO + go/types | 高 | 100% | 高 | ⭐⭐ |
| B: go/analysis Bridge | 中 | 95% | 中 | ⭐⭐⭐⭐ |
| C: 纯Rust启发式 | 高 | 70% | 低 | ⭐⭐⭐ |

**推荐方案B**：使用外部Go进程提供类型信息，通过JSON通信。

```rust
// Rust端
let types = go_analyzer.analyze_file("main.go");
// Go端 (单独进程)
// 使用go/analysis分析，输出JSON
```

## 规则实现模板

```rust
// 5分钟添加一条新规则
use woof::rules::*;

pub struct MyRule;

impl Rule for MyRule {
    fn code(&self) -> &'static str { "E999" }
    fn name(&self) -> &'static str { "my-rule" }
    fn description(&self) -> &'static str { "What this rule checks" }
    
    fn check(&self, node: Node, source: &str, file: &str) -> Vec<Diagnostic> {
        if node.kind() != "target_node_kind" {
            return vec![];
        }
        
        // 你的检查逻辑
        if should_report(node, source) {
            vec![Diagnostic {
                file_path: file.to_string(),
                line: get_line(source, node.start_byte()),
                column: get_column(source, node.start_byte()),
                message: "Issue description".to_string(),
                code: self.code().to_string(),
                severity: Severity::Warning,
                fix: None,
            }]
        } else {
            vec![]
        }
    }
}
```

## 与golangci-lint的对比

| 维度 | woof (当前) | woof (6周后) | golangci-lint |
|------|-------------|--------------|---------------|
| 速度 | 1000x | 500x | 1x (基准) |
| 规则数 | 10 | 20 | 50+ |
| 类型检查 | ❌ | ⚠️ 基础 | ✅ 完整 |
| IDE支持 | ❌ | ✅ 基础 | ✅ 完整 |
| CI/CD | ⚠️ | ✅ | ✅ |
| 配置 | 基础 | 完整 | 复杂 |
| 准确性 | 70% | 85% | 90%+ |

## 快速决策树

```
是否应该用woof替换golangci-lint？
│
├─> 需要最快的反馈速度？
│   └─> 是 → woof ✅
│
├─> 只需要基础lint规则？
│   └─> 是 → woof ✅
│
├─> 资源受限(内存/CPU)？
│   └─> 是 → woof ✅
│
├─> 需要类型感知的高级检查？
│   └─> 是 → golangci-lint (暂时)
│
├─> 团队需要严格的代码标准？
│   └─> 是 → golangci-lint (暂时)
│
└─> 需要定制化规则？
    └─> 是 → golangci-lint (暂时)
```

## 使用场景推荐

### ✅ woof适合
- 大型代码库的快速检查
- 开发时实时反馈 (IDE集成后)
- 内存/CPU受限环境
- 基础lint规则即可满足

### ❌ woof不适合（当前）
- 需要严格的类型检查
- 安全关键代码审计
- 复杂的定制化需求
- 需要20+不同lint工具

## 启动命令

```bash
# 开发（快速反馈）
woof check . --watch

# CI（严格检查）
woof check . --format github --exit-code-on-issue

# 修复
woof check . --fix

# 格式化
woof format .

# 初始化配置
woof init
```

## 配置文件示例 (woof.toml)

```toml
[global]
target-go-version = "1.21"

[linter]
select = ["E", "S"]      # 启用规则类别
ignore = ["E101"]        # 禁用特定规则

[linter.severity]
E001 = "error"           # 提升严重性
E101 = "info"            # 降低严重性

[linter.rules.line-too-long]
max-length = 100
ignore-comments = true

[formatter]
use-tabs = true
line-length = 100
```

## 关键成功指标 (KPI)

| 指标 | 当前 | 6周目标 |
|------|------|---------|
| 规则数量 | 10 | 20 |
| 误报率 | 30% | <10% |
| 单文件时间 | 4ms | <10ms |
| 项目时间(100文件) | 10ms | <100ms |

## 资源投入

**最低配置**（达到可用状态）：
- 1名Rust开发者
- 6周全职时间

**推荐配置**（达到有竞争力）：
- 2名Rust开发者
- 1名Go顾问（类型检查）
- 12周全职时间

## 下一步行动

如果你现在就想开始使用woof：

1. **今天就做**：
   ```bash
   cargo build --release
   ./target/release/woof check your-project/
   ```

2. **本周内**：
   - 实现抑制注释
   - 修复重复诊断问题

3. **本月内**：
   - 添加10条核心规则
   - 添加SARIF输出

4. **持续关注**：
   - 类型检查集成进展
   - LSP服务器开发

---

**记住**：woof的核心优势是**速度**。保持这个优势的同时逐步增加功能，是生产化的关键。
