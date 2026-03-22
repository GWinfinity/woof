# P0 Phase 1 完整实现计划

## 目标

实现 **23 条核心并发安全与基础错误规则**，确保 woof 能够检测最常见的生产事故。

---

## Phase 1 完整规则清单（23条）

### Category A: Goroutine 循环变量捕获（3条）- 最高优先级

| 规则 | 名称 | 描述 | 实现复杂度 |
|------|------|------|-----------|
| **F831** | loop-variable-capture | 循环变量在 goroutine 中捕获 | 中等 |
| **SA2002** | t-failnow-goroutine | 在 goroutine 中调用 t.FailNow | 简单 |
| **SA2000** | sync-waitgroup-add-goroutine | WaitGroup.Add 在 goroutine 中调用 | 简单 |

**为什么这3条最优先：**
- F831 是 Go 最著名的并发 Bug
- SA2002 和 SA2000 是并发测试/同步的常见错误
- 都能通过 AST 分析准确检测

---

### Category B: Channel 安全（4条）

| 规则 | 名称 | 描述 | 实现复杂度 |
|------|------|------|-----------|
| **C003** | channel-close-send | 向已关闭的 channel 发送数据 | 中等 |
| **SA1004** | suspicious-channel-size | 可疑的 channel 缓冲大小 | 简单 |
| **SA1005** | infinite-wait | 无限等待（没有接收者的 channel） | 中等 |
| **B003** | goroutine-leak | Goroutine 泄露（无退出条件） | 中等 |

**检测示例：**
```go
// C003: 向已关闭 channel 发送
close(ch)
ch <- value  // panic

// SA1004: 可疑 channel 大小
make(chan int, 1000)  // 可能过大
make(chan int, 10)    // 可能应该无缓冲

// SA1005: 无限等待
<-ch  // ch 可能永远不会关闭

// B003: Goroutine 泄露
go func() {
    for {
        doWork()  // 没有退出条件
    }
}()
```

---

### Category C: Mutex/Lock 安全（5条）

| 规则 | 名称 | 描述 | 实现复杂度 |
|------|------|------|-----------|
| **C001** | mutex-copy | Mutex 按值传递（拷贝） | 简单 ✅ |
| **SA2001** | empty-critical-section | 空的临界区 | 简单 ✅ |
| **SA2003** | defer-lock-order | defer Lock 顺序错误 | 简单 |
| **B001** | mutex-zero-value | Mutex 零值使用问题 | 简单 |
| **B002** | defer-in-loop | 循环中使用 defer | 中等 |

**检测示例：**
```go
// C001: Mutex 拷贝
func process(m sync.Mutex) {}  // 应该是指针

// SA2001: 空临界区
mu.Lock()
mu.Unlock()  // 之间没有代码

// SA2003: defer Lock 顺序错误
mu.Lock()
defer mu.Lock()  // 错误！应该是 Unlock

// B001: Mutex 零值（某些情况下可以，但需要检查）
var mu sync.Mutex  // 通常可以，但在某些上下文中有问题

// B002: 循环中使用 defer
for _, f := range files {
    f, _ := os.Open(f)
    defer f.Close()  // 循环结束才执行，可能积累大量未关闭文件
}
```

---

### Category D: Context 安全（2条）

| 规则 | 名称 | 描述 | 实现复杂度 |
|------|------|------|-----------|
| **B302** | context-cancel-not-called | context.CancelFunc 未被调用 | 中等 |
| **SA1012** | nil-context | 传递 nil context | 简单 ✅ |

**检测示例：**
```go
// B302: cancel 未调用
ctx, cancel := context.WithTimeout(parent, time.Second)
// 缺少 defer cancel()

// SA1012: nil context
doSomething(nil)  // 应该使用 context.Background()
```

---

### Category E: Printf/格式化安全（2条）

| 规则 | 名称 | 描述 | 实现复杂度 |
|------|------|------|-----------|
| **SA5009** | printf-format-mismatch | Printf 格式与参数不匹配 | 中等 |
| **SA1006** | printf-dynamic-format | Printf 动态格式字符串 | 简单 ✅ |

**检测示例：**
```go
// SA5009: 格式不匹配
fmt.Printf("%d", "string")  // %d 期望整数
fmt.Printf("%s %s", "arg")  // 参数不足

// SA1006: 动态格式
fmt.Printf(userInput)  // 安全风险
```

---

### Category F: 运行时错误（4条）

| 规则 | 名称 | 描述 | 实现复杂度 |
|------|------|------|-----------|
| **C003** | send-on-closed-channel | 向已关闭 channel 发送（重复，见Category B） | - |
| **SA5000** | assignment-to-nil-map | 向 nil map 赋值 | 中等 ✅ |
| **SA5007** | infinite-recursion | 无限递归 | 简单 ✅ |
| **SA5011** | nil-pointer-dereference | nil 指针解引用 | 中等 ✅ |

---

### Category G: API 误用（2条）

| 规则 | 名称 | 描述 | 实现复杂度 |
|------|------|------|-----------|
| **SA1019** | deprecated-function | 使用已废弃函数 | 简单 ✅ |
| **SA1029** | context-withvalue-key | context.WithValue 键类型不当 | 简单 |

---

## 实现顺序（按优先级）

```
Week 1: 最高优先级 - Goroutine 安全
├─ F831: loop-variable-capture
├─ SA2002: t-failnow-goroutine
├─ SA2000: sync-waitgroup-add-goroutine
└─ B003: goroutine-leak

Week 2: Channel 安全
├─ C003: channel-close-send
├─ SA1004: suspicious-channel-size
├─ SA1005: infinite-wait
└─ B002: defer-in-loop

Week 3: Mutex/Lock 完整覆盖
├─ SA2003: defer-lock-order
├─ B001: mutex-zero-value
└─ 完善 C001 和 SA2001

Week 4: Context 和 Printf
├─ B302: context-cancel-not-called
├─ SA5009: printf-format-mismatch
└─ SA1029: context-withvalue-key
```

---

## 技术实现要点

### F831: 循环变量捕获检测

```rust
pub struct LoopVariableCapture;

impl Rule for LoopVariableCapture {
    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // 1. 找到 for/range 语句
        if node.kind() == "for_statement" || node.kind() == "range_clause" {
            // 2. 提取循环变量名
            let loop_vars = extract_loop_variables(&node, source);
            
            // 3. 查找循环体内的 go 语句
            let go_statements = find_go_statements(&node);
            
            for go_stmt in go_statements {
                // 4. 检查闭包是否直接使用循环变量（而非参数）
                if closure_uses_variable_directly(&go_stmt, &loop_vars, source) {
                    report_diagnostic("循环变量在 goroutine 中捕获，应通过参数传递");
                }
            }
        }
    }
}
```

### C003: 向已关闭 Channel 发送

```rust
pub struct SendOnClosedChannel;

impl Rule for SendOnClosedChannel {
    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // 1. 检测 close(ch) 调用
        if is_close_call(&node, source) {
            let channel_name = extract_channel_name(&node, source);
            
            // 2. 在同一块内查找后续的 ch <- value
            let subsequent_sends = find_send_operations_after(
                &node, 
                channel_name, 
                source
            );
            
            for send in subsequent_sends {
                report_diagnostic("向已关闭的 channel 发送数据会导致 panic");
            }
        }
    }
}
```

### SA5009: Printf 格式匹配

```rust
pub struct PrintfFormatMismatch;

impl Rule for PrintfFormatMismatch {
    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        if is_printf_call(&node, source) {
            // 1. 提取格式字符串
            let format_str = extract_format_string(&node, source)?;
            
            // 2. 解析格式动词 (%d, %s, etc.)
            let verbs = parse_format_verbs(&format_str);
            
            // 3. 统计实际参数数量
            let arg_count = count_arguments(&node);
            
            // 4. 检查数量匹配
            if verbs.len() != arg_count {
                report_diagnostic(format!(
                    "Printf 格式动词数量 ({}) 与参数数量 ({}) 不匹配",
                    verbs.len(), arg_count
                ));
            }
            
            // 5. 简单类型检查（可选增强）
            for (i, verb) in verbs.iter().enumerate() {
                if let Some(arg) = get_argument(i) {
                    if !type_matches_verb(arg, verb) {
                        report_diagnostic(format!(
                            "第 {} 个参数类型与格式动词 {} 不匹配",
                            i + 1, verb
                        ));
                    }
                }
            }
        }
    }
}
```

### B302: Context Cancel 未调用

```rust
pub struct ContextCancelNotCalled;

impl Rule for ContextCancelNotCalled {
    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // 1. 检测 context.WithCancel/WithTimeout 调用
        if is_context_with_cancel(&node, source) {
            // 2. 提取 cancel 变量名
            let cancel_var = extract_cancel_variable(&node, source)?;
            
            // 3. 检查函数剩余部分
            let func_body = get_function_body(&node)?;
            let body_text = &source[func_body.byte_range()];
            
            // 4. 检查是否有 defer cancel() 或 cancel() 调用
            if !body_text.contains(&format!("defer {}()", cancel_var))
                && !body_text.contains(&format!("{}()", cancel_var)) {
                report_diagnostic("context.CancelFunc 应该被调用，建议 defer cancel()");
            }
        }
    }
}
```

---

## 完成后 P0 规则总数

当前: 23条
Phase 1 新增: 23条（包含并发、channel、mutex、context、printf）
总计: **46条核心规则**

这将使 woof 具备业界领先的并发安全检测能力！
