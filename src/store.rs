use byterag_core::Database;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use crate::parser::parse_file;
use crate::types::GraphNode as SampleNode;

pub struct GraphStore {
    target_dir: PathBuf,
    db: Arc<Database>,
}

impl GraphStore {
    /// Open (or create) the `.byterag` DB without scanning the tree.
    /// MCP `initialize` can answer immediately; call `index_directory` in the background.
    pub fn open(target_dir: PathBuf) -> Self {
        let db_dir = target_dir.join(".byterag");
        if !db_dir.exists() {
            let _ = fs::create_dir_all(&db_dir);
        }

        // Pre-byterag-core leftover; graph lives in Database (wal/wos), not JSON.
        let legacy_json = db_dir.join("graph_store.json");
        if legacy_json.exists() {
            let _ = fs::remove_file(&legacy_json);
        }

        let db = Database::open(&db_dir).unwrap_or_else(|_| {
            Arc::new(Database::open_in_memory().expect("Failed to open ByteRAG Database"))
        });

        Self { target_dir, db }
    }

    pub fn target_dir(&self) -> &PathBuf {
        &self.target_dir
    }

    /// Switch indexing root and reopen the `.byterag` DB under the new path (does not index).
    pub fn set_target_dir(&mut self, target_dir: PathBuf) {
        *self = Self::open(target_dir);
    }

    pub fn index_directory(&self) {
        eprintln!(
            "byterag: indexing start {}",
            self.target_dir.display()
        );
        let mut file_count = 0usize;
        for entry in walkdir::WalkDir::new(&self.target_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                let path_str = path.to_string_lossy();
                if path_str.contains("target")
                    || path_str.contains(".git")
                    || path_str.contains(".byterag")
                {
                    continue;
                }
                if let Ok(content) = fs::read_to_string(path) {
                    file_count += 1;
                    let (nodes, edges) = parse_file(&path_str, &content);
                    for node in nodes {
                        let val = serde_json::to_vec(&node).unwrap_or_default();
                        let _ = self.db.insert("nodes", node.id.as_bytes(), &val);
                    }

                    for (idx, edge) in edges.iter().enumerate() {
                        let key = format!("edge:{}:{}", edge.source, idx);
                        let val = serde_json::to_vec(&edge).unwrap_or_default();
                        let _ = self.db.insert("edges", key.as_bytes(), &val);
                    }
                }
            }
        }
        eprintln!(
            "byterag: indexing done {} ({} files)",
            self.target_dir.display(),
            file_count
        );
    }

    pub fn query_subgraph(&self, seed_ids: &[String], max_depth: usize) -> serde_json::Value {
        let mut matched_nodes = Vec::new();
        let matched_edges: Vec<serde_json::Value> = Vec::new();

        for seed in seed_ids {
            if let Ok(Some(val)) = self.db.get("nodes", seed.as_bytes()) {
                if let Ok(node) = serde_json::from_slice::<SampleNode>(&val) {
                    matched_nodes.push(node);
                }
            }
        }

        serde_json::json!({
            "nodes": matched_nodes,
            "edges": matched_edges,
            "query_seeds": seed_ids,
            "max_depth": max_depth
        })
    }

    pub fn search_symbols(&self, query: &str) -> Vec<SampleNode> {
        let q = query.to_lowercase();
        let mut results = Vec::new();

        if let Ok(Some(val)) = self.db.get("nodes", q.as_bytes()) {
            if let Ok(node) = serde_json::from_slice::<SampleNode>(&val) {
                results.push(node);
            }
        }

        results
    }
}
