use tree_sitter::Node;

/// Returns a string representation of a tree-sitter node and its children for debugging purposes.
///
/// # Arguments
///
/// * `node` - The tree-sitter node to debug
/// * `source_code` - The source code text that the node was parsed from
///
/// # Output Format
///
/// For each node, prints its kind and the corresponding source code text, with child nodes indented.
pub fn debug_node(node: &Node, source_code: &str) -> String {
    debug_node_with_indentation(node, source_code, 0)
}

fn debug_node_with_indentation(node: &Node, source_code: &str, indent: usize) -> String {
    let mut result = String::new();
    let indent_str = " ".repeat(indent);

    result.push_str(&format!(
        "{}{:?}: {}\n",
        indent_str,
        node,
        node.utf8_text(source_code.as_bytes()).unwrap()
    ));

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        result.push_str(&debug_node_with_indentation(
            &child,
            source_code,
            indent + 2,
        ));
    }

    result
}
