use super::common::push_def;
use crate::types::{qualified_id, trim_signature, EdgeType, GraphEdge, GraphNode, NodeType};
use regex::Regex;

pub fn parse_cpp(
    norm_path: &str,
    file_id: &str,
    lang: &str,
    lines: &[&str],
    content: &str,
    nodes: &mut Vec<GraphNode>,
    edges: &mut Vec<GraphEdge>,
) {
    let inc_re = Regex::new(r#"#include\s+[<"]([^>"]+)[>"]"#).unwrap();
    for cap in inc_re.captures_iter(content) {
        edges.push(GraphEdge {
            source: file_id.to_string(),
            target: format!("file:{}", &cap[1]),
            edge_type: EdgeType::Includes,
        });
    }

    let class_re = Regex::new(r#"(?:class|struct)\s+([A-Za-z_][A-Za-z0-9_]*)"#).unwrap();
    let inherit_re = Regex::new(
        r#"(?:class|struct)\s+([A-Za-z_][A-Za-z0-9_]*)\s*:\s*(?:public|protected|private)?\s*([A-Za-z_][A-Za-z0-9_:]*)"#,
    )
    .unwrap();

    for (idx, line) in lines.iter().enumerate() {
        if let Some(cap) = inherit_re.captures(line) {
            let name = cap[1].to_string();
            let base = cap[2].to_string();
            let is_class = line.contains("class");
            let kind = if is_class { "class" } else { "struct" };
            let n_type = if is_class {
                NodeType::Class
            } else {
                NodeType::Struct
            };
            let node_id = qualified_id(kind, &name, norm_path);
            if !nodes.iter().any(|n| n.id == node_id) {
                push_def(
                    nodes,
                    edges,
                    file_id,
                    norm_path,
                    lang,
                    kind,
                    &name,
                    n_type,
                    idx + 1,
                    trim_signature(line),
                );
            }
            edges.push(GraphEdge {
                source: node_id,
                target: format!("type:{}", base),
                edge_type: EdgeType::Extends,
            });
            continue;
        }
        if let Some(cap) = class_re.captures(line) {
            let name = cap[1].to_string();
            let is_class = line.contains("class");
            let kind = if is_class { "class" } else { "struct" };
            let n_type = if is_class {
                NodeType::Class
            } else {
                NodeType::Struct
            };
            let node_id = qualified_id(kind, &name, norm_path);
            if nodes.iter().any(|n| n.id == node_id) {
                continue;
            }
            push_def(
                nodes,
                edges,
                file_id,
                norm_path,
                lang,
                kind,
                &name,
                n_type,
                idx + 1,
                trim_signature(line),
            );
        }
    }
}
