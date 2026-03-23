//! P0 Phase 1: Concurrency Safety Rules
//! 
//! Category A: Goroutine Variable Capture
//! Category B: Channel Safety
//! Category C: Mutex/Lock Safety
//! Category D: Context Safety

use super::{Rule, RuleCategory, RuleMetadata, RulePriority};
use crate::{Diagnostic, Severity};
use tree_sitter::Node;

// ==================== Category A: Goroutine Variable Capture ====================

/// F831: loop-variable-capture - 循环变量在 goroutine 中捕获
/// 
/// 这是 Go 最著名的并发 Bug！在 Go 1.22 之前，循环变量是共享的，
/// 如果在 goroutine 中直接使用，所有 goroutine 都会看到最后一个值。
pub struct LoopVariableCapture;

impl Rule for LoopVariableCapture {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "F831",
            name: "loop-variable-capture",
            description: "循环变量在 goroutine 中捕获，应通过参数传递",
            category: RuleCategory::Concurrency,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 只检查 for 语句
        if node.kind() == "for_statement" || node.kind() == "range_clause" {
            let loop_vars = self.extract_loop_variables(node, source);
            if loop_vars.is_empty() {
                return diagnostics;
            }
            
            // 在循环体内查找 go 语句
            let go_statements = self.find_go_statements(node, source);
            
            for go_stmt in go_statements {
                // 获取 go 语句中的函数字面量
                if let Some(func_lit) = self.get_func_literal(go_stmt, source) {
                    let func_text = &source[func_lit.byte_range()];
                    
                    // 检查是否直接使用了循环变量（而不是作为参数）
                    for var_name in &loop_vars {
                        // 简单检查：函数体使用了变量名，且不在参数列表中
                        if func_text.contains(var_name) {
                            // 检查是否是参数（简单检查）
                            let is_parameter = self.is_variable_parameter(func_lit, var_name, source);
                            
                            if !is_parameter {
                                let pos = go_stmt.start_position();
                                diagnostics.push(Diagnostic {
                                    code: "F831".to_string(),
                                    message: format!(
                                        "循环变量 '{}' 在 goroutine 中捕获。应通过参数传递: go func({} {}) {{ ... }}({})",
                                        var_name, var_name, var_name, var_name
                                    ),
                                    severity: self.default_severity(),
                                    file_path: file_path.to_string(),
                                    line: pos.row + 1,
                                    column: pos.column + 1,
                                    fix: None,
                                });
                                break; // 只报告一次
                            }
                        }
                    }
                }
            }
        }
        
        diagnostics
    }
}

impl LoopVariableCapture {
    /// 提取循环变量名
    fn extract_loop_variables(&self, node: Node, source: &str) -> Vec<String> {
        let mut vars = vec![];
        
        if node.kind() == "range_clause" {
            // range 语句：for k, v := range ...
            if let Some(left) = node.child_by_field_name("left") {
                let mut cursor = left.walk();
                for child in left.children(&mut cursor) {
                    if child.kind() == "identifier" {
                        let name = source[child.byte_range()].to_string();
                        if name != "_" {
                            vars.push(name);
                        }
                    }
                }
            }
        } else if node.kind() == "for_statement" {
            // 传统 for 循环：for i := 0; i < n; i++
            // 提取初始化语句中的变量
            if let Some(init) = node.child_by_field_name("initializer") {
                let init_text = &source[init.byte_range()];
                // 简单提取短变量声明的变量名
                if let Some(pos) = init_text.find(":=") {
                    let left = &init_text[..pos];
                    for part in left.split(',') {
                        let name = part.trim().to_string();
                        if !name.is_empty() && name != "_" {
                            vars.push(name);
                        }
                    }
                }
            }
        }
        
        vars
    }
    
    /// 查找循环体内的所有 go 语句
    fn find_go_statements<'a>(&self, node: Node<'a>, _source: &str) -> Vec<Node<'a>> {
        let mut statements = vec![];
        
        // 获取循环体
        let body = if node.kind() == "range_clause" {
            node.parent()
                .and_then(|parent| parent.child_by_field_name("body"))
        } else {
            node.child_by_field_name("body")
        };
        
        if let Some(body) = body {
            self.collect_go_statements(body, &mut statements);
        }
        
        statements
    }
    
    fn collect_go_statements<'a>(&self, node: Node<'a>, statements: &mut Vec<Node<'a>>) {
        if node.kind() == "go_statement" {
            statements.push(node);
        }
        
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.collect_go_statements(child, statements);
        }
    }
    
    /// 获取 go 语句中的函数字面量
    fn get_func_literal<'a>(&self, go_stmt: Node<'a>, _source: &str) -> Option<Node<'a>> {
        let mut cursor = go_stmt.walk();
        for child in go_stmt.children(&mut cursor) {
            if child.kind() == "func_literal" || child.kind() == "call_expression" {
                // 如果是调用表达式，检查被调用的是不是函数字面量
                if child.kind() == "call_expression" {
                    if let Some(func_node) = child.child_by_field_name("function") {
                        if func_node.kind() == "func_literal" {
                            return Some(func_node);
                        }
                    }
                } else {
                    return Some(child);
                }
            }
        }
        None
    }
    
    /// 检查变量是否是函数的参数
    fn is_variable_parameter<'a>(&self, func_lit: Node<'a>, var_name: &str, source: &str) -> bool {
        if let Some(params) = func_lit.child_by_field_name("parameters") {
            let params_text = &source[params.byte_range()];
            // 简单检查参数列表中是否包含变量名
            // 注意：这是简化版，实际应该精确解析参数名
            for param in params_text.split(',') {
                if param.trim().starts_with(var_name) {
                    return true;
                }
            }
        }
        false
    }
}

/// B003: goroutine-leak - Goroutine 泄露
///
/// 检测那些永远不会退出的 goroutine，可能导致资源耗尽。
pub struct GoroutineLeak;

impl Rule for GoroutineLeak {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "B003",
            name: "goroutine-leak",
            description: "Goroutine 可能没有退出条件，会导致资源泄露",
            category: RuleCategory::Concurrency,
            priority: RulePriority::Required,
            default_severity: Severity::Warning,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        if node.kind() == "go_statement" {
            let _go_text = &source[node.byte_range()];
            
            // 获取函数字面量
            if let Some(func_lit) = self.get_func_literal(node, source) {
                let func_text = &source[func_lit.byte_range()];
                
                // 检查是否有退出条件
                let has_exit_condition = self.has_exit_condition(func_text);
                let has_context_check = self.has_context_check(func_text);
                let has_channel_receive = self.has_channel_receive(func_text);
                let has_select = func_text.contains("select");
                
                // 如果没有明显的退出机制，报告泄露风险
                if !has_exit_condition && !has_context_check && !has_channel_receive && !has_select {
                    // 检查是否是无限循环
                    if self.is_infinite_loop(func_text) {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            code: "B003".to_string(),
                            message: "Goroutine 包含无限循环且没有退出条件，可能导致资源泄露".to_string(),
                            severity: self.default_severity(),
                            file_path: file_path.to_string(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                    }
                }
            }
        }
        
        diagnostics
    }
}

impl GoroutineLeak {
    fn get_func_literal<'a>(&self, go_stmt: Node<'a>, _source: &str) -> Option<Node<'a>> {
        let mut cursor = go_stmt.walk();
        for child in go_stmt.children(&mut cursor) {
            if child.kind() == "func_literal" {
                return Some(child);
            }
            if child.kind() == "call_expression" {
                if let Some(func_node) = child.child_by_field_name("function") {
                    if func_node.kind() == "func_literal" {
                        return Some(func_node);
                    }
                }
            }
        }
        None
    }
    
    /// 检查是否有退出条件（return, panic, os.Exit 等）
    fn has_exit_condition(&self, func_text: &str) -> bool {
        func_text.contains("return") || 
        func_text.contains("panic(") ||
        func_text.contains("os.Exit(")
    }
    
    /// 检查是否有 context 取消检查
    fn has_context_check(&self, func_text: &str) -> bool {
        func_text.contains("ctx.Done()") ||
        func_text.contains("context.Done()") ||
        func_text.contains("<-ctx.Done()")
    }
    
    /// 检查是否有 channel 接收（可能是退出信号）
    fn has_channel_receive(&self, func_text: &str) -> bool {
        func_text.contains("<-") && !func_text.contains("<-")
    }
    
    /// 检查是否是无限循环
    fn is_infinite_loop(&self, func_text: &str) -> bool {
        // 检测 for { ... } 或 for true { ... }
        func_text.contains("for {") || 
        func_text.contains("for true {") ||
        func_text.contains("for ; ; {")
    }
}

// ==================== Category B: Channel Safety ====================

/// C003: send-on-closed-channel - 向已关闭的 channel 发送数据
///
/// 向已关闭的 channel 发送数据会导致 panic。
pub struct SendOnClosedChannel;

impl Rule for SendOnClosedChannel {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "C003",
            name: "send-on-closed-channel",
            description: "向已关闭的 channel 发送数据会导致 panic",
            category: RuleCategory::Concurrency,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 检测 close(ch) 调用
        if node.kind() == "call_expression" {
            let call_text = &source[node.byte_range()];
            
            if call_text.starts_with("close(") {
                // 提取 channel 名
                if let Some(channel_name) = self.extract_channel_name(node, source) {
                    // 在同一块内查找后续的 send 操作
                    if let Some(parent) = node.parent() {
                        let sends = self.find_send_operations_after(node, &channel_name, parent, source);
                        
                        for send in sends {
                            let pos = send.start_position();
                            diagnostics.push(Diagnostic {
                                code: "C003".to_string(),
                                message: format!(
                                    "向已关闭的 channel '{}' 发送数据会导致 panic",
                                    channel_name
                                ),
                                severity: self.default_severity(),
                                file_path: file_path.to_string(),
                                line: pos.row + 1,
                                column: pos.column + 1,
                                fix: None,
                            });
                        }
                    }
                }
            }
        }
        
        diagnostics
    }
}

impl SendOnClosedChannel {
    fn extract_channel_name(&self, close_call: Node, source: &str) -> Option<String> {
        // 从 close(ch) 提取 ch
        if let Some(args) = close_call.child_by_field_name("arguments") {
            let mut cursor = args.walk();
            for child in args.children(&mut cursor) {
                if child.kind() == "identifier" {
                    return Some(source[child.byte_range()].to_string());
                }
            }
        }
        None
    }
    
    fn find_send_operations_after<'a>(
        &self,
        close_node: Node<'a>,
        channel_name: &str,
        parent: Node<'a>,
        source: &str
    ) -> Vec<Node<'a>> {
        let mut sends = vec![];
        let mut found_close = false;
        
        let mut cursor = parent.walk();
        for child in parent.children(&mut cursor) {
            // 找到 close 语句之后
            if child.id() == close_node.id() || self.contains_node(child, close_node) {
                found_close = true;
                continue;
            }
            
            if found_close {
                // 检查是否是向该 channel 的发送操作
                if self.is_send_to_channel(child, channel_name, source) {
                    sends.push(child);
                }
            }
        }
        
        sends
    }
    
    fn contains_node<'a>(&self, parent: Node<'a>, target: Node<'a>) -> bool {
        if parent.id() == target.id() {
            return true;
        }
        let mut cursor = parent.walk();
        for child in parent.children(&mut cursor) {
            if self.contains_node(child, target) {
                return true;
            }
        }
        false
    }
    
    fn is_send_to_channel<'a>(&self, node: Node<'a>, channel_name: &str, source: &str) -> bool {
        let node_text = &source[node.byte_range()];
        // 检查是否是 ch <- value 模式
        node_text.starts_with(&format!("{} <-", channel_name))
    }
}

/// SA1004: suspicious-channel-size - 可疑的 channel 缓冲大小
///
/// 检测可能不合理的 channel 缓冲大小。
pub struct SuspiciousChannelSize;

impl Rule for SuspiciousChannelSize {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "SA1004",
            name: "suspicious-channel-size",
            description: "可疑的 channel 缓冲大小",
            category: RuleCategory::Concurrency,
            priority: RulePriority::Required,
            default_severity: Severity::Info,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        if node.kind() == "call_expression" {
            let call_text = &source[node.byte_range()];
            
            // 检查 make(chan T, size) 调用
            if call_text.contains("make(") && call_text.contains("chan") {
                if let Some(size) = self.extract_channel_size(node, source) {
                    // 检查可疑的大小
                    if size > 1000 {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            code: "SA1004".to_string(),
                            message: format!(
                                "channel 缓冲大小 ({}) 过大，可能导致内存浪费。考虑是否真的需要缓冲 channel",
                                size
                            ),
                            severity: self.default_severity(),
                            file_path: file_path.to_string(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                    } else if size == 0 && call_text.contains(",") {
                        // 明确指定了 0 缓冲，这是可以的
                    }
                }
            }
        }
        
        diagnostics
    }
}

impl SuspiciousChannelSize {
    fn extract_channel_size(&self, node: Node, source: &str) -> Option<i64> {
        // 从 make(chan T, size) 提取 size
        if let Some(args) = node.child_by_field_name("arguments") {
            let args_text = &source[args.byte_range()];
            // 提取第二个参数（缓冲大小）
            let parts: Vec<&str> = args_text.split(',').collect();
            if parts.len() >= 2 {
                let size_str = parts[1].trim().trim_matches(')').trim();
                if let Ok(size) = size_str.parse::<i64>() {
                    return Some(size);
                }
            }
        }
        None
    }
}

/// SA1005: infinite-wait - 无限等待（没有接收者的 channel）
///
/// 检测可能导致永久阻塞的 channel 操作。
pub struct InfiniteWait;

impl Rule for InfiniteWait {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "SA1005",
            name: "infinite-wait",
            description: "channel 接收可能无限等待",
            category: RuleCategory::Concurrency,
            priority: RulePriority::Required,
            default_severity: Severity::Warning,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 检测 <-ch 操作
        if node.kind() == "unary_expression" || node.kind() == "expression_statement" {
            let node_text = &source[node.byte_range()];
            
            // 检查是否是 channel 接收
            if node_text.contains("<-") && !node_text.contains("chan") {
                // 提取 channel 名
                if let Some(channel_name) = self.extract_channel_from_receive(node_text) {
                    // 简单启发式：检查是否有 select 或 timeout
                    if let Some(parent_func) = find_parent_function(&node) {
                        let func_text = &source[parent_func.byte_range()];
                        
                        let has_select = func_text.contains("select");
                        let has_timeout = func_text.contains("time.After") || 
                                         func_text.contains("context.WithTimeout");
                        let has_ok_check = node_text.contains("ok") || 
                                          node_text.contains(",");
                        
                        if !has_select && !has_timeout && !has_ok_check {
                            let pos = node.start_position();
                            diagnostics.push(Diagnostic {
                                code: "SA1005".to_string(),
                                message: format!(
                                    "从 channel '{}' 接收可能无限等待。建议使用 select 配合超时，或检查 ok 值",
                                    channel_name
                                ),
                                severity: self.default_severity(),
                                file_path: file_path.to_string(),
                                line: pos.row + 1,
                                column: pos.column + 1,
                                fix: None,
                            });
                        }
                    }
                }
            }
        }
        
        diagnostics
    }
}

impl InfiniteWait {
    fn extract_channel_from_receive(&self, text: &str) -> Option<String> {
        // 从 "<-ch" 或 "v := <-ch" 提取 ch
        if let Some(pos) = text.find("<-") {
            let after = &text[pos + 2..];
            let channel = after.trim().split_whitespace().next()?;
            return Some(channel.to_string());
        }
        None
    }
}

/// B002: defer-in-loop - 循环中使用 defer
///
/// 在循环中使用 defer 会导致资源延迟释放，可能积累大量未释放资源。
pub struct DeferInLoop;

impl Rule for DeferInLoop {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "B002",
            name: "defer-in-loop",
            description: "循环中使用 defer 可能导致资源延迟释放",
            category: RuleCategory::Concurrency,
            priority: RulePriority::Required,
            default_severity: Severity::Warning,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        if node.kind() == "defer_statement" {
            // 检查是否在循环体内
            if self.is_inside_loop(node, source) {
                let defer_text = &source[node.byte_range()];
                
                // 检查是否是资源释放操作
                if self.is_resource_release(defer_text) {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        code: "B002".to_string(),
                        message: "在循环中使用 defer 会导致资源延迟到函数返回时才释放。建议在循环内直接调用 Close()".to_string(),
                        severity: self.default_severity(),
                        file_path: file_path.to_string(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
            }
        }
        
        diagnostics
    }
}

impl DeferInLoop {
    fn is_inside_loop(&self, node: Node, _source: &str) -> bool {
        let mut current = node;
        while let Some(parent) = current.parent() {
            if parent.kind() == "for_statement" || 
               parent.kind() == "range_clause" {
                return true;
            }
            current = parent;
        }
        false
    }
    
    fn is_resource_release(&self, defer_text: &str) -> bool {
        defer_text.contains("Close()") ||
        defer_text.contains("Unlock()") ||
        defer_text.contains("Release()") ||
        defer_text.contains("Stop()")
    }
}

// ==================== Category C: Mutex/Lock Safety ====================

/// SA2003: defer-lock-order - defer Lock 顺序错误
///
/// defer Lock() 而不是 defer Unlock() 会导致死锁。
pub struct DeferLockOrder;

impl Rule for DeferLockOrder {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "SA2003",
            name: "defer-lock-order",
            description: "defer Lock() 顺序错误，应该是 defer Unlock()",
            category: RuleCategory::Concurrency,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        if node.kind() == "defer_statement" {
            let defer_text = &source[node.byte_range()];
            
            // 检测 defer mu.Lock()
            if defer_text.contains("Lock()") && !defer_text.contains("Unlock()") {
                // 检查前面是否有对应的 Lock
                if let Some(parent) = node.parent() {
                    if self.has_lock_before(node, parent, source) {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            code: "SA2003".to_string(),
                            message: "defer Lock() 应该是 defer Unlock()。当前代码会导致死锁".to_string(),
                            severity: self.default_severity(),
                            file_path: file_path.to_string(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                    }
                }
            }
        }
        
        diagnostics
    }
}

impl DeferLockOrder {
    fn has_lock_before(&self, defer_node: Node, parent: Node, source: &str) -> bool {
        let _found_defer = false;
        let _ = _found_defer; // suppress unused warning
        let defer_line = defer_node.start_position().row;
        
        let mut cursor = parent.walk();
        for child in parent.children(&mut cursor) {
            let child_line = child.start_position().row;
            
            if child_line >= defer_line {
                break;
            }
            
            let child_text = &source[child.byte_range()];
            if child_text.contains("Lock()") && !child_text.contains("defer") {
                return true;
            }
        }
        
        false
    }
}

/// B001: mutex-zero-value - Mutex 零值使用问题
///
/// 某些上下文中使用 Mutex 零值可能有问题。
pub struct MutexZeroValue;

impl Rule for MutexZeroValue {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "B001",
            name: "mutex-zero-value",
            description: "检查 Mutex 零值使用是否安全",
            category: RuleCategory::Concurrency,
            priority: RulePriority::Required,
            default_severity: Severity::Info,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 检查 var mu sync.Mutex 声明
        if node.kind() == "var_declaration" || node.kind() == "field_declaration" {
            let text = &source[node.byte_range()];
            
            if text.contains("sync.Mutex") || text.contains("sync.RWMutex") {
                // 检查是否是嵌入到 struct 中且exported
                if self.is_embedded_exported_field(node, source) {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        code: "B001".to_string(),
                        message: "导出的 struct 中嵌入 sync.Mutex 可能会导致复制问题。考虑使用指针或私有化".to_string(),
                        severity: self.default_severity(),
                        file_path: file_path.to_string(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
            }
        }
        
        diagnostics
    }
}

impl MutexZeroValue {
    fn is_embedded_exported_field(&self, node: Node, source: &str) -> bool {
        // 简化检查：检查是否在导出的 struct 定义中
        if let Some(parent) = node.parent() {
            if parent.kind() == "struct_type" {
                // 检查 struct 名是否导出
                if let Some(grandparent) = parent.parent() {
                    if grandparent.kind() == "type_declaration" {
                        if let Some(name) = grandparent.child_by_field_name("name") {
                            let name_text = &source[name.byte_range()];
                            return name_text.chars().next().unwrap().is_uppercase();
                        }
                    }
                }
            }
        }
        false
    }
}

// ==================== Category D: Context Safety ====================

/// B302: context-cancel-not-called - context.CancelFunc 未被调用
///
/// 创建 context 时返回的 cancel 函数应该被调用，否则会导致资源泄露。
pub struct ContextCancelNotCalled;

impl Rule for ContextCancelNotCalled {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "B302",
            name: "context-cancel-not-called",
            description: "context.CancelFunc 应该被调用，建议 defer cancel()",
            category: RuleCategory::Concurrency,
            priority: RulePriority::Required,
            default_severity: Severity::Warning,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 检测短变量声明：ctx, cancel := context.With...
        if node.kind() == "short_var_declaration" {
            let text = &source[node.byte_range()];
            
            if text.contains("context.WithCancel") || 
               text.contains("context.WithTimeout") ||
               text.contains("context.WithDeadline") {
                
                // 提取 cancel 变量名（通常是第二个变量）
                if let Some(cancel_var) = self.extract_cancel_variable(text) {
                    // 检查函数剩余部分是否调用了 cancel
                    if let Some(parent_func) = find_parent_function(&node) {
                        let func_text = &source[parent_func.byte_range()];
                        
                        let has_defer_cancel = func_text.contains(&format!("defer {}()", cancel_var));
                        let has_direct_cancel = self.has_cancel_call(func_text, &cancel_var);
                        
                        if !has_defer_cancel && !has_direct_cancel {
                            let pos = node.start_position();
                            diagnostics.push(Diagnostic {
                                code: "B302".to_string(),
                                message: format!(
                                    "context.CancelFunc '{}' 应该被调用。建议添加 defer {}()",
                                    cancel_var, cancel_var
                                ),
                                severity: self.default_severity(),
                                file_path: file_path.to_string(),
                                line: pos.row + 1,
                                column: pos.column + 1,
                                fix: None,
                            });
                        }
                    }
                }
            }
        }
        
        diagnostics
    }
}

impl ContextCancelNotCalled {
    fn extract_cancel_variable(&self, text: &str) -> Option<String> {
        // 从 "ctx, cancel := context.With..." 提取 "cancel"
        if let Some(pos) = text.find(":=") {
            let left = &text[..pos];
            let parts: Vec<&str> = left.split(',').collect();
            if parts.len() >= 2 {
                return Some(parts[1].trim().to_string());
            }
        }
        None
    }
    
    fn has_cancel_call(&self, text: &str, var_name: &str) -> bool {
        // 检查是否有直接调用（非 defer）
        for line in text.lines() {
            if line.contains(&format!("{}()", var_name)) && !line.contains("defer") {
                return true;
            }
        }
        false
    }
}

// ==================== Category E: Printf Safety ====================

/// SA5009: printf-format-mismatch - Printf 格式与参数不匹配
///
/// 格式字符串中的动词与提供的参数不匹配会导致运行时 panic。
pub struct PrintfFormatMismatch;

impl Rule for PrintfFormatMismatch {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "SA5009",
            name: "printf-format-mismatch",
            description: "Printf 格式动词与参数数量或类型不匹配",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        if node.kind() == "call_expression" {
            let call_text = &source[node.byte_range()];
            
            // 检查是否是 Printf 类函数
            if self.is_printf_function(call_text) {
                // 提取格式字符串和参数
                if let (Some(format_str), arg_count) = self.extract_format_info(node, source) {
                    // 解析格式动词
                    let verb_count = self.count_format_verbs(&format_str);
                    
                    // 检查数量匹配
                    if verb_count != arg_count {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            code: "SA5009".to_string(),
                            message: format!(
                                "Printf 格式动词数量 ({}) 与参数数量 ({}) 不匹配",
                                verb_count, arg_count
                            ),
                            severity: self.default_severity(),
                            file_path: file_path.to_string(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                    }
                }
            }
        }
        
        diagnostics
    }
}

impl PrintfFormatMismatch {
    fn is_printf_function(&self, text: &str) -> bool {
        text.contains("fmt.Printf") ||
        text.contains("fmt.Sprintf") ||
        text.contains("fmt.Fprintf") ||
        text.contains("log.Printf") ||
        text.contains("log.Fatalf")
    }
    
    fn extract_format_info(&self, node: Node, source: &str) -> (Option<String>, usize) {
        if let Some(args) = node.child_by_field_name("arguments") {
            // 提取第一个参数（格式字符串）
            let mut cursor = args.walk();
            let mut children: Vec<Node> = vec![];
            for child in args.children(&mut cursor) {
                if child.kind() != "(" && child.kind() != ")" && child.kind() != "," {
                    children.push(child);
                }
            }
            
            if !children.is_empty() {
                let format_node = &children[0];
                let format_text = &source[format_node.byte_range()];
                
                // 如果是字符串字面量，提取它
                let format_str = if format_node.kind() == "interpreted_string_literal" {
                    Some(format_text.trim_matches('"').to_string())
                } else if format_node.kind() == "raw_string_literal" {
                    Some(format_text.trim_matches('`').to_string())
                } else {
                    None
                };
                
                // 参数数量（不包括格式字符串）
                let arg_count = if children.len() > 1 { children.len() - 1 } else { 0 };
                
                return (format_str, arg_count);
            }
        }
        (None, 0)
    }
    
    fn count_format_verbs(&self, format_str: &str) -> usize {
        let mut count = 0;
        let chars: Vec<char> = format_str.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            if chars[i] == '%' && i + 1 < chars.len() {
                // 跳过 %%（转义的百分号）
                if chars[i + 1] == '%' {
                    i += 2;
                    continue;
                }
                
                // 跳过 [n] 形式的参数索引
                let mut j = i + 1;
                if chars[j] == '[' {
                    while j < chars.len() && chars[j] != ']' {
                        j += 1;
                    }
                    j += 1; // skip ']'
                }
                
                // 跳过 flags、width、precision
                while j < chars.len() && !self.is_verb_char(chars[j]) {
                    j += 1;
                }
                
                // 检查是否是有效的动词
                if j < chars.len() && self.is_verb_char(chars[j]) {
                    count += 1;
                }
                
                i = j + 1;
            } else {
                i += 1;
            }
        }
        
        count
    }
    
    fn is_verb_char(&self, c: char) -> bool {
        matches!(c, 'v' | 'd' | 'b' | 'o' | 'O' | 'x' | 'X' | 'f' | 'F' | 'g' | 'G' | 
                 'e' | 'E' | 's' | 'q' | 'p' | 't' | 'T' | 'c' | 'U' | 'w')
    }
}

// ==================== Category F: API Misuse ====================

/// SA1029: context-withvalue-key - context.WithValue 键类型不当
///
/// context.WithValue 应该使用自定义类型作为 key，避免冲突。
pub struct ContextWithValueKey;

impl Rule for ContextWithValueKey {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "SA1029",
            name: "context-withvalue-key",
            description: "context.WithValue 应该使用自定义类型作为 key，避免与其他代码冲突",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Info,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        if node.kind() == "call_expression" {
            let call_text = &source[node.byte_range()];
            
            if call_text.contains("context.WithValue") || call_text.contains("WithValue(") {
                // 提取第二个参数（key）
                if let Some(key_type) = self.extract_key_type(node, source) {
                    // 检查是否是基本类型（string, int 等）
                    if self.is_basic_type(&key_type) {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            code: "SA1029".to_string(),
                            message: format!(
                                "context.WithValue 使用基本类型 '{}' 作为 key 可能导致与其他代码冲突。建议定义自定义类型: type contextKey string",
                                key_type
                            ),
                            severity: self.default_severity(),
                            file_path: file_path.to_string(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                    }
                }
            }
        }
        
        diagnostics
    }
}

impl ContextWithValueKey {
    fn extract_key_type(&self, node: Node, source: &str) -> Option<String> {
        // 简化版：从调用中提取第二个参数
        if let Some(args) = node.child_by_field_name("arguments") {
            let args_text = &source[args.byte_range()];
            
            // 简单解析：context.WithValue(ctx, "key", value)
            // 寻找第二个逗号前的内容
            let parts: Vec<&str> = args_text.split(',').collect();
            if parts.len() >= 3 {
                let key = parts[1].trim();
                
                // 检查 key 的类型
                if key.starts_with('"') || key.starts_with("`") {
                    return Some("string".to_string());
                }
                if key.parse::<i64>().is_ok() {
                    return Some("int".to_string());
                }
            }
        }
        None
    }
    
    fn is_basic_type(&self, type_name: &str) -> bool {
        matches!(type_name, "string" | "int" | "int64" | "uint" | "bool")
    }
}

// ==================== Helper Functions ====================

fn find_parent_function<'a>(node: &'a Node<'a>) -> Option<Node<'a>> {
    let mut current = *node;
    
    while let Some(parent) = current.parent() {
        if parent.kind() == "function_declaration" || parent.kind() == "method_declaration" ||
           parent.kind() == "func_literal" {
            return Some(parent);
        }
        current = parent;
    }
    
    None
}

// ==================== Rules Export ====================

pub fn get_p0_concurrency_rules() -> Vec<Box<dyn Rule>> {
    vec![
        // Category A: Goroutine Variable Capture
        Box::new(LoopVariableCapture),
        Box::new(GoroutineLeak),
        
        // Category B: Channel Safety
        Box::new(SendOnClosedChannel),
        Box::new(SuspiciousChannelSize),
        Box::new(InfiniteWait),
        Box::new(DeferInLoop),
        
        // Category C: Mutex/Lock Safety
        Box::new(DeferLockOrder),
        Box::new(MutexZeroValue),
        
        // Category D: Context Safety
        Box::new(ContextCancelNotCalled),
        
        // Category E: Printf Safety
        Box::new(PrintfFormatMismatch),
        
        // Category F: API Misuse
        Box::new(ContextWithValueKey),
    ]
}
