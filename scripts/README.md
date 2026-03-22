# Woof Linter Crawler

Go 社区 Linter 规则爬虫 - 用于抓取和生成 woof linter 规则。

## 概述

本项目包含两个爬虫脚本:

1. **linter_crawler.py** - 抓取成熟的 Go linter 规则
2. **linter_crawler_v2.py** - 生成 Go 新特性相关规则

## 快速开始

### 运行主爬虫

```bash
cd /path/to/woof
python3 scripts/linter_crawler.py
```

### 运行新特性爬虫

```bash
python3 scripts/linter_crawler_v2.py
```

## 爬虫输出

### linter_crawler.py 输出

| 文件 | 描述 |
|------|------|
| `output/linters.json` | 44个 linter 信息 |
| `output/rules.json` | 626条社区规则 |
| `output/woof_rules.json` | woof 格式规则 |
| `output/generated_rules.rs` | Rust 代码模板 (13,796行) |
| `output/CRAWLER_REPORT.md` | 抓取报告 |

### linter_crawler_v2.py 输出

| 文件 | 描述 |
|------|------|
| `output/new_feature_rules.json` | 71条新特性规则 |
| `output/new_feature_rules.rs` | Rust 代码模板 |
| `output/NEW_FEATURE_RULES.md` | 新特性规则文档 |

## 规则统计

### 社区 Linter 规则 (626条)

```
来源分布:
- staticcheck: 270条 (43.1%)
- go-critic: 202条 (32.3%)
- revive: 154条 (24.6%)

类别分布:
- F (逻辑错误): 190条
- S (风格规范): 242条
- SIM (简化建议): 130条
- B (代码质量): 16条
- P (性能优化): 32条
- C (并发问题): 12条
- D (文档规范): 4条
```

### 新特性规则 (71条)

```
- 泛型规则: 17条
- Fuzzing 规则: 16条
- Workspace 规则: 19条
- Go 1.20+ 规则: 6条
- Go 1.21+ 规则: 8条
- Go 1.22+ 规则: 5条
```

## 集成到 woof

### 1. 复制生成的规则代码

```bash
# 创建新规则模块
cp scripts/output/generated_rules.rs src/rules/generated.rs
cp scripts/output/new_feature_rules.rs src/rules/new_features.rs
```

### 2. 修改 src/rules/mod.rs

```rust
pub mod generated;
pub mod new_features;

pub fn get_all_rules() -> Vec<Box<dyn Rule>> {
    let mut rules: Vec<Box<dyn Rule>> = vec![];
    
    // 原有规则...
    rules.extend(codestyle::get_rules());
    rules.extend(logic::get_rules());
    // ...
    
    // 添加生成的规则
    rules.extend(generated::get_generated_rules());
    rules.extend(new_features::get_new_feature_rules());
    
    rules
}
```

### 3. 实现检查逻辑

每个生成的规则都有 `TODO: 实现检查逻辑` 注释，需要:

1. 理解规则的检测目标
2. 使用 tree-sitter 遍历 AST
3. 实现具体的检查逻辑
4. 返回 Diagnostic 列表

示例:

```rust
fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
    let mut diagnostics = vec![];
    
    // 示例: 检查未使用的类型参数
    if node.kind() == "type_parameter_list" {
        // 检查类型参数是否在函数体内使用
        // ...
    }
    
    diagnostics
}
```

### 4. 添加测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_unused_type_parameter() {
        let rule = UnusedTypeParameter;
        // 测试代码...
    }
}
```

## 爬虫实现细节

### linter_crawler.py

从以下源抓取规则:

1. **golangci-lint**: 收集 44 个内置 linter 信息
2. **staticcheck**: 收集 SA/S/ST 系列规则 (270条)
3. **go-critic**: 收集 202 条检查规则
4. **revive**: 收集 154 条规则

### linter_crawler_v2.py

针对 Go 新特性手工整理规则:

- **泛型规则**: 类型参数、约束、实例化检查
- **Fuzzing 规则**: Fuzz 测试规范、最佳实践
- **Workspace 规则**: 多模块开发检查
- **版本升级规则**: Go 1.20/1.21/1.22 新特性

## 扩展爬虫

### 添加新的 linter 源

1. 创建新的 Crawler 类继承 `LinterCrawler`
2. 实现 `crawl()` 方法
3. 在 `main()` 中添加 crawler 实例

示例:

```python
class MyLinterCrawler(LinterCrawler):
    def crawl(self) -> List[Dict]:
        rules = []
        # 抓取逻辑...
        return rules
```

### 添加新特性规则

在 `NewFeatureRules` 类中添加新的规则字典:

```python
MY_NEW_RULES = {
    "NEW001": {"name": "my-rule", "description": "...", "severity": "warning"},
}
```

## 参考

- [golangci-lint](https://golangci-lint.run/)
- [staticcheck](https://staticcheck.io/)
- [go-critic](https://go-critic.com/)
- [revive](https://github.com/mgechev/revive)
- [Go Release Notes](https://go.dev/doc/devel/release)

## 许可证

MIT License
