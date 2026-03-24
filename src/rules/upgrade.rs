//! Upgrade rules - Go 版本升级建议 (Go 1.22/1.23/1.24/1.25+)
//!
//! 这些规则根据配置的 target_go_version 自动启用/禁用。
//! 默认目标版本是 1.21，不会触发这些规则。
//! 设置 target_go_version = "1.25" 以启用所有规则。

use super::{Rule, RuleCategory, RuleMetadata, RulePriority};
use crate::{Diagnostic, Severity};
use tree_sitter::Node;

// ==================== Go 1.22 Rules ====================

/// UP1221: 使用整数 range 替代传统 for 循环
pub struct Up1221IntegerRange;

impl Rule for Up1221IntegerRange {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1221",
            name: "use-integer-range",
            description: "Go 1.22+ 建议使用 for i := range N",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
            min_go_version: Some("1.22"),
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        if node.kind() == "for_statement" {
            let for_text = &source[node.byte_range()];
            if for_text.contains("for ")
                && for_text.contains(":= 0")
                && for_text.contains("i++")
                && (for_text.contains("i < ") || for_text.contains("i <="))
            {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    code: "UP1221".to_string(),
                    message: "Go 1.22+ 建议使用 for i := range N".to_string(),
                    severity: Severity::Info,
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

/// UP1222: 使用 math/rand/v2
pub struct Up1222MathRandV2;

impl Rule for Up1222MathRandV2 {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1222",
            name: "use-math-rand-v2",
            description: "Go 1.22+ 建议使用 math/rand/v2 (ChaCha8 算法，性能更好)",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
            min_go_version: Some("1.22"),
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        if node.kind() == "import_spec" {
            let import_text = &source[node.byte_range()];
            if import_text.contains("math/rand") && !import_text.contains("math/rand/v2") {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    code: "UP1222".to_string(),
                    message: "Go 1.22+ 建议使用 math/rand/v2 (ChaCha8 算法，性能更好)".to_string(),
                    severity: Severity::Info,
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

/// UP1223: 使用 HTTP 路由通配符
pub struct Up1223HttpRoute;

impl Rule for Up1223HttpRoute {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1223",
            name: "use-http-route-pattern",
            description: "Go 1.22+ net/http 支持路由通配符 (如 /items/{id})",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
            min_go_version: Some("1.22"),
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        if node.kind() == "call_expression" {
            let call_text = &source[node.byte_range()];
            if call_text.contains("strings.Split")
                && source.contains("r.URL.Path")
                && source.contains("http.HandleFunc")
            {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    code: "UP1223".to_string(),
                    message: "Go 1.22+ net/http 支持路由通配符 (如 /items/{id})".to_string(),
                    severity: Severity::Info,
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

/// UP1224: 循环变量改进
pub struct Up1224Loopvar;

impl Rule for Up1224Loopvar {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1224",
            name: "loopvar-improved",
            description: "Go 1.22+ 循环变量每次迭代都是新的，无需传参",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
            min_go_version: Some("1.22"),
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        if node.kind() == "go_statement" {
            let text = &source[node.byte_range()];
            if text.contains("go func(") && text.contains(")(v)") && source.contains("for ") {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    code: "UP1224".to_string(),
                    message: "Go 1.22+ 循环变量每次迭代都是新的，无需传参".to_string(),
                    severity: Severity::Info,
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

/// UP1225: 使用 slices.Concat
pub struct Up1225SlicesConcat;

impl Rule for Up1225SlicesConcat {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1225",
            name: "use-slices-concat",
            description: "Go 1.22+ 建议使用 slices.Concat",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
            min_go_version: Some("1.22"),
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        if node.kind() == "call_expression" {
            let call_text = &source[node.byte_range()];
            if call_text.contains("append(") && call_text.contains("...") {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    code: "UP1225".to_string(),
                    message: "Go 1.22+ 建议使用 slices.Concat".to_string(),
                    severity: Severity::Info,
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

// ==================== Go 1.23 Rules ====================

/// UP1231: 使用 slices 迭代器
pub struct Up1231SlicesIter;

impl Rule for Up1231SlicesIter {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1231",
            name: "use-slices-iterator",
            description: "Go 1.23+ 建议使用 slices.All/Values/Backward 迭代器",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
            min_go_version: Some("1.23"),
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        if node.kind() == "for_statement" {
            let for_text = &source[node.byte_range()];
            if for_text.contains("range ") && !source.contains("slices") {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    code: "UP1231".to_string(),
                    message: "Go 1.23+ 可以使用 slices.All/Values/Backward 迭代器".to_string(),
                    severity: Severity::Info,
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

/// UP1232: 使用 unique.Make
pub struct Up1232UniqueMake;

impl Rule for Up1232UniqueMake {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1232",
            name: "use-unique-make",
            description: "Go 1.23+ 建议使用 unique.Make 替代手动 interning",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
            min_go_version: Some("1.23"),
        }
    }

    fn check(&self, _node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        if source.contains("intern")
            && source.contains("map[string]")
            && !source.contains("unique.Make")
        {
            diagnostics.push(Diagnostic {
                code: "UP1232".to_string(),
                message: "Go 1.23+ 建议使用 unique.Make 替代手动字符串 interning".to_string(),
                severity: Severity::Info,
                file_path: file_path.to_string(),
                line: 1,
                column: 1,
                fix: None,
            });
        }
        diagnostics
    }
}

/// UP1233: 使用 iter.Seq
pub struct Up1233IterSeq;

impl Rule for Up1233IterSeq {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1233",
            name: "use-iter-seq",
            description: "Go 1.23+ 建议使用 iter.Seq 替代返回 channel 的遍历函数",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
            min_go_version: Some("1.23"),
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        if node.kind() == "function_declaration" {
            let func_text = &source[node.byte_range()];
            if func_text.contains("func ") && func_text.contains("<-chan ") {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    code: "UP1233".to_string(),
                    message: "Go 1.23+ 建议使用 iter.Seq 替代返回 channel 的遍历函数".to_string(),
                    severity: Severity::Info,
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

/// UP1234: 使用 maps 迭代器
pub struct Up1234MapsIter;

impl Rule for Up1234MapsIter {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1234",
            name: "use-maps-iterator",
            description: "Go 1.23+ 建议使用 maps.Keys/Values",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
            min_go_version: Some("1.23"),
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        if node.kind() == "for_statement" {
            let text = &source[node.byte_range()];
            if text.contains("range ")
                && source.contains("append(")
                && !source.contains("maps.Keys")
            {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    code: "UP1234".to_string(),
                    message: "Go 1.23+ 建议使用 maps.Keys/Values".to_string(),
                    severity: Severity::Info,
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

/// UP1235: 使用 sync.Map.Clear
pub struct Up1235SyncMapClear;

impl Rule for Up1235SyncMapClear {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1235",
            name: "sync-map-clear",
            description: "Go 1.23+ sync.Map.Clear() 替代手动遍历删除",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
            min_go_version: Some("1.23"),
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        if node.kind() == "for_statement" {
            let text = &source[node.byte_range()];
            if text.contains("Range(") && text.contains("Delete(") && source.contains("sync.Map") {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    code: "UP1235".to_string(),
                    message: "Go 1.23+ sync.Map.Clear() 替代手动遍历删除".to_string(),
                    severity: Severity::Info,
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

// ==================== Go 1.24 Rules ====================

/// UP1241: 使用 crypto/rand.Text()
pub struct Up1241RandText;

impl Rule for Up1241RandText {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1241",
            name: "use-rand-text",
            description: "Go 1.24+ 建议使用 crypto/rand.Text() 生成安全随机字符串",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Recommended,
            default_severity: Severity::Info,
            min_go_version: Some("1.24"),
        }
    }

    fn check(&self, _node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        if source.contains("randomString") || source.contains("generateToken") {
            diagnostics.push(Diagnostic {
                code: "UP1241".to_string(),
                message: "Go 1.24+ 建议使用 crypto/rand.Text() 生成密码安全的随机字符串"
                    .to_string(),
                severity: Severity::Info,
                file_path: file_path.to_string(),
                line: 1,
                column: 1,
                fix: None,
            });
        }
        diagnostics
    }
}

/// UP1242: 使用 slog.DiscardHandler
pub struct Up1242SlogDiscard;

impl Rule for Up1242SlogDiscard {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1242",
            name: "use-slog-discard",
            description: "Go 1.24+ 建议使用 slog.DiscardHandler 替代自定义的无操作 handler",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
            min_go_version: Some("1.24"),
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        if node.kind() == "method_declaration" {
            let text = &source[node.byte_range()];
            if text.contains("slog.Handler") && text.contains("return false") {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    code: "UP1242".to_string(),
                    message: "Go 1.24+ 建议使用 slog.DiscardHandler".to_string(),
                    severity: Severity::Info,
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

/// UP1243: 使用 testing.T.Context()
pub struct Up1243TestingContext;

impl Rule for Up1243TestingContext {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1243",
            name: "use-testing-context",
            description: "Go 1.24+ 测试建议使用 t.Context()",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Recommended,
            default_severity: Severity::Info,
            min_go_version: Some("1.24"),
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        if file_path.contains("_test.go") && node.kind() == "call_expression" {
            let text = &source[node.byte_range()];
            if text.contains("context.Background()") || text.contains("context.TODO()") {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    code: "UP1243".to_string(),
                    message: "Go 1.24+ 测试建议使用 t.Context()".to_string(),
                    severity: Severity::Info,
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

/// UP1244: 使用 maphash.Comparable
pub struct Up1244MaphashComparable;

impl Rule for Up1244MaphashComparable {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1244",
            name: "use-maphash-comparable",
            description: "Go 1.24+ 建议使用 maphash.Comparable",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
            min_go_version: Some("1.24"),
        }
    }

    fn check(&self, _node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        if source.contains("interface{}") && source.contains("hash") && !source.contains("maphash")
        {
            diagnostics.push(Diagnostic {
                code: "UP1244".to_string(),
                message: "Go 1.24+ 建议使用 maphash.Comparable 替代类型断言式哈希".to_string(),
                severity: Severity::Info,
                file_path: file_path.to_string(),
                line: 1,
                column: 1,
                fix: None,
            });
        }
        diagnostics
    }
}

// ==================== Go 1.25 Rules ====================

/// UP1251: 考虑使用 encoding/json/v2
pub struct Up1251JsonV2;

impl Rule for Up1251JsonV2 {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1251",
            name: "consider-json-v2",
            description: "Go 1.25+ 可考虑使用实验性的 encoding/json/v2 (3-10x 性能提升)",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
            min_go_version: Some("1.25"),
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        if node.kind() == "import_declaration" && source.contains("encoding/json") {
            let count =
                source.matches("json.Marshal").count() + source.matches("json.Unmarshal").count();
            if count >= 3 {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    code: "UP1251".to_string(),
                    message: format!("Go 1.25+ json/v2 可提升性能 ({} 次调用)", count),
                    severity: Severity::Info,
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

/// UP1252: 使用 testing/synctest
pub struct Up1252Synctest;

impl Rule for Up1252Synctest {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1252",
            name: "use-synctest",
            description: "Go 1.25+ 建议使用 testing/synctest 测试并发代码",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Recommended,
            default_severity: Severity::Info,
            min_go_version: Some("1.25"),
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        if file_path.contains("_test.go") && node.kind() == "call_expression" {
            let text = &source[node.byte_range()];
            if text.contains("time.Sleep") && source.contains("go func") {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    code: "UP1252".to_string(),
                    message: "Go 1.25+ 建议使用 testing/synctest 替代 time.Sleep 测试并发"
                        .to_string(),
                    severity: Severity::Info,
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

/// UP1253: 容器感知 GOMAXPROCS 已自动启用
pub struct Up1253ContainerAware;

impl Rule for Up1253ContainerAware {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1253",
            name: "container-aware-gomaxprocs",
            description: "Go 1.25+ 运行时自动容器感知 GOMAXPROCS，无需手动配置",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
            min_go_version: Some("1.25"),
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        if node.kind() == "call_expression" {
            let text = &source[node.byte_range()];
            if text.contains("automaxprocs") || text.contains("runtime.GOMAXPROCS") {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    code: "UP1253".to_string(),
                    message: "Go 1.25+ 运行时自动容器感知 GOMAXPROCS".to_string(),
                    severity: Severity::Warning,
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

/// UP1254: runtime.GOROOT 已弃用
pub struct Up1254GorootDeprecated;

impl Rule for Up1254GorootDeprecated {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1254",
            name: "runtime-goroot-deprecated",
            description: "Go 1.25+ runtime.GOROOT() 已弃用",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
            min_go_version: Some("1.25"),
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        if node.kind() == "call_expression" {
            let text = &source[node.byte_range()];
            if text.contains("runtime.GOROOT") {
                let pos = node.start_position();
                diagnostics.push(Diagnostic {
                    code: "UP1254".to_string(),
                    message: "Go 1.25+ runtime.GOROOT() 已弃用".to_string(),
                    severity: Severity::Warning,
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

/// 获取所有升级规则
pub fn get_upgrade_rules() -> Vec<Box<dyn Rule>> {
    vec![
        // Go 1.22
        Box::new(Up1221IntegerRange),
        Box::new(Up1222MathRandV2),
        Box::new(Up1223HttpRoute),
        Box::new(Up1224Loopvar),
        Box::new(Up1225SlicesConcat),
        // Go 1.23
        Box::new(Up1231SlicesIter),
        Box::new(Up1232UniqueMake),
        Box::new(Up1233IterSeq),
        Box::new(Up1234MapsIter),
        Box::new(Up1235SyncMapClear),
        // Go 1.24
        Box::new(Up1241RandText),
        Box::new(Up1242SlogDiscard),
        Box::new(Up1243TestingContext),
        Box::new(Up1244MaphashComparable),
        // Go 1.25
        Box::new(Up1251JsonV2),
        Box::new(Up1252Synctest),
        Box::new(Up1253ContainerAware),
        Box::new(Up1254GorootDeprecated),
    ]
}
