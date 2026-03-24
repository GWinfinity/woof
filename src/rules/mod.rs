//! Lint rules implementation.
//!
//! This module contains all the lint rules organized by category.
//!
//! # Rule Categories
//!
//! | Category | Prefix | Description |
//! |----------|--------|-------------|
//! | P0 | P0 | Critical rules (staticcheck, concurrency, generics) |
//! | Codestyle | E | Code style and syntax errors |
//! | Logic | F | Logic errors and potential runtime issues |
//! | Bugbear | B | Code quality and anti-patterns |
//! | Imports | I | Import sorting and grouping |
//! | Style | S | Style conventions |
//! | Docs | D | Documentation and comments |
//!
//! # Example
//!
//! ```
//! use woofmt::rules::{get_all_rules, RuleCategory, get_rules_by_category};
//!
//! // Get all rules
//! let all_rules = get_all_rules();
//!
//! // Get rules by category
//! let codestyle_rules = get_rules_by_category(RuleCategory::Codestyle);
//! ```

pub mod bugbear; // B-series (Uber Go Style)
pub mod builtin;
pub mod codestyle; // E-series
pub mod imports; // I-series
pub mod logic; // F-series
pub mod p0_concurrency; // P0 Phase 1: 并发安全规则
pub mod p0_critical; // P0 核心规则
pub mod p0_enhanced; // P0 Phase 3: 增强规则和高价值新增
pub mod p0_runtime; // P0 Phase 2: 运行时错误和代码结构规则
pub mod style;
pub mod upgrade; // Go 版本升级规则

use crate::{Diagnostic, Severity};
use tree_sitter::Node;

/// 诊断位置
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiagnosticPosition {
    pub line: usize,
    pub column: usize,
}

impl From<DiagnosticPosition> for (usize, usize) {
    fn from(pos: DiagnosticPosition) -> (usize, usize) {
        (pos.line, pos.column)
    }
}

/// 为 Diagnostic 添加从 Position 的转换
impl Diagnostic {
    pub fn with_position(
        code: String,
        message: String,
        severity: Severity,
        file_path: String,
        position: DiagnosticPosition,
    ) -> Self {
        Self {
            code,
            message,
            severity,
            file_path,
            line: position.line,
            column: position.column,
            fix: None,
        }
    }
}

/// 规则优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RulePriority {
    /// P0: 必须实现 - 基础风格和错误检测
    Required = 0,
    /// P1: 推荐 - 代码质量和最佳实践
    Recommended = 1,
    /// P2: 可选 - 性能、升级建议
    Optional = 2,
}

/// 规则分类（对应 Ruff/Flake8 系列）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleCategory {
    /// E-series: 代码风格与语法错误 (gocodestyle)
    Codestyle,
    /// F-series: 逻辑错误与潜在运行时问题 (goflakes)
    Logic,
    /// B-series: 代码质量与反模式 (go-bugbear)
    Bugbear,
    /// I-series: 导入排序与分组 (goimports)
    Imports,
    /// UP-series: 升级到现代 Go 语法 (goupgrade)
    Upgrade,
    /// SIM-series: 代码简化建议 (go-simplify)
    Simplify,
    /// S-series: 风格与命名
    Style,
    /// D-series: 文档与注释
    Docs,
    /// P-series: 性能优化
    Performance,
    /// C-series: 并发问题
    Concurrency,
    /// SEC-series: 安全问题
    Security,
}

impl RuleCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            RuleCategory::Codestyle => "E",
            RuleCategory::Logic => "F",
            RuleCategory::Bugbear => "B",
            RuleCategory::Imports => "I",
            RuleCategory::Upgrade => "UP",
            RuleCategory::Simplify => "SIM",
            RuleCategory::Style => "S",
            RuleCategory::Docs => "D",
            RuleCategory::Performance => "P",
            RuleCategory::Concurrency => "C",
            RuleCategory::Security => "SEC",
        }
    }
}

/// 规则元数据
pub struct RuleMetadata {
    pub code: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub category: RuleCategory,
    pub priority: RulePriority,
    pub default_severity: Severity,
    /// 最低要求的 Go 版本（可选）
    /// 例如 "1.22" 表示该规则只在目标 Go 版本 >= 1.22 时启用
    pub min_go_version: Option<&'static str>,
}

/// 规则 trait - 所有 lint 规则必须实现
pub trait Rule: Send + Sync {
    /// 规则元数据
    fn metadata(&self) -> RuleMetadata;

    /// 检查节点
    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic>;

    /// 便捷方法
    fn code(&self) -> &'static str {
        self.metadata().code
    }

    fn name(&self) -> &'static str {
        self.metadata().name
    }

    fn description(&self) -> &'static str {
        self.metadata().description
    }

    fn default_severity(&self) -> Severity {
        self.metadata().default_severity
    }
}

/// 获取所有规则
pub fn get_all_rules() -> Vec<Box<dyn Rule>> {
    let mut rules: Vec<Box<dyn Rule>> = vec![];

    // P0: 核心关键规则 (staticcheck, 并发, 泛型, Fuzzing, Workspace)
    rules.extend(p0_critical::get_p0_rules());

    // P0 Phase 1: 并发安全规则 (11条)
    rules.extend(p0_concurrency::get_p0_concurrency_rules());

    // P0 Phase 2: 运行时错误和代码结构规则 (15条)
    rules.extend(p0_runtime::get_p0_runtime_rules());

    // P0 Phase 3: 增强规则和高价值新增 (4条)
    rules.extend(p0_enhanced::get_p0_enhanced_rules());

    // E-series: 代码风格 (gocodestyle)
    rules.extend(codestyle::get_rules());

    // F-series: 逻辑错误 (goflakes)
    rules.extend(logic::get_rules());

    // I-series: 导入 (goimports)
    rules.extend(imports::get_rules());

    // S-series: 风格规范
    rules.extend(style::get_rules());

    // D-series: 文档规范
    rules.extend(builtin::get_doc_rules());

    // B-series: Uber Go Style / Bugbear
    rules.extend(bugbear::get_rules());

    // UP-series: Go 版本升级规则 (Go 1.22/1.23/1.24/1.25+)
    rules.extend(upgrade::get_upgrade_rules());

    rules
}

/// 根据类别获取规则
pub fn get_rules_by_category(category: RuleCategory) -> Vec<Box<dyn Rule>> {
    get_all_rules()
        .into_iter()
        .filter(|r| r.metadata().category == category)
        .collect()
}

/// 根据优先级获取规则
pub fn get_rules_by_priority(priority: RulePriority) -> Vec<Box<dyn Rule>> {
    get_all_rules()
        .into_iter()
        .filter(|r| r.metadata().priority == priority)
        .collect()
}

/// 解析版本字符串为可比数字 (e.g., "1.22" -> 1022)
pub fn parse_version(v: &str) -> Option<u32> {
    let parts: Vec<&str> = v.split('.').collect();
    if parts.len() >= 2 {
        let major = parts[0].parse::<u32>().ok()?;
        let minor = parts[1].parse::<u32>().ok()?;
        Some(major * 1000 + minor)
    } else {
        None
    }
}

/// 获取启用的规则，根据配置的目标 Go 版本过滤
pub fn get_enabled_rules(config: &crate::config::Config) -> Vec<Box<dyn Rule>> {
    let target_version = parse_version(&config.global.target_go_version).unwrap_or(1021); // 默认 1.21

    get_all_rules()
        .into_iter()
        .filter(|r| {
            // 检查规则是否有最低 Go 版本要求
            if let Some(min_ver_str) = r.metadata().min_go_version {
                if let Some(min_ver) = parse_version(min_ver_str) {
                    // 只有当目标版本 >= 规则最低版本时才启用
                    return target_version >= min_ver;
                }
            }
            // 没有版本要求的规则总是启用
            true
        })
        .collect()
}

#[cfg(test)]
mod tests;
