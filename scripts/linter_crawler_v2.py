#!/usr/bin/env python3
"""
Go Linter Crawler V2 - 针对 Go 新特性的规则抓取

专门抓取:
1. 泛型 (Generics) 相关规则 - Go 1.18+
2. Fuzzing 相关规则 - Go 1.18+
3. Workspace 相关规则 - Go 1.18+
4. 其他新特性规则
"""

import json
from datetime import datetime
from typing import Dict, List, Any


class NewFeatureRules:
    """Go 新特性规则集合"""
    
    # ========== 泛型 (Generics) 相关规则 ==========
    GENERICS_RULES = {
        # 类型参数相关
        "GEN001": {"name": "unused-type-parameter", "description": "未使用的类型参数", "severity": "warning"},
        "GEN002": {"name": "type-parameter-shadow", "description": "类型参数遮蔽外部类型", "severity": "error"},
        "GEN003": {"name": "any-constraint", "description": "使用 any 约束而非 interface{}", "severity": "info"},
        "GEN004": {"name": "comparable-misuse", "description": "comparable 约束的误用", "severity": "error"},
        "GEN005": {"name": "type-set-redundant", "description": "冗余的类型集合", "severity": "warning"},
        "GEN006": {"name": "generic-interface-method", "description": "泛型接口中的方法限制", "severity": "warning"},
        "GEN007": {"name": "type-inference-fail", "description": "无法推断类型参数", "severity": "error"},
        "GEN008": {"name": "type-param-too-many", "description": "过多的类型参数", "severity": "warning"},
        "GEN009": {"name": "constraint-unnecessary", "description": "不必要的类型约束", "severity": "info"},
        "GEN010": {"name": "generic-naming", "description": "类型参数命名不规范 (应为大写字母)", "severity": "warning"},
        "GEN011": {"name": "type-assertion-in-generic", "description": "泛型函数中不必要的类型断言", "severity": "warning"},
        "GEN012": {"name": "reflect-in-generic", "description": "泛型函数中使用 reflect 而非类型参数", "severity": "info"},
        "GEN013": {"name": "pointer-receiver-generic", "description": "泛型类型的指针接收器检查", "severity": "warning"},
        "GEN014": {"name": "type-param-bounds", "description": "类型参数边界检查", "severity": "error"},
        "GEN015": {"name": "instantiation-error", "description": "泛型实例化错误", "severity": "error"},
        
        # 泛型性能相关
        "GEN101": {"name": "generic-boxing", "description": "泛型装箱操作可能影响性能", "severity": "info"},
        "GEN102": {"name": "generic-monomorphization", "description": "建议泛型单态化优化", "severity": "info"},
    }
    
    # ========== Fuzzing 相关规则 ==========
    FUZZING_RULES = {
        # Fuzzing 测试规范
        "FUZZ001": {"name": "fuzz-test-signature", "description": "Fuzzing 测试函数签名错误", "severity": "error"},
        "FUZZ002": {"name": "fuzz-target-name", "description": "Fuzzing 目标命名不规范", "severity": "warning"},
        "FUZZ003": {"name": "fuzz-missing-corpus", "description": "缺少 Fuzzing 语料库目录", "severity": "info"},
        "FUZZ004": {"name": "fuzz-seed-corpus", "description": "建议使用种子语料库", "severity": "info"},
        "FUZZ005": {"name": "fuzz-parallel", "description": "Fuzzing 测试应考虑并行执行", "severity": "info"},
        "FUZZ006": {"name": "fuzz-cleanup", "description": "Fuzzing 测试后缺少清理", "severity": "warning"},
        "FUZZ007": {"name": "fuzz-add-duplicate", "description": "Fuzzing 中添加重复语料", "severity": "warning"},
        "FUZZ008": {"name": "fuzz-skipped", "description": "Fuzzing 测试被跳过", "severity": "info"},
        "FUZZ009": {"name": "fuzz-timeout", "description": "Fuzzing 测试应设置超时", "severity": "warning"},
        "FUZZ010": {"name": "fuzz-panic-recover", "description": "Fuzzing 中不建议使用 panic/recover", "severity": "warning"},
        "FUZZ011": {"name": "fuzz-global-state", "description": "Fuzzing 测试使用了全局状态", "severity": "error"},
        "FUZZ012": {"name": "fuzz-non-deterministic", "description": "Fuzzing 测试非确定性行为", "severity": "error"},
        
        # Fuzzing 最佳实践
        "FUZZ101": {"name": "fuzz-input-validation", "description": "Fuzzing 输入应充分验证", "severity": "warning"},
        "FUZZ102": {"name": "fuzz-expensive-op", "description": "Fuzzing 循环中有昂贵操作", "severity": "warning"},
        "FUZZ103": {"name": "fuzz-file-operation", "description": "Fuzzing 测试中避免文件操作", "severity": "info"},
        "FUZZ104": {"name": "fuzz-network-access", "description": "Fuzzing 测试中避免网络访问", "severity": "warning"},
    }
    
    # ========== Workspace 相关规则 ==========
    WORKSPACE_RULES = {
        # Workspace 结构
        "WS001": {"name": "workspace-go-version", "description": "Workspace 中 Go 版本不一致", "severity": "error"},
        "WS002": {"name": "workspace-module-path", "description": "Workspace 模块路径冲突", "severity": "error"},
        "WS003": {"name": "workspace-missing-module", "description": "Workspace 缺少必要模块", "severity": "warning"},
        "WS004": {"name": "workspace-orphan-module", "description": "Workspace 中孤立的模块", "severity": "info"},
        "WS005": {"name": "workspace-replace-directive", "description": "Workspace 中不当的 replace 指令", "severity": "warning"},
        "WS006": {"name": "workspace-exclude-directive", "description": "Workspace 中不当的 exclude 指令", "severity": "warning"},
        "WS007": {"name": "workspace-use-directive", "description": "Workspace 中 use 指令问题", "severity": "error"},
        "WS008": {"name": "workspace-nested", "description": "嵌套的 Workspace 配置", "severity": "error"},
        "WS009": {"name": "workspace-vendor-conflict", "description": "Workspace 与 vendor 冲突", "severity": "warning"},
        "WS010": {"name": "workspace-local-path", "description": "Workspace 使用相对路径可能的问题", "severity": "info"},
        
        # Workspace 依赖
        "WS101": {"name": "workspace-dep-version-mismatch", "description": "Workspace 依赖版本不一致", "severity": "warning"},
        "WS102": {"name": "workspace-dep-override", "description": "Workspace 依赖覆盖", "severity": "info"},
        "WS103": {"name": "workspace-dep-cycle", "description": "Workspace 模块间循环依赖", "severity": "error"},
        "WS104": {"name": "workspace-dep-deprecated", "description": "Workspace 使用已弃用依赖", "severity": "warning"},
        "WS105": {"name": "workspace-dep-security", "description": "Workspace 依赖存在安全漏洞", "severity": "error"},
        
        # go.work 文件规范
        "WS201": {"name": "gowork-format", "description": "go.work 文件格式错误", "severity": "error"},
        "WS202": {"name": "gowork-syntax", "description": "go.work 文件语法错误", "severity": "error"},
        "WS203": {"name": "gowork-duplicate-use", "description": "go.work 中重复的 use 指令", "severity": "warning"},
        "WS204": {"name": "gowork-missing-directive", "description": "go.work 缺少必要指令", "severity": "error"},
    }
    
    # ========== Go 1.20+ 新特性规则 ==========
    GO120_RULES = {
        # context.WithCancelCause
        "UP1201": {"name": "use-context-with-cancel-cause", "description": "使用 context.WithCancelCause 替代 WithCancel", "severity": "info"},
        
        # errors.Join
        "UP1202": {"name": "use-errors-join", "description": "使用 errors.Join 替代自定义错误合并", "severity": "info"},
        
        # slices 和 maps 包
        "UP1203": {"name": "use-slices-binary-search", "description": "使用 slices.BinarySearch 替代手动实现", "severity": "info"},
        "UP1204": {"name": "use-maps-clone", "description": "使用 maps.Clone 替代手动拷贝", "severity": "info"},
        "UP1205": {"name": "use-maps-delete-func", "description": "使用 maps.DeleteFunc 替代循环删除", "severity": "info"},
        
        # atomic 类型
        "UP1206": {"name": "use-atomic-types", "description": "使用 atomic.Int64 等类型替代 atomic.AddInt64", "severity": "info"},
    }
    
    # ========== Go 1.21+ 新特性规则 ==========
        # slog 包
    GO121_RULES = {
        "UP1211": {"name": "use-slog-context", "description": "使用 slog 的 Context 方法", "severity": "info"},
        "UP1212": {"name": "slog-attr-optimization", "description": "slog 属性优化建议", "severity": "info"},
        "UP1213": {"name": "slog-group-nesting", "description": "slog group 嵌套过深", "severity": "warning"},
        "UP1214": {"name": "slog-duplicate-attrs", "description": "slog 重复属性", "severity": "warning"},
        
        # slices 包增强
        "UP1215": {"name": "use-slices-sort-func", "description": "使用 slices.SortFunc 替代 sort.Slice", "severity": "info"},
        "UP1216": {"name": "use-slices-reverse", "description": "使用 slices.Reverse", "severity": "info"},
        
        # testing 包增强
        "UP1217": {"name": "use-testing-option", "description": "使用 testing 包的新 Option 类型", "severity": "info"},
        "UP1218": {"name": "test-skip-f", "description": "使用 t.Skipf 而非 fmt.Sprintf + t.Skip", "severity": "info"},
    }
    
    # ========== Go 1.22+ 新特性规则 ==========
    GO122_RULES = {
        # for 循环变量语义变化
        "UP1221": {"name": "loopvar-capture-fixed", "description": "Go 1.22+ 循环变量捕获已修复，无需 workarounds", "severity": "info"},
        "UP1222": {"name": "range-over-int", "description": "使用整数 range (Go 1.22+)", "severity": "info"},
        "UP1223": {"name": "range-over-func", "description": "使用函数 range (Go 1.23+)", "severity": "info"},
        
        # net/http 增强
        "UP1224": {"name": "use-http-servemux-patterns", "description": "使用 http.ServeMux 的新模式语法", "severity": "info"},
        "UP1225": {"name": "http-route-conflict", "description": "HTTP 路由模式冲突", "severity": "error"},
    }


def generate_new_feature_rules():
    """生成新特性规则"""
    
    features = NewFeatureRules()
    
    all_rules = []
    
    # 收集泛型规则
    for code, info in features.GENERICS_RULES.items():
        all_rules.append({
            "code": code,
            "name": info["name"],
            "description": info["description"],
            "category": "UP",
            "severity": info["severity"],
            "source": "generics",
            "min_go_version": "1.18"
        })
    
    # 收集 Fuzzing 规则
    for code, info in features.FUZZING_RULES.items():
        all_rules.append({
            "code": code,
            "name": info["name"],
            "description": info["description"],
            "category": "D",
            "severity": info["severity"],
            "source": "fuzzing",
            "min_go_version": "1.18"
        })
    
    # 收集 Workspace 规则
    for code, info in features.WORKSPACE_RULES.items():
        all_rules.append({
            "code": code,
            "name": info["name"],
            "description": info["description"],
            "category": "UP",
            "severity": info["severity"],
            "source": "workspace",
            "min_go_version": "1.18"
        })
    
    # 收集 Go 1.20+ 规则
    for code, info in features.GO120_RULES.items():
        all_rules.append({
            "code": code,
            "name": info["name"],
            "description": info["description"],
            "category": "UP",
            "severity": info["severity"],
            "source": "go1.20",
            "min_go_version": "1.20"
        })
    
    # 收集 Go 1.21+ 规则
    for code, info in features.GO121_RULES.items():
        all_rules.append({
            "code": code,
            "name": info["name"],
            "description": info["description"],
            "category": "UP",
            "severity": info["severity"],
            "source": "go1.21",
            "min_go_version": "1.21"
        })
    
    # 收集 Go 1.22+ 规则
    for code, info in features.GO122_RULES.items():
        all_rules.append({
            "code": code,
            "name": info["name"],
            "description": info["description"],
            "category": "UP",
            "severity": info["severity"],
            "source": "go1.22",
            "min_go_version": "1.22"
        })
    
    return all_rules


def generate_markdown_doc(rules: List[Dict]):
    """生成 Markdown 文档"""
    
    md = f"""# Woof 新特性规则集

> 自动生成于 {datetime.now().isoformat()}

本规则集专门针对 Go 语言的新特性，包括泛型、Fuzzing、Workspace 等。

## 规则概览

总计 **{len(rules)}** 条规则

"""
    
    # 按来源分组
    sources = {}
    for rule in rules:
        source = rule["source"]
        if source not in sources:
            sources[source] = []
        sources[source].append(rule)
    
    # 生成各组规则文档
    source_titles = {
        "generics": ("泛型规则 (Generics)", "针对 Go 1.18+ 泛型特性的检查规则"),
        "fuzzing": ("Fuzzing 规则", "针对 Go 1.18+ Fuzzing 测试的检查规则"),
        "workspace": ("Workspace 规则", "针对 Go 1.18+ Workspace 多模块开发的检查规则"),
        "go1.20": ("Go 1.20+ 规则", "Go 1.20 新特性相关规则"),
        "go1.21": ("Go 1.21+ 规则", "Go 1.21 新特性相关规则"),
        "go1.22": ("Go 1.22+ 规则", "Go 1.22 新特性相关规则"),
    }
    
    for source, source_rules in sorted(sources.items()):
        title, desc = source_titles.get(source, (source, ""))
        md += f"## {title}\n\n{desc}\n\n"
        md += f"共 {len(source_rules)} 条规则\n\n"
        md += "| 代码 | 名称 | 描述 | 级别 |\n"
        md += "|------|------|------|------|\n"
        
        for rule in source_rules:
            severity_emoji = {
                "error": "🔴",
                "warning": "🟡",
                "info": "🔵"
            }.get(rule["severity"], "⚪")
            md += f"| `{rule['code']}` | {rule['name']} | {rule['description']} | {severity_emoji} {rule['severity']} |\n"
        
        md += "\n"
    
    # 添加实现建议
    md += """## 实现建议

### 优先级

1. **P0 (必须)** 🔴: 错误级别规则，可能导致运行时问题
2. **P1 (推荐)** 🟡: 警告级别规则，建议遵循的最佳实践
3. **P2 (可选)** 🔵: 信息级别规则，代码改进建议

### 实现顺序

建议按以下顺序实现：

1. **泛型规则 (GENxxx)**
   - 优先实现 GEN001-GEN010 (核心泛型检查)
   - 然后实现 GEN101-GEN102 (性能相关)

2. **Fuzzing 规则 (FUZZxxx)**
   - 优先实现 FUZZ001, FUZZ011 (基础规范)
   - 然后实现其他规则

3. **Workspace 规则 (WSxxx)**
   - 优先实现 WS001-WS010 (Workspace 结构)
   - 然后实现依赖相关规则

4. **Go 版本升级规则 (UP12xx)**
   - 按 Go 版本顺序实现
   - 优先实现高影响规则

## 参考

- [Go 1.18 Release Notes - Generics](https://go.dev/doc/go1.18)
- [Go 1.18 Release Notes - Fuzzing](https://go.dev/doc/go1.18)
- [Go 1.18 Release Notes - Workspace](https://go.dev/doc/go1.18)
- [Go 1.20 Release Notes](https://go.dev/doc/go1.20)
- [Go 1.21 Release Notes](https://go.dev/doc/go1.21)
- [Go 1.22 Release Notes](https://go.dev/doc/go1.22)
"""
    
    return md


def generate_rust_module(rules: List[Dict]) -> str:
    """生成 Rust 模块代码"""
    
    rust = f"""//! Woof 新特性规则集
//! 针对 Go 泛型、Fuzzing、Workspace 等新特性
//! 生成时间: {datetime.now().isoformat()}
//! 规则数量: {len(rules)}

use crate::rules::{{Rule, RuleMetadata, RuleCategory, RulePriority}};
use crate::{{Diagnostic, Severity}};
use tree_sitter::Node;

"""
    
    # 按来源分组生成模块
    sources = {}
    for rule in rules:
        source = rule["source"]
        if source not in sources:
            sources[source] = []
        sources[source].append(rule)
    
    for source, source_rules in sorted(sources.items()):
        rust += f"// ========== {source.upper()} RULES ==========\n\n"
        
        for rule in source_rules:
            struct_name = rule["name"].replace("-", "_").upper()
            category = get_category_enum(rule["category"])
            priority = get_priority(rule["severity"])
            severity = rule["severity"].capitalize()
            
            rust += f"""pub struct {struct_name};

impl Rule for {struct_name} {{
    fn metadata(&self) -> RuleMetadata {{
        RuleMetadata {{
            code: "{rule['code']}",
            name: "{rule['name']}",
            description: "{rule['description']}",
            category: RuleCategory::{category},
            priority: RulePriority::{priority},
            default_severity: Severity::{severity},
        }}
    }}

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {{
        // TODO: 实现检查逻辑
        // 最低支持版本: Go {rule.get('min_go_version', '1.18')}
        vec![]
    }}
}}

"""
    
    # 生成 get_rules 函数
    rust += "// ========== GET RULES ==========\n\n"
    rust += "pub fn get_new_feature_rules() -> Vec<Box<dyn Rule>> {\n"
    rust += "    vec![\n"
    
    for source, source_rules in sorted(sources.items()):
        rust += f"        // {source} rules\n"
        for rule in source_rules:
            struct_name = rule["name"].replace("-", "_").upper()
            rust += f"        Box::new({struct_name}),\n"
    
    rust += "    ]\n}\n"
    
    return rust


def get_category_enum(category: str) -> str:
    mapping = {
        "E": "Codestyle",
        "F": "Logic",
        "B": "Bugbear",
        "I": "Imports",
        "UP": "Upgrade",
        "SIM": "Simplify",
        "S": "Style",
        "D": "Docs",
        "P": "Performance",
        "C": "Concurrency",
        "SEC": "Security",
    }
    return mapping.get(category, "Upgrade")


def get_priority(severity: str) -> str:
    if severity == "error":
        return "Required"
    elif severity == "warning":
        return "Recommended"
    return "Optional"


def main():
    print("=" * 60)
    print("Go Linter Crawler V2 - 新特性规则生成")
    print("=" * 60)
    
    # 生成规则
    rules = generate_new_feature_rules()
    
    print(f"\n生成规则统计:")
    print(f"  - 泛型规则: {len([r for r in rules if r['source'] == 'generics'])}")
    print(f"  - Fuzzing 规则: {len([r for r in rules if r['source'] == 'fuzzing'])}")
    print(f"  - Workspace 规则: {len([r for r in rules if r['source'] == 'workspace'])}")
    print(f"  - Go 1.20+ 规则: {len([r for r in rules if r['source'] == 'go1.20'])}")
    print(f"  - Go 1.21+ 规则: {len([r for r in rules if r['source'] == 'go1.21'])}")
    print(f"  - Go 1.22+ 规则: {len([r for r in rules if r['source'] == 'go1.22'])}")
    print(f"\n总计: {len(rules)} 条规则")
    
    import os
    os.makedirs("scripts/output", exist_ok=True)
    
    # 保存 JSON
    with open("scripts/output/new_feature_rules.json", "w", encoding="utf-8") as f:
        json.dump(rules, f, indent=2, ensure_ascii=False)
    print("\nSaved to scripts/output/new_feature_rules.json")
    
    # 生成 Markdown 文档
    md = generate_markdown_doc(rules)
    with open("scripts/output/NEW_FEATURE_RULES.md", "w", encoding="utf-8") as f:
        f.write(md)
    print("Saved to scripts/output/NEW_FEATURE_RULES.md")
    
    # 生成 Rust 代码
    rust = generate_rust_module(rules)
    with open("scripts/output/new_feature_rules.rs", "w", encoding="utf-8") as f:
        f.write(rust)
    print("Saved to scripts/output/new_feature_rules.rs")
    
    print("\n" + "=" * 60)
    print("Done!")
    print("=" * 60)


if __name__ == "__main__":
    main()
