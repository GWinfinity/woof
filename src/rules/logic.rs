//! F-series: goflakes - 逻辑错误与潜在运行时问题
//!
//! 包括：未使用变量、重定义、未定义变量、控制流问题等

use super::{Rule, RuleCategory, RuleMetadata, RulePriority};
use crate::{Diagnostic, Severity};
use tree_sitter::Node;

// ==================== F4xx: 未使用 ====================

/// F401: unused-import - 未使用的导入
pub struct UnusedImport;

impl Rule for UnusedImport {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "F401",
            name: "unused-import",
            description: "导入的包未被使用",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        if node.kind() == "source_file" {
            // 收集所有导入
            let mut imports: Vec<(String, Node)> = vec![];
            let mut cursor = node.walk();
            
            for child in node.children(&mut cursor) {
                if child.kind() == "import_declaration" {
                    // 检查是否是 blank import (_)
                    let import_text = &source[child.byte_range()];
                    
                    // 提取包名或别名
                    if let Some(path_node) = child.child_by_field_name("path") {
                        let path = source[path_node.byte_range()].trim_matches('"');
                        let pkg_name = path.rfind('/').map(|i| &path[i+1..]).unwrap_or(path);
                        
                        // 检查是否是 _ import（这是有意使用的）
                        if !import_text.contains("_ ") {
                            imports.push((pkg_name.to_string(), child));
                        }
                    }
                }
            }
            
            // 检查文件其余部分是否使用了导入
            let file_text = source.to_string();
            for (pkg_name, import_node) in imports {
                // 简单检查：包名后跟点号
                let pattern = format!("{}.", pkg_name);
                let count = file_text.matches(&pattern).count();
                
                // 检查是否在 import 语句本身（排除自己）
                let import_text = &source[import_node.byte_range()];
                let self_count = import_text.matches(&pattern).count();
                
                if count <= self_count {
                    let pos = import_node.start_position();
                    diagnostics.push(Diagnostic {
                        code: "F401".to_string(),
                        message: format!("导入 '{}' 未被使用", pkg_name),
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

/// F402: unused-variable - 未使用的变量
pub struct UnusedVariable;

impl Rule for UnusedVariable {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "F402",
            name: "unused-variable",
            description: "变量被声明但未被使用",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 检查短变量声明
        if node.kind() == "short_var_declaration" || node.kind() == "var_declaration" {
            let _var_text = &source[node.byte_range()];
            
            // 提取变量名
            if let Some(left_node) = node.child_by_field_name("left") {
                let mut cursor = left_node.walk();
                for child in left_node.children(&mut cursor) {
                    if child.kind() == "identifier" {
                        let var_name = &source[child.byte_range()];
                        
                        // 检查是否是 _
                        if var_name != "_" {
                            // 检查是否在后续被使用（简化版：只在当前块中检查）
                            if let Some(parent_block) = node.parent() {
                                let block_text = &source[parent_block.byte_range()];
                                let node_end = node.end_byte() - parent_block.start_byte();
                                let after_node = &block_text[node_end..];
                                
                                // 简单检查变量名是否出现（不包括声明处）
                                let usage_count = after_node.matches(var_name).count();
                                if usage_count == 0 {
                                    let pos = child.start_position();
                                    diagnostics.push(Diagnostic {
                                        code: "F402".to_string(),
                                        message: format!("变量 '{}' 被声明但未被使用", var_name),
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
        }
        
        diagnostics
    }
}

/// F403: unused-parameter - 未使用的参数
pub struct UnusedParameter;

impl Rule for UnusedParameter {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "F403",
            name: "unused-parameter",
            description: "函数参数未被使用",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        if node.kind() == "function_declaration" || node.kind() == "method_declaration" {
            // 获取参数列表
            if let Some(params) = node.child_by_field_name("parameters") {
                let func_body = node.child_by_field_name("body");
                
                if let Some(body) = func_body {
                    let body_text = &source[body.byte_range()];
                    
                    let mut cursor = params.walk();
                    for param in params.children(&mut cursor) {
                        if param.kind() == "parameter_declaration" {
                            let _param_text = &source[param.byte_range()];
                            
                            // 提取参数名
                            if let Some(name_node) = param.child_by_field_name("name") {
                                let param_name = &source[name_node.byte_range()];
                                
                                // 检查是否在函数体中使用
                                if param_name != "_" && !body_text.contains(param_name) {
                                    let pos = name_node.start_position();
                                    diagnostics.push(Diagnostic {
                                        code: "F403".to_string(),
                                        message: format!("参数 '{}' 未被使用", param_name),
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
        }
        
        diagnostics
    }
}

/// F404: unused-return-value - 未使用的重要返回值
pub struct UnusedReturnValue;

impl Rule for UnusedReturnValue {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "F404",
            name: "unused-return-value",
            description: "重要函数的返回值未被检查",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 检查调用表达式
        if node.kind() == "call_expression" {
            let call_text = &source[node.byte_range()];
            
            // 检查是否忽略错误（常见模式）
            let important_funcs = [
                "json.Marshal", "json.Unmarshal", 
                "io.ReadAll", "os.Open", "os.Create",
                "http.NewRequest", "url.Parse",
            ];
            
            for func in &important_funcs {
                if call_text.contains(func) {
                    // 检查是否在赋值或作为参数使用
                    if let Some(parent) = node.parent() {
                        if parent.kind() == "expression_statement" {
                            let pos = node.start_position();
                            diagnostics.push(Diagnostic {
                                code: "F404".to_string(),
                                message: format!("{} 的返回值应被检查", func),
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

/// F405: unused-label - 未使用的标签
pub struct UnusedLabel;

impl Rule for UnusedLabel {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "F405",
            name: "unused-label",
            description: "标签定义后未被使用",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        if node.kind() == "labeled_statement" {
            // 提取标签名
            let node_text = &source[node.byte_range()];
            if let Some(colon_pos) = node_text.find(':') {
                let label_name = &node_text[..colon_pos].trim();
                
                // 检查是否有 goto 或 break/continue 使用
                if let Some(parent) = node.parent() {
                    let parent_text = &source[parent.byte_range()];
                    let goto_pattern = format!("goto {}", label_name);
                    let break_pattern = format!("break {}", label_name);
                    let continue_pattern = format!("continue {}", label_name);
                    
                    if !parent_text.contains(&goto_pattern) 
                        && !parent_text.contains(&break_pattern)
                        && !parent_text.contains(&continue_pattern) {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            code: "F405".to_string(),
                            message: format!("标签 '{}' 未被使用", label_name),
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

// ==================== F8xx: 变量问题 ====================

/// F811: redefined-variable - 变量重定义
pub struct RedefinedVariable;

impl Rule for RedefinedVariable {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "F811",
            name: "redefined-variable",
            description: "变量在同一作用域内被重复定义",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        if node.kind() == "block" {
            use std::collections::HashSet;
            let mut defined: HashSet<String> = HashSet::new();
            
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                // 检查短变量声明
                if child.kind() == "short_var_declaration" {
                    if let Some(left) = child.child_by_field_name("left") {
                        let mut left_cursor = left.walk();
                        for id in left.children(&mut left_cursor) {
                            if id.kind() == "identifier" {
                                let name = source[id.byte_range()].to_string();
                                
                                if defined.contains(&name) {
                                    let pos = id.start_position();
                                    diagnostics.push(Diagnostic {
                                        code: "F811".to_string(),
                                        message: format!("变量 '{}' 在同一作用域内重复定义", name),
                                        severity: self.default_severity(),
                                        file_path: file_path.to_string(),
                                        line: pos.row + 1,
                                        column: pos.column + 1,
                                        fix: None,
                                    });
                                } else {
                                    defined.insert(name);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        diagnostics
    }
}

/// F812: undefined-variable - 未定义的变量
pub struct UndefinedVariable;

impl Rule for UndefinedVariable {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "F812",
            name: "undefined-variable",
            description: "使用了未定义的变量",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 这是一个简化的实现
        // 完整实现需要完整的符号表分析
        if node.kind() == "identifier" {
            let name = &source[node.byte_range()];
            
            // 忽略常见内置标识符
            let builtins = ["nil", "true", "false", "iota", "append", "cap", 
                           "close", "complex", "copy", "delete", "imag", "len",
                           "make", "new", "panic", "print", "println", "real", "recover"];
            
            if !builtins.contains(&name) {
                // 这里应该检查变量是否已定义
                // 简化版：只检查明显的错误模式
                if name.starts_with('_') && name.len() > 1 {
                    // 可能是未导出的标识符误用
                    if let Some(parent) = node.parent() {
                        if parent.kind() == "selector_expression" {
                            let pos = node.start_position();
                            diagnostics.push(Diagnostic {
                                code: "F812".to_string(),
                                message: format!("未导出的标识符 '{}' 可能无法访问", name),
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

/// F831: loop-variable-capture - 循环变量捕获
pub struct LoopVariableCapture;

impl Rule for LoopVariableCapture {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "F831",
            name: "loop-variable-capture",
            description: "goroutine 或闭包中捕获循环变量",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 检查 for 循环中的 goroutine
        if node.kind() == "for_statement" || node.kind() == "range_clause" {
            let loop_text = &source[node.byte_range()];
            
            // 检查是否包含 go 关键字
            if loop_text.contains("go ") || loop_text.contains("go func") {
                // 提取循环变量名
                let var_names = extract_loop_vars(&node, source);
                
                // 检查 goroutine 是否直接使用循环变量
                for var_name in var_names {
                    // 简单检查：goroutine 中使用了循环变量但没有作为参数传递
                    if loop_text.contains("go func()") 
                        && loop_text.contains(&var_name)
                        && !loop_text.contains(&format!("go func({}", var_name)) {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            code: "F831".to_string(),
                            message: format!("goroutine 中捕获循环变量 '{}'，应通过参数传递", var_name),
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

fn extract_loop_vars(node: &Node, source: &str) -> Vec<String> {
    let mut vars = vec![];
    
    if node.kind() == "range_clause" {
        if let Some(left) = node.child_by_field_name("left") {
            let mut cursor = left.walk();
            for child in left.children(&mut cursor) {
                if child.kind() == "identifier" {
                    vars.push(source[child.byte_range()].to_string());
                }
            }
        }
    }
    
    vars
}

/// F841: unused-assignment - 未使用的赋值
pub struct UnusedAssignment;

impl Rule for UnusedAssignment {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "F841",
            name: "unused-assignment",
            description: "变量被赋值但未被使用",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 检查赋值语句
        if node.kind() == "assignment_statement" {
            if let Some(left) = node.child_by_field_name("left") {
                let mut cursor = left.walk();
                for child in left.children(&mut cursor) {
                    if child.kind() == "identifier" {
                        let var_name = &source[child.byte_range()];
                        
                        // 检查后续是否使用
                        if let Some(parent) = node.parent() {
                            let block_text = &source[parent.byte_range()];
                            let node_end = node.end_byte() - parent.start_byte();
                            let after = &block_text[node_end..];
                            
                            if !after.contains(var_name) {
                                let pos = child.start_position();
                                diagnostics.push(Diagnostic {
                                    code: "F841".to_string(),
                                    message: format!("变量 '{}' 被赋值后未被使用", var_name),
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

// ==================== F9xx: 控制流 ====================

/// F901: unreachable-code - 不可达代码
pub struct UnreachableCode;

impl Rule for UnreachableCode {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "F901",
            name: "unreachable-code",
            description: "return/panic 后的代码不可达",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        if node.kind() == "block" {
            let mut saw_terminator = false;
            let mut cursor = node.walk();
            
            for child in node.children(&mut cursor) {
                if saw_terminator {
                    // 发现了不可达代码
                    if !matches!(child.kind(), "}" | "{") && !child.kind().contains("comment") {
                        let pos = child.start_position();
                        diagnostics.push(Diagnostic {
                            code: "F901".to_string(),
                            message: "不可达代码".to_string(),
                            severity: self.default_severity(),
                            file_path: file_path.to_string(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                        break; // 只报告第一个
                    }
                }
                
                // 检查是否是终止语句
                let child_text = &source[child.byte_range()];
                if child.kind() == "return_statement" 
                    || child.kind() == "call_expression" && child_text.contains("panic(")
                    || child.kind() == "call_expression" && child_text.contains("os.Exit(") {
                    saw_terminator = true;
                }
            }
        }
        
        diagnostics
    }
}

/// F902: duplicate-case - 重复的 case
pub struct DuplicateCase;

impl Rule for DuplicateCase {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "F902",
            name: "duplicate-case",
            description: "switch 中有重复的 case",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        if node.kind() == "expression_case" || node.kind() == "type_case" {
            use std::collections::HashSet;
            let mut seen_cases: HashSet<String> = HashSet::new();
            
            if let Some(parent) = node.parent() {
                let mut cursor = parent.walk();
                for sibling in parent.children(&mut cursor) {
                    if sibling.kind() == node.kind() {
                        let case_text = source[sibling.byte_range()].to_string();
                        
                        if seen_cases.contains(&case_text) {
                            let pos = sibling.start_position();
                            diagnostics.push(Diagnostic {
                                code: "F902".to_string(),
                                message: "重复的 case".to_string(),
                                severity: self.default_severity(),
                                file_path: file_path.to_string(),
                                line: pos.row + 1,
                                column: pos.column + 1,
                                fix: None,
                            });
                        } else {
                            seen_cases.insert(case_text);
                        }
                    }
                }
            }
        }
        
        diagnostics
    }
}

/// F903: missing-return - 缺少 return
pub struct MissingReturn;

impl Rule for MissingReturn {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "F903",
            name: "missing-return",
            description: "非 void 函数缺少 return 语句",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        if node.kind() == "function_declaration" || node.kind() == "func_literal" {
            // 检查是否有返回类型
            let has_return_type = node.child_by_field_name("result").is_some();
            
            if has_return_type {
                // 检查函数体是否有 return
                if let Some(body) = node.child_by_field_name("body") {
                    let body_text = &source[body.byte_range()];
                    
                    if !body_text.contains("return") {
                        let pos = body.start_position();
                        diagnostics.push(Diagnostic {
                            code: "F903".to_string(),
                            message: "函数声明了返回类型但缺少 return 语句".to_string(),
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

// ==================== 规则导出 ====================

pub fn get_rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(UnusedImport),
        Box::new(UnusedVariable),
        Box::new(UnusedParameter),
        Box::new(UnusedReturnValue),
        Box::new(UnusedLabel),
        Box::new(RedefinedVariable),
        Box::new(UndefinedVariable),
        Box::new(LoopVariableCapture),
        Box::new(UnusedAssignment),
        Box::new(UnreachableCode),
        Box::new(DuplicateCase),
        Box::new(MissingReturn),
    ]
}
