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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub name: String,
    pub node_type: NodeType,
    pub file_path: String,
    pub language: String,
    pub line: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub edge_type: EdgeType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GraphData {
    pub nodes: std::collections::HashMap<String, GraphNode>,
    pub edges: Vec<GraphEdge>,
}

/// Build a file-scoped symbol id, e.g. `struct:Database@core/byterag-core/src/engine/database.rs`.
pub fn qualified_id(kind: &str, name: &str, rel_path: &str) -> String {
    format!(
        "{}:{}@{}",
        kind,
        name,
        rel_path.replace('\\', "/")
    )
}

pub fn file_node_id(rel_path: &str) -> String {
    format!("file:{}", rel_path.replace('\\', "/"))
}
