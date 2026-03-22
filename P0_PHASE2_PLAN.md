# P0 Phase 2 实现计划

## 目标

实现 **15 条核心规则**，覆盖运行时错误、代码结构和 API 误用，使 woof 达到 **50 条 P0 核心规则**。

---

## Phase 2 规则清单（15条）

### Category A: 运行时错误（5条）

| 规则 | 名称 | 描述 | 严重性 | 实现复杂度 |
|------|------|------|--------|-----------|
| **SA5000** | assignment-to-nil-map | 向 nil map 赋值 | 🔴 Error | 中等 |
| **SA5007** | infinite-recursion | 无限递归 | 🔴 Error | 中等 |
| **SA5011** | possible-nil-pointer-dereference | nil 指针解引用 | 🔴 Error | 中等 |
| **SA5002** | infinite-empty-loop | 空的无限循环 | 🟡 Warning | 简单 |
| **F901** | unreachable-code | 不可达代码 | 🟡 Warning | 简单 |

### Category B: 代码结构（4条）

| 规则 | 名称 | 描述 | 严重性 | 实现复杂度 |
|------|------|------|--------|-----------|
| **F903** | missing-return | 缺少 return 语句 | 🔴 Error | 简单 |
| **SA4006** | ineffectual-assignment | 无效赋值 | 🟡 Warning | 中等 |
| **SA4017** | discard-pure-function-result | 丢弃纯函数返回值 | 🟡 Warning | 简单 |
| **SA5010** | impossible-type-assertion | 不可能的类型断言 | 🔴 Error | 中等 |

### Category C: API 误用（4条）

| 规则 | 名称 | 描述 | 严重性 | 实现复杂度 |
|------|------|------|--------|-----------|
| **SA1002** | time-parse-format | time.Parse 格式错误 | 🟡 Warning | 简单 |
| **SA1020** | invalid-network-address | 无效网络地址 | 🟡 Warning | 简单 |
| **SA1007** | http-header-format | HTTP Header 格式错误 | 🟡 Warning | 简单 |
| **SA5008** | invalid-struct-tag | 无效 struct tag | 🔴 Error | 中等 |

### Category D: 资源管理（2条）

| 规则 | 名称 | 描述 | 严重性 | 实现复杂度 |
|------|------|------|--------|-----------|
| **B301** | missing-defer-close | 缺少 defer Close | 🟡 Warning | 中等 |
| **B302-v2** | context-cancel-pattern | context 取消模式检查 | 🟡 Warning | 中等 |

---

## 实现顺序

```
Week 1: 运行时错误
├─ SA5000: nil map 赋值（完善版）
├─ SA5007: 无限递归（完善版）
├─ SA5011: nil 指针解引用（完善版）
└─ SA5002: 空无限循环

Week 2: 代码结构
├─ F901: 不可达代码
├─ F903: 缺少 return
├─ SA4006: 无效赋值
└─ SA4017: 丢弃纯函数结果

Week 3: API 误用
├─ SA1002: time.Parse 格式
├─ SA1020: 无效网络地址
├─ SA1007: HTTP Header 格式
└─ SA5010: 不可能类型断言

Week 4: 资源管理 + 完善
├─ B301: 缺少 defer Close
├─ SA5008: 无效 struct tag
└─ 测试和优化
```

---

## 技术实现要点

### SA5000: nil map 赋值（完善版）

```rust
// 需要类型追踪
detect_nil_map_assignment(node, source) {
    // 1. 找到 var m map[K]V 声明
    // 2. 追踪 m 是否被 make() 初始化
    // 3. 检查 m[key] = value 赋值
}
```

### SA5007: 无限递归（完善版）

```rust
// 需要递归分析
detect_infinite_recursion(func_node, source) {
    // 1. 获取函数名
    // 2. 检查函数体是否直接调用自身（无终止条件）
    // 3. 简单情况：func f() { return f() }
}
```

### SA5011: nil 指针解引用（完善版）

```rust
// 需要基本类型追踪
detect_nil_dereference(node, source) {
    // 1. 检查 x.Field 或 x.Method()
    // 2. 检查 x 是否可能为 nil（有 nil 检查但未返回）
}
```

---

## 完成后总计

| 阶段 | 规则数 | 累计 |
|------|--------|------|
| P0 原有 | 24条 | 24条 |
| Phase 1 | 11条 | 35条 |
| **Phase 2** | **15条** | **50条** |

**50 条 P0 核心规则** 将使 woof 具备业界领先的错误检测能力！
