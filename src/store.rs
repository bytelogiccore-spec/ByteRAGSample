use byterag_core::Database;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::parser::parse_file;
use crate::types::{GraphEdge, GraphNode as SampleNode};

const DEFAULT_SEARCH_LIMIT: usize = 50;

pub struct GraphStore {
    target_dir: PathBuf,
    db: Arc<Database>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct SearchScore(u8);

impl GraphStore {
    pub fn open(target_dir: PathBuf) -> Self {
        let db_dir = target_dir.join(".byterag");
        if !db_dir.exists() {
            let _ = fs::create_dir_all(&db_dir);
        }

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

    pub fn set_target_dir(&mut self, target_dir: PathBuf) {
        *self = Self::open(target_dir);
    }

    pub fn index_directory(&self) {
        eprintln!("byterag: indexing start {}", self.target_dir.display());
        self.clear_table("nodes");
        self.clear_table("edges");

        let mut file_count = 0usize;
        for entry in walkdir::WalkDir::new(&self.target_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let path_str = path.to_string_lossy();
            if path_str.contains("target")
                || path_str.contains(".git")
                || path_str.contains(".byterag")
            {
                continue;
            }
            let Ok(content) = fs::read_to_string(path) else {
                continue;
            };
            file_count += 1;
            let rel_path = self.rel_path(path);
            let (nodes, edges) = parse_file(&rel_path, &content);
            for node in nodes {
                let val = serde_json::to_vec(&node).unwrap_or_default();
                let _ = self.db.insert("nodes", node.id.as_bytes(), &val);
            }
            for (idx, edge) in edges.iter().enumerate() {
                let key = format!("edge:{}:{}", edge.source, idx);
                let val = serde_json::to_vec(edge).unwrap_or_default();
                let _ = self.db.insert("edges", key.as_bytes(), &val);
            }
        }
        eprintln!(
            "byterag: indexing done {} ({} files)",
            self.target_dir.display(),
            file_count
        );
    }

    pub fn query_subgraph(&self, seed_ids: &[String], max_depth: usize) -> serde_json::Value {
        let resolved = self.resolve_seeds(seed_ids);
        if resolved.is_empty() {
            return serde_json::json!({
                "nodes": [],
                "edges": [],
                "query_seeds": seed_ids,
                "resolved_seeds": [],
                "max_depth": max_depth
            });
        }

        let all_edges = self.load_all_edges();
        let mut visited_nodes: HashSet<String> = HashSet::new();
        let mut visited_edges: HashSet<String> = HashSet::new();
        let mut node_map: HashMap<String, SampleNode> = HashMap::new();
        let mut edge_list: Vec<GraphEdge> = Vec::new();
        let mut frontier: VecDeque<String> = resolved.iter().cloned().collect();
        for id in &resolved {
            visited_nodes.insert(id.clone());
            if let Some(node) = self.get_node(id) {
                node_map.insert(id.clone(), node);
            }
        }

        for _depth in 0..max_depth {
            if frontier.is_empty() {
                break;
            }
            let level_size = frontier.len();
            for _ in 0..level_size {
                let Some(current) = frontier.pop_front() else {
                    break;
                };
                for edge in &all_edges {
                    if edge.source != current && edge.target != current {
                        continue;
                    }
                    let edge_key = format!("{}->{}", edge.source, edge.target);
                    if !visited_edges.insert(edge_key) {
                        continue;
                    }
                    edge_list.push(edge.clone());

                    for neighbor in [&edge.source, &edge.target] {
                        if *neighbor == current {
                            continue;
                        }
                        if visited_nodes.insert(neighbor.clone()) {
                            if let Some(node) = self.get_node(neighbor) {
                                node_map.insert(neighbor.clone(), node);
                            }
                            frontier.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }

        let mut nodes: Vec<SampleNode> = node_map.into_values().collect();
        nodes.sort_by(|a, b| a.id.cmp(&b.id));

        serde_json::json!({
            "nodes": nodes,
            "edges": edge_list,
            "query_seeds": seed_ids,
            "resolved_seeds": resolved,
            "max_depth": max_depth
        })
    }

    pub fn search_symbols(&self, query: &str) -> Vec<SampleNode> {
        self.search_symbols_with_limit(query, DEFAULT_SEARCH_LIMIT)
    }

    pub fn search_symbols_with_limit(&self, query: &str, limit: usize) -> Vec<SampleNode> {
        let q = query.trim();
        if q.is_empty() {
            return Vec::new();
        }

        let q_lower = q.to_lowercase();
        let mut scored: Vec<(SearchScore, SampleNode)> = Vec::new();

        if let Some(node) = self.get_node(q) {
            scored.push((SearchScore(100), node));
        }

        let Ok(entries) = self.db.scan("nodes") else {
            return Self::dedupe_and_limit(scored, limit);
        };

        for (_key, val) in entries {
            let Ok(node) = serde_json::from_slice::<SampleNode>(&val) else {
                continue;
            };
            if scored.iter().any(|(_, n)| n.id == node.id) {
                continue;
            }
            let id_lower = node.id.to_lowercase();
            let name_lower = node.name.to_lowercase();
            let path_lower = node.file_path.to_lowercase();

            let score = if id_lower == q_lower {
                SearchScore(95)
            } else if id_lower.starts_with(&format!("{q_lower}@"))
                || id_lower.starts_with(&format!("{q_lower}:"))
            {
                SearchScore(90)
            } else if name_lower == q_lower {
                SearchScore(85)
            } else if id_lower.contains(&q_lower) {
                SearchScore(75)
            } else if name_lower.contains(&q_lower) {
                SearchScore(70)
            } else if path_lower.contains(&q_lower) {
                SearchScore(60)
            } else {
                continue;
            };
            scored.push((score, node));
        }

        Self::dedupe_and_limit(scored, limit)
    }

    fn dedupe_and_limit(mut scored: Vec<(SearchScore, SampleNode)>, limit: usize) -> Vec<SampleNode> {
        scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.id.cmp(&b.1.id)));
        scored.into_iter().take(limit).map(|(_, n)| n).collect()
    }

    fn resolve_seeds(&self, seed_ids: &[String]) -> Vec<String> {
        let mut resolved = Vec::new();
        let mut seen = HashSet::new();
        for seed in seed_ids {
            if seed.is_empty() {
                continue;
            }
            if let Some(node) = self.get_node(seed) {
                if seen.insert(node.id.clone()) {
                    resolved.push(node.id);
                }
                continue;
            }
            let seed_lower = seed.to_lowercase();
            let Ok(entries) = self.db.scan("nodes") else {
                continue;
            };
            for (_key, val) in entries {
                let Ok(node) = serde_json::from_slice::<SampleNode>(&val) else {
                    continue;
                };
                let id_lower = node.id.to_lowercase();
                let name_lower = node.name.to_lowercase();
                let matches = id_lower == seed_lower
                    || id_lower.starts_with(&format!("{seed_lower}@"))
                    || (name_lower == seed_lower && !seed.contains('@'));
                if matches && seen.insert(node.id.clone()) {
                    resolved.push(node.id);
                }
            }
        }
        resolved.sort();
        resolved
    }

    fn get_node(&self, id: &str) -> Option<SampleNode> {
        let val = self.db.get("nodes", id.as_bytes()).ok()??;
        serde_json::from_slice(&val).ok()
    }

    fn load_all_edges(&self) -> Vec<GraphEdge> {
        let Ok(entries) = self.db.scan("edges") else {
            return Vec::new();
        };
        entries
            .into_iter()
            .filter_map(|(_key, val)| serde_json::from_slice::<GraphEdge>(&val).ok())
            .collect()
    }

    fn clear_table(&self, table: &str) {
        let Ok(entries) = self.db.scan(table) else {
            return;
        };
        for (key, _) in entries {
            let _ = self.db.delete(table, &key);
        }
    }

    fn rel_path(&self, path: &Path) -> String {
        path.strip_prefix(&self.target_dir)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_else(|_| path.to_string_lossy().replace('\\', "/"))
    }
}
