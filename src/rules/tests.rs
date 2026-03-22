#[cfg(test)]
mod p0_critical_test;

#[cfg(test)]
mod tests {
    use super::super::*;
    use tree_sitter::Parser;

    fn parse_go(source: &str) -> tree_sitter::Tree {
        let mut parser = Parser::new();
        let language = tree_sitter_go::LANGUAGE.into();
        parser.set_language(&language).unwrap();
        parser.parse(source, None).unwrap()
    }

    fn run_rule(rule: &dyn Rule, source: &str) -> Vec<Diagnostic> {
        let tree = parse_go(source);
        let root = tree.root_node();
        rule.check(root, source, "test.go")
    }

    #[test]
    fn test_line_too_long() {
        let rule = builtin::LineTooLong;
        let source = r#"package main
// This is a very very very very very very very very very very very very very very very very very very long line that exceeds the maximum line length
func main() {}
"#;
        let diags = run_rule(&rule, source);
        assert!(!diags.is_empty(), "Should detect long line");
        assert_eq!(diags[0].code, "E101");
    }

    #[test]
    fn test_trailing_whitespace() {
        let rule = builtin::TrailingWhitespace;
        let source = "package main  \nfunc main() {}\n";
        let diags = run_rule(&rule, source);
        assert!(!diags.is_empty());
        assert_eq!(diags[0].code, "E201");
    }

    #[test]
    fn test_empty_block() {
        let rule = builtin::EmptyBlock;
        let source = r#"package main
func main() {
    if true {
    }
}
"#;
        let tree = parse_go(source);
        let root = tree.root_node();
        
        // Find block nodes
        let blocks = crate::linter::visitor::find_nodes_by_kind(root, "block", source);
        let mut found_empty = false;
        
        for block in blocks {
            let diags = rule.check(block, source, "test.go");
            if !diags.is_empty() {
                found_empty = true;
                assert_eq!(diags[0].code, "E301");
            }
        }
        
        assert!(found_empty);
    }

    #[test]
    fn test_mixed_tabs_spaces() {
        let rule = builtin::MixedTabsSpaces;
        let source = "package main\n\t func main() {}\n";
        let diags = run_rule(&rule, source);
        assert!(!diags.is_empty());
        assert_eq!(diags[0].code, "E401");
    }

    #[test]
    fn test_exported_missing_doc() {
        let rule = builtin::ExportedMissingDoc;
        let source = r#"package main
func ExportedFunction() {}
"#;
        let tree = parse_go(source);
        let root = tree.root_node();
        
        // Find function declarations
        let funcs = crate::linter::visitor::find_nodes_by_kind(root, "function_declaration", source);
        let mut found_missing = false;
        
        for func in funcs {
            let diags = rule.check(func, source, "test.go");
            if !diags.is_empty() {
                found_missing = true;
                assert_eq!(diags[0].code, "D001");
            }
        }
        
        assert!(found_missing);
    }

    #[test]
    fn test_exported_with_doc() {
        let rule = builtin::ExportedMissingDoc;
        let source = r#"package main
// ExportedFunction does something
func ExportedFunction() {}
"#;
        let tree = parse_go(source);
        let root = tree.root_node();
        
        let funcs = crate::linter::visitor::find_nodes_by_kind(root, "function_declaration", source);
        
        for func in funcs {
            let diags = rule.check(func, source, "test.go");
            assert!(diags.is_empty(), "Exported function with doc should not trigger");
        }
    }
}
