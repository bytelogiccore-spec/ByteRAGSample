use byterag_codegraph::types::{EdgeType, NodeType};

#[test]
fn test_node_type_parsing() {
    assert_eq!(NodeType::parse("class"), Some(NodeType::Class));
    assert_eq!(NodeType::parse("struct"), Some(NodeType::Struct));
    assert_eq!(NodeType::parse("fn"), Some(NodeType::Function));
    assert_eq!(NodeType::parse("def"), Some(NodeType::Function));
    assert_eq!(NodeType::parse("interface"), Some(NodeType::Interface));
    assert_eq!(NodeType::parse("trait"), Some(NodeType::Trait));
    assert_eq!(NodeType::parse("mod"), Some(NodeType::Namespace));
    assert_eq!(NodeType::parse("invalid_type"), None);
}

#[test]
fn test_edge_type_as_str() {
    assert_eq!(EdgeType::Defines.as_str(), "defines");
    assert_eq!(EdgeType::Calls.as_str(), "calls");
    assert_eq!(EdgeType::Imports.as_str(), "imports");
    assert_eq!(EdgeType::Extends.as_str(), "extends");
    assert_eq!(EdgeType::Implements.as_str(), "implements");
}
