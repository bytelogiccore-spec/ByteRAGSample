use super::common::push_def;
use crate::types::{EdgeType, GraphEdge, GraphNode, NodeType};
use regex::Regex;

pub fn parse_rust_extras(
    norm_path: &str,
    file_id: &str,
    lang: &str,
    lines: &[&str],
    nodes: &mut Vec<GraphNode>,
    edges: &mut Vec<GraphEdge>,
) {
    let mod_re = Regex::new(r#"(?:pub\s+)?mod\s+([a-z_][a-z0-9_]*)"#).unwrap();
    for (idx, line) in lines.iter().enumerate() {
        if let Some(cap) = mod_re.captures(line.trim()) {
            let name = cap[1].to_string();
            push_def(
                nodes,
                edges,
                file_id,
                norm_path,
                lang,
                "mod",
                &name,
                NodeType::Namespace,
                idx + 1,
                crate::types::trim_signature(line),
            );
        }
    }

    let use_re = Regex::new(r#"^\s*use\s+([A-Za-z0-9_:{}*,\s]+);?"#).unwrap();
    for line in lines {
        if let Some(cap) = use_re.captures(line) {
            let path = cap[1].trim().trim_end_matches(';').to_string();
            edges.push(GraphEdge {
                source: file_id.to_string(),
                target: format!("use:{}", path),
                edge_type: EdgeType::Imports,
            });
        }
    }

    let impl_for_re =
        Regex::new(r#"impl(?:\s*<[^>]*>)?\s+([A-Za-z0-9_:]+)\s+for\s+([A-Za-z0-9_:]+)"#).unwrap();
    let impl_type_re = Regex::new(r#"impl(?:\s*<[^>]*>)?\s+([A-Za-z0-9_:]+)\s*\{"#).unwrap();
    for line in lines {
        if let Some(cap) = impl_for_re.captures(line) {
            let trait_name = &cap[1];
            let type_name = &cap[2];
            let type_id = nodes
                .iter()
                .find(|n| n.name == type_name)
                .map(|n| n.id.clone())
                .unwrap_or_else(|| format!("type:{}", type_name));
            edges.push(GraphEdge {
                source: type_id,
                target: format!("trait:{}", trait_name),
                edge_type: EdgeType::Implements,
            });
        } else if let Some(cap) = impl_type_re.captures(line) {
            let type_name = &cap[1];
            if type_name.contains(" for ") {
                continue;
            }
            let _ = type_name; // inherent impl — no edge beyond defines
        }
    }
}
