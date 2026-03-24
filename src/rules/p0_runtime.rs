//! P0 Phase 2: Runtime Errors and Code Structure Rules
//!
//! Category A: Runtime Errors
//! Category B: Code Structure
//! Category C: API Misuse
//! Category D: Resource Management

use super::{Rule, RuleCategory, RuleMetadata, RulePriority};
use crate::{Diagnostic, Severity};
use tree_sitter::Node;

// ==================== Category A: Runtime Errors ====================

/// SA5000: assignment-to-nil-map - 向 nil map 赋值
///
/// 向 nil map 赋值会导致运行时 panic。
/// 需要追踪变量声明和赋值。
pub struct AssignmentToNilMap;

impl Rule for AssignmentToNilMap {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "SA5000",
            name: "assignment-to-nil-map",
            description: "向 nil map 赋值会导致 panic",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        // 检查赋值语句 m[key] = value
        if node.kind() == "assignment_statement" {
            let text = &source[node.byte_range()];

            // 检查是否是 map 赋值（包含 [ 和 ]= 模式）
            if text.contains('[') && text.contains("]=") {
                // 提取 map 变量名
                if let Some(map_name) = self.extract_map_name(text) {
                    // 检查该变量在同作用域内是否有 make 初始化
                    if let Some(parent) = find_parent_block(&node) {
                        if !self.has_make_initialization(&map_name, parent, source, &node) {
                            let pos = node.start_position();
                            diagnostics.push(Diagnostic {
                                code: "SA5000".to_string(),
                                message: format!(
                                    "map '{}' 可能是 nil，赋值会导致 panic。使用 make() 初始化: make(map[KeyType]ValueType)",
                                    map_name
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

impl AssignmentToNilMap {
    fn extract_map_name(&self, text: &str) -> Option<String> {
        // 从 "m[key] = value" 提取 "m"
        let trimmed = text.trim();
        if let Some(bracket_pos) = trimmed.find('[') {
            let name = trimmed[..bracket_pos].trim();
            if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                return Some(name.to_string());
            }
        }
        None
    }

    fn has_make_initialization<'a>(
        &self,
        map_name: &str,
        block: Node<'a>,
        source: &str,
        current_node: &Node<'a>,
    ) -> bool {
        let current_line = current_node.start_position().row;

        // 遍历块内所有语句
        let mut cursor = block.walk();
        for child in block.children(&mut cursor) {
            // 只检查当前赋值之前的语句
            if child.start_position().row >= current_line {
                break;
            }

            let text = &source[child.byte_range()];

            // 检查是否有 make 初始化
            // 模式: m = make(map[...]...) 或 m := make(map[...]...)
            if text.contains(&format!("{} = make(map[", map_name))
                || text.contains(&format!("{} := make(map[", map_name))
            {
                return true;
            }

            // 检查短变量声明中的 make
            if text.contains(":=") && text.contains("make(map[") {
                let parts: Vec<&str> = text.split(":=").collect();
                if parts.len() >= 2 {
                    let left = parts[0].trim();
                    if left == map_name || left.split(',').any(|s| s.trim() == map_name) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

/// SA5007: infinite-recursion - 无限递归
///
/// 检测函数无条件地递归调用自身，会导致栈溢出。
pub struct InfiniteRecursion;

impl Rule for InfiniteRecursion {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "SA5007",
            name: "infinite-recursion",
            description: "函数无限递归会导致栈溢出",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        if node.kind() == "function_declaration" || node.kind() == "method_declaration" {
            if let Some(name_node) = node.child_by_field_name("name") {
                let func_name = &source[name_node.byte_range()];

                if let Some(body) = node.child_by_field_name("body") {
                    let body_text = &source[body.byte_range()];

                    // 检查是否是明显的无限递归
                    // 1. 函数体直接调用自身
                    // 2. 没有终止条件
                    if self.is_direct_recursive_call(body_text, func_name)
                        && !self.has_termination_condition(&body, source, func_name) {
                            let pos = name_node.start_position();
                            diagnostics.push(Diagnostic {
                                code: "SA5007".to_string(),
                                message: format!("函数 '{}' 可能无限递归，缺少终止条件", func_name),
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

impl InfiniteRecursion {
    fn is_direct_recursive_call(&self, body_text: &str, func_name: &str) -> bool {
        // 简单检测：函数体包含 func_name( 调用
        // 需要排除注释和字符串中的匹配
        let pattern = format!("{}(", func_name);
        body_text.contains(&pattern)
    }

    fn has_termination_condition(&self, body: &Node, source: &str, func_name: &str) -> bool {
        let body_text = &source[body.byte_range()];

        // 检查是否有 if 语句
        if !body_text.contains("if ") {
            return false;
        }

        // 检查是否有 return 语句（除了递归调用处的 return）
        let return_count = body_text.matches("return").count();
        let recursive_calls = body_text.matches(&format!("{}(", func_name)).count();

        // 如果有 return 且递归调用少于 return，可能有终止条件
        return_count > recursive_calls
    }
}

/// SA5011: possible-nil-pointer-dereference - 可能的 nil 指针解引用
///
/// 检查可能的 nil 指针解引用。
pub struct PossibleNilPointerDereference;

impl Rule for PossibleNilPointerDereference {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "SA5011",
            name: "possible-nil-pointer-dereference",
            description: "可能的 nil 指针解引用",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        // 检查 selector_expression: x.Field 或 x.Method()
        if node.kind() == "selector_expression" || node.kind() == "call_expression" {
            // 提取被访问的对象
            if let Some(obj_name) = self.extract_object_name(&node, source) {
                // 检查同作用域内是否有 nil 检查但未返回
                if let Some(parent) = find_parent_block(&node) {
                    if self.has_nil_check_no_return(&obj_name, parent, source, &node) {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            code: "SA5011".to_string(),
                            message: format!(
                                "可能的 nil 指针解引用: '{}'。在 nil 检查后应添加 return 或处理 nil 情况",
                                obj_name
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

impl PossibleNilPointerDereference {
    fn extract_object_name<'a>(&self, node: &Node<'a>, source: &str) -> Option<String> {
        if node.kind() == "selector_expression" {
            // x.Field -> 获取 x
            if let Some(operand) = node.child_by_field_name("operand") {
                if operand.kind() == "identifier" {
                    return Some(source[operand.byte_range()].to_string());
                }
            }
        } else if node.kind() == "call_expression" {
            // x.Method() -> 获取 x
            if let Some(func) = node.child_by_field_name("function") {
                if func.kind() == "selector_expression" {
                    if let Some(operand) = func.child_by_field_name("operand") {
                        if operand.kind() == "identifier" {
                            return Some(source[operand.byte_range()].to_string());
                        }
                    }
                }
            }
        }
        None
    }

    fn has_nil_check_no_return<'a>(
        &self,
        obj_name: &str,
        block: Node<'a>,
        source: &str,
        current_node: &Node<'a>,
    ) -> bool {
        let current_line = current_node.start_position().row;
        let mut found_nil_check = false;
        let mut found_return_after_check = false;

        let mut cursor = block.walk();
        for child in block.children(&mut cursor) {
            let child_line = child.start_position().row;
            let text = &source[child.byte_range()];

            // 在当前语句之前检查
            if child_line >= current_line {
                break;
            }

            // 检查 nil 检查
            if text.contains(&format!("{} == nil", obj_name))
                || text.contains(&format!("{} != nil", obj_name))
            {
                found_nil_check = true;
                found_return_after_check = false; // 重置
                continue;
            }

            // nil 检查后是否有 return
            if found_nil_check && text.contains("return") {
                found_return_after_check = true;
            }
        }

        // 有 nil 检查但没有 return 处理
        found_nil_check && !found_return_after_check
    }
}

/// SA5002: infinite-empty-loop - 空的无限循环
///
/// 空的 for {} 循环会浪费 CPU。
pub struct InfiniteEmptyLoop;

impl Rule for InfiniteEmptyLoop {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "SA5002",
            name: "infinite-empty-loop",
            description: "空的无限循环会浪费 CPU",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Warning,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        if node.kind() == "for_statement" || node.kind() == "range_clause" {
            let text = &source[node.byte_range()];

            // 检查无限循环 for { ... }
            if text.starts_with("for {") || text.contains("for {\n") {
                if let Some(body) = node.child_by_field_name("body") {
                    let body_text = &source[body.byte_range()];

                    // 检查循环体是否为空或只有注释
                    if self.is_empty_or_comment_only(body_text) {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            code: "SA5002".to_string(),
                            message: "空的无限循环会浪费 CPU。考虑添加 sleep、select 或 break 条件"
                                .to_string(),
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

impl InfiniteEmptyLoop {
    fn is_empty_or_comment_only(&self, body_text: &str) -> bool {
        // 去除空白和注释后是否为空
        let lines: Vec<&str> = body_text.lines().collect();
        for line in lines.iter().skip(1) {
            // 跳过 {
            let trimmed = line.trim();
            if !trimmed.is_empty()
                && !trimmed.starts_with("//")
                && !trimmed.starts_with("/*")
                && trimmed != "}"
            {
                return false;
            }
        }
        true
    }
}

// ==================== Category B: Code Structure ====================

/// F901: unreachable-code - 不可达代码
///
/// return/panic 后的代码不会被执行。
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
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        if node.kind() == "block" {
            let mut saw_terminator = false;
            let mut cursor = node.walk();

            for child in node.children(&mut cursor) {
                if saw_terminator {
                    // 检查是否是注释、空白或结束符
                    let text = &source[child.byte_range()];
                    if !text.trim().is_empty() && text != "}" && !text.starts_with("//") {
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
                if self.is_terminator(&child, source) {
                    saw_terminator = true;
                }
            }
        }

        diagnostics
    }
}

impl UnreachableCode {
    fn is_terminator(&self, node: &Node, source: &str) -> bool {
        let text = &source[node.byte_range()];

        node.kind() == "return_statement"
            || text.starts_with("panic(")
            || text.starts_with("os.Exit(")
            || text.starts_with("log.Fatal")
    }
}

/// F903: missing-return - 缺少 return 语句
///
/// 非 void 函数缺少 return 语句。
pub struct MissingReturn;

impl Rule for MissingReturn {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "F903",
            name: "missing-return",
            description: "函数声明了返回类型但可能缺少 return 语句",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        if node.kind() == "function_declaration" || node.kind() == "method_declaration" {
            // 检查是否有返回类型
            if let Some(result) = node.child_by_field_name("result") {
                let result_text = &source[result.byte_range()];

                // 排除 named return values (它们可以 naked return)
                if !result_text.contains("(") && !result_text.trim().is_empty() {
                    if let Some(body) = node.child_by_field_name("body") {
                        let body_text = &source[body.byte_range()];

                        // 简单检查：函数体是否有 return
                        // 注意：这是一个简化版，没有处理所有控制流分支
                        if !body_text.contains("return") {
                            let pos = node.start_position();
                            diagnostics.push(Diagnostic {
                                code: "F903".to_string(),
                                message: format!(
                                    "函数声明了返回类型 '{}' 但函数体缺少 return 语句",
                                    result_text.trim()
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

/// SA4006: ineffectual-assignment - 无效赋值
///
/// 变量被赋值但从未被读取。
pub struct IneffectualAssignment;

impl Rule for IneffectualAssignment {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "SA4006",
            name: "ineffectual-assignment",
            description: "变量被赋值但从未被读取",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Warning,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        // 检查赋值语句 x = value
        if node.kind() == "assignment_statement" {
            if let Some(left) = node.child_by_field_name("left") {
                let left_text = &source[left.byte_range()];

                // 简单情况：单变量赋值
                if !left_text.contains(',') {
                    let var_name = left_text.trim();

                    // 排除 _
                    if var_name != "_" {
                        // 检查后续是否被读取
                        if let Some(parent) = find_parent_block(&node) {
                            if !self.is_variable_read_after(var_name, parent, source, &node) {
                                let pos = left.start_position();
                                diagnostics.push(Diagnostic {
                                    code: "SA4006".to_string(),
                                    message: format!("变量 '{}' 被赋值后未被读取", var_name),
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

impl IneffectualAssignment {
    fn is_variable_read_after<'a>(
        &self,
        var_name: &str,
        block: Node<'a>,
        source: &str,
        current_node: &Node<'a>,
    ) -> bool {
        let current_line = current_node.start_position().row;
        let mut cursor = block.walk();

        for child in block.children(&mut cursor) {
            // 只检查当前赋值之后的语句
            if child.start_position().row <= current_line {
                continue;
            }

            let text = &source[child.byte_range()];

            // 简单检查：变量名是否出现在右侧
            // 注意：这是一个简化版，可能误判
            if text.contains(var_name) && !text.contains(&format!("{} =", var_name)) {
                return true;
            }
        }

        false
    }
}

/// SA4017: discard-pure-function-result - 丢弃纯函数返回值
///
/// 丢弃没有副作用的函数的返回值。
pub struct DiscardPureFunctionResult;

impl Rule for DiscardPureFunctionResult {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "SA4017",
            name: "discard-pure-function-result",
            description: "丢弃纯函数的返回值，函数调用无意义",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Warning,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        // 检查表达式语句中的纯函数调用
        if node.kind() == "expression_statement" {
            let text = &source[node.byte_range()];

            // 检查是否是纯函数调用
            if let Some(func_name) = self.extract_pure_function_call(text) {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    code: "SA4017".to_string(),
                    message: format!(
                        "纯函数 '{}' 的返回值被丢弃，调用无意义。应该使用返回值或移除调用",
                        func_name
                    ),
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

impl DiscardPureFunctionResult {
    fn extract_pure_function_call(&self, text: &str) -> Option<String> {
        // 纯函数列表
        let pure_functions = [
            "strings.Replace",
            "strings.Trim",
            "strings.ToLower",
            "strings.ToUpper",
            "strings.Contains",
            "strings.HasPrefix",
            "strings.HasSuffix",
            "bytes.Trim",
            "bytes.Replace",
            "bytes.ToLower",
            "bytes.ToUpper",
            "math.Max",
            "math.Min",
            "math.Abs",
            "sort.Ints",
            "sort.Strings",
        ];

        for func in &pure_functions {
            if text.contains(func) && text.contains("(") {
                return Some(func.to_string());
            }
        }

        None
    }
}

/// SA5010: impossible-type-assertion - 不可能的类型断言
///
/// 类型断言不可能成功。
pub struct ImpossibleTypeAssertion;

impl Rule for ImpossibleTypeAssertion {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "SA5010",
            name: "impossible-type-assertion",
            description: "类型断言不可能成功",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        if node.kind() == "type_assertion_expression" {
            let text = &source[node.byte_range()];

            // 简单检查：明显的类型不匹配
            // 例如: var x int; x.(string)
            if let Some(type_info) = self.extract_type_info(text, source) {
                if self.is_impossible_assertion(&type_info) {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        code: "SA5010".to_string(),
                        message: "类型断言不可能成功，类型不兼容".to_string(),
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

impl ImpossibleTypeAssertion {
    fn extract_type_info(&self, _text: &str, _source: &str) -> Option<String> {
        // 简化版：实际实现需要类型推断
        None
    }

    fn is_impossible_assertion(&self, _type_info: &str) -> bool {
        // 简化版：实际实现需要类型系统
        false
    }
}

// ==================== Category C: API Misuse ====================

/// SA1002: time-parse-format - time.Parse 格式错误
///
/// 检查 time.Parse 是否使用了错误的日期格式。
pub struct TimeParseFormat;

impl Rule for TimeParseFormat {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "SA1002",
            name: "time-parse-format",
            description: "time.Parse 使用了错误的日期格式。Go 使用 '2006-01-02' 作为参考时间",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Warning,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        if node.kind() == "call_expression" {
            let text = &source[node.byte_range()];

            if text.contains("time.Parse") {
                if let Some(format_str) = self.extract_format_string(node, source) {
                    // 检查是否使用了其他语言的格式
                    if self.is_wrong_format(&format_str) {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            code: "SA1002".to_string(),
                            message: format!(
                                "time.Parse 使用了错误的日期格式: '{}'. Go 使用参考时间 '2006-01-02 15:04:05'",
                                format_str
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

impl TimeParseFormat {
    fn extract_format_string(&self, node: Node, source: &str) -> Option<String> {
        if let Some(args) = node.child_by_field_name("arguments") {
            let mut cursor = args.walk();
            let mut children = vec![];

            for child in args.children(&mut cursor) {
                if child.kind() != "(" && child.kind() != ")" && child.kind() != "," {
                    children.push(child);
                }
            }

            if !children.is_empty() {
                let format_node = &children[0];
                if format_node.kind() == "interpreted_string_literal" {
                    return Some(
                        source[format_node.byte_range()]
                            .trim_matches('"')
                            .to_string(),
                    );
                }
            }
        }
        None
    }

    fn is_wrong_format(&self, format: &str) -> bool {
        // 检查其他语言的常见格式
        let wrong_patterns = [
            "yyyy", // Java/C# 格式
            "YYYY", // 错误的大小写
            "MM",   // 月份应该使用 01
            "DD",   // 日期应该使用 02
            "dd",   // 错误的大小写
            "HH",   // 小时应该使用 15
            "mm",   // 分钟应该使用 04
            "ss",   // 秒应该使用 05
            "%Y",   // strftime 格式
            "%m", "%d",
        ];

        for pattern in &wrong_patterns {
            if format.contains(pattern) {
                return true;
            }
        }

        false
    }
}

/// SA1020: invalid-network-address - 无效的网络地址
///
/// 检查网络地址格式是否正确。
pub struct InvalidNetworkAddress;

impl Rule for InvalidNetworkAddress {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "SA1020",
            name: "invalid-network-address",
            description: "无效的网络地址格式",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Warning,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        if node.kind() == "call_expression" {
            let text = &source[node.byte_range()];

            if text.contains("net.Listen") || text.contains("net.Dial") {
                if let Some(addr) = self.extract_address(node, source) {
                    if self.is_invalid_address(&addr) {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            code: "SA1020".to_string(),
                            message: format!(
                                "无效的网络地址: '{}'. 格式应为 'host:port'，端口范围 1-65535",
                                addr
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

impl InvalidNetworkAddress {
    fn extract_address(&self, node: Node, source: &str) -> Option<String> {
        if let Some(args) = node.child_by_field_name("arguments") {
            let args_text = &source[args.byte_range()];
            // 简化提取第二个参数
            let parts: Vec<&str> = args_text.split(',').collect();
            if parts.len() >= 2 {
                let addr = parts[1].trim().trim_matches('"').trim_matches(')');
                return Some(addr.to_string());
            }
        }
        None
    }

    fn is_invalid_address(&self, addr: &str) -> bool {
        // 检查端口范围
        if let Some(colon_pos) = addr.rfind(':') {
            let port_str = &addr[colon_pos + 1..];
            if let Ok(port) = port_str.parse::<u32>() {
                return port == 0 || port > 65535;
            }
        }
        false
    }
}

/// SA1007: http-header-format - HTTP Header 格式错误
///
/// 检查 HTTP Header 是否使用了错误的格式。
pub struct HTTPHeaderFormat;

impl Rule for HTTPHeaderFormat {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "SA1007",
            name: "http-header-format",
            description: "HTTP Header 键应该使用 CanonicalHeaderKey 格式",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Info,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        if node.kind() == "assignment_statement" || node.kind() == "short_var_declaration" {
            let text = &source[node.byte_range()];

            // 检查 HTTP Header 赋值
            if (text.contains("Header()") || text.contains("http.Header"))
                && self.has_non_canonical_header(text) {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        code: "SA1007".to_string(),
                        message: "HTTP Header 键应该使用规范格式（首字母大写，如 'Content-Type'）"
                            .to_string(),
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

impl HTTPHeaderFormat {
    fn has_non_canonical_header(&self, text: &str) -> bool {
        // 简单的启发式检查
        // 非规范格式: "content-type", "x-custom-header"
        // 规范格式: "Content-Type", "X-Custom-Header"

        // 检查小写的常见 header
        let lowercase_headers = [
            "\"content-type\"",
            "\"content-length\"",
            "\"accept-encoding\"",
            "\"user-agent\"",
        ];

        for header in &lowercase_headers {
            if text.contains(header) {
                return true;
            }
        }

        false
    }
}

/// SA5008: invalid-struct-tag - 无效的 struct tag
///
/// 检查 struct tag 格式是否正确。
pub struct InvalidStructTag;

impl Rule for InvalidStructTag {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "SA5008",
            name: "invalid-struct-tag",
            description: "无效的 struct tag 格式",
            category: RuleCategory::Logic,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        if node.kind() == "field_declaration" {
            let text = &source[node.byte_range()];

            // 检查是否有 tag
            if text.contains('`') {
                if let Some(tag) = self.extract_tag(text) {
                    if self.has_tag_error(&tag) {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            code: "SA5008".to_string(),
                            message: format!(
                                "无效的 struct tag: '{}'. 格式应为 'key:\"value\"'，注意引号匹配",
                                tag
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

impl InvalidStructTag {
    fn extract_tag(&self, text: &str) -> Option<String> {
        if let Some(start) = text.find('`') {
            if let Some(end) = text[start + 1..].find('`') {
                return Some(text[start + 1..start + 1 + end].to_string());
            }
        }
        None
    }

    fn has_tag_error(&self, tag: &str) -> bool {
        // 检查引号是否匹配
        let double_quotes = tag.matches('"').count();
        if double_quotes % 2 != 0 {
            return true;
        }

        // 检查是否有重复的 key
        let keys: Vec<&str> = tag
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.split(':').next().unwrap_or(""))
            .collect();

        for (i, key) in keys.iter().enumerate() {
            if keys.iter().skip(i + 1).any(|k| k == key) {
                return true;
            }
        }

        false
    }
}

// ==================== Category D: Resource Management ====================

/// B301: missing-defer-close - 缺少 defer Close
///
/// 打开的资源应该有 defer Close。
pub struct MissingDeferClose;

impl Rule for MissingDeferClose {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "B301",
            name: "missing-defer-close",
            description: "打开的资源应该有 defer Close()",
            category: RuleCategory::Bugbear,
            priority: RulePriority::Required,
            default_severity: Severity::Warning,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        // 检查资源打开调用
        if node.kind() == "short_var_declaration" || node.kind() == "assignment_statement" {
            let text = &source[node.byte_range()];

            if self.is_resource_open(text) {
                // 提取变量名
                if let Some(var_name) = self.extract_variable_name(text) {
                    // 检查后续是否有 defer Close
                    if let Some(parent) = find_parent_function(&node) {
                        if !self.has_defer_close(&var_name, parent, source, &node) {
                            let pos = node.start_position();
                            diagnostics.push(Diagnostic {
                                code: "B301".to_string(),
                                message: format!(
                                    "资源 '{}' 打开后应该使用 defer {}.Close() 确保关闭",
                                    var_name, var_name
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

impl MissingDeferClose {
    fn is_resource_open(&self, text: &str) -> bool {
        text.contains("os.Open(")
            || text.contains("os.Create(")
            || text.contains("net.Listen(")
            || text.contains("sql.Open(")
    }

    fn extract_variable_name(&self, text: &str) -> Option<String> {
        // 从 "f, err := os.Open(...)" 提取 "f"
        if let Some(pos) = text.find(":=") {
            let left = &text[..pos];
            let vars: Vec<&str> = left.split(',').collect();
            if !vars.is_empty() {
                return Some(vars[0].trim().to_string());
            }
        }

        // 从 "f, err = os.Open(...)" 提取 "f"
        if let Some(pos) = text.find("= ") {
            let left = &text[..pos];
            let vars: Vec<&str> = left.split(',').collect();
            if !vars.is_empty() {
                return Some(vars[0].trim().to_string());
            }
        }

        None
    }

    fn has_defer_close<'a>(
        &self,
        var_name: &str,
        func: Node<'a>,
        source: &str,
        _current_node: &Node<'a>,
    ) -> bool {
        let func_text = &source[func.byte_range()];

        // 简单检查：函数体内是否有 defer var.Close()
        let close_pattern = format!("defer {}.Close()", var_name);
        let close_pattern_alt = format!("defer defer {}.Close()", var_name);

        func_text.contains(&close_pattern) || func_text.contains(&close_pattern_alt)
    }
}

// ==================== Helper Functions ====================

fn find_parent_block<'a>(node: &'a Node<'a>) -> Option<Node<'a>> {
    let mut current = *node;

    while let Some(parent) = current.parent() {
        if parent.kind() == "block"
            || parent.kind() == "function_declaration"
            || parent.kind() == "method_declaration"
        {
            return Some(parent);
        }
        current = parent;
    }

    None
}

fn find_parent_function<'a>(node: &'a Node<'a>) -> Option<Node<'a>> {
    let mut current = *node;

    while let Some(parent) = current.parent() {
        if parent.kind() == "function_declaration"
            || parent.kind() == "method_declaration"
            || parent.kind() == "func_literal"
        {
            return Some(parent);
        }
        current = parent;
    }

    None
}

// ==================== Rules Export ====================

pub fn get_p0_runtime_rules() -> Vec<Box<dyn Rule>> {
    vec![
        // Category A: Runtime Errors (5条)
        Box::new(AssignmentToNilMap),
        Box::new(InfiniteRecursion),
        Box::new(PossibleNilPointerDereference),
        Box::new(InfiniteEmptyLoop),
        // Category B: Code Structure (4条)
        Box::new(UnreachableCode),
        Box::new(MissingReturn),
        Box::new(IneffectualAssignment),
        Box::new(DiscardPureFunctionResult),
        Box::new(ImpossibleTypeAssertion),
        // Category C: API Misuse (4条)
        Box::new(TimeParseFormat),
        Box::new(InvalidNetworkAddress),
        Box::new(HTTPHeaderFormat),
        Box::new(InvalidStructTag),
        // Category D: Resource Management (2条)
        Box::new(MissingDeferClose),
    ]
}
