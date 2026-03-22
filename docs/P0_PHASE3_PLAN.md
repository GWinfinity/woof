# P0 Phase 3 实现计划

## 目标

实现 **50+ 条 P0 核心规则**，完善类型系统和网络地址验证，并新增高价值规则。

---

## Phase 3 任务清单

### 任务 1: 完善现有规则 (2条)

| 规则 | 改进内容 | 优先级 |
|------|----------|--------|
| **SA5010** | 增强 ImpossibleTypeAssertion，添加常见不可能类型断言检测 | 高 |
| **SA1020** | 增强 InvalidNetworkAddress，添加完整 IP 格式验证 | 高 |

### 任务 2: 新增高价值规则 (2-3条)

| 规则 | 名称 | 描述 | 严重性 | 类型 |
|------|------|------|--------|------|
| **SA9001** | defer-in-range-loop | range 循环中的 defer | Warning | 并发/资源 |
| **SA9002** | non-ascii-identifier | 非 ASCII 标识符 | Info | 风格 |
| **SA1024** | string-offset-copy | 错误的 string 切割 | Error | 运行时 |

---

## 实现详情

### SA5010: ImpossibleTypeAssertion (增强版)

```rust
// 检测以下不可能类型断言:
// 1. nil.(ConcreteType) - nil 接口断言
// 2. *T.(T) - 指针断言值类型
// 3. []byte.(string) - slice 断言 string
// 4. 明显的类型不匹配
```

### SA1020: InvalidNetworkAddress (增强版)

```rust
// 增强验证:
// 1. IPv4 格式: x.x.x.x (0-255)
// 2. IPv6 格式: [xxxx:xxxx::xxxx]:port
// 3. 主机名合法性
// 4. 端口范围: 1-65535
```

### SA9001: DeferInRangeLoop

```rust
// 检测 for range 循环中的 defer
// defer 在 range 循环中不会按预期执行
for _, v := range items {
    defer cleanup(v) // 警告: defer 在 range 循环中累积
}
```

### SA9002: NonASCIIIdentifier

```rust
// 检测非 ASCII 标识符
// Go 支持 Unicode 标识符，但建议使用 ASCII
var 变量名 string // 警告: 非 ASCII 标识符
```

### SA1024: StringOffsetCopy

```rust
// 检测可能导致内存泄漏的 string 操作
// string(b) 如果 b 来自大切片，会导致整个底层数组无法 GC
s := string(largeBuffer[:10]) // 警告: 引用大数组
```

---

## 完成后总计

| 阶段 | 规则数 | 累计 |
|------|--------|------|
| P0 原有 | 24条 | 24条 |
| Phase 1 | 11条 | 35条 |
| Phase 2 | 14条 | 49条 |
| **Phase 3** | **3-4条** | **52-53条** |

**超额完成 50 条 P0 核心规则目标！**
