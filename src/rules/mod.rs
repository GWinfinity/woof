pub mod builtin;
pub mod style;
pub mod codestyle;    // E-series
pub mod logic;        // F-series
pub mod imports;      // I-series
pub mod bugbear;      // B-series (Uber Go Style)
pub mod p0_critical;  // P0 核心规则
pub mod p0_concurrency; // P0 Phase 1: 并发安全规则
pub mod p0_runtime;   // P0 Phase 2: 运行时错误和代码结构规则
pub mod p0_enhanced;  // P0 Phase 3: 增强规则和高价值新增

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

/// 获取启用的规则（简化版：返回所有规则）
/// TODO: 根据配置过滤
pub fn get_enabled_rules(_config: &crate::config::Config) -> Vec<Box<dyn Rule>> {
    get_all_rules()
}
