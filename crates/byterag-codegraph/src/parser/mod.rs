pub mod calls;
pub mod common;
pub mod cpp;
pub mod csharp;
pub mod python;
pub mod rust;
pub mod typescript;

use crate::types::{file_node_id, EdgeType, GraphEdge, GraphNode, NodeType};
use std::collections::HashMap;

pub use calls::add_intra_file_calls;
pub use common::parse_common_defs;
pub use cpp::parse_cpp;
pub use csharp::parse_csharp;
pub use python::parse_python_imports;
pub use rust::parse_rust_extras;
pub use typescript::parse_ts_relations;

pub fn parse_file(rel_path: &str, content: &str) -> (Vec<GraphNode>, Vec<GraphEdge>) {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let norm_path = rel_path.replace('\\', "/");
    let file_id = file_node_id(&norm_path);
    let file_name = norm_path.split('/').last().unwrap_or(&norm_path).to_string();

    let ext = norm_path.split('.').last().unwrap_or("").to_lowercase();
    let lang = match ext.as_str() {
        "cpp" | "h" | "hpp" | "cxx" | "cc" => "cpp",
        "cs" => "csharp",
        "rs" => "rust",
        "ts" | "tsx" | "js" | "jsx" => "typescript",
        "py" => "python",
        _ => "unknown",
    };

    if lang == "unknown" {
        return (nodes, edges);
    }

    nodes.push(GraphNode {
        id: file_id.clone(),
        name: file_name,
        node_type: NodeType::File,
        file_path: norm_path.clone(),
        language: lang.to_string(),
        line: None,
        signature: None,
    });

    let lines: Vec<&str> = content.lines().collect();

    if lang == "cpp" {
        parse_cpp(&norm_path, &file_id, lang, &lines, content, &mut nodes, &mut edges);
    } else if lang == "csharp" {
        parse_csharp(&norm_path, &file_id, lang, &lines, &mut nodes, &mut edges);
    } else if lang == "rust" || lang == "typescript" || lang == "python" {
        parse_common_defs(&norm_path, &file_id, lang, &lines, &mut nodes, &mut edges);
        if lang == "rust" {
            parse_rust_extras(&norm_path, &file_id, lang, &lines, &mut nodes, &mut edges);
        }
        if lang == "typescript" {
            parse_ts_relations(&norm_path, &file_id, lang, &lines, &mut nodes, &mut edges);
        }
        if lang == "python" {
            parse_python_imports(&file_id, &lines, &mut edges);
        }
    }

    add_intra_file_calls(&lines, &nodes, &file_id, &mut edges);

    (nodes, edges)
}

/// Resolve unresolved `use:...` / bare import targets against known symbol names in the index.
pub fn resolve_import_edges(nodes: &[GraphNode], edges: &mut [GraphEdge]) {
    let mut by_name: HashMap<String, Vec<String>> = HashMap::new();
    for n in nodes {
        by_name.entry(n.name.clone()).or_default().push(n.id.clone());
    }

    for edge in edges.iter_mut() {
        if edge.edge_type != EdgeType::Imports && edge.edge_type != EdgeType::Using {
            continue;
        }
        if !edge.target.starts_with("use:")
            && !edge.target.starts_with("namespace:")
            && !edge.target.starts_with("import:")
        {
            continue;
        }

        let raw = edge
            .target
            .split_once(':')
            .map(|(_, rest)| rest)
            .unwrap_or(edge.target.as_str());
        let leaf = raw
            .rsplit("::")
            .next()
            .unwrap_or(raw)
            .rsplit('.')
            .next()
            .unwrap_or(raw)
            .trim_matches(|c| c == '{' || c == '}' || c == '*' || c == ' ')
            .split(',')
            .next()
            .unwrap_or("")
            .trim();
        if leaf.is_empty() || leaf == "*" || leaf == "self" || leaf == "super" || leaf == "crate" {
            continue;
        }
        if let Some(ids) = by_name.get(leaf) {
            if ids.len() == 1 {
                edge.target = ids[0].clone();
            }
        }
    }
}
