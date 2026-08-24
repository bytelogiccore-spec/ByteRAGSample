use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NodeType {
    File,
    Class,
    Struct,
    Interface,
    Enum,
    Function,
    Trait,
    Namespace,
}

impl NodeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            NodeType::File => "file",
            NodeType::Class => "class",
            NodeType::Struct => "struct",
            NodeType::Interface => "interface",
            NodeType::Enum => "enum",
            NodeType::Function => "function",
            NodeType::Trait => "trait",
            NodeType::Namespace => "namespace",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "file" => Some(NodeType::File),
            "class" => Some(NodeType::Class),
            "struct" => Some(NodeType::Struct),
            "interface" => Some(NodeType::Interface),
            "enum" => Some(NodeType::Enum),
            "function" | "fn" | "def" => Some(NodeType::Function),
            "trait" => Some(NodeType::Trait),
            "namespace" | "mod" => Some(NodeType::Namespace),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum EdgeType {
    Defines,
    Implements,
    Extends,
    Inherits,
    Imports,
    Includes,
    Using,
    Calls,
}

impl EdgeType {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            EdgeType::Defines => "defines",
            EdgeType::Implements => "implements",
            EdgeType::Extends => "extends",
            EdgeType::Inherits => "inherits",
            EdgeType::Imports => "imports",
            EdgeType::Includes => "includes",
            EdgeType::Using => "using",
            EdgeType::Calls => "calls",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "defines" => Some(EdgeType::Defines),
            "implements" => Some(EdgeType::Implements),
            "extends" => Some(EdgeType::Extends),
            "inherits" => Some(EdgeType::Inherits),
            "imports" => Some(EdgeType::Imports),
            "includes" => Some(EdgeType::Includes),
            "using" => Some(EdgeType::Using),
            "calls" => Some(EdgeType::Calls),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub name: String,
    pub node_type: NodeType,
    pub file_path: String,
    pub language: String,
    pub line: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub edge_type: EdgeType,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GraphData {
    pub nodes: std::collections::HashMap<String, GraphNode>,
    pub edges: Vec<GraphEdge>,
}

/// Build a file-scoped symbol id, e.g. `struct:Database@core/byterag-core/src/engine/database.rs`.
pub fn qualified_id(kind: &str, name: &str, rel_path: &str) -> String {
    format!("{}:{}@{}", kind, name, rel_path.replace('\\', "/"))
}

pub fn file_node_id(rel_path: &str) -> String {
    format!("file:{}", rel_path.replace('\\', "/"))
}

pub fn trim_signature(line: &str) -> Option<String> {
    let s = line.trim();
    if s.is_empty() {
        return None;
    }
    let mut out = s.to_string();
    if out.len() > 160 {
        out.truncate(157);
        out.push_str("...");
    }
    Some(out)
}
