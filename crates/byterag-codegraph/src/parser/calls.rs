use crate::types::{EdgeType, GraphEdge, GraphNode, NodeType};
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn add_intra_file_calls(
    lines: &[&str],
    nodes: &[GraphNode],
    file_id: &str,
    edges: &mut Vec<GraphEdge>,
) {
    let fn_ids: HashMap<&str, &str> = nodes
        .iter()
        .filter(|n| n.node_type == NodeType::Function)
        .map(|n| (n.name.as_str(), n.id.as_str()))
        .collect();
    if fn_ids.is_empty() {
        return;
    }

    let call_re = Regex::new(r#"\b([A-Za-z_][A-Za-z0-9_]*)\s*\("#).unwrap();
    let mut seen: HashSet<(String, String)> = HashSet::new();
    let keywords: HashSet<&str> = [
        "if", "for", "while", "switch", "catch", "return", "match", "fn", "def", "class", "struct",
        "impl", "new", "typeof", "await", "async", "pub", "use", "mod", "let", "const", "var",
    ]
    .into_iter()
    .collect();

    // Attribute caller: prefer enclosing function by line order
    let mut fn_by_line: Vec<(usize, &str)> = nodes
        .iter()
        .filter(|n| n.node_type == NodeType::Function)
        .filter_map(|n| n.line.map(|l| (l, n.id.as_str())))
        .collect();
    fn_by_line.sort_by_key(|(l, _)| *l);

    for (idx, line) in lines.iter().enumerate() {
        let line_no = idx + 1;
        let caller = fn_by_line
            .iter()
            .rev()
            .find(|(l, _)| *l <= line_no)
            .map(|(_, id)| *id)
            .unwrap_or(file_id);

        for cap in call_re.captures_iter(line) {
            let name = &cap[1];
            if keywords.contains(name) {
                continue;
            }
            let Some(callee) = fn_ids.get(name) else {
                continue;
            };
            if *callee == caller {
                continue;
            }
            let key = (caller.to_string(), (*callee).to_string());
            if !seen.insert(key.clone()) {
                continue;
            }
            edges.push(GraphEdge {
                source: key.0,
                target: key.1,
                edge_type: EdgeType::Calls,
            });
        }
    }
}
