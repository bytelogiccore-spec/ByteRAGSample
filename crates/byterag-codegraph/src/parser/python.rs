use crate::types::{EdgeType, GraphEdge};
use regex::Regex;

pub fn parse_python_imports(file_id: &str, lines: &[&str], edges: &mut Vec<GraphEdge>) {
    let import_re = Regex::new(r#"^\s*(?:from\s+([A-Za-z0-9_.]+)\s+)?import\s+([A-Za-z0-9_.,\s*]+)"#).unwrap();
    for line in lines {
        if let Some(cap) = import_re.captures(line) {
            let module = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let names = cap[2].trim();
            let target = if module.is_empty() {
                format!("import:{}", names.split(',').next().unwrap_or(names).trim())
            } else {
                format!("import:{}.{}", module, names.split(',').next().unwrap_or("").trim())
            };
            edges.push(GraphEdge {
                source: file_id.to_string(),
                target,
                edge_type: EdgeType::Imports,
            });
        }
    }
}
