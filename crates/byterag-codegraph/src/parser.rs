use crate::types::{
    file_node_id, qualified_id, trim_signature, EdgeType, GraphEdge, GraphNode, NodeType,
};
use regex::Regex;
use std::collections::{HashMap, HashSet};

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

fn push_def(
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

fn parse_cpp(
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

fn parse_csharp(
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

fn parse_common_defs(
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

fn parse_rust_extras(
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
                trim_signature(line),
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

fn parse_ts_relations(
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

fn parse_python_imports(file_id: &str, lines: &[&str], edges: &mut Vec<GraphEdge>) {
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

fn add_intra_file_calls(
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
