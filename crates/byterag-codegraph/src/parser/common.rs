use crate::types::{qualified_id, trim_signature, EdgeType, GraphEdge, GraphNode, NodeType};
use regex::Regex;

pub(crate) fn push_def(
    nodes: &mut Vec<GraphNode>,
    edges: &mut Vec<GraphEdge>,
    file_id: &str,
    norm_path: &str,
    lang: &str,
    kind: &str,
    name: &str,
    n_type: NodeType,
    line: usize,
    signature: Option<String>,
) {
    let node_id = qualified_id(kind, name, norm_path);
    nodes.push(GraphNode {
        id: node_id.clone(),
        name: name.to_string(),
        node_type: n_type,
        file_path: norm_path.to_string(),
        language: lang.to_string(),
        line: Some(line),
        signature,
    });
    edges.push(GraphEdge {
        source: file_id.to_string(),
        target: node_id,
        edge_type: EdgeType::Defines,
    });
}

pub fn parse_common_defs(
    norm_path: &str,
    file_id: &str,
    lang: &str,
    lines: &[&str],
    nodes: &mut Vec<GraphNode>,
    edges: &mut Vec<GraphEdge>,
) {
    let def_re = Regex::new(
        r#"(?:pub\s+(?:\(crate\)\s+)?)?(?:async\s+)?(struct|enum|trait|class|fn|def|interface)\s+([A-Za-z0-9_]+)"#,
    )
    .unwrap();
    for (idx, line) in lines.iter().enumerate() {
        if let Some(cap) = def_re.captures(line) {
            let kind_str = &cap[1];
            let name = cap[2].to_string();
            let n_type = match kind_str {
                "struct" => NodeType::Struct,
                "enum" => NodeType::Enum,
                "trait" => NodeType::Trait,
                "interface" => NodeType::Interface,
                "fn" | "def" => NodeType::Function,
                _ => NodeType::Class,
            };
            let id_kind = if kind_str == "def" { "fn" } else { kind_str };
            push_def(
                nodes,
                edges,
                file_id,
                norm_path,
                lang,
                id_kind,
                &name,
                n_type,
                idx + 1,
                trim_signature(line),
            );
        }
    }
}
