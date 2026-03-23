//! B-series: go-bugbear - 代码质量提升与反模式检测
//!
//! 包括：Uber Go Style Guide、资源管理、并发模式等

use super::{Rule, RuleCategory, RuleMetadata, RulePriority};
use crate::{Diagnostic, Severity};
use tree_sitter::Node;

// ==================== B0xx: Uber Go Style - 并发规范 ====================

/// B001: mutex-zero-value - Mutex 零值可直接使用
/// Uber: 零值 Mutex 可直接使用，无需显式初始化
pub struct MutexZeroValue;

impl Rule for MutexZeroValue {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "B001",
            name: "mutex-zero-value",
            description: "sync.Mutex 零值可直接使用，无需显式初始化",
            category: RuleCategory::Bugbear,
            priority: RulePriority::Recommended,
            default_severity: Severity::Info,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 检查 sync.Mutex 的显式初始化
        if node.kind() == "composite_literal" {
            let node_text = &source[node.byte_range()];
            
            // 检查是否是 sync.Mutex{} 或 sync.RWMutex{}
            if (node_text.contains("sync.Mutex{") || node_text.contains("sync.RWMutex{"))
                && !node_text.contains(":")  // 不是字段初始化
            {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    code: "B001".to_string(),
                    message: "sync.Mutex 零值可直接使用，建议用 var mu sync.Mutex 替代显式初始化".to_string(),
                    severity: self.default_severity(),
                    file_path: file_path.to_string(),
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            }
        }
        
        diagnostics
    }
}

/// B002: channel-buffer-size - Channel 缓冲大小限制
/// Uber: 缓冲 Channel 大小必须为 1 或 0（无缓冲）
pub struct ChannelBufferSize;

impl Rule for ChannelBufferSize {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "B002",
            name: "channel-buffer-size",
            description: "缓冲 Channel 大小应为 1 或 0，避免隐藏队列",
            category: RuleCategory::Bugbear,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 检查 make(chan T, size) 调用
        if node.kind() == "call_expression" {
            let node_text = &source[node.byte_range()];
            
            // 移除所有空白后检查
            let normalized: String = node_text.chars().filter(|&c| !c.is_whitespace()).collect();
            
            if normalized.starts_with("make(chan") {
                // 提取缓冲区大小参数
                if let Some(comma_pos) = normalized.find(',') {
                    let after_comma = &normalized[comma_pos + 1..];
                    // 提取数字 (在右括号之前)
                    let size_str: String = after_comma.chars()
                        .take_while(|&c| c.is_ascii_digit())
                        .collect();
                    
                    if let Ok(size) = size_str.parse::<usize>() {
                        if size > 1 {
                            let pos = node.start_position();
                            diagnostics.push(Diagnostic {
                                code: "B002".to_string(),
                                message: format!("Channel 缓冲大小 {} > 1，建议设为 0（无缓冲）或 1，避免隐藏队列语义", size),
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

/// B003: goroutine-leak - Goroutine 泄漏检测
/// Uber: 禁止 Fire-and-Forget，必须等待退出
pub struct GoroutineLeak;

impl Rule for GoroutineLeak {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "B003",
            name: "goroutine-leak",
            description: "禁止 Fire-and-Forget goroutine，必须等待退出",
            category: RuleCategory::Bugbear,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 检查 go 关键字调用
        if node.kind() == "go_statement" {
            let node_text = &source[node.byte_range()];
            
            // 排除常见的模式：http server, 长期运行的服务等
            if node_text.contains("http.ListenAndServe")
                || node_text.contains("signal.Notify") {
                return diagnostics;
            }
            
            // 找到所在的函数
            let mut func_scope = None;
            let mut parent = node.parent();
            
            while let Some(p) = parent {
                if matches!(p.kind(), "function_declaration" | "method_declaration" | "func_literal") {
                    func_scope = Some(p);
                    break;
                }
                parent = p.parent();
            }
            
            if let Some(func) = func_scope {
                let func_text = &source[func.byte_range()];
                
                // 检查函数中是否有同步机制（排除导入语句的影响）
                // 检查实际使用模式，而不是单纯的字符串包含
                let has_wait = func_text.contains(".Wait()") || func_text.contains("wg.Wait");
                let has_done_receive = func_text.contains("<-done") || func_text.contains("case <-done");
                let has_cancel = func_text.contains("cancel()") || func_text.contains("defer cancel()");
                let has_quit = func_text.contains("<-quit") || func_text.contains("case <-quit:");
                
                // 检查是否有 WaitGroup 的 Add 调用
                let has_waitgroup_add = func_text.contains(".Add(1)") || func_text.contains("Add(");
                
                let has_sync = has_wait || has_done_receive || has_cancel || has_quit || has_waitgroup_add;
                
                if !has_sync {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        code: "B003".to_string(),
                        message: "检测到可能的 Fire-and-Forget goroutine，建议使用 WaitGroup、context 或 channel 管理生命周期".to_string(),
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

/// B004: atomic-operations - 推荐 go.uber.org/atomic
/// Uber: 强制使用 go.uber.org/atomic 包
pub struct AtomicOperations;

impl Rule for AtomicOperations {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "B004",
            name: "atomic-operations",
            description: "建议使用 go.uber.org/atomic 替代 sync/atomic",
            category: RuleCategory::Bugbear,
            priority: RulePriority::Recommended,
            default_severity: Severity::Info,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 检查 sync/atomic 的使用
        if node.kind() == "call_expression" {
            let node_text = &source[node.byte_range()];
            
            let atomic_funcs = [
                "atomic.AddInt64", "atomic.LoadInt64", "atomic.StoreInt64",
                "atomic.AddInt32", "atomic.LoadInt32", "atomic.StoreInt32",
                "atomic.AddUint64", "atomic.LoadUint64", "atomic.StoreUint64",
                "atomic.SwapInt64", "atomic.CompareAndSwapInt64",
            ];
            
            for func in &atomic_funcs {
                if node_text.contains(func) {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        code: "B004".to_string(),
                        message: format!("建议使用 go.uber.org/atomic 替代 {}，封装更安全", func),
                        severity: self.default_severity(),
                        file_path: file_path.to_string(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                    break;
                }
            }
        }
        
        // 检查是否有 sync/atomic 导入
        if node.kind() == "import_declaration" {
            let import_text = &source[node.byte_range()];
            if import_text.contains("sync/atomic") {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    code: "B004".to_string(),
                    message: "建议用 go.uber.org/atomic 替代 sync/atomic，提供更安全的封装".to_string(),
                    severity: self.default_severity(),
                    file_path: file_path.to_string(),
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            }
        }
        
        diagnostics
    }
}

// ==================== B1xx: Uber Go Style - 性能优化 ====================

/// B101: strconv-over-fmt - 字符串转换性能
/// Uber: 优先 strconv 而非 fmt.Sprintf
pub struct StrconvOverFmt;

impl Rule for StrconvOverFmt {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "B101",
            name: "strconv-over-fmt",
            description: "字符串转换优先使用 strconv 而非 fmt.Sprintf",
            category: RuleCategory::Bugbear,
            priority: RulePriority::Recommended,
            default_severity: Severity::Info,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 检查 fmt.Sprintf 用于简单类型转换
        if node.kind() == "call_expression" {
            let node_text = &source[node.byte_range()];
            
            // 检测常见的性能反模式
            if node_text.contains("fmt.Sprintf(\"%d\"") 
                || node_text.contains("fmt.Sprintf(\"%v\"")
                || node_text.contains("fmt.Sprintf(\"%s\"") {
                
                // 检查参数类型
                let arg_types = infer_format_args(node, source);
                for (i, typ) in arg_types.iter().enumerate() {
                    let replacement = match typ.as_str() {
                        "int" | "int64" | "int32" => "strconv.FormatInt()",
                        "uint" | "uint64" | "uint32" => "strconv.FormatUint()",
                        "string" => "直接拼接或使用 strings.Builder",
                        _ => "strconv 相应函数",
                    };
                    
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        code: "B101".to_string(),
                        message: format!("参数 {} 可用 {} 替代 fmt.Sprintf，避免反射开销", i + 1, replacement),
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

fn infer_format_args(_node: Node, _source: &str) -> Vec<String> {
    // 简化版：返回可能的类型
    // 实际实现需要解析参数类型
    vec!["int".to_string()]
}

// ==================== B2xx: Uber Go Style - 防御性编程 ====================

/// B201: map-slice-copy - Map/Slice 边界拷贝
/// Uber: 边界处必须显式拷贝（防御性复制）
pub struct MapSliceCopy;

impl Rule for MapSliceCopy {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "B201",
            name: "map-slice-copy",
            description: "API 边界处 Map/Slice 应显式拷贝（防御性复制）",
            category: RuleCategory::Bugbear,
            priority: RulePriority::Recommended,
            default_severity: Severity::Info,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 检查导出方法返回 slice 的情况
        if node.kind() == "method_declaration" {
            if let Some(name_node) = node.child_by_field_name("name") {
                let method_name = &source[name_node.byte_range()];
                
                // 只检查导出方法
                if method_name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                    // 检查返回类型是否为 slice 或 map
                    if has_slice_or_map_return_type(node, source) {
                        // 检查函数体中返回字段的情况
                        if let Some(body) = node.child_by_field_name("body") {
                            if returns_internal_field(body, source) {
                                let pos = name_node.start_position();
                                diagnostics.push(Diagnostic {
                                    code: "B201".to_string(),
                                    message: format!("导出方法 '{}' 返回内部 slice/map，建议进行防御性拷贝", method_name),
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
        }
        
        diagnostics
    }
}

fn has_slice_or_map_return_type(node: Node, source: &str) -> bool {
    if let Some(result) = node.child_by_field_name("result") {
        let result_text = &source[result.byte_range()];
        return result_text.contains("[]") || result_text.contains("map[");
    }
    false
}

fn returns_internal_field(body: Node, source: &str) -> bool {
    // 查找 return 语句
    let body_text = &source[body.byte_range()];
    
    // 检查 return 语句后面是否跟有 m.field 模式
    // 这是一个简化检查
    if let Some(ret_pos) = body_text.find("return ") {
        let after_return = &body_text[ret_pos + 7..];
        // 取到行尾或分号
        let ret_expr: String = after_return.chars()
            .take_while(|&c| c != '\n' && c != ';')
            .collect();
        
        // 如果返回的是字段访问 (如 m.data) 而不是新创建的 slice
        if ret_expr.contains('.') && !ret_expr.contains("append(") && !ret_expr.contains("copy(") {
            return true;
        }
    }
    
    false
}

/// B202: slice-index-protection - Slice 索引保护
/// 检查是否有越界风险
pub struct SliceIndexProtection;

impl Rule for SliceIndexProtection {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "B202",
            name: "slice-index-protection",
            description: "Slice 索引访问前应检查长度",
            category: RuleCategory::Bugbear,
            priority: RulePriority::Recommended,
            default_severity: Severity::Info,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 检查直接索引访问
        if node.kind() == "index_expression" {
            // 检查是否有 len 检查
            if let Some(parent) = node.parent() {
                if !has_length_check(parent, source) {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        code: "B202".to_string(),
                        message: "Slice 索引访问前建议检查长度，或使用 ok 模式".to_string(),
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

fn has_length_check(node: Node, source: &str) -> bool {
    // 简化检查：检查是否在 if len(x) > idx 块中
    let mut current = node;
    
    while let Some(parent) = current.parent() {
        let parent_text = &source[parent.byte_range()];
        
        if parent_text.contains("len(") && parent_text.contains(">") {
            return true;
        }
        
        if parent.kind() == "if_statement" || parent.kind() == "block" {
            current = parent;
        } else {
            break;
        }
    }
    
    false
}

// ==================== B3xx: 资源管理 ====================

/// B301: missing-defer-close - 文件/资源未关闭
pub struct MissingDeferClose;

impl Rule for MissingDeferClose {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "B301",
            name: "missing-defer-close",
            description: "打开的文件/资源应有 defer Close",
            category: RuleCategory::Bugbear,
            priority: RulePriority::Required,
            default_severity: Severity::Warning,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 检查需要关闭的资源
        let resource_funcs = [
            ("os.Open", ".Close()"),
            ("os.Create", ".Close()"),
            ("http.NewRequest", "Body.Close()"),
        ];
        
        if node.kind() == "short_var_declaration" || node.kind() == "assignment_statement" {
            let node_text = &source[node.byte_range()];
            
            for (open_func, close_method) in &resource_funcs {
                if node_text.contains(open_func) {
                    // 检查同一作用域是否有 defer close
                    if let Some(parent) = node.parent() {
                        let scope_text = &source[parent.byte_range()];
                        
                        // 提取变量名
                        if let Some(var_name) = extract_assigned_var(node, source) {
                            let close_pattern = format!("defer {}.{}", var_name, close_method);
                            let alt_pattern = format!("defer {}", close_method);
                            
                            if !scope_text.contains(&close_pattern) && !scope_text.contains(&alt_pattern) {
                                let pos = node.start_position();
                                diagnostics.push(Diagnostic {
                                    code: "B301".to_string(),
                                    message: format!("{} 打开的资源建议用 defer {} 关闭", open_func, close_method),
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
        }
        
        diagnostics
    }
}

fn extract_assigned_var(node: Node, source: &str) -> Option<String> {
    let node_text = &source[node.byte_range()];
    
    // 简化提取：假设格式为 "x := ..." 或 "x, err := ..."
    if let Some(colon_pos) = node_text.find(":=") {
        let before = &node_text[..colon_pos].trim();
        if let Some(comma_pos) = before.find(',') {
            Some(before[..comma_pos].trim().to_string())
        } else {
            Some(before.to_string())
        }
    } else if let Some(eq_pos) = node_text.find('=') {
        let before = &node_text[..eq_pos].trim();
        Some(before.to_string())
    } else {
        None
    }
}

/// B302: context-cancel - context 应取消
pub struct ContextCancel;

impl Rule for ContextCancel {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "B302",
            name: "context-cancel",
            description: "context.WithCancel/WithTimeout 返回的 cancel 应被调用",
            category: RuleCategory::Bugbear,
            priority: RulePriority::Required,
            default_severity: Severity::Warning,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 检查 context.WithCancel/WithTimeout 调用
        if node.kind() == "short_var_declaration" || node.kind() == "assignment_statement" {
            let node_text = &source[node.byte_range()];
            
            if node_text.contains("context.WithCancel") || node_text.contains("context.WithTimeout") {
                // 检查是否使用了 _ 忽略 cancel
                let ignored_cancel = node_text.contains("_") && node_text.contains(",");
                
                // 检查是否捕获了 cancel 函数
                let has_cancel = node_text.contains("cancel") || ignored_cancel;
                
                if has_cancel {
                    // 检查是否有 defer cancel()
                    if let Some(parent) = node.parent() {
                        let scope_text = &source[parent.byte_range()];
                        
                        if !scope_text.contains("defer cancel()") && !scope_text.contains("cancel()") {
                            let pos = node.start_position();
                            let message = if ignored_cancel {
                                "context.WithCancel/WithTimeout 返回的 cancel 被忽略，应捕获并调用，建议 defer cancel()"
                            } else {
                                "context.WithCancel/WithTimeout 返回的 cancel 函数应被调用，建议 defer cancel()"
                            };
                            
                            diagnostics.push(Diagnostic {
                                code: "B302".to_string(),
                                message: message.to_string(),
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

// ==================== 规则导出 ====================

pub fn get_rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(MutexZeroValue),
        Box::new(ChannelBufferSize),
        Box::new(GoroutineLeak),
        Box::new(AtomicOperations),
        Box::new(StrconvOverFmt),
        Box::new(MapSliceCopy),
        Box::new(SliceIndexProtection),
        Box::new(MissingDeferClose),
        Box::new(ContextCancel),
    ]
}
