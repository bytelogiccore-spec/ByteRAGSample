use crate::parser::parse_file;
use crate::types::{GraphData, GraphNode};
use std::collections::{HashSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct GraphStore {
    target_dir: PathBuf,
    data: GraphData,
}

impl GraphStore {
    pub fn new(target_dir: PathBuf) -> Self {
        let mut store = Self {
            target_dir,
            data: GraphData::default(),
        };
        store.load();
        store
    }

    pub fn set_target_dir(&mut self, dir: PathBuf) {
        self.target_dir = dir;
        self.load();
    }

    fn store_path(&self) -> PathBuf {
        let dir = self.target_dir.join(".byterag");
        if !dir.exists() {
            let _ = fs::create_dir_all(&dir);
        }
        dir.join("graph_store.json")
    }

    pub fn load(&mut self) {
        let path = self.store_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(parsed) = serde_json::from_str::<GraphData>(&content) {
                    self.data = parsed;
                    return;
                }
            }
        }
        self.data = GraphData::default();
    }

    pub fn save(&self) {
        let path = self.store_path();
        if let Ok(content) = serde_json::to_string_pretty(&self.data) {
            let _ = fs::write(path, content);
        }
    }

    pub fn index_directory(&mut self) {
        self.data = GraphData::default();

        for entry in WalkDir::new(&self.target_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                let path_str = path.to_string_lossy();
                if path_str.contains("target") || path_str.contains(".git") || path_str.contains(".byterag") {
                    continue;
                }
                if let Ok(content) = fs::read_to_string(path) {
                    let (nodes, edges) = parse_file(&path_str, &content);
                    for node in nodes {
                        self.data.nodes.insert(node.id.clone(), node);
                    }
                    self.data.edges.extend(edges);
                }
            }
        }
        self.save();
    }

    pub fn query_subgraph(&self, seed_ids: &[String], max_depth: usize) -> GraphData {
        let mut visited = HashSet::new();
        let mut matched_edges = Vec::new();
        let mut queue = VecDeque::new();

        for id in seed_ids {
            queue.push_back(id.clone());
        }

        for _ in 0..max_depth {
            let mut next_queue = VecDeque::new();
            while let Some(curr) = queue.pop_front() {
                if visited.insert(curr.clone()) {
                    for edge in &self.data.edges {
                        if edge.source == curr || edge.target == curr {
                            matched_edges.push(edge.clone());
                            let neighbor = if edge.source == curr { &edge.target } else { &edge.source };
                            if !visited.contains(neighbor) {
                                next_queue.push_back(neighbor.clone());
                            }
                        }
                    }
                }
            }
            queue = next_queue;
        }

        let mut nodes = std::collections::HashMap::new();
        for id in visited {
            if let Some(node) = self.data.nodes.get(&id) {
                nodes.insert(id, node.clone());
            }
        }

        GraphData {
            nodes,
            edges: matched_edges,
        }
    }

    pub fn search_symbols(&self, query: &str) -> Vec<GraphNode> {
        let q = query.to_lowercase();
        self.data
            .nodes
            .values()
            .filter(|n| n.name.to_lowercase().contains(&q) || n.id.to_lowercase().contains(&q))
            .cloned()
            .collect()
    }
}
