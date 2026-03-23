//! P0 Phase 3: Enhanced Rules and High-Value Additions
//!
//! Task 1: Enhanced existing rules (SA5010, SA1020)
//! Task 2: New high-value rules (SA9001, SA9002, SA1024)

use super::{Rule, RuleCategory, RuleMetadata, RulePriority};
use crate::{Diagnostic, Severity};
use tree_sitter::Node;

// ==================== Task 1: Enhanced Existing Rules ====================

/// SA5010: impossible-type-assertion (Enhanced)
///
/// 检测明显不可能成功的类型断言。
/// 增强版检测更多不可能的模式。
pub struct ImpossibleTypeAssertionEnhanced;

impl Rule for ImpossibleTypeAssertionEnhanced {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "SA5010",
            name: "impossible-type-assertion",
            description: "类型断言不可能成功，类型不兼容",
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
            
            // 检测模式 1: nil.(Type)
            if text.starts_with("nil.(" ) || text.contains("nil.(") {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    code: "SA5010".to_string(),
                    message: "nil 接口无法进行类型断言".to_string(),
                    severity: self.default_severity(),
                    file_path: file_path.to_string(),
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
                return diagnostics;
            }
            
            // 检测模式 2: 尝试从指针断言到值类型
            // *T.(T) - 指针断言值类型通常有问题
            if let Some(operand) = node.child_by_field_name("operand") {
                let operand_text = &source[operand.byte_range()];
                
                // 提取断言类型
                if let Some(type_node) = node.child_by_field_name("type") {
                    let type_text = &source[type_node.byte_range()];
                    
                    // 检查指针到值的断言
                    if self.is_pointer_to_value_assertion(operand_text, type_text) {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            code: "SA5010".to_string(),
                            message: format!(
                                "类型断言 '{}' 不可能成功: 指针类型无法断言为值类型 '{}'",
                                operand_text, type_text
                            ),
                            severity: self.default_severity(),
                            file_path: file_path.to_string(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                        return diagnostics;
                    }
                    
                    // 检查 slice 到 string 的断言
                    if self.is_slice_to_string_assertion(operand_text, type_text) {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            code: "SA5010".to_string(),
                            message: format!(
                                "类型断言 '{}' 不可能成功: slice 无法直接断言为 string，应使用 string() 转换",
                                operand_text
                            ),
                            severity: self.default_severity(),
                            file_path: file_path.to_string(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                        return diagnostics;
                    }
                    
                    // 检查 map 到 slice 的断言
                    if self.is_map_to_slice_assertion(operand_text, type_text) {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            code: "SA5010".to_string(),
                            message: format!(
                                "类型断言 '{}' 不可能成功: map 和 slice 是不兼容的类型",
                                operand_text
                            ),
                            severity: self.default_severity(),
                            file_path: file_path.to_string(),
                            line: pos.row + 1,
                            column: pos.column + 1,
                            fix: None,
                        });
                        return diagnostics;
                    }
                }
            }
        }
        
        diagnostics
    }
}

impl ImpossibleTypeAssertionEnhanced {
    fn is_pointer_to_value_assertion(&self, operand: &str, assert_type: &str) -> bool {
        // 简单的启发式检查: *X.(X) 模式
        if operand.starts_with('*') {
            let inner = &operand[1..];
            // 去掉可能的括号
            let inner_clean = inner.trim_start_matches('(').trim_end_matches(')');
            let type_clean = assert_type.trim_start_matches('(').trim_end_matches(')');
            
            if inner_clean == type_clean {
                return true;
            }
        }
        false
    }
    
    fn is_slice_to_string_assertion(&self, operand: &str, assert_type: &str) -> bool {
        // []byte.(string) 或 []rune.(string) 等
        operand.starts_with("[]") && assert_type == "string"
    }
    
    fn is_map_to_slice_assertion(&self, operand: &str, assert_type: &str) -> bool {
        // map[K]V.([]T) 模式
        operand.starts_with("map[") && assert_type.starts_with("[]")
    }
}

/// SA1020: invalid-network-address (Enhanced)
///
/// 检测无效的网络地址格式。
/// 增强版添加完整 IP 格式验证。
pub struct InvalidNetworkAddressEnhanced;

impl Rule for InvalidNetworkAddressEnhanced {
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
            
            if text.contains("net.Listen") || 
               text.contains("net.Dial") || 
               text.contains("net.ResolveTCPAddr") ||
               text.contains("net.ResolveUDPAddr") {
                
                if let Some(addr) = self.extract_address(node, source) {
                    // 检查各种无效模式
                    if let Some(error) = self.validate_address(&addr) {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            code: "SA1020".to_string(),
                            message: format!("无效的网络地址 '{}': {}", addr, error),
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

impl InvalidNetworkAddressEnhanced {
    fn extract_address(&self, node: Node, source: &str) -> Option<String> {
        if let Some(args) = node.child_by_field_name("arguments") {
            let args_text = &source[args.byte_range()];
            
            // 尝试提取第二个参数（地址）
            // 模式: net.Listen("tcp", ":8080")
            let parts: Vec<&str> = args_text.split(',').collect();
            if parts.len() >= 2 {
                let addr = parts[1].trim();
                // 移除各种边界字符
                let cleaned = addr
                    .trim_start_matches('"')
                    .trim_end_matches(')')
                    .trim_end_matches('"')
                    .trim_matches('(')
                    .trim_matches(')')
                    .trim_matches('`')
                    .trim();
                return Some(cleaned.to_string());
            }
        }
        None
    }
    
    fn validate_address(&self, addr: &str) -> Option<String> {
        // 空地址检查
        if addr.is_empty() {
            return Some("地址不能为空".to_string());
        }
        
        // 检查 IPv6 格式 [host]:port
        if addr.starts_with('[') {
            return self.validate_ipv6_address(addr);
        }
        
        // 检查是否是纯端口（如 ":8080"）
        if addr.starts_with(':') {
            return self.validate_port(&addr[1..]);
        }
        
        // 检查 host:port 格式
        if let Some(colon_pos) = addr.rfind(':') {
            let host = &addr[..colon_pos];
            let port = &addr[colon_pos + 1..];
            
            // 验证端口
            if let Some(err) = self.validate_port(port) {
                return Some(err);
            }
            
            // 验证主机部分
            if !host.is_empty() {
                return self.validate_host(host);
            }
        } else {
            // 没有端口，可能是主机名或 IP
            return self.validate_host(addr);
        }
        
        None
    }
    
    fn validate_ipv6_address(&self, addr: &str) -> Option<String> {
        // IPv6 格式: [::1]:8080 或 [2001:db8::1]:8080
        if let Some(close_bracket) = addr.find(']') {
            let ipv6_part = &addr[1..close_bracket];
            let rest = &addr[close_bracket + 1..];
            
            // 验证 IPv6 地址格式（简化版）
            if ipv6_part.is_empty() {
                return Some("IPv6 地址不能为空".to_string());
            }
            
            // 检查端口部分
            if rest.starts_with(':') {
                return self.validate_port(&rest[1..]);
            } else if !rest.is_empty() {
                return Some("IPv6 地址后应有 :port".to_string());
            }
        } else {
            return Some("IPv6 地址缺少闭合括号 ']'".to_string());
        }
        
        None
    }
    
    fn validate_port(&self, port_str: &str) -> Option<String> {
        if port_str.is_empty() {
            return Some("端口号不能为空".to_string());
        }
        
        match port_str.parse::<u32>() {
            Ok(port) => {
                if port == 0 {
                    return Some("端口号不能为 0".to_string());
                }
                if port > 65535 {
                    return Some(format!("端口号 {} 超出范围 (1-65535)", port));
                }
            }
            Err(_) => {
                return Some(format!("'{}' 不是有效的端口号", port_str));
            }
        }
        
        None
    }
    
    fn validate_host(&self, host: &str) -> Option<String> {
        // 检查是否是 IPv4 地址
        if host.contains('.') {
            return self.validate_ipv4(host);
        }
        
        // 检查主机名合法性（简化版）
        // 主机名只能包含字母、数字、连字符和点
        for c in host.chars() {
            if !c.is_alphanumeric() && c != '-' && c != '.' && c != '_' {
                return Some(format!("主机名包含非法字符 '{}'", c));
            }
        }
        
        // 检查连续的点
        if host.contains("..") {
            return Some("主机名不能包含连续的点".to_string());
        }
        
        // 检查开头和结尾
        if host.starts_with('-') || host.ends_with('-') {
            return Some("主机名不能以连字符开头或结尾".to_string());
        }
        if host.starts_with('.') || host.ends_with('.') {
            return Some("主机名不能以点开头或结尾".to_string());
        }
        
        None
    }
    
    fn validate_ipv4(&self, ip: &str) -> Option<String> {
        let parts: Vec<&str> = ip.split('.').collect();
        
        if parts.len() != 4 {
            return Some(format!("IPv4 地址应该有 4 个部分，找到 {}", parts.len()));
        }
        
        for part in &parts {
            if part.is_empty() {
                return Some("IPv4 地址部分不能为空".to_string());
            }
            
            // 检查前导零
            if part.len() > 1 && part.starts_with('0') {
                return Some(format!("IPv4 地址部分 '{}' 有前导零", part));
            }
            
            match part.parse::<u8>() {
                Ok(_) => {}
                Err(_) => {
                    return Some(format!("IPv4 地址部分 '{}' 超出范围 (0-255)", part));
                }
            }
        }
        
        None
    }
}

// ==================== Task 2: New High-Value Rules ====================

/// SA9001: defer-in-range-loop
///
/// 检测 range 循环中的 defer 语句。
/// defer 在循环中会累积，可能导致资源泄漏或意外行为。
pub struct DeferInRangeLoop;

impl Rule for DeferInRangeLoop {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "SA9001",
            name: "defer-in-range-loop",
            description: "range 循环中的 defer 会累积执行，可能导致资源问题",
            category: RuleCategory::Bugbear,
            priority: RulePriority::Required,
            default_severity: Severity::Warning,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 检查 for range 循环
        if node.kind() == "for_statement" {
            let text = &source[node.byte_range()];
            
            // 检查是否是 range 循环
            if text.contains("range ") {
                if let Some(body) = node.child_by_field_name("body") {
                    // 在循环体内查找 defer
                    if self.has_defer_in_body(&body, source) {
                        let pos = node.start_position();
                        diagnostics.push(Diagnostic {
                            code: "SA9001".to_string(),
                            message: "range 循环中的 defer 会累积执行。考虑将逻辑提取为函数或使用闭包".to_string(),
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

impl DeferInRangeLoop {
    fn has_defer_in_body<'a>(&self, body: &Node<'a>, source: &str) -> bool {
        let body_text = &source[body.byte_range()];
        
        // 简单检查：循环体内是否有 defer
        if !body_text.contains("defer ") {
            return false;
        }
        
        // 递归遍历检查
        self.traverse_for_defer(*body, source)
    }
    
    fn traverse_for_defer<'a>(&self, node: Node<'a>, source: &str) -> bool {
        if node.kind() == "defer_statement" {
            return true;
        }
        
        // 如果遇到函数字面量，停止递归（defer 在那里是安全的）
        if node.kind() == "func_literal" {
            return false;
        }
        
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if self.traverse_for_defer(child, source) {
                return true;
            }
        }
        
        false
    }
}

/// SA9002: non-ascii-identifier
///
/// 检测非 ASCII 标识符。
/// Go 支持 Unicode 标识符，但建议使用 ASCII 以保证可读性和兼容性。
pub struct NonASCIIIdentifier;

impl Rule for NonASCIIIdentifier {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "SA9002",
            name: "non-ascii-identifier",
            description: "建议使用 ASCII 标识符以保证代码可读性",
            category: RuleCategory::Style,
            priority: RulePriority::Required,
            default_severity: Severity::Info,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 检查标识符
        if node.kind() == "identifier" {
            let name = &source[node.byte_range()];
            
            // 跳过常见的小写标识符（通常是关键字或标准库）
            if name.len() <= 1 {
                return diagnostics;
            }
            
            // 检查是否包含非 ASCII 字符
            if self.contains_non_ascii(name) {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    code: "SA9002".to_string(),
                    message: format!(
                        "标识符 '{}' 包含非 ASCII 字符，建议使用英文字母",
                        name
                    ),
                    severity: self.default_severity(),
                    file_path: file_path.to_string(),
                    line: pos.row + 1,
                    column: pos.column + 1,
                    fix: None,
                });
            }
        }
        
        // 检查函数声明
        if node.kind() == "function_declaration" || node.kind() == "method_declaration" {
            if let Some(name_node) = node.child_by_field_name("name") {
                let name = &source[name_node.byte_range()];
                
                if self.contains_non_ascii(name) {
                    let pos = name_node.start_position();
                    diagnostics.push(Diagnostic {
                        code: "SA9002".to_string(),
                        message: format!(
                            "函数名 '{}' 包含非 ASCII 字符，建议使用英文字母",
                            name
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
        
        // 检查类型声明
        if node.kind() == "type_declaration" {
            if let Some(spec) = node.child_by_field_name("spec") {
                if let Some(name_node) = spec.child_by_field_name("name") {
                    let name = &source[name_node.byte_range()];
                    
                    if self.contains_non_ascii(name) {
                        let pos = name_node.start_position();
                        diagnostics.push(Diagnostic {
                            code: "SA9002".to_string(),
                            message: format!(
                                "类型名 '{}' 包含非 ASCII 字符，建议使用英文字母",
                                name
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

impl NonASCIIIdentifier {
    fn contains_non_ascii(&self, s: &str) -> bool {
        s.chars().any(|c| !c.is_ascii())
    }
}

/// SA1024: string-offset-copy
///
/// 检测可能导致内存泄漏的 string 转换。
/// 当从大的 byte slice 创建小 string 时，
/// 底层数组无法被 GC，导致内存泄漏。
pub struct StringOffsetCopy;

impl Rule for StringOffsetCopy {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "SA1024",
            name: "string-offset-copy",
            description: "从大 buffer 创建的 string 可能导致内存泄漏，建议使用 string.Clone() 或 copy",
            category: RuleCategory::Performance,
            priority: RulePriority::Required,
            default_severity: Severity::Warning,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 检查类型转换: string(slice)
        if node.kind() == "call_expression" || node.kind() == "type_conversion_expression" {
            let text = &source[node.byte_range()];
            
            // 检查是否是 string() 转换
            if text.starts_with("string(") && self.has_slice_subscript(text) {
                // 检查是否有注释说明这是有意为之
                if !self.has_intentional_comment(&node, source) {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        code: "SA1024".to_string(),
                        message: "从大 buffer 切片创建的 string 可能导致内存泄漏。使用 string.Clone() 或 copy([]byte, data) 后再转换".to_string(),
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

impl StringOffsetCopy {
    fn has_slice_subscript(&self, text: &str) -> bool {
        // 检查是否包含切片操作 [:n] 或 [m:n]
        text.contains("[") && text.contains("]") && 
        (text.contains(":") || text.contains(".."))
    }
    
    fn has_intentional_comment<'a>(&self, node: &Node<'a>, source: &str) -> bool {
        // 检查前一行是否有 "//nolint:" 或 "// intentional:" 等注释
        let start_line = node.start_position().row;
        
        if start_line > 0 {
            let lines: Vec<&str> = source.lines().collect();
            if let Some(prev_line) = lines.get(start_line.saturating_sub(1)) {
                let trimmed = prev_line.trim();
                if trimmed.starts_with("//nolint") || 
                   trimmed.contains("intentional") ||
                   trimmed.contains("nolint:SA1024") {
                    return true;
                }
            }
        }
        
        false
    }
}

/// SA9003: missing-cancel-check
///
/// 检测 context.Context 取消后是否检查了 ctx.Done()。
/// 长时间运行的循环应该检查 context 取消。
pub struct MissingCancelCheck;

impl Rule for MissingCancelCheck {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "SA9003",
            name: "missing-cancel-check",
            description: "循环中使用了 context 但没有检查 ctx.Done()，可能导致 goroutine 泄漏",
            category: RuleCategory::Bugbear,
            priority: RulePriority::Required,
            default_severity: Severity::Warning,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        
        // 检查无限循环或长时间循环
        if node.kind() == "for_statement" {
            let text = &source[node.byte_range()];
            
            // 检查是否是无限循环
            let is_infinite = text.starts_with("for {") || 
                             (text.starts_with("for ") && !text.contains(";"));
            
            if is_infinite || text.contains("for {") {
                if let Some(body) = node.child_by_field_name("body") {
                    let body_text = &source[body.byte_range()];
                    
                    // 检查函数签名是否有 context 参数
                    if self.has_context_param(&node, source) {
                        // 检查循环体内是否有 select 或 ctx.Done() 检查
                        if !body_text.contains("ctx.Done()") && 
                           !body_text.contains("context.Done()") &&
                           !body_text.contains("select {") {
                            let pos = node.start_position();
                            diagnostics.push(Diagnostic {
                                code: "SA9003".to_string(),
                                message: "循环中接收了 context 参数但没有检查 ctx.Done()，可能导致 goroutine 泄漏。建议添加 select case <-ctx.Done(): return".to_string(),
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

impl MissingCancelCheck {
    fn has_context_param<'a>(&self, node: &Node<'a>, source: &str) -> bool {
        // 向上查找函数声明
        let mut current = *node;
        
        while let Some(parent) = current.parent() {
            if parent.kind() == "function_declaration" || 
               parent.kind() == "method_declaration" {
                let func_text = &source[parent.byte_range()];
                // 简单检查是否包含 context.Context
                return func_text.contains("context.Context") ||
                       func_text.contains("ctx context.Context");
            }
            current = parent;
        }
        
        false
    }
}

// ==================== Rules Export ====================

pub fn get_p0_enhanced_rules() -> Vec<Box<dyn Rule>> {
    vec![
        // Task 1: Enhanced existing rules
        Box::new(ImpossibleTypeAssertionEnhanced),
        Box::new(InvalidNetworkAddressEnhanced),
        
        // Task 2: New high-value rules
        Box::new(DeferInRangeLoop),
        Box::new(NonASCIIIdentifier),
        Box::new(StringOffsetCopy),
        Box::new(MissingCancelCheck),
    ]
}
