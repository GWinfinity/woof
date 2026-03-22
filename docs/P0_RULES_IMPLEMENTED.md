# P0 核心规则实现报告

## 概述

成功实现了 **23 条 P0 级别核心规则**，涵盖了 staticcheck、并发、泛型、Fuzzing 和 Workspace 的关键错误检测。

## 已实现的规则

### SA1xxx: 标准库 API 使用错误 (5条)

| 代码 | 名称 | 描述 | 严重性 |
|------|------|------|--------|
| SA1000 | invalid-regex | 无效的正则表达式 | 🔴 Error |
| SA1001 | invalid-template | 无效的 text/template 或 html/template | 🔴 Error |
| SA1006 | printf-dynamic-format | Printf 使用动态格式字符串 | 🟡 Warning |
| SA1012 | nil-context | 向函数传递 nil context | 🔴 Error |
| SA1019 | deprecated-function | 使用了已弃用的函数 | 🟡 Warning |

**检测示例：**
```go
// SA1000: 无效的正则表达式
regexp.MustCompile("[a-z")  // 未闭合的 [

// SA1006: 动态格式字符串
fmt.Printf(format)  // 不安全

// SA1012: nil context
doSomething(nil)  // 应使用 context.Background()

// SA1019: 已弃用的函数
ioutil.ReadFile("file.txt")  // 应使用 os.ReadFile
```

### SA2xxx: 并发问题 (3条)

| 代码 | 名称 | 描述 | 严重性 |
|------|------|------|--------|
| SA2000 | sync-waitgroup-add-goroutine | WaitGroup.Add 在 goroutine 中调用 | 🔴 Error |
| SA2001 | empty-critical-section | 空的临界区 | 🟡 Warning |
| SA2002 | t-failnow-goroutine | 在 goroutine 中调用 t.FailNow | 🔴 Error |

**检测示例：**
```go
// SA2000: 错误的 WaitGroup 使用
go func() {
    wg.Add(1)  // 错误：应在 goroutine 外调用
    defer wg.Done()
}()

// SA2001: 空的临界区
mu.Lock()
mu.Unlock()  // 之间没有代码，可能是 defer Unlock()

// SA2002: 在 goroutine 中调用 FailNow
go func() {
    t.FailNow()  // 不被允许
}()
```

### SA5xxx: 运行时问题 (5条)

| 代码 | 名称 | 描述 | 严重性 |
|------|------|------|--------|
| SA5000 | assignment-to-nil-map | 向 nil map 赋值 | 🔴 Error |
| SA5001 | defer-close-before-check | 在检查错误前 defer Close | 🟡 Warning |
| SA5007 | infinite-recursion | 无限递归 | 🔴 Error |
| SA5008 | invalid-struct-tag | 无效的 struct tag | 🔴 Error |
| SA5011 | possible-nil-pointer-dereference | 可能的 nil 指针解引用 | 🔴 Error |

**检测示例：**
```go
// SA5000: 向 nil map 赋值
var m map[string]int
m["key"] = 1  // panic

// SA5007: 无限递归
func infinite(n int) int {
    return infinite(n)  // 无限循环
}

// SA5011: nil 指针解引用
if x == nil {
    println(x.Field)  // 解引用 nil
}
```

### C001: 并发相关 (1条)

| 代码 | 名称 | 描述 | 严重性 |
|------|------|------|--------|
| C001 | bad-lock | 锁的常见误用 | 🔴 Error |

**检测示例：**
```go
// 错误：Mutex 按值传递
func process(mu sync.Mutex) {  // 应该使用 *sync.Mutex
    mu.Lock()
    defer mu.Unlock()
}

// 错误：Lock 后没有 Unlock
func bad() {
    mu.Lock()
    // 缺少 Unlock
}
```

### GENxxx: 泛型规则 (3条)

| 代码 | 名称 | 描述 | 严重性 |
|------|------|------|--------|
| GEN002 | type-parameter-shadow | 类型参数遮蔽外部类型 | 🔴 Error |
| GEN004 | comparable-misuse | comparable 约束使用不当 | 🔴 Error |
| GEN007 | type-inference-fail | 无法推断类型参数 | 🔴 Error |

**检测示例：**
```go
// GEN002: 类型参数名遮蔽内置类型
func process[int any](x int) {}  // int 遮蔽了内置类型

// GEN004: comparable 误用
func equal[T comparable](a, b T) bool {
    return a == b  // 如果 T 是切片会失败
}

// GEN007: 类型推断失败
process()  // 无法推断类型参数
```

### FUZZxxx: Fuzzing 规则 (3条)

| 代码 | 名称 | 描述 | 严重性 |
|------|------|------|--------|
| FUZZ001 | fuzz-test-signature | Fuzzing 函数签名错误 | 🔴 Error |
| FUZZ011 | fuzz-global-state | Fuzzing 使用全局状态 | 🔴 Error |
| FUZZ012 | fuzz-non-deterministic | Fuzzing 非确定性行为 | 🔴 Error |

**检测示例：**
```go
// FUZZ001: 错误的签名
func FuzzSomething(x int) {}  // 应该接受 *testing.F

// FUZZ011: 使用全局状态
var counter int
func FuzzSomething(f *testing.F) {
    f.Fuzz(func(t *testing.T, data []byte) {
        counter++  // 修改全局状态
    })
}

// FUZZ012: 非确定性
f.Fuzz(func(t *testing.T, data []byte) {
    x := time.Now()  // 非确定性
})
```

### WSxxx: Workspace 规则 (4条)

| 代码 | 名称 | 描述 | 严重性 |
|------|------|------|--------|
| WS001 | workspace-go-version | Workspace Go 版本不一致 | 🔴 Error |
| WS002 | workspace-module-path | 模块路径冲突 | 🔴 Error |
| WS103 | workspace-dep-cycle | 模块间循环依赖 | 🔴 Error |
| WS105 | workspace-dep-security | 依赖存在安全漏洞 | 🔴 Error |

**检测示例：**
```go
// go.work 文件
// WS001: Go 版本过旧
go 1.16  // 应使用 1.18+

// go.mod 文件
// WS002: 大写字母
module EXAMPLE.COM/MODULE  // 不应有大写字母

// WS103: 循环依赖
replace example.com/my-module => ./
```

## 文件结构

```
woof/src/rules/
├── p0_critical.rs       # P0 规则实现 (570+ 行)
├── tests/
│   └── p0_critical_test.rs  # 测试用例
└── mod.rs               # 注册 P0 规则
```

## 集成方式

P0 规则已自动注册到规则系统中：

```rust
// src/rules/mod.rs
pub fn get_all_rules() -> Vec<Box<dyn Rule>> {
    let mut rules: Vec<Box<dyn Rule>> = vec![];
    
    // P0: 核心关键规则 (首先加载，确保优先检查)
    rules.extend(p0_critical::get_p0_rules());
    
    // 其他规则...
    rules
}
```

## 测试

运行 P0 规则测试：

```bash
cd woof
cargo test --lib p0
```

## 使用示例

```bash
# 编译 woof
cargo build --release

# 检查代码
./target/release/woof check ./test_code.go

# 示例输出：
# test_code.go:5:5: SA1019: ioutil.ReadFile 已弃用，请使用 os.ReadFile
# test_code.go:8:2: SA5000: 可能向 nil map 赋值
# test_code.go:12:5: GEN002: 类型参数 'int' 遮蔽了内置类型
```

## 规则优先级

P0 规则具有以下特点：
1. **高严重性**：大部分是 Error 级别
2. **高准确性**：检测明确的问题
3. **低误报率**：避免干扰正常代码

## 后续扩展

建议后续实现的规则：
- SA1002-SA1005, SA1007-SA1011: 更多标准库 API 检查
- SA2003: defer Lock 错误
- SA5002-SA5006, SA5009-SA5010: 更多运行时问题
- GEN001, GEN003, GEN005: 更多泛型检查
- FUZZ002-FUZZ010: 更多 Fuzzing 检查
- WS003-WS010, WS101-WS105: 更多 Workspace 检查

## 总结

通过实现这 **23 条 P0 规则**，woof 现在能够检测：
- ✅ 常见的 API 使用错误
- ✅ 危险的并发模式
- ✅ 运行时 panic 风险
- ✅ Go 1.18+ 泛型问题
- ✅ Fuzzing 测试规范
- ✅ Workspace 配置错误

这为 woof 提供了坚实的错误检测基础，帮助开发者在编码阶段就发现并修复关键问题。
