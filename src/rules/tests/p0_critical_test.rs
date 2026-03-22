//! P0 Critical Rules 测试

use crate::rules::p0_critical::*;
use crate::rules::Rule;
use crate::Diagnostic;
use tree_sitter::Parser;

fn parse_source(source: &str) -> (tree_sitter::Tree, String) {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_go::language().into()).unwrap();
    let tree = parser.parse(source, None).unwrap();
    (tree, source.to_string())
}

fn collect_diagnostics(rule: &dyn Rule, tree: &tree_sitter::Tree, source: &str) -> Vec<Diagnostic> {
    let mut diagnostics = vec![];
    let root = tree.root_node();
    
    fn walk(node: tree_sitter::Node, rule: &dyn Rule, source: &str, file_path: &str, diagnostics: &mut Vec<Diagnostic>) {
        diagnostics.extend(rule.check(node, source, file_path));
        
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            walk(child, rule, source, file_path, diagnostics);
        }
    }
    
    walk(root, rule, source, "test.go", &mut diagnostics);
    diagnostics
}

// ==================== SA1xxx 测试 ====================

#[test]
fn test_invalid_regex() {
    let rule = InvalidRegex;
    let source = r#"
package test
import "regexp"
func test() {
    re := regexp.MustCompile("[a-z")  // 未闭合的 [
}
"#;
    let (tree, src) = parse_source(source);
    let diagnostics = collect_diagnostics(&rule, &tree, &src);
    assert!(!diagnostics.is_empty(), "应检测到无效的正则表达式");
    assert_eq!(diagnostics[0].code, "SA1000");
}

#[test]
fn test_invalid_template() {
    let rule = InvalidTemplate;
    let source = r#"
package test
import "text/template"
func test() {
    t := template.Must(template.New("test").Parse("{{.Name"))  // 未闭合的 {{
}
"#;
    let (tree, src) = parse_source(source);
    let diagnostics = collect_diagnostics(&rule, &tree, &src);
    assert!(!diagnostics.is_empty(), "应检测到无效的模板");
    assert_eq!(diagnostics[0].code, "SA1001");
}

#[test]
fn test_printf_dynamic_format() {
    let rule = PrintfDynamicFormat;
    let source = r#"
package test
import "fmt"
func test(format string) {
    fmt.Printf(format)  // 动态格式字符串
}
"#;
    let (tree, src) = parse_source(source);
    let diagnostics = collect_diagnostics(&rule, &tree, &src);
    assert!(!diagnostics.is_empty(), "应检测到动态格式字符串");
    assert_eq!(diagnostics[0].code, "SA1006");
}

#[test]
fn test_nil_context() {
    let rule = NilContext;
    let source = r#"
package test
import "context"
func test() {
    doSomething(nil)  // 传递 nil context
}
func doSomething(ctx context.Context) {}
"#;
    let (tree, src) = parse_source(source);
    let diagnostics = collect_diagnostics(&rule, &tree, &src);
    assert!(!diagnostics.is_empty(), "应检测到 nil context");
    assert_eq!(diagnostics[0].code, "SA1012");
}

#[test]
fn test_deprecated_function() {
    let rule = DeprecatedFunction;
    let source = r#"
package test
import "io/ioutil"
func test() {
    ioutil.ReadFile("test.txt")  // 已弃用
}
"#;
    let (tree, src) = parse_source(source);
    let diagnostics = collect_diagnostics(&rule, &tree, &src);
    assert!(!diagnostics.is_empty(), "应检测到已弃用的函数");
    assert_eq!(diagnostics[0].code, "SA1019");
}

// ==================== SA2xxx 测试 ====================

#[test]
fn test_sync_waitgroup_add_goroutine() {
    let rule = SyncWaitgroupAddGoroutine;
    let source = r#"
package test
import "sync"
func test() {
    var wg sync.WaitGroup
    go func() {
        wg.Add(1)  // 错误：在 goroutine 中调用
        defer wg.Done()
    }()
    wg.Wait()
}
"#;
    let (tree, src) = parse_source(source);
    let diagnostics = collect_diagnostics(&rule, &tree, &src);
    assert!(!diagnostics.is_empty(), "应检测到 WaitGroup.Add 在 goroutine 中调用");
    assert_eq!(diagnostics[0].code, "SA2000");
}

#[test]
fn test_empty_critical_section() {
    let rule = EmptyCriticalSection;
    let source = r#"
package test
import "sync"
func test() {
    var mu sync.Mutex
    mu.Lock()
    mu.Unlock()  // 空的临界区
}
"#;
    let (tree, src) = parse_source(source);
    let diagnostics = collect_diagnostics(&rule, &tree, &src);
    assert!(!diagnostics.is_empty(), "应检测到空的临界区");
    assert_eq!(diagnostics[0].code, "SA2001");
}

#[test]
fn test_test_failnow_goroutine() {
    let rule = TestFailNowGoroutine;
    let source = r#"
package test
import "testing"
func TestSomething(t *testing.T) {
    go func() {
        t.FailNow()  // 错误：在 goroutine 中调用
    }()
}
"#;
    let (tree, src) = parse_source(source);
    let diagnostics = collect_diagnostics(&rule, &tree, &src);
    assert!(!diagnostics.is_empty(), "应检测到 t.FailNow 在 goroutine 中调用");
    assert_eq!(diagnostics[0].code, "SA2002");
}

// ==================== SA5xxx 测试 ====================

#[test]
fn test_assignment_to_nil_map() {
    let rule = AssignmentToNilMap;
    let source = r#"
package test
func test() {
    var m map[string]int
    m["key"] = 1  // 向 nil map 赋值
}
"#;
    let (tree, src) = parse_source(source);
    let diagnostics = collect_diagnostics(&rule, &tree, &src);
    assert!(!diagnostics.is_empty(), "应检测到向 nil map 赋值");
    assert_eq!(diagnostics[0].code, "SA5000");
}

#[test]
fn test_infinite_recursion() {
    let rule = InfiniteRecursion;
    let source = r#"
package test
func infinite(n int) int {
    return infinite(n)  // 无限递归
}
"#;
    let (tree, src) = parse_source(source);
    let diagnostics = collect_diagnostics(&rule, &tree, &src);
    assert!(!diagnostics.is_empty(), "应检测到无限递归");
    assert_eq!(diagnostics[0].code, "SA5007");
}

// ==================== 并发规则测试 ====================

#[test]
fn test_bad_lock_mutex_by_value() {
    let rule = BadLock;
    let source = r#"
package test
import "sync"
func process(mu sync.Mutex) {  // 错误：Mutex 按值传递
    mu.Lock()
    defer mu.Unlock()
}
"#;
    let (tree, src) = parse_source(source);
    let diagnostics = collect_diagnostics(&rule, &tree, &src);
    assert!(!diagnostics.is_empty(), "应检测到 Mutex 按值传递");
    assert_eq!(diagnostics[0].code, "C001");
}

// ==================== 泛型规则测试 ====================

#[test]
fn test_type_parameter_shadow() {
    let rule = TypeParameterShadow;
    let source = r#"
package test
func process[int any](x int) {}  // 错误：int 遮蔽了内置类型
"#;
    let (tree, src) = parse_source(source);
    let diagnostics = collect_diagnostics(&rule, &tree, &src);
    assert!(!diagnostics.is_empty(), "应检测到类型参数遮蔽内置类型");
    assert_eq!(diagnostics[0].code, "GEN002");
}

// ==================== Fuzzing 规则测试 ====================

#[test]
fn test_fuzz_test_signature() {
    let rule = FuzzTestSignature;
    let source = r#"
package test
import "testing"
func FuzzSomething(x int) {  // 错误：缺少 *testing.F
}
"#;
    let (tree, src) = parse_source(source);
    let diagnostics = collect_diagnostics(&rule, &tree, &src);
    assert!(!diagnostics.is_empty(), "应检测到 Fuzz 函数签名错误");
    assert_eq!(diagnostics[0].code, "FUZZ001");
}

#[test]
fn test_fuzz_global_state() {
    let rule = FuzzGlobalState;
    let source = r#"
package test
import "testing"
var counter int
func FuzzSomething(f *testing.F) {
    f.Fuzz(func(t *testing.T, data []byte) {
        counter++  // 错误：修改全局状态
    })
}
"#;
    let (tree, src) = parse_source(source);
    let diagnostics = collect_diagnostics(&rule, &tree, &src);
    assert!(!diagnostics.is_empty(), "应检测到 Fuzzing 使用全局状态");
    assert_eq!(diagnostics[0].code, "FUZZ011");
}

#[test]
fn test_fuzz_non_deterministic() {
    let rule = FuzzNonDeterministic;
    let source = r#"
package test
import (
    "testing"
    "time"
)
func FuzzSomething(f *testing.F) {
    f.Fuzz(func(t *testing.T, data []byte) {
        x := time.Now()  // 错误：非确定性
        _ = x
    })
}
"#;
    let (tree, src) = parse_source(source);
    let diagnostics = collect_diagnostics(&rule, &tree, &src);
    assert!(!diagnostics.is_empty(), "应检测到 Fuzzing 非确定性行为");
    assert_eq!(diagnostics[0].code, "FUZZ012");
}

// ==================== Workspace 规则测试 ====================

#[test]
fn test_workspace_go_version() {
    let rule = WorkspaceGoVersion;
    let source = r#"go 1.16

use (
    ./module1
    ./module2
)
"#;
    let tree = tree_sitter::Tree::default();
    let diagnostics = rule.check(tree.root_node(), source, "go.work");
    assert!(!diagnostics.is_empty(), "应检测到 Workspace Go 版本过旧");
    assert_eq!(diagnostics[0].code, "WS001");
}
