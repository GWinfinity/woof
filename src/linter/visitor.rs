use crate::Diagnostic;
use crate::rules::Rule;
use tree_sitter::{Node, TreeCursor};

/// AST visitor that applies rules to each node
pub struct Visitor<'a> {
    rules: &'a [Box<dyn Rule>],
    source: &'a str,
    file_path: &'a str,
}

impl<'a> Visitor<'a> {
    pub fn new(rules: &'a [Box<dyn Rule>], source: &'a str, file_path: &'a str) -> Self {
        Self {
            rules,
            source,
            file_path,
        }
    }

    pub fn visit(&mut self, node: Node, diagnostics: &mut Vec<Diagnostic>) {
        // Apply each rule to the current node
        for rule in self.rules {
            let node_diagnostics = rule.check(node, self.source, self.file_path);
            diagnostics.extend(node_diagnostics);
        }

        // Visit children
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                self.visit(child, diagnostics);
                
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }
}

/// Visitor pattern for specific node types
pub trait NodeVisitor {
    fn visit(&mut self, node: Node);
    fn should_visit_children(&self, _node: Node) -> bool {
        true
    }
}

/// Walk the tree with a specific visitor
pub fn walk_tree(cursor: &mut TreeCursor, visitor: &mut dyn NodeVisitor) {
    let node = cursor.node();
    visitor.visit(node);
    
    if visitor.should_visit_children(node) && cursor.goto_first_child() {
        walk_tree(cursor, visitor);
        
        while cursor.goto_next_sibling() {
            walk_tree(cursor, visitor);
        }
        
        cursor.goto_parent();
    }
}

/// Get all nodes of a specific kind in the tree
pub fn find_nodes_by_kind<'a>(root: Node<'a>, kind: &str, _source: &'a str) -> Vec<Node<'a>> {
    let mut results = Vec::new();
    let mut cursor = root.walk();
    
    fn find_recursive<'a>(cursor: &mut TreeCursor<'a>, kind: &str, results: &mut Vec<Node<'a>>) {
        let node = cursor.node();
        if node.kind() == kind {
            results.push(node);
        }
        
        if cursor.goto_first_child() {
            loop {
                find_recursive(cursor, kind, results);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
    }
    
    find_recursive(&mut cursor, kind, &mut results);
    results
}

/// Find the first parent node of a specific kind
pub fn find_parent_by_kind<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    let mut current = node;
    while let Some(parent) = current.parent() {
        if parent.kind() == kind {
            return Some(parent);
        }
        current = parent;
    }
    None
}

/// Check if a node is inside a specific kind of parent
pub fn is_inside(node: Node, kind: &str) -> bool {
    find_parent_by_kind(node, kind).is_some()
}

/// Get the package name from the root node
pub fn get_package_name(root: Node, source: &str) -> Option<String> {
    if root.kind() != "source_file" {
        return None;
    }
    
    let mut cursor = root.walk();
    if cursor.goto_first_child() {
        loop {
            let node = cursor.node();
            if node.kind() == "package_clause" {
                let text = &source[node.start_byte()..node.end_byte()];
                // Extract package name from "package name"
                return text.split_whitespace().nth(1).map(|s| s.to_string());
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
    None
}
