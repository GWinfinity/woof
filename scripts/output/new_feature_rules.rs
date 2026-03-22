//! Woof 新特性规则集
//! 针对 Go 泛型、Fuzzing、Workspace 等新特性
//! 生成时间: 2026-03-22T10:07:16.248435
//! 规则数量: 71

use crate::rules::{Rule, RuleMetadata, RuleCategory, RulePriority};
use crate::{Diagnostic, Severity};
use tree_sitter::Node;

// ========== FUZZING RULES ==========

pub struct FUZZ_TEST_SIGNATURE;

impl Rule for FUZZ_TEST_SIGNATURE {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "FUZZ001",
            name: "fuzz-test-signature",
            description: "Fuzzing 测试函数签名错误",
            category: RuleCategory::Docs,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct FUZZ_TARGET_NAME;

impl Rule for FUZZ_TARGET_NAME {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "FUZZ002",
            name: "fuzz-target-name",
            description: "Fuzzing 目标命名不规范",
            category: RuleCategory::Docs,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct FUZZ_MISSING_CORPUS;

impl Rule for FUZZ_MISSING_CORPUS {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "FUZZ003",
            name: "fuzz-missing-corpus",
            description: "缺少 Fuzzing 语料库目录",
            category: RuleCategory::Docs,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct FUZZ_SEED_CORPUS;

impl Rule for FUZZ_SEED_CORPUS {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "FUZZ004",
            name: "fuzz-seed-corpus",
            description: "建议使用种子语料库",
            category: RuleCategory::Docs,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct FUZZ_PARALLEL;

impl Rule for FUZZ_PARALLEL {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "FUZZ005",
            name: "fuzz-parallel",
            description: "Fuzzing 测试应考虑并行执行",
            category: RuleCategory::Docs,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct FUZZ_CLEANUP;

impl Rule for FUZZ_CLEANUP {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "FUZZ006",
            name: "fuzz-cleanup",
            description: "Fuzzing 测试后缺少清理",
            category: RuleCategory::Docs,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct FUZZ_ADD_DUPLICATE;

impl Rule for FUZZ_ADD_DUPLICATE {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "FUZZ007",
            name: "fuzz-add-duplicate",
            description: "Fuzzing 中添加重复语料",
            category: RuleCategory::Docs,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct FUZZ_SKIPPED;

impl Rule for FUZZ_SKIPPED {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "FUZZ008",
            name: "fuzz-skipped",
            description: "Fuzzing 测试被跳过",
            category: RuleCategory::Docs,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct FUZZ_TIMEOUT;

impl Rule for FUZZ_TIMEOUT {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "FUZZ009",
            name: "fuzz-timeout",
            description: "Fuzzing 测试应设置超时",
            category: RuleCategory::Docs,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct FUZZ_PANIC_RECOVER;

impl Rule for FUZZ_PANIC_RECOVER {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "FUZZ010",
            name: "fuzz-panic-recover",
            description: "Fuzzing 中不建议使用 panic/recover",
            category: RuleCategory::Docs,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct FUZZ_GLOBAL_STATE;

impl Rule for FUZZ_GLOBAL_STATE {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "FUZZ011",
            name: "fuzz-global-state",
            description: "Fuzzing 测试使用了全局状态",
            category: RuleCategory::Docs,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct FUZZ_NON_DETERMINISTIC;

impl Rule for FUZZ_NON_DETERMINISTIC {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "FUZZ012",
            name: "fuzz-non-deterministic",
            description: "Fuzzing 测试非确定性行为",
            category: RuleCategory::Docs,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct FUZZ_INPUT_VALIDATION;

impl Rule for FUZZ_INPUT_VALIDATION {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "FUZZ101",
            name: "fuzz-input-validation",
            description: "Fuzzing 输入应充分验证",
            category: RuleCategory::Docs,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct FUZZ_EXPENSIVE_OP;

impl Rule for FUZZ_EXPENSIVE_OP {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "FUZZ102",
            name: "fuzz-expensive-op",
            description: "Fuzzing 循环中有昂贵操作",
            category: RuleCategory::Docs,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct FUZZ_FILE_OPERATION;

impl Rule for FUZZ_FILE_OPERATION {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "FUZZ103",
            name: "fuzz-file-operation",
            description: "Fuzzing 测试中避免文件操作",
            category: RuleCategory::Docs,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct FUZZ_NETWORK_ACCESS;

impl Rule for FUZZ_NETWORK_ACCESS {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "FUZZ104",
            name: "fuzz-network-access",
            description: "Fuzzing 测试中避免网络访问",
            category: RuleCategory::Docs,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

// ========== GENERICS RULES ==========

pub struct UNUSED_TYPE_PARAMETER;

impl Rule for UNUSED_TYPE_PARAMETER {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "GEN001",
            name: "unused-type-parameter",
            description: "未使用的类型参数",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct TYPE_PARAMETER_SHADOW;

impl Rule for TYPE_PARAMETER_SHADOW {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "GEN002",
            name: "type-parameter-shadow",
            description: "类型参数遮蔽外部类型",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct ANY_CONSTRAINT;

impl Rule for ANY_CONSTRAINT {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "GEN003",
            name: "any-constraint",
            description: "使用 any 约束而非 interface{}",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct COMPARABLE_MISUSE;

impl Rule for COMPARABLE_MISUSE {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "GEN004",
            name: "comparable-misuse",
            description: "comparable 约束的误用",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct TYPE_SET_REDUNDANT;

impl Rule for TYPE_SET_REDUNDANT {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "GEN005",
            name: "type-set-redundant",
            description: "冗余的类型集合",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct GENERIC_INTERFACE_METHOD;

impl Rule for GENERIC_INTERFACE_METHOD {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "GEN006",
            name: "generic-interface-method",
            description: "泛型接口中的方法限制",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct TYPE_INFERENCE_FAIL;

impl Rule for TYPE_INFERENCE_FAIL {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "GEN007",
            name: "type-inference-fail",
            description: "无法推断类型参数",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct TYPE_PARAM_TOO_MANY;

impl Rule for TYPE_PARAM_TOO_MANY {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "GEN008",
            name: "type-param-too-many",
            description: "过多的类型参数",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct CONSTRAINT_UNNECESSARY;

impl Rule for CONSTRAINT_UNNECESSARY {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "GEN009",
            name: "constraint-unnecessary",
            description: "不必要的类型约束",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct GENERIC_NAMING;

impl Rule for GENERIC_NAMING {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "GEN010",
            name: "generic-naming",
            description: "类型参数命名不规范 (应为大写字母)",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct TYPE_ASSERTION_IN_GENERIC;

impl Rule for TYPE_ASSERTION_IN_GENERIC {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "GEN011",
            name: "type-assertion-in-generic",
            description: "泛型函数中不必要的类型断言",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct REFLECT_IN_GENERIC;

impl Rule for REFLECT_IN_GENERIC {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "GEN012",
            name: "reflect-in-generic",
            description: "泛型函数中使用 reflect 而非类型参数",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct POINTER_RECEIVER_GENERIC;

impl Rule for POINTER_RECEIVER_GENERIC {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "GEN013",
            name: "pointer-receiver-generic",
            description: "泛型类型的指针接收器检查",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct TYPE_PARAM_BOUNDS;

impl Rule for TYPE_PARAM_BOUNDS {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "GEN014",
            name: "type-param-bounds",
            description: "类型参数边界检查",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct INSTANTIATION_ERROR;

impl Rule for INSTANTIATION_ERROR {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "GEN015",
            name: "instantiation-error",
            description: "泛型实例化错误",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct GENERIC_BOXING;

impl Rule for GENERIC_BOXING {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "GEN101",
            name: "generic-boxing",
            description: "泛型装箱操作可能影响性能",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct GENERIC_MONOMORPHIZATION;

impl Rule for GENERIC_MONOMORPHIZATION {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "GEN102",
            name: "generic-monomorphization",
            description: "建议泛型单态化优化",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

// ========== GO1.20 RULES ==========

pub struct USE_CONTEXT_WITH_CANCEL_CAUSE;

impl Rule for USE_CONTEXT_WITH_CANCEL_CAUSE {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1201",
            name: "use-context-with-cancel-cause",
            description: "使用 context.WithCancelCause 替代 WithCancel",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.20
        vec![]
    }
}

pub struct USE_ERRORS_JOIN;

impl Rule for USE_ERRORS_JOIN {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1202",
            name: "use-errors-join",
            description: "使用 errors.Join 替代自定义错误合并",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.20
        vec![]
    }
}

pub struct USE_SLICES_BINARY_SEARCH;

impl Rule for USE_SLICES_BINARY_SEARCH {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1203",
            name: "use-slices-binary-search",
            description: "使用 slices.BinarySearch 替代手动实现",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.20
        vec![]
    }
}

pub struct USE_MAPS_CLONE;

impl Rule for USE_MAPS_CLONE {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1204",
            name: "use-maps-clone",
            description: "使用 maps.Clone 替代手动拷贝",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.20
        vec![]
    }
}

pub struct USE_MAPS_DELETE_FUNC;

impl Rule for USE_MAPS_DELETE_FUNC {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1205",
            name: "use-maps-delete-func",
            description: "使用 maps.DeleteFunc 替代循环删除",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.20
        vec![]
    }
}

pub struct USE_ATOMIC_TYPES;

impl Rule for USE_ATOMIC_TYPES {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1206",
            name: "use-atomic-types",
            description: "使用 atomic.Int64 等类型替代 atomic.AddInt64",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.20
        vec![]
    }
}

// ========== GO1.21 RULES ==========

pub struct USE_SLOG_CONTEXT;

impl Rule for USE_SLOG_CONTEXT {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1211",
            name: "use-slog-context",
            description: "使用 slog 的 Context 方法",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.21
        vec![]
    }
}

pub struct SLOG_ATTR_OPTIMIZATION;

impl Rule for SLOG_ATTR_OPTIMIZATION {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1212",
            name: "slog-attr-optimization",
            description: "slog 属性优化建议",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.21
        vec![]
    }
}

pub struct SLOG_GROUP_NESTING;

impl Rule for SLOG_GROUP_NESTING {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1213",
            name: "slog-group-nesting",
            description: "slog group 嵌套过深",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.21
        vec![]
    }
}

pub struct SLOG_DUPLICATE_ATTRS;

impl Rule for SLOG_DUPLICATE_ATTRS {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1214",
            name: "slog-duplicate-attrs",
            description: "slog 重复属性",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.21
        vec![]
    }
}

pub struct USE_SLICES_SORT_FUNC;

impl Rule for USE_SLICES_SORT_FUNC {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1215",
            name: "use-slices-sort-func",
            description: "使用 slices.SortFunc 替代 sort.Slice",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.21
        vec![]
    }
}

pub struct USE_SLICES_REVERSE;

impl Rule for USE_SLICES_REVERSE {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1216",
            name: "use-slices-reverse",
            description: "使用 slices.Reverse",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.21
        vec![]
    }
}

pub struct USE_TESTING_OPTION;

impl Rule for USE_TESTING_OPTION {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1217",
            name: "use-testing-option",
            description: "使用 testing 包的新 Option 类型",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.21
        vec![]
    }
}

pub struct TEST_SKIP_F;

impl Rule for TEST_SKIP_F {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1218",
            name: "test-skip-f",
            description: "使用 t.Skipf 而非 fmt.Sprintf + t.Skip",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.21
        vec![]
    }
}

// ========== GO1.22 RULES ==========

pub struct LOOPVAR_CAPTURE_FIXED;

impl Rule for LOOPVAR_CAPTURE_FIXED {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1221",
            name: "loopvar-capture-fixed",
            description: "Go 1.22+ 循环变量捕获已修复，无需 workarounds",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.22
        vec![]
    }
}

pub struct RANGE_OVER_INT;

impl Rule for RANGE_OVER_INT {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1222",
            name: "range-over-int",
            description: "使用整数 range (Go 1.22+)",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.22
        vec![]
    }
}

pub struct RANGE_OVER_FUNC;

impl Rule for RANGE_OVER_FUNC {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1223",
            name: "range-over-func",
            description: "使用函数 range (Go 1.23+)",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.22
        vec![]
    }
}

pub struct USE_HTTP_SERVEMUX_PATTERNS;

impl Rule for USE_HTTP_SERVEMUX_PATTERNS {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1224",
            name: "use-http-servemux-patterns",
            description: "使用 http.ServeMux 的新模式语法",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.22
        vec![]
    }
}

pub struct HTTP_ROUTE_CONFLICT;

impl Rule for HTTP_ROUTE_CONFLICT {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "UP1225",
            name: "http-route-conflict",
            description: "HTTP 路由模式冲突",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.22
        vec![]
    }
}

// ========== WORKSPACE RULES ==========

pub struct WORKSPACE_GO_VERSION;

impl Rule for WORKSPACE_GO_VERSION {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "WS001",
            name: "workspace-go-version",
            description: "Workspace 中 Go 版本不一致",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct WORKSPACE_MODULE_PATH;

impl Rule for WORKSPACE_MODULE_PATH {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "WS002",
            name: "workspace-module-path",
            description: "Workspace 模块路径冲突",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct WORKSPACE_MISSING_MODULE;

impl Rule for WORKSPACE_MISSING_MODULE {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "WS003",
            name: "workspace-missing-module",
            description: "Workspace 缺少必要模块",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct WORKSPACE_ORPHAN_MODULE;

impl Rule for WORKSPACE_ORPHAN_MODULE {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "WS004",
            name: "workspace-orphan-module",
            description: "Workspace 中孤立的模块",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct WORKSPACE_REPLACE_DIRECTIVE;

impl Rule for WORKSPACE_REPLACE_DIRECTIVE {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "WS005",
            name: "workspace-replace-directive",
            description: "Workspace 中不当的 replace 指令",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct WORKSPACE_EXCLUDE_DIRECTIVE;

impl Rule for WORKSPACE_EXCLUDE_DIRECTIVE {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "WS006",
            name: "workspace-exclude-directive",
            description: "Workspace 中不当的 exclude 指令",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct WORKSPACE_USE_DIRECTIVE;

impl Rule for WORKSPACE_USE_DIRECTIVE {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "WS007",
            name: "workspace-use-directive",
            description: "Workspace 中 use 指令问题",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct WORKSPACE_NESTED;

impl Rule for WORKSPACE_NESTED {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "WS008",
            name: "workspace-nested",
            description: "嵌套的 Workspace 配置",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct WORKSPACE_VENDOR_CONFLICT;

impl Rule for WORKSPACE_VENDOR_CONFLICT {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "WS009",
            name: "workspace-vendor-conflict",
            description: "Workspace 与 vendor 冲突",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct WORKSPACE_LOCAL_PATH;

impl Rule for WORKSPACE_LOCAL_PATH {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "WS010",
            name: "workspace-local-path",
            description: "Workspace 使用相对路径可能的问题",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct WORKSPACE_DEP_VERSION_MISMATCH;

impl Rule for WORKSPACE_DEP_VERSION_MISMATCH {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "WS101",
            name: "workspace-dep-version-mismatch",
            description: "Workspace 依赖版本不一致",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct WORKSPACE_DEP_OVERRIDE;

impl Rule for WORKSPACE_DEP_OVERRIDE {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "WS102",
            name: "workspace-dep-override",
            description: "Workspace 依赖覆盖",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Optional,
            default_severity: Severity::Info,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct WORKSPACE_DEP_CYCLE;

impl Rule for WORKSPACE_DEP_CYCLE {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "WS103",
            name: "workspace-dep-cycle",
            description: "Workspace 模块间循环依赖",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct WORKSPACE_DEP_DEPRECATED;

impl Rule for WORKSPACE_DEP_DEPRECATED {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "WS104",
            name: "workspace-dep-deprecated",
            description: "Workspace 使用已弃用依赖",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct WORKSPACE_DEP_SECURITY;

impl Rule for WORKSPACE_DEP_SECURITY {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "WS105",
            name: "workspace-dep-security",
            description: "Workspace 依赖存在安全漏洞",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct GOWORK_FORMAT;

impl Rule for GOWORK_FORMAT {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "WS201",
            name: "gowork-format",
            description: "go.work 文件格式错误",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct GOWORK_SYNTAX;

impl Rule for GOWORK_SYNTAX {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "WS202",
            name: "gowork-syntax",
            description: "go.work 文件语法错误",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct GOWORK_DUPLICATE_USE;

impl Rule for GOWORK_DUPLICATE_USE {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "WS203",
            name: "gowork-duplicate-use",
            description: "go.work 中重复的 use 指令",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Recommended,
            default_severity: Severity::Warning,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

pub struct GOWORK_MISSING_DIRECTIVE;

impl Rule for GOWORK_MISSING_DIRECTIVE {
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            code: "WS204",
            name: "gowork-missing-directive",
            description: "go.work 缺少必要指令",
            category: RuleCategory::Upgrade,
            priority: RulePriority::Required,
            default_severity: Severity::Error,
        }
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // TODO: 实现检查逻辑
        // 最低支持版本: Go 1.18
        vec![]
    }
}

// ========== GET RULES ==========

pub fn get_new_feature_rules() -> Vec<Box<dyn Rule>> {
    vec![
        // fuzzing rules
        Box::new(FUZZ_TEST_SIGNATURE),
        Box::new(FUZZ_TARGET_NAME),
        Box::new(FUZZ_MISSING_CORPUS),
        Box::new(FUZZ_SEED_CORPUS),
        Box::new(FUZZ_PARALLEL),
        Box::new(FUZZ_CLEANUP),
        Box::new(FUZZ_ADD_DUPLICATE),
        Box::new(FUZZ_SKIPPED),
        Box::new(FUZZ_TIMEOUT),
        Box::new(FUZZ_PANIC_RECOVER),
        Box::new(FUZZ_GLOBAL_STATE),
        Box::new(FUZZ_NON_DETERMINISTIC),
        Box::new(FUZZ_INPUT_VALIDATION),
        Box::new(FUZZ_EXPENSIVE_OP),
        Box::new(FUZZ_FILE_OPERATION),
        Box::new(FUZZ_NETWORK_ACCESS),
        // generics rules
        Box::new(UNUSED_TYPE_PARAMETER),
        Box::new(TYPE_PARAMETER_SHADOW),
        Box::new(ANY_CONSTRAINT),
        Box::new(COMPARABLE_MISUSE),
        Box::new(TYPE_SET_REDUNDANT),
        Box::new(GENERIC_INTERFACE_METHOD),
        Box::new(TYPE_INFERENCE_FAIL),
        Box::new(TYPE_PARAM_TOO_MANY),
        Box::new(CONSTRAINT_UNNECESSARY),
        Box::new(GENERIC_NAMING),
        Box::new(TYPE_ASSERTION_IN_GENERIC),
        Box::new(REFLECT_IN_GENERIC),
        Box::new(POINTER_RECEIVER_GENERIC),
        Box::new(TYPE_PARAM_BOUNDS),
        Box::new(INSTANTIATION_ERROR),
        Box::new(GENERIC_BOXING),
        Box::new(GENERIC_MONOMORPHIZATION),
        // go1.20 rules
        Box::new(USE_CONTEXT_WITH_CANCEL_CAUSE),
        Box::new(USE_ERRORS_JOIN),
        Box::new(USE_SLICES_BINARY_SEARCH),
        Box::new(USE_MAPS_CLONE),
        Box::new(USE_MAPS_DELETE_FUNC),
        Box::new(USE_ATOMIC_TYPES),
        // go1.21 rules
        Box::new(USE_SLOG_CONTEXT),
        Box::new(SLOG_ATTR_OPTIMIZATION),
        Box::new(SLOG_GROUP_NESTING),
        Box::new(SLOG_DUPLICATE_ATTRS),
        Box::new(USE_SLICES_SORT_FUNC),
        Box::new(USE_SLICES_REVERSE),
        Box::new(USE_TESTING_OPTION),
        Box::new(TEST_SKIP_F),
        // go1.22 rules
        Box::new(LOOPVAR_CAPTURE_FIXED),
        Box::new(RANGE_OVER_INT),
        Box::new(RANGE_OVER_FUNC),
        Box::new(USE_HTTP_SERVEMUX_PATTERNS),
        Box::new(HTTP_ROUTE_CONFLICT),
        // workspace rules
        Box::new(WORKSPACE_GO_VERSION),
        Box::new(WORKSPACE_MODULE_PATH),
        Box::new(WORKSPACE_MISSING_MODULE),
        Box::new(WORKSPACE_ORPHAN_MODULE),
        Box::new(WORKSPACE_REPLACE_DIRECTIVE),
        Box::new(WORKSPACE_EXCLUDE_DIRECTIVE),
        Box::new(WORKSPACE_USE_DIRECTIVE),
        Box::new(WORKSPACE_NESTED),
        Box::new(WORKSPACE_VENDOR_CONFLICT),
        Box::new(WORKSPACE_LOCAL_PATH),
        Box::new(WORKSPACE_DEP_VERSION_MISMATCH),
        Box::new(WORKSPACE_DEP_OVERRIDE),
        Box::new(WORKSPACE_DEP_CYCLE),
        Box::new(WORKSPACE_DEP_DEPRECATED),
        Box::new(WORKSPACE_DEP_SECURITY),
        Box::new(GOWORK_FORMAT),
        Box::new(GOWORK_SYNTAX),
        Box::new(GOWORK_DUPLICATE_USE),
        Box::new(GOWORK_MISSING_DIRECTIVE),
    ]
}
