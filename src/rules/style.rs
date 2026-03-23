//! S-series: style - 风格与命名规范
//!
//! 包括：命名规范、风格一致性等

use super::{Rule, RuleCategory, RuleMetadata, RulePriority};
use crate::{Diagnostic, Severity};
use tree_sitter::Node;

/// S001: Naked return - 避免裸返回
pub struct NakedReturn;

impl Rule for NakedReturn {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "S001",
            name: "naked-return",
            description: "避免在有命名返回值的函数中使用裸返回",
            category: RuleCategory::Style,
            priority: RulePriority::Recommended,
            default_severity: Severity::Info,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        if node.kind() == "return_statement" {
            let text = &source[node.byte_range()];
            // 裸返回没有表达式
            if text.trim() == "return" {
                // 检查是否在有命名返回值的函数中
                if let Some(func) = find_parent_by_kind(node, "function_declaration") {
                    if has_named_returns(func, source) {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            code: "S001".to_string(),
                            message: "避免在有命名返回值的函数中使用裸返回".to_string(),
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

/// S002: Unchecked error - 未检查的错误
pub struct UncheckedError;

impl Rule for UncheckedError {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "S002",
            name: "unchecked-error",
            description: "错误返回值应被检查",
            category: RuleCategory::Style,
            priority: RulePriority::Required,
            default_severity: Severity::Warning,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        if node.kind() == "call_expression" {
            let func_name = get_function_name(node, source);
            
            if func_returns_error(&func_name) {
                if !is_error_checked(node) {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        code: "S002".to_string(),
                        message: format!("'{}' 的错误返回值未被检查", func_name),
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

/// S003: Redundant slice - 冗余的切片表达式
pub struct RedundantSlice;

impl Rule for RedundantSlice {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "S003",
            name: "redundant-slice",
            description: "完整的切片表达式 [:] 是冗余的",
            category: RuleCategory::Style,
            priority: RulePriority::Recommended,
            default_severity: Severity::Info,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        if node.kind() == "slice_expression" {
            let text = &source[node.byte_range()];
            if text.ends_with("[:]") {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    code: "S003".to_string(),
                    message: "冗余的完整切片 [:]，直接使用值即可".to_string(),
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

/// S004: Unused parameter - 未使用的参数
pub struct UnusedParameter;

impl Rule for UnusedParameter {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "S004",
            name: "unused-parameter",
            description: "函数参数未被使用",
            category: RuleCategory::Style,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        if node.kind() == "function_declaration" || node.kind() == "method_declaration" {
            if let Some(params) = find_child_by_kind(node, "parameter_list") {
                let param_names = extract_parameter_names(params, source);
                
                if let Some(body) = find_child_by_kind(node, "block") {
                    let body_text = &source[body.byte_range()];
                    
                    for param in param_names {
                        if param != "_" && !body_text.contains(&param) {
                            let pos = params.start_position();
                            diagnostics.push(Diagnostic {
                                code: "S004".to_string(),
                                message: format!("参数 '{}' 未被使用", param),
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

/// S005: CamelCase function - 函数名应为驼峰式
pub struct CamelCaseFunction;

impl Rule for CamelCaseFunction {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "S005",
            name: "camel-case-function",
            description: "函数名应使用驼峰式命名",
            category: RuleCategory::Style,
            priority: RulePriority::Recommended,
            default_severity: Severity::Info,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        if node.kind() == "function_declaration" || node.kind() == "method_declaration" {
            if let Some(name_node) = node.child_by_field_name("name") {
                let name = &source[name_node.byte_range()];
                
                if name.contains('_') && !name.starts_with("Test") && !name.starts_with("Benchmark") && !name.starts_with("Example") {
                    let pos = name_node.start_position();
                    diagnostics.push(Diagnostic {
                        code: "S005".to_string(),
                        message: format!("函数名 '{}' 应使用驼峰式命名", name),
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

/// S006: Receiver name - 接收器名应简短一致
pub struct ReceiverName;

impl Rule for ReceiverName {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "S006",
            name: "receiver-name",
            description: "接收器名应简短（1-2字符）且一致",
            category: RuleCategory::Style,
            priority: RulePriority::Recommended,
            default_severity: Severity::Info,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        if node.kind() == "method_declaration" {
            if let Some(recv) = node.child_by_field_name("receiver") {
                let recv_text = &source[recv.byte_range()];
                
                // 提取接收器名
                let parts: Vec<&str> = recv_text.trim_matches(&['(', ')'][..])
                    .split_whitespace()
                    .collect();
                
                if let Some(name) = parts.first() {
                    if name.len() > 2 && !name.starts_with("self") && !name.starts_with("this") {
                        let pos = recv.start_position();
                        diagnostics.push(Diagnostic {
                            code: "S006".to_string(),
                            message: format!("接收器名 '{}' 应简短（1-2字符）", name),
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

/// S007: Error variable naming - error 变量命名
pub struct ErrorVarNaming;

impl Rule for ErrorVarNaming {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "S007",
            name: "error-var-naming",
            description: "error 类型变量应命名为 ErrXxx",
            category: RuleCategory::Style,
            priority: RulePriority::Recommended,
            default_severity: Severity::Info,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        if node.kind() == "const_declaration" || node.kind() == "var_declaration" {
            let decl_text = &source[node.byte_range()];
            
            // 检查是否是 error 类型
            if decl_text.contains("errors.New") || decl_text.contains("fmt.Errorf") {
                // 提取变量名
                if let Some(name_node) = node.child_by_field_name("name") {
                    let name = &source[name_node.byte_range()];
                    
                    if !name.starts_with("Err") && !name.ends_with("Error") {
                        let pos = name_node.start_position();
                        diagnostics.push(Diagnostic {
                            code: "S007".to_string(),
                            message: format!("error 变量 '{}' 应以 'Err' 开头", name),
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

/// S008: Package name - 包名规范
pub struct PackageName;

impl Rule for PackageName {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "S008",
            name: "package-name",
            description: "包名不应包含下划线或大写字母",
            category: RuleCategory::Style,
            priority: RulePriority::Required,
            default_severity: Severity::Warning,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        if node.kind() == "package_clause" {
            if let Some(name_node) = node.child_by_field_name("name") {
                let name = &source[name_node.byte_range()];
                
                if name.contains('_') {
                    let pos = name_node.start_position();
                    diagnostics.push(Diagnostic {
                        code: "S008".to_string(),
                        message: format!("包名 '{}' 不应包含下划线", name),
                        severity: self.default_severity(),
                        file_path: file_path.to_string(),
                        line: pos.row + 1,
                        column: pos.column + 1,
                        fix: None,
                    });
                }
                
                if name.chars().any(|c| c.is_uppercase()) {
                    let pos = name_node.start_position();
                    diagnostics.push(Diagnostic {
                        code: "S008".to_string(),
                        message: format!("包名 '{}' 不应包含大写字母", name),
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

// Helper functions
fn has_named_returns(func: Node, source: &str) -> bool {
    let text = &source[func.byte_range()];
    
    if let Some(end) = text.rfind(')') {
        let after_params = &text[end..];
        if let Some(ret_start) = after_params.find('(') {
            let ret_section = &after_params[ret_start..];
            return ret_section.chars().filter(|&c| c == ' ').count() >= 1;
        }
    }
    false
}

fn get_function_name(call: Node, source: &str) -> String {
    let text = &source[call.byte_range()];
    if let Some(paren) = text.find('(') {
        text[..paren].trim().to_string()
    } else {
        text.to_string()
    }
}

fn func_returns_error(func_name: &str) -> bool {
    let error_funcs = [
        "os.Open", "os.Create", "os.ReadFile", "os.WriteFile",
        "json.Marshal", "json.Unmarshal", "json.NewEncoder", "json.NewDecoder",
        "io.ReadAll", "io.Copy",
        "http.NewRequest", "http.Get", "http.Post",
        "fmt.Errorf", "fmt.Sscanf", "fmt.Fscanf",
    ];
    
    error_funcs.iter().any(|&f| func_name.contains(f))
}

fn is_error_checked(node: Node) -> bool {
    if let Some(parent) = node.parent() {
        let parent_kind = parent.kind();
        if matches!(parent_kind, "assignment_statement" | "short_var_declaration" | "if_statement") {
            return true;
        }
    }
    false
}

fn find_child_by_kind<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            if cursor.node().kind() == kind {
                return Some(cursor.node());
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
    None
}

fn find_parent_by_kind<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    let mut current = node;
    while let Some(parent) = current.parent() {
        if parent.kind() == kind {
            return Some(parent);
        }
        current = parent;
    }
    None
}

fn extract_parameter_names(params: Node, source: &str) -> Vec<String> {
    let mut names = vec![];
    let text = &source[params.byte_range()];
    
    let inner = text.trim_start_matches('(').trim_end_matches(')');
    
    for param in inner.split(',') {
        let parts: Vec<&str> = param.trim().split_whitespace().collect();
        if !parts.is_empty() {
            let first = parts[0];
            if !first.contains('.') && !first.starts_with("[]") && !first.starts_with('*') {
                names.push(first.to_string());
            }
        }
    }
    
    names
}

// 规则导出
pub fn get_rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(NakedReturn),
        Box::new(UncheckedError),
        Box::new(RedundantSlice),
        Box::new(UnusedParameter),
        Box::new(CamelCaseFunction),
        Box::new(ReceiverName),
        Box::new(ErrorVarNaming),
        Box::new(PackageName),
    ]
}
