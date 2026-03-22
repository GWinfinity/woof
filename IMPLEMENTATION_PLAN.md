# Woof 生产化实施计划

## Week 1: 阻塞问题修复

### Day 1-2: 抑制注释系统

```rust
// src/rules/mod.rs
pub trait Rule: Send + Sync {
    // ... existing methods
    
    /// Check if this rule is suppressed at the given position
    fn is_suppressed(&self, source: &str, line: usize) -> bool {
        let line_text = source.lines().nth(line.saturating_sub(1))?;
        
        // Check for nolint comment
        if let Some(idx) = line_text.find("//") {
            let comment = &line_text[idx..];
            if comment.contains("nolint") {
                // Check if specific rule is mentioned
                if comment.contains(&format!("nolint:{}", self.code())) {
                    return true;
                }
                // Check for wildcard: nolint:E0*
                if comment.contains("nolint:E0*") && self.code().starts_with("E0") {
                    return true;
                }
                // Check for general nolint
                if comment.trim() == "// nolint" {
                    return true;
                }
            }
        }
        false
    }
}

// 改进的visitor
pub fn visit_with_suppression(&mut self, node: Node, diagnostics: &mut Vec<Diagnostic>) {
    let line = get_line(self.source, node.start_byte());
    
    for rule in self.rules {
        // Check suppression before running rule
        if !rule.is_suppressed(self.source, line) {
            let node_diagnostics = rule.check(node, self.source, self.file_path);
            diagnostics.extend(node_diagnostics);
        }
    }
    
    // Visit children...
}
```

### Day 3-4: 修复重复诊断

当前问题：`LineTooLong` 规则在visitor中被调用多次。

解决方案：标记文件级规则

```rust
// src/rules/mod.rs
pub trait Rule: Send + Sync {
    /// Returns true if this rule should only run once per file
    fn is_file_level(&self) -> bool {
        false
    }
}

// 在builtin.rs中
impl Rule for LineTooLong {
    fn code(&self) -> &'static str { "E101" }
    
    fn is_file_level(&self) -> bool {
        true  // 只运行一次
    }
}
```

### Day 5-7: 添加5条简单规则

```rust
// src/rules/basic.rs

/// E002: Unused variable
pub struct UnusedVariable;

impl Rule for UnusedVariable {
    fn code(&self) -> &'static str { "E002" }
    fn name(&self) -> &'static str { "unused-variable" }
    
    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        if node.kind() != "short_var_declaration" && node.kind() != "var_spec" {
            return vec![];
        }
        
        let mut diags = Vec::new();
        let mut cursor = node.walk();
        
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "identifier" {
                    let name = node_text(child, source);
                    
                    // Check if used in the function scope
                    if let Some(func) = find_parent_by_kind(node, "function_declaration") {
                        let func_text = node_text(func, source);
                        if !is_identifier_used_except_decl(func_text, name, node.end_byte()) {
                            diags.push(Diagnostic {
                                file_path: file_path.to_string(),
                                line: get_line(source, child.start_byte()),
                                column: get_column(source, child.start_byte()),
                                message: format!("Variable '{}' is unused", name),
                                code: self.code().to_string(),
                                severity: self.default_severity(),
                                fix: Some(Fix {
                                    description: format!("Remove unused variable '{}'", name),
                                    replacement: String::new(),
                                    start_byte: node.start_byte(),
                                    end_byte: find_statement_end(node, source),
                                }),
                            });
                        }
                    }
                }
                if !cursor.goto_next_sibling() { break; }
            }
        }
        
        diags
    }
}
```

## Week 2-3: 核心规则实现

### 批量规则实现模板

```rust
// src/rules/quick_add.rs

/// 快速添加规则的宏
macro_rules! define_simple_rule {
    ($name:ident, $code:expr, $node_kind:expr, $check_fn:expr) => {
        pub struct $name;
        
        impl Rule for $name {
            fn code(&self) -> &'static str { $code }
            fn name(&self) -> &'static str { stringify!($name) }
            fn description(&self) -> &'static str { "Auto-generated rule" }
            
            fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
                if node.kind() != $node_kind {
                    return vec![];
                }
                
                let check: fn(Node, &str, &str) -> Vec<Diagnostic> = $check_fn;
                check(node, source, file_path)
            }
        }
    };
}

// 使用示例
define_simple_rule!(
    SnakeCase,
    "S005",
    "function_declaration",
    |node, source, file_path| {
        // 检查函数名是否snake_case
        let name = get_function_name(node, source);
        if is_snake_case(&name) {
            return vec![Diagnostic {
                file_path: file_path.to_string(),
                line: get_line(source, node.start_byte()),
                column: get_column(source, node.start_byte()),
                message: format!("Function '{}' uses snake_case, use CamelCase", name),
                code: "S005".to_string(),
                severity: Severity::Warning,
                fix: None,
            }];
        }
        vec![]
    }
);
```

## Week 4: 类型检查集成

### 方案：go/analysis Bridge

```rust
// src/analysis/bridge.rs

use serde::{Deserialize, Serialize};
use std::process::Command;

/// 调用Go分析器获取类型信息
pub struct GoAnalyzer;

#[derive(Deserialize)]
pub struct TypeInfo {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub type_name: String,
    pub is_nilable: bool,
}

impl GoAnalyzer {
    pub fn analyze_package(&self, dir: &Path) -> anyhow::Result<Vec<TypeInfo>> {
        // 调用嵌入的Go分析工具
        let output = Command::new("woof-analysis-helper")
            .arg(dir)
            .output()?;
        
        let info: Vec<TypeInfo> = serde_json::from_slice(&output.stdout)?;
        Ok(info)
    }
}

// 辅助工具（Go代码，编译后嵌入）
/*
// tools/analysis-helper/main.go
package main

import (
    "encoding/json"
    "go/ast"
    "go/parser"
    "go/token"
    "go/types"
    "os"
)

func main() {
    dir := os.Args[1]
    
    // 解析包
    fset := token.NewFileSet()
    pkgs, err := parser.ParseDir(fset, dir, nil, 0)
    if err != nil {
        panic(err)
    }
    
    var results []map[string]interface{}
    
    for _, pkg := range pkgs {
        for filename, file := range pkg.Files {
            ast.Inspect(file, func(n ast.Node) bool {
                if expr, ok := n.(ast.Expr); ok {
                    tv := &types.TypeAndValue{}
                    // 获取类型信息
                    results = append(results, map[string]interface{}{
                        "file": filename,
                        "line": fset.Position(n.Pos()).Line,
                        "type_name": tv.Type.String(),
                        "is_nilable": types.IsInterface(tv.Type) || 
                                     types.IsPointer(tv.Type),
                    })
                }
                return true
            })
        }
    }
    
    json.NewEncoder(os.Stdout).Encode(results)
}
*/
```

## Week 5: LSP服务器

```rust
// src/lsp/mod.rs

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

pub struct WoofLsp {
    client: Client,
    linter: Linter,
    documents: DashMap<Url, String>, // 缓存打开的文件
}

#[tower_lsp::async_trait]
impl LanguageServer for WoofLsp {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions {
                        identifier: Some("woof".to_string()),
                        inter_file_dependencies: true,
                        workspace_diagnostics: false,
                    },
                )),
                document_formatting_provider: Some(true),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let content = params.text_document.text;
        
        self.documents.insert(uri.clone(), content.clone());
        
        // Lint and publish diagnostics
        let diagnostics = self.linter.lint_string(&content);
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        
        // 增量更新
        if let Some(mut doc) = self.documents.get_mut(&uri) {
            for change in params.content_changes {
                if let Some(range) = change.range {
                    // 应用增量更新
                    apply_change(&mut doc, range, &change.text);
                } else {
                    // 全量替换
                    *doc = change.text;
                }
            }
            
            // 重新lint
            let diagnostics = self.linter.lint_string(&doc);
            self.client
                .publish_diagnostics(uri, diagnostics, None)
                .await;
        }
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = &params.text_document.uri;
        
        if let Some(content) = self.documents.get(uri) {
            let formatted = self.linter.format_string(&content);
            let end_line = content.lines().count() as u32;
            
            Ok(Some(vec![TextEdit {
                range: Range {
                    start: Position::new(0, 0),
                    end: Position::new(end_line, 0),
                },
                new_text: formatted,
            }]))
        } else {
            Ok(None)
        }
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let uri = &params.text_document.uri;
        
        if let Some(content) = self.documents.get(uri) {
            let diagnostics = self.linter.lint_string(&content);
            
            let actions: Vec<CodeActionOrCommand> = diagnostics
                .into_iter()
                .filter_map(|d| d.fix)
                .map(|fix| {
                    CodeActionOrCommand::CodeAction(CodeAction {
                        title: fix.description,
                        edit: Some(WorkspaceEdit {
                            changes: Some(maplit::hashmap! {
                                uri.clone() => vec![TextEdit {
                                    range: Range {
                                        start: Position::new(0, 0),
                                        end: Position::new(0, 0),
                                    },
                                    new_text: fix.replacement,
                                }],
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    })
                })
                .collect();
            
            Ok(Some(actions))
        } else {
            Ok(None)
        }
    }
}
```

## Week 6: CI/CD集成和发布

### GitHub Actions

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        target:
          - x86_64-unknown-linux-gnu
          - x86_64-apple-darwin
          - aarch64-apple-darwin
          - x86_64-pc-windows-msvc
    
    runs-on: ${{ matrix.os }}
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: dtolnay/rust-action@stable
        with:
          targets: ${{ matrix.target }}
      
      - name: Build
        run: cargo build --release --target ${{ matrix.target }}
      
      - name: Package
        shell: bash
        run: |
          cd target/${{ matrix.target }}/release
          if [[ "${{ matrix.target }}" == *"windows"* ]]; then
            7z a ../../../woof-${{ matrix.target }}.zip woof.exe
          else
            tar czvf ../../../woof-${{ matrix.target }}.tar.gz woof
          fi
          cd -
      
      - name: Upload
        uses: actions/upload-release-asset@v1
        with:
          upload_url: ${{ github.event.release.upload_url }}
          asset_path: ./woof-${{ matrix.target }}.${{ matrix.ext }}
          asset_name: woof-${{ matrix.target }}.${{ matrix.ext }}
          asset_content_type: application/octet-stream
```

### Homebrew Formula

```ruby
# woof.rb
class Woof < Formula
  desc "Extremely fast Go linter and formatter"
  homepage "https://github.com/yourusername/woof"
  version "1.0.0"
  
  if OS.mac? && Hardware::CPU.intel?
    url "https://github.com/yourusername/woof/releases/download/v1.0.0/woof-x86_64-apple-darwin.tar.gz"
    sha256 "..."
  elsif OS.mac? && Hardware::CPU.arm?
    url "https://github.com/yourusername/woof/releases/download/v1.0.0/woof-aarch64-apple-darwin.tar.gz"
    sha256 "..."
  elsif OS.linux?
    url "https://github.com/yourusername/woof/releases/download/v1.0.0/woof-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "..."
  end
  
  def install
    bin.install "woof"
  end
  
  test do
    system "#{bin}/woof", "--version"
  end
end
```

## 测试策略

```rust
// tests/regression_tests.rs

/// 使用真实项目测试
#[test]
fn test_against_kubernetes() {
    let temp = tempfile::tempdir().unwrap();
    
    // 克隆kubernetes项目
    Command::new("git")
        .args(["clone", "--depth", "1", "https://github.com/kubernetes/kubernetes.git", 
               temp.path().to_str().unwrap()])
        .output()
        .expect("Failed to clone");
    
    let start = Instant::now();
    let result = woof::lint_path(temp.path(), &Config::default());
    let duration = start.elapsed();
    
    // 断言性能
    assert!(duration < Duration::from_secs(5), "Should lint kubernetes in <5s");
    
    // 断言没有崩溃
    assert!(result.is_ok());
}

/// 对比测试：woof vs golangci-lint
#[test]
fn test_accuracy_comparison() {
    let scenarios = load_test_scenarios();
    
    for scenario in scenarios {
        let woof_result = run_woof(&scenario);
        let golangci_result = run_golangci(&scenario);
        
        // 计算准确率
        let precision = calculate_precision(&woof_result, &golangci_result);
        let recall = calculate_recall(&woof_result, &golangci_result);
        
        assert!(
            precision > 0.8 && recall > 0.6,
            "Precision: {}, Recall: {} for {}",
            precision, recall, scenario.name
        );
    }
}
```

## 里程碑检查清单

### ✅ Week 1 完成标准
- [ ] `// nolint` 抑制注释工作
- [ ] 没有重复诊断
- [ ] 新增5条规则

### ✅ Week 2-3 完成标准
- [ ] 总共20条规则
- [ ] 所有规则有测试
- [ ] 误报率 < 20%

### ✅ Week 4-5 完成标准
- [ ] 基础类型检查工作
- [ ] LSP服务器可运行
- [ ] VS Code扩展可用

### ✅ Week 6 完成标准
- [ ] GitHub Actions集成
- [ ] 多平台二进制发布
- [ ] 文档完整

## 风险与缓解

| 风险 | 影响 | 可能性 | 缓解措施 |
|------|------|--------|----------|
| 类型检查集成困难 | 高 | 中 | 先使用简单启发式，逐步完善 |
| 规则误报率高 | 高 | 中 | 添加抑制注释，持续调优 |
| 性能退化 | 中 | 低 | 持续基准测试，优化热点 |
| 社区接受度低 | 中 | 中 | 提供优质文档，性能对比 |

## 资源需求

| 资源 | 数量 | 用途 |
|------|------|------|
| Rust开发者 | 1-2人 | 核心开发 |
| Go专家 | 0.5人 | 类型检查咨询 |
| 测试环境 | 云服务器 | CI/CD, 性能测试 |
| 时间 | 6周全职 | 开发周期 |
