# Woof Linter Rules - 完整整合报告

> 生成时间: $(date -Iseconds)

## 执行摘要

本次爬虫从 Go 社区抓取了 **697 条** linter 规则，用于充实 woof 的规则体系。

### 规则来源分布

| 来源 | 规则数量 | 占比 |
|------|---------|------|
| staticcheck | 270 | 38.7% |
| go-critic | 202 | 29.0% |
| revive | 154 | 22.1% |
| 新特性规则 (泛型/Fuzzing/Workspace) | 71 | 10.2% |
| **总计** | **697** | **100%** |

### 按类别分布

| 类别 | 代码前缀 | 规则数量 | 描述 |
|------|---------|---------|------|
| 逻辑错误 | F | 190 | bugs, correctness issues |
| 风格规范 | S | 242 | style, naming conventions |
| 简化建议 | SIM | 130 | code simplifications |
| 代码质量 | B | 16 | complexity, quality metrics |
| 性能优化 | P | 32 | performance improvements |
| 并发问题 | C | 12 | concurrency issues |
| 文档规范 | D | 4 | documentation standards |
| 新特性规则 | GEN/FUZZ/WS/UP | 71 | Go 1.18+ 新特性 |

---

## 第一部分: 社区 Linter 规则 (626条)

来自 staticcheck、go-critic、revive 等成熟 linter 的规则。

### StaticCheck 规则 (270条)

StaticCheck 是 Go 最先进的静态分析工具之一。

**主要系列:**
- **SA系列 (bugs)**: 113条 - 检测代码中的 bug
- **S系列 (simplifications)**: 40条 - 代码简化建议
- **ST系列 (style)**: 23条 - 风格检查

**高优先级规则示例:**

| 代码 | 名称 | 描述 | 严重性 |
|------|------|------|--------|
| SA5000 | assignment-to-nil-map | 向 nil map 赋值 | 🔴 Error |
| SA5011 | possible-nil-pointer-dereference | 可能的 nil 指针解引用 | 🔴 Error |
| SA2000 | sync-waitgroup-add-goroutine | WaitGroup.Add 在 goroutine 中调用 | 🔴 Error |
| SA4017 | discard-return-no-side-effects | 丢弃无副作用函数的返回值 | 🟡 Warning |
| SA6000 | regexp-match-in-loop | 循环中使用 regexp.Match | 🟡 Warning |

### Go-Critic 规则 (202条)

Go-critic 提供了大量代码检查，涵盖性能、风格和潜在问题。

**规则分类:**
- **性能相关**: 9条 - 如 prealloc, rangeValCopy
- **Bug 检测**: 23条 - 如 badLock, dupArg
- **风格检查**: 170条 - 代码风格一致性

**高优先级规则示例:**

| 代码 | 名称 | 描述 | 分类 |
|------|------|------|------|
| CRITIC_BADLOCK | badLock | 锁的常见误用 | 🔴 Bug |
| CRITIC_DUPARG | dupArg | 可疑的重复参数 | 🔴 Bug |
| CRITIC_DEFERINLOOP | deferInLoop | 循环中使用 defer | 🔴 Bug |
| CRITIC_HUGEPARAM | hugeParam | 过大的参数 | 🟡 Performance |
| CRITIC_RANGEVALCOPY | rangeValCopy | range 循环中的值拷贝 | 🟡 Performance |

### Revive 规则 (154条)

Revive 是 golint 的增强替代品，提供可配置的风格检查。

**主要规则:**
- 复杂度检查 (cyclomatic, cognitive-complexity)
- 命名规范 (var-naming, receiver-naming)
- 最佳实践 (error-naming, context-as-argument)

**高优先级规则示例:**

| 代码 | 名称 | 描述 |
|------|------|------|
| REVIVE_DATARACE | datarace | 潜在的数据竞争 |
| REVIVE_UNHANDLED_ERROR | unhandled-error | 未处理的错误 |
| REVIVE_WAITGROUP_BY_VALUE | waitgroup-by-value | WaitGroup 按值传递 |
| REVIVE_CYCLOMATIC | cyclomatic | 圈复杂度过高 |

---

## 第二部分: Go 新特性规则 (71条)

专门针对 Go 1.18+ 引入的新特性。

### 泛型规则 (17条)

针对 Go 1.18 引入的泛型特性。

| 代码 | 名称 | 描述 | 级别 |
|------|------|------|------|
| GEN001 | unused-type-parameter | 未使用的类型参数 | 🟡 |
| GEN002 | type-parameter-shadow | 类型参数遮蔽外部类型 | 🔴 |
| GEN004 | comparable-misuse | comparable 约束误用 | 🔴 |
| GEN007 | type-inference-fail | 无法推断类型参数 | 🔴 |
| GEN010 | generic-naming | 类型参数命名不规范 | 🟡 |
| GEN101 | generic-boxing | 泛型装箱性能影响 | 🔵 |

### Fuzzing 规则 (16条)

针对 Go 1.18 引入的 Fuzzing 测试。

| 代码 | 名称 | 描述 | 级别 |
|------|------|------|------|
| FUZZ001 | fuzz-test-signature | Fuzzing 函数签名错误 | 🔴 |
| FUZZ011 | fuzz-global-state | Fuzzing 使用全局状态 | 🔴 |
| FUZZ012 | fuzz-non-deterministic | Fuzzing 非确定性行为 | 🔴 |
| FUZZ009 | fuzz-timeout | 应设置 Fuzzing 超时 | 🟡 |
| FUZZ102 | fuzz-expensive-op | Fuzzing 中有昂贵操作 | 🟡 |

### Workspace 规则 (19条)

针对 Go 1.18 引入的 Workspace 多模块开发。

| 代码 | 名称 | 描述 | 级别 |
|------|------|------|------|
| WS001 | workspace-go-version | Workspace Go 版本不一致 | 🔴 |
| WS002 | workspace-module-path | 模块路径冲突 | 🔴 |
| WS103 | workspace-dep-cycle | 模块间循环依赖 | 🔴 |
| WS105 | workspace-dep-security | 依赖存在安全漏洞 | 🔴 |
| WS201 | gowork-format | go.work 格式错误 | 🔴 |

### Go 版本升级规则 (19条)

针对新版本 Go 的最佳实践。

**Go 1.20+:**
- UP1202: 使用 errors.Join
- UP1204: 使用 maps.Clone
- UP1206: 使用 atomic.Int64 等类型

**Go 1.21+:**
- UP1211-UP1214: slog 包最佳实践
- UP1215-UP1216: slices 包增强

**Go 1.22+:**
- UP1221: 循环变量语义修复
- UP1222: 整数 range
- UP1224: HTTP 路由新模式

---

## 实施路线图

### 阶段 1: 核心规则 (P0) - 建议立即实现

约 **150 条** 核心规则，包括:

**错误检测 (F-series):**
- SA1000-SA1030 (API 使用错误)
- SA5000-SA5012 (运行时问题)
- GEN002, GEN004, GEN007 (泛型错误)
- FUZZ001, FUZZ011, FUZZ012 (Fuzzing 错误)
- WS001, WS002, WS103, WS105 (Workspace 错误)

**并发问题 (C-series):**
- SA2000-SA2003 (并发模式)
- CRITIC_BADLOCK
- REVIVE_DATARACE

### 阶段 2: 推荐规则 (P1) - 建议 3 个月内实现

约 **300 条** 推荐规则，包括:

- staticcheck S系列和ST系列
- go-critic 主要规则
- revive 主要规则
- 泛型和 Fuzzing 最佳实践

### 阶段 3: 完整规则 (P2) - 建议 6 个月内实现

剩余的 **247 条** 规则，包括:

- 风格细节规则
- 性能优化建议
- 高级检查

---

## 建议的优先级矩阵

| 规则来源 | P0 (必须) | P1 (推荐) | P2 (可选) |
|---------|----------|----------|----------|
| staticcheck SA | 100条 | 100条 | 70条 |
| staticcheck S/ST | 10条 | 30条 | 23条 |
| go-critic | 20条 | 100条 | 82条 |
| revive | 15条 | 70条 | 69条 |
| 泛型规则 | 5条 | 8条 | 4条 |
| Fuzzing 规则 | 5条 | 6条 | 5条 |
| Workspace 规则 | 8条 | 6条 | 5条 |
| Go 升级规则 | 5条 | 10条 | 4条 |

---

## 使用方式

### 生成文件

爬虫生成了以下文件:

```
scripts/output/
├── CRAWLER_REPORT.md          # 社区 linter 规则报告
├── NEW_FEATURE_RULES.md       # 新特性规则文档
├── INTEGRATED_REPORT.md       # 本整合报告
├── linters.json               # 44个 linter 信息
├── rules.json                 # 626条社区规则
├── woof_rules.json           # woof 格式规则
├── new_feature_rules.json    # 71条新特性规则
├── generated_rules.rs        # 社区规则 Rust 代码模板
└── new_feature_rules.rs      # 新特性规则 Rust 代码模板
```

### 集成到 woof

1. **复制规则文件**: 将 `.rs` 文件复制到 `src/rules/` 目录
2. **注册规则**: 在 `src/rules/mod.rs` 中导入并注册新规则
3. **实现检查逻辑**: 根据 TODO 注释实现每个规则的 check 方法
4. **添加测试**: 为每条规则添加单元测试

### 运行爬虫

```bash
# 抓取社区 linter 规则
cd woof
python3 scripts/linter_crawler.py

# 生成新特性规则
python3 scripts/linter_crawler_v2.py
```

---

## 参考链接

- [golangci-lint](https://golangci-lint.run/)
- [staticcheck](https://staticcheck.io/)
- [go-critic](https://go-critic.com/)
- [revive](https://github.com/mgechev/revive)
- [Go 1.18 Release Notes](https://go.dev/doc/go1.18)
- [Go 1.20 Release Notes](https://go.dev/doc/go1.20)
- [Go 1.21 Release Notes](https://go.dev/doc/go1.21)
- [Go 1.22 Release Notes](https://go.dev/doc/go1.22)

---

*本报告由 Go Linter Crawler 自动生成*
