use crate::types::{qualified_id, file_node_id, EdgeType, GraphEdge, GraphNode, NodeType};
use regex::Regex;

pub fn parse_file(rel_path: &str, content: &str) -> (Vec<GraphNode>, Vec<GraphEdge>) {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let norm_path = rel_path.replace('\\', "/");
    let file_node_id = file_node_id(&norm_path);
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
        id: file_node_id.clone(),
        name: file_name,
        node_type: NodeType::File,
        file_path: norm_path.clone(),
        language: lang.to_string(),
        line: None,
    });

    let lines: Vec<&str> = content.lines().collect();

    // 1. C++ Parsing logic
    if lang == "cpp" {
        let inc_re = Regex::new(r#"#include\s+[<"]([^>"]+)[>"]"#).unwrap();
        for cap in inc_re.captures_iter(content) {
            edges.push(GraphEdge {
                source: file_node_id.clone(),
                target: format!("file:{}", &cap[1]),
                edge_type: EdgeType::Includes,
            });
        }

        let class_re = Regex::new(r#"(?:class|struct)\s+([A-Za-z_][A-Za-z0-9_]*)"#).unwrap();
        for (idx, line) in lines.iter().enumerate() {
            if let Some(cap) = class_re.captures(line) {
                let name = cap[1].to_string();
                let is_class = line.contains("class");
                let kind = if is_class { "class" } else { "struct" };
                let n_type = if is_class {
                    NodeType::Class
                } else {
                    NodeType::Struct
                };
                let node_id = qualified_id(kind, &name, &norm_path);

                nodes.push(GraphNode {
                    id: node_id.clone(),
                    name,
                    node_type: n_type,
                    file_path: norm_path.clone(),
                    language: lang.to_string(),
                    line: Some(idx + 1),
                });
                edges.push(GraphEdge {
                    source: file_node_id.clone(),
                    target: node_id,
                    edge_type: EdgeType::Defines,
                });
            }
        }
    }

    // 2. C# Parsing logic
    if lang == "csharp" {
        let using_re = Regex::new(r#"^using\s+([A-Za-z0-9_.]+);"#).unwrap();
        let type_re = Regex::new(
            r#"(?:public|private|protected|internal|static)*\s*(class|interface|struct|enum)\s+([A-Za-z_][A-Za-z0-9_]*)"#,
        )
        .unwrap();

        for (idx, line) in lines.iter().enumerate() {
            let line_trim = line.trim();
            if let Some(cap) = using_re.captures(line_trim) {
                edges.push(GraphEdge {
                    source: file_node_id.clone(),
                    target: format!("namespace:{}", &cap[1]),
                    edge_type: EdgeType::Using,
                });
            } else if let Some(cap) = type_re.captures(line_trim) {
                let kind_str = &cap[1];
                let name = cap[2].to_string();
                let n_type = match kind_str {
                    "interface" => NodeType::Interface,
                    "struct" => NodeType::Struct,
                    "enum" => NodeType::Enum,
                    _ => NodeType::Class,
                };
                let node_id = qualified_id(kind_str, &name, &norm_path);

                nodes.push(GraphNode {
                    id: node_id.clone(),
                    name,
                    node_type: n_type,
                    file_path: norm_path.clone(),
                    language: lang.to_string(),
                    line: Some(idx + 1),
                });
                edges.push(GraphEdge {
                    source: file_node_id.clone(),
                    target: node_id,
                    edge_type: EdgeType::Defines,
                });
            }
        }
    }

    // 3. Rust, TypeScript, Python definitions
    if lang == "rust" || lang == "typescript" || lang == "python" {
        let def_re = Regex::new(
            r#"(?:pub\s+)?(struct|enum|trait|class|fn|def|interface)\s+([A-Za-z0-9_]+)"#,
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
                let node_id = qualified_id(kind_str, &name, &norm_path);

                nodes.push(GraphNode {
                    id: node_id.clone(),
                    name,
                    node_type: n_type,
                    file_path: norm_path.clone(),
                    language: lang.to_string(),
                    line: Some(idx + 1),
                });
                edges.push(GraphEdge {
                    source: file_node_id.clone(),
                    target: node_id,
                    edge_type: EdgeType::Defines,
                });
            }
        }

        if lang == "rust" {
            let mod_re = Regex::new(r#"(?:pub\s+)?mod\s+([a-z_][a-z0-9_]*)"#).unwrap();
            for (idx, line) in lines.iter().enumerate() {
                if let Some(cap) = mod_re.captures(line.trim()) {
                    let name = cap[1].to_string();
                    let node_id = qualified_id("mod", &name, &norm_path);
                    nodes.push(GraphNode {
                        id: node_id.clone(),
                        name,
                        node_type: NodeType::Namespace,
                        file_path: norm_path.clone(),
                        language: lang.to_string(),
                        line: Some(idx + 1),
                    });
                    edges.push(GraphEdge {
                        source: file_node_id.clone(),
                        target: node_id,
                        edge_type: EdgeType::Defines,
                    });
                }
            }

            let use_re = Regex::new(r#"^\s*use\s+([A-Za-z0-9_:{}*]+)"#).unwrap();
            for line in &lines {
                if let Some(cap) = use_re.captures(line) {
                    edges.push(GraphEdge {
                        source: file_node_id.clone(),
                        target: format!("use:{}", &cap[1]),
                        edge_type: EdgeType::Imports,
                    });
                }
            }
        }
    }

    (nodes, edges)
}
