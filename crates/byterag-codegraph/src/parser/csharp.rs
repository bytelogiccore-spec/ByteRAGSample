use super::common::push_def;
use crate::types::{qualified_id, trim_signature, EdgeType, GraphEdge, GraphNode, NodeType};
use regex::Regex;

pub fn parse_csharp(
    norm_path: &str,
    file_id: &str,
    lang: &str,
    lines: &[&str],
    nodes: &mut Vec<GraphNode>,
    edges: &mut Vec<GraphEdge>,
) {
    let using_re = Regex::new(r#"^using\s+([A-Za-z0-9_.]+);"#).unwrap();
    let type_re = Regex::new(
        r#"(?:public|private|protected|internal|static|sealed|abstract|partial|\s)*\s*(class|interface|struct|enum)\s+([A-Za-z_][A-Za-z0-9_]*)(?:\s*:\s*([^{]+))?"#,
    )
    .unwrap();

    for (idx, line) in lines.iter().enumerate() {
        let line_trim = line.trim();
        if let Some(cap) = using_re.captures(line_trim) {
            edges.push(GraphEdge {
                source: file_id.to_string(),
                target: format!("namespace:{}", &cap[1]),
                edge_type: EdgeType::Using,
            });
            continue;
        }
        if let Some(cap) = type_re.captures(line_trim) {
            let kind_str = &cap[1];
            let name = cap[2].to_string();
            let n_type = match kind_str {
                "interface" => NodeType::Interface,
                "struct" => NodeType::Struct,
                "enum" => NodeType::Enum,
                _ => NodeType::Class,
            };
            let node_id = qualified_id(kind_str, &name, norm_path);
            push_def(
                nodes,
                edges,
                file_id,
                norm_path,
                lang,
                kind_str,
                &name,
                n_type,
                idx + 1,
                trim_signature(line),
            );
            if let Some(bases) = cap.get(3) {
                for base in bases.as_str().split(',') {
                    let base = base.trim();
                    if base.is_empty() {
                        continue;
                    }
                    let edge_type = if kind_str == "interface" || base.starts_with('I') {
                        EdgeType::Implements
                    } else {
                        EdgeType::Extends
                    };
                    edges.push(GraphEdge {
                        source: node_id.clone(),
                        target: format!("type:{}", base),
                        edge_type,
                    });
                }
            }
        }
    }
}
