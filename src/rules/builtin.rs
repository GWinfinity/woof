//! Builtin rules - 基础规则（向后兼容）
//!
//! 注意：这些规则将逐步迁移到新的分类模块中

use super::{Rule, RuleCategory, RuleMetadata, RulePriority};
use crate::{Diagnostic, Severity};
use tree_sitter::Node;

/// D001: Exported identifier missing documentation
pub struct ExportedMissingDoc;

impl Rule for ExportedMissingDoc {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "D001",
            name: "exported-missing-doc",
            description: "导出的标识符应有文档注释",
            category: RuleCategory::Docs,
            priority: RulePriority::Recommended,
            default_severity: Severity::Info,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        // Check for exported functions, types, variables, constants
        let check_kinds = [
            "function_declaration",
            "type_declaration",
            "var_declaration",
            "const_declaration",
        ];

        if check_kinds.contains(&node.kind()) {
            // Check if there's a comment before this node
            let mut has_doc_comment = false;

            // Look for comment before node in source
            let prev_text = &source[..node.start_byte()];
            if let Some(last_comment) = prev_text.rfind("//") {
                let after_comment = &prev_text[last_comment..];
                if !after_comment.contains('\n')
                    || after_comment.find('\n').unwrap() > after_comment.find("//").unwrap()
                {
                    has_doc_comment = true;
                }
            }

            // Check if name is exported (starts with uppercase)
            if let Some(name_node) = get_declaration_name(node) {
                let name = &source[name_node.byte_range()];
                if name
                    .chars()
                    .next()
                    .map(|c| c.is_uppercase())
                    .unwrap_or(false)
                    && !has_doc_comment
                {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        code: "D001".to_string(),
                        message: format!("导出的 '{}' 应有文档注释", name),
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

/// D002: package documentation missing
pub struct PackageDocMissing;

impl Rule for PackageDocMissing {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "D002",
            name: "package-doc-missing",
            description: "包缺少文档注释",
            category: RuleCategory::Docs,
            priority: RulePriority::Recommended,
            default_severity: Severity::Info,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        if node.kind() == "package_clause" {
            // 检查 package 前面是否有注释
            let prev_text = &source[..node.start_byte()];

            let has_doc = prev_text.trim().ends_with("*/")
                || prev_text.lines().rev().any(|line| {
                    let trimmed = line.trim();
                    trimmed.starts_with("//") && !trimmed.starts_with("//go:")
                });

            if !has_doc {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    code: "D002".to_string(),
                    message: "包应有一个文档注释".to_string(),
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

/// D003: TODO format
pub struct TodoFormat;

impl Rule for TodoFormat {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "D003",
            name: "todo-format",
            description: "TODO 注释应符合格式",
            category: RuleCategory::Docs,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        if node.kind() == "comment" {
            let comment_text = &source[node.byte_range()];
            let lower = comment_text.to_lowercase();

            // 检查 TODO 格式
            if lower.contains("todo") {
                // 检查是否有用户名和描述
                if !comment_text.contains("TODO(") && !comment_text.contains("todo(") {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        code: "D003".to_string(),
                        message: "TODO 应包含用户名: TODO(username): description".to_string(),
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

/// D004: deprecated format
pub struct DeprecatedFormat;

impl Rule for DeprecatedFormat {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "D004",
            name: "deprecated-format",
            description: "Deprecated 注释应符合格式",
            category: RuleCategory::Docs,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
            min_go_version: None,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        if node.kind() == "comment" {
            let comment_text = &source[node.byte_range()];
            let lower = comment_text.to_lowercase();

            if lower.contains("deprecated") {
                // 应该在新的一行并且大写
                if !comment_text.contains("Deprecated:") {
                    let pos = node.start_position();
                    diagnostics.push(Diagnostic {
                        code: "D004".to_string(),
                        message: "Deprecated 注释应以 'Deprecated:' 开头".to_string(),
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
fn get_declaration_name(node: Node) -> Option<Node> {
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            if cursor.node().kind() == "identifier" {
                return Some(cursor.node());
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
    None
}

/// 获取文档规则
pub fn get_doc_rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(ExportedMissingDoc),
        Box::new(PackageDocMissing),
        Box::new(TodoFormat),
        Box::new(DeprecatedFormat),
    ]
}
