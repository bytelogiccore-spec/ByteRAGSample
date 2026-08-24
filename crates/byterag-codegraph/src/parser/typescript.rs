use super::common::push_def;
use crate::types::{qualified_id, trim_signature, EdgeType, GraphEdge, GraphNode, NodeType};
use regex::Regex;

pub fn parse_ts_relations(
    norm_path: &str,
    file_id: &str,
    lang: &str,
    lines: &[&str],
    nodes: &mut Vec<GraphNode>,
    edges: &mut Vec<GraphEdge>,
) {
    let class_ext = Regex::new(
        r#"class\s+([A-Za-z_][A-Za-z0-9_]*)(?:\s+extends\s+([A-Za-z0-9_.]+))?(?:\s+implements\s+([^{]+))?"#,
    )
    .unwrap();
    let iface_ext = Regex::new(
        r#"interface\s+([A-Za-z_][A-Za-z0-9_]*)(?:\s+extends\s+([^{]+))?"#,
    )
    .unwrap();
    let import_re = Regex::new(r#"^\s*import\s+.+?\s+from\s+['\"]([^'\"]+)['\"]"#).unwrap();

    for (idx, line) in lines.iter().enumerate() {
        if let Some(cap) = import_re.captures(line) {
            edges.push(GraphEdge {
                source: file_id.to_string(),
                target: format!("import:{}", &cap[1]),
                edge_type: EdgeType::Imports,
            });
        }
        if let Some(cap) = class_ext.captures(line) {
            let name = &cap[1];
            let node_id = qualified_id("class", name, norm_path);
            if !nodes.iter().any(|n| n.id == node_id) {
                push_def(
                    nodes,
                    edges,
                    file_id,
                    norm_path,
                    lang,
                    "class",
                    name,
                    NodeType::Class,
                    idx + 1,
                    trim_signature(line),
                );
            }
            if let Some(base) = cap.get(2) {
                edges.push(GraphEdge {
                    source: node_id.clone(),
                    target: format!("type:{}", base.as_str()),
                    edge_type: EdgeType::Extends,
                });
            }
            if let Some(impls) = cap.get(3) {
                for iface in impls.as_str().split(',') {
                    let iface = iface.trim();
                    if iface.is_empty() {
                        continue;
                    }
                    edges.push(GraphEdge {
                        source: node_id.clone(),
                        target: format!("type:{}", iface),
                        edge_type: EdgeType::Implements,
                    });
                }
            }
        }
        if let Some(cap) = iface_ext.captures(line) {
            let name = &cap[1];
            let node_id = qualified_id("interface", name, norm_path);
            if !nodes.iter().any(|n| n.id == node_id) {
                push_def(
                    nodes,
                    edges,
                    file_id,
                    norm_path,
                    lang,
                    "interface",
                    name,
                    NodeType::Interface,
                    idx + 1,
                    trim_signature(line),
                );
            }
            if let Some(bases) = cap.get(2) {
                for base in bases.as_str().split(',') {
                    let base = base.trim();
                    if base.is_empty() {
                        continue;
                    }
                    edges.push(GraphEdge {
                        source: node_id.clone(),
                        target: format!("type:{}", base),
                        edge_type: EdgeType::Extends,
                    });
                }
            }
        }
    }
}
