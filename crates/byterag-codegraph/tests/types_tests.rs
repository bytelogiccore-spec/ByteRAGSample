use byterag_codegraph::types::{EdgeType, NodeType};

/// @test_id: TC-TYPES-001
/// @title: NodeType 문자열 파서 유효성 검증
/// @purpose: 언어별 키워드(class, struct, fn, def, interface, trait, mod)가 NodeType 열거형으로 정확히 매핑되는지 검증한다.
/// @expected: 정확한 NodeType Variant 반환, 유효하지 않은 문자열은 None 반환
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

/// @test_id: TC-TYPES-002
/// @title: EdgeType 관계 문자열 직렬화 검증
/// @purpose: defines, calls, imports, extends, implements 등 의존성 엣지 식별자가 규격에 맞게 변환되는지 검증한다.
/// @expected: 각 EdgeType이 소문자 표준 관계명 반환
#[test]
fn test_edge_type_as_str() {
    assert_eq!(EdgeType::Defines.as_str(), "defines");
    assert_eq!(EdgeType::Calls.as_str(), "calls");
    assert_eq!(EdgeType::Imports.as_str(), "imports");
    assert_eq!(EdgeType::Extends.as_str(), "extends");
    assert_eq!(EdgeType::Implements.as_str(), "implements");
}
