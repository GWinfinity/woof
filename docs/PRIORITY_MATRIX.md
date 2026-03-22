# Woof 生产化优先级矩阵

基于**影响力**和**实施难度**的优先级分析。

## 优先级矩阵图

```
影响力
  ^
  │
  │  ┌─────────────┬─────────────┐
  │  │   Phase 2   │   Phase 1   │
  │  │             │  (立即)     │
高│  │ • 类型检查   │ • 抑制注释  │
  │  │ • IDE LSP   │ • 20条规则  │
  │  │ • 50条规则  │ • 错误改进  │
  │  │             │ • CI输出    │
  │  ├─────────────┼─────────────┤
  │  │   Phase 3   │  低优先级   │
  │  │             │             │
  │  │ • 100+规则  │ • 插件系统  │
  │  │ • SSA分析   │ • WASM支持  │
  │  │ • 企业功能  │ • 分布式    │
  │  │             │             │
  │  └─────────────┴─────────────┘
  │         难                    易
  └────────────────────────────────────> 实施难度
```

## P0 - 阻塞性问题（必须解决）

| 项目 | 问题 | 解决方案 | 预估时间 |
|------|------|----------|----------|
| 抑制注释 | 无法跳过误报 | 支持 `// nolint` | 2天 |
| 重复诊断 | E101等重复报告 | 添加file-level rule过滤 | 1天 |
| 规则精度 | 误报率高 | 添加过滤逻辑 | 1周 |

## P1 - 核心功能（6周内完成）

### 1. 抑制注释系统 ⭐⭐⭐⭐⭐
```rust
// 实现优先级：最高
// 原因：生产环境必须有方式跳过误报

// 支持格式：
// nolint                    - 跳过所有规则
// nolint:E001               - 跳过E001
// nolint:E001,S002          - 跳过多个规则
// nolint:E0*                - 跳过E0xx规则
// nolint-next-line:E001     - 跳过下一行
```

### 2. 规则扩展（20条核心规则）⭐⭐⭐⭐⭐
```rust
// 优先级排序（按检出价值）
1. unused-variable (E002)      - 高价值，易实现
2. unreachable-code (E004)     - 高价值，中等难度
3. shadow-variable (E005)      - 高价值，中等难度
4. var-naming (S007)           - 高价值，易实现
5. snake-case (S005)           - 高价值，易实现
6. stuttering (S006)           - 中等价值，易实现
7. receiver-naming (S009)      - 中等价值，易实现
8. error-naming (S010)         - 中等价值，易实现
9. slice-preallocation (P001)  - 高价值，中等难度
10. string-concatenation (P002) - 中等价值，易实现
11. range-loop-copy (P004)     - 高价值，易实现
12. cyclomatic-complexity      - 高价值，易实现
13. function-length            - 高价值，易实现
14. defer-in-loop (C005)       - 高价值，易实现
15. context-propagation (C006) - 中等价值，中等难度
16. unnecessary-allocation     - 中等价值，中等难度
17. package-comments (S008)    - 低价值，易实现
18. many-parameters            - 中等价值，易实现
19. long-parameter-list        - 中等价值，易实现
20. redundant-type (S026)      - 中等价值，易实现
```

### 3. 错误信息改进 ⭐⭐⭐⭐
```
改进前：
test.go:10:5: warning [E001] Unused import

改进后：
error[E001]: Import 'fmt' is unused
  --> test.go:10:5
   |
8  | import (
9  |     "fmt"
   |     ^^^^^ unused import
10 |     "os"
11 | )
   |
   = help: Remove the import or use it
   = note: Run `woof check --fix` to auto-fix
```

### 4. SARIF输出格式 ⭐⭐⭐⭐
```json
{
  "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
  "version": "2.1.0",
  "runs": [{
    "tool": {
      "driver": {
        "name": "woof",
        "version": "1.0.0"
      }
    },
    "results": [...]
  }]
}
```
## P2 - 重要功能（3个月内）

### 5. 类型检查集成（go/analysis bridge）⭐⭐⭐⭐⭐
```rust
// 架构设计
┌─────────────┐      ┌──────────────┐      ┌─────────────┐
│   Woof      │ <--> │ go/analysis  │ <--> │  go/types   │
│  (Rust)     │      │  (Bridge)    │      │   (Go)      │
└─────────────┘      └──────────────┘      └─────────────┘
     │                      │
     └----------------------┘
          JSON-RPC / gRPC
```

### 6. LSP服务器 ⭐⭐⭐⭐
```rust
// 支持功能
- [ ] textDocument/diagnostic
- [ ] textDocument/formatting
- [ ] textDocument/codeAction (快速修复)
- [ ] workspace/didChangeWatchedFiles
```

### 7. 配置系统增强 ⭐⭐⭐⭐
```toml
# 优先级功能
[linter]
severity-overrides = { E001 = "error" }

[[linter.exclude]]
rule = "D001"
patterns = ["internal/"]
```

## P3 - 优化功能（6个月内）

### 8. 增量检查 ⭐⭐⭐
- 文件哈希缓存
- 只检查变更文件
- 依赖变更检测

### 9. 高级并发分析 ⭐⭐⭐
- 调用图构建
- 数据流分析
- 竞争条件检测

### 10. 性能优化 ⭐⭐⭐
- AST内存池
- 并行解析
- 流式处理

## P4 - 未来规划（12个月内）

### 11. SSA分析 ⭐⭐
- 静态单赋值形式
- 常量传播
- 死代码消除检测

### 12. 插件系统 ⭐⭐
- WASM插件
- 第三方规则
- 自定义检查器

### 13. 企业功能 ⭐
- 集中式配置
- 报告聚合
- 团队统计

## 快速实施检查清单

### Week 1: 阻塞修复
- [ ] 实现抑制注释 `// nolint`
- [ ] 修复重复诊断问题
- [ ] 添加5条简单规则

### Week 2-3: 核心规则
- [ ] 实现15条规则（总计20条）
- [ ] 改进错误信息格式
- [ ] 添加SARIF输出

### Week 4-6: 集成
- [ ] 基础LSP支持
- [ ] GitHub Actions
- [ ] 真实项目测试

### 预估总工作量
| 阶段 | 工作量 | 产出 |
|------|--------|------|
| P0修复 | 1周 | 可用工具 |
| P1核心 | 5周 | 有竞争力 |
| P2重要 | 6周 | 生产就绪 |
| P3优化 | 12周 | 行业领先 |

## 最低生产要求（MVP）

如果资源有限，以下是最低生产要求：

1. **抑制注释** - 必须有
2. **20条规则** - 覆盖最常见的issue
3. **CI输出** - GitHub Actions格式
4. **配置文件** - 基本配置
5. **安装包** - 二进制分发

预计时间：**6周**

这样woof就可以作为golangci-lint的轻量级替代品用于生产环境。
