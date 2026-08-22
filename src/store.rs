use byterag_core::graph::csr::CsrGraph;
use byterag_core::Database;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::parser::{parse_file, resolve_import_edges};
use crate::types::{EdgeType, GraphEdge, GraphNode as SampleNode, NodeType};

const EXCLUDE_DIR_NAMES: &[&str] = &[
    "target",
    ".git",
    ".byterag",
    "node_modules",
    "dist",
    "build",
    "__pycache__",
];

pub struct GraphStore {
    target_dir: PathBuf,
    db: Arc<Database>,
    indexing: Arc<AtomicBool>,
    last_indexed_at: Arc<Mutex<Option<u64>>>,
    last_file_count: Arc<Mutex<usize>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct SearchScore(u8);

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
struct FileMetaStored {
    mtime_secs: u64,
    node_ids: Vec<String>,
    edge_keys: Vec<String>,
}

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

        Self {
            target_dir,
            db,
            indexing: Arc::new(AtomicBool::new(false)),
            last_indexed_at: Arc::new(Mutex::new(None)),
            last_file_count: Arc::new(Mutex::new(0)),
        }
    }

    pub fn target_dir(&self) -> &PathBuf {
        &self.target_dir
    }

    pub fn set_target_dir(&mut self, target_dir: PathBuf) {
        *self = Self::open(target_dir);
    }

    pub fn index_directory(&self) {
        self.indexing.store(true, Ordering::SeqCst);
        eprintln!("byterag: indexing start {}", self.target_dir.display());

        let mut seen_files: HashSet<String> = HashSet::new();
        let mut file_count = 0usize;
        let mut parsed = 0usize;

        let walk = walkdir::WalkDir::new(&self.target_dir).into_iter();
        for entry in walk.filter_map(|e| e.ok()) {
            let path = entry.path();
            if Self::path_excluded(path) {
                continue;
            }
            if !path.is_file() {
                continue;
            }

            let rel = self.rel_path(path);
            seen_files.insert(rel.clone());

            let mtime = file_mtime_secs(path).unwrap_or(0);
            if let Some(meta) = self.get_file_meta(&rel) {
                if meta.mtime_secs == mtime {
                    file_count += 1;
                    continue;
                }
                self.remove_file_artifacts(&meta);
            }

            let Ok(content) = fs::read_to_string(path) else {
                continue;
            };
            file_count += 1;
            parsed += 1;
            let (nodes, mut edges) = parse_file(&rel, &content);

            // Per-file import resolve against local defs first; global pass later.
            resolve_import_edges(&nodes, &mut edges);

            let mut node_ids = Vec::new();
            let mut edge_keys = Vec::new();
            for node in &nodes {
                let val = serde_json::to_vec(node).unwrap_or_default();
                let _ = self.db.insert("nodes", node.id.as_bytes(), &val);
                node_ids.push(node.id.clone());
            }
            for (idx, edge) in edges.iter().enumerate() {
                let key = format!("edge:{}:{}:{}", rel, edge.source, idx);
                let val = serde_json::to_vec(edge).unwrap_or_default();
                let _ = self.db.insert("edges", key.as_bytes(), &val);
                edge_keys.push(key);
            }
            let meta = FileMetaStored {
                mtime_secs: mtime,
                node_ids,
                edge_keys,
            };
            if let Ok(bytes) = serde_json::to_vec(&meta) {
                let _ = self.db.insert("file_meta", rel.as_bytes(), &bytes);
            }
            let _ = (nodes, edges);
        }

        // Drop deleted files from previous index.
        if let Ok(entries) = self.db.scan("file_meta") {
            for (key, val) in entries {
                let rel = String::from_utf8_lossy(&key).to_string();
                if seen_files.contains(&rel) {
                    continue;
                }
                if let Ok(meta) = serde_json::from_slice::<FileMetaStored>(&val) {
                    self.remove_file_artifacts(&meta);
                }
                let _ = self.db.delete("file_meta", &key);
            }
        }

        // Global import resolve across the whole graph.
        self.re_resolve_all_imports();

        if let Err(err) = self.db.flush() {
            eprintln!("byterag: flush after index failed: {err}");
        }

        let now = now_unix_secs();
        if let Ok(mut g) = self.last_indexed_at.lock() {
            *g = Some(now);
        }
        if let Ok(mut g) = self.last_file_count.lock() {
            *g = file_count;
        }
        self.indexing.store(false, Ordering::SeqCst);

        eprintln!(
            "byterag: indexing done {} ({} files, {} reparsed)",
            self.target_dir.display(),
            file_count,
            parsed
        );
    }

    pub fn index_status(&self) -> serde_json::Value {
        let nodes = self.count_table("nodes");
        let edges = self.count_table("edges");
        let files = self
            .last_file_count
            .lock()
            .ok()
            .map(|g| *g)
            .unwrap_or_else(|| self.count_table("file_meta"));
        let last = self
            .last_indexed_at
            .lock()
            .ok()
            .and_then(|g| *g);
        serde_json::json!({
            "target_dir": self.target_dir.display().to_string(),
            "indexing": self.indexing.load(Ordering::SeqCst),
            "files": files,
            "nodes": nodes,
            "edges": edges,
            "last_indexed_at": last
        })
    }

    pub fn get_symbol(&self, seed: &str) -> Option<SampleNode> {
        let resolved = self.resolve_seeds(&[seed.to_string()]);
        let id = resolved.first()?;
        self.get_node(id)
    }

    pub fn get_neighbors(
        &self,
        seed: &str,
        edge_types: Option<&[EdgeType]>,
    ) -> serde_json::Value {
        let resolved = self.resolve_seeds(&[seed.to_string()]);
        if resolved.is_empty() {
            return serde_json::json!({
                "seed": seed,
                "resolved": [],
                "node": null,
                "neighbors": [],
                "edges": []
            });
        }
        let id = &resolved[0];
        let node = self.get_node(id);
        let mut neighbor_ids: HashSet<String> = HashSet::new();
        let mut edges_out: Vec<GraphEdge> = Vec::new();
        for edge in self.load_all_edges() {
            if let Some(types) = edge_types {
                if !types.is_empty() && !types.contains(&edge.edge_type) {
                    continue;
                }
            }
            if &edge.source == id {
                neighbor_ids.insert(edge.target.clone());
                edges_out.push(edge);
            } else if &edge.target == id {
                neighbor_ids.insert(edge.source.clone());
                edges_out.push(edge);
            }
        }
        let mut neighbors: Vec<SampleNode> = neighbor_ids
            .into_iter()
            .filter_map(|nid| self.get_node(&nid))
            .collect();
        neighbors.sort_by(|a, b| a.id.cmp(&b.id));
        serde_json::json!({
            "seed": seed,
            "resolved": resolved,
            "node": node,
            "neighbors": neighbors,
            "edges": edges_out
        })
    }

    pub fn find_path(&self, from: &str, to: &str, max_depth: usize) -> serde_json::Value {
        let from_ids = self.resolve_seeds(&[from.to_string()]);
        let to_ids = self.resolve_seeds(&[to.to_string()]);
        if from_ids.is_empty() || to_ids.is_empty() {
            return serde_json::json!({
                "found": false,
                "path": [],
                "edges": [],
                "from": from,
                "to": to
            });
        }
        let start = from_ids[0].clone();
        let goals: HashSet<String> = to_ids.into_iter().collect();

        let all_edges = self.load_all_edges();
        let mut adj: HashMap<String, Vec<(String, GraphEdge)>> = HashMap::new();
        for e in &all_edges {
            adj.entry(e.source.clone())
                .or_default()
                .push((e.target.clone(), e.clone()));
            adj.entry(e.target.clone())
                .or_default()
                .push((e.source.clone(), e.clone()));
        }

        let mut prev: HashMap<String, (String, GraphEdge)> = HashMap::new();
        let mut visited = HashSet::new();
        let mut q = VecDeque::new();
        visited.insert(start.clone());
        q.push_back((start.clone(), 0usize));

        let mut found_goal: Option<String> = None;
        while let Some((cur, depth)) = q.pop_front() {
            if goals.contains(&cur) && cur != start {
                found_goal = Some(cur);
                break;
            }
            if depth >= max_depth {
                continue;
            }
            if let Some(neis) = adj.get(&cur) {
                for (nxt, edge) in neis {
                    if visited.insert(nxt.clone()) {
                        prev.insert(nxt.clone(), (cur.clone(), edge.clone()));
                        q.push_back((nxt.clone(), depth + 1));
                    }
                }
            }
        }

        let Some(goal) = found_goal.or_else(|| {
            if goals.contains(&start) {
                Some(start.clone())
            } else {
                None
            }
        }) else {
            return serde_json::json!({
                "found": false,
                "path": [],
                "edges": [],
                "from_resolved": from_ids,
                "to_resolved": goals.into_iter().collect::<Vec<_>>(),
            });
        };

        let mut path_nodes = vec![goal.clone()];
        let mut path_edges = Vec::new();
        let mut cur = goal;
        while let Some((p, e)) = prev.get(&cur) {
            path_edges.push(e.clone());
            path_nodes.push(p.clone());
            cur = p.clone();
            if cur == start {
                break;
            }
        }
        path_nodes.reverse();
        path_edges.reverse();

        let nodes: Vec<SampleNode> = path_nodes
            .iter()
            .filter_map(|id| self.get_node(id))
            .collect();

        serde_json::json!({
            "found": true,
            "path": path_nodes,
            "nodes": nodes,
            "edges": path_edges,
            "from_resolved": from_ids,
        })
    }

    pub fn blast_radius(&self, seed: &str, depth: usize) -> serde_json::Value {
        let sub = self.query_subgraph(&[seed.to_string()], depth, None);
        let nodes = sub
            .get("nodes")
            .and_then(|n| n.as_array())
            .cloned()
            .unwrap_or_default();
        let edges = sub
            .get("edges")
            .and_then(|n| n.as_array())
            .cloned()
            .unwrap_or_default();

        let mut fan_out: HashMap<String, usize> = HashMap::new();
        let mut fan_in: HashMap<String, usize> = HashMap::new();
        for e in &edges {
            if let (Some(s), Some(t)) = (
                e.get("source").and_then(|v| v.as_str()),
                e.get("target").and_then(|v| v.as_str()),
            ) {
                *fan_out.entry(s.to_string()).or_default() += 1;
                *fan_in.entry(t.to_string()).or_default() += 1;
            }
        }
        let mut hubs_out: Vec<(String, usize)> = fan_out.into_iter().collect();
        let mut hubs_in: Vec<(String, usize)> = fan_in.into_iter().collect();
        hubs_out.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        hubs_in.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        hubs_out.truncate(10);
        hubs_in.truncate(10);

        serde_json::json!({
            "seed": seed,
            "depth": depth,
            "reachable_nodes": nodes.len(),
            "edges": edges.len(),
            "nodes": nodes,
            "fan_out_hubs": hubs_out.into_iter().map(|(id, n)| json_obj_id_count(&id, n)).collect::<Vec<_>>(),
            "fan_in_hubs": hubs_in.into_iter().map(|(id, n)| json_obj_id_count(&id, n)).collect::<Vec<_>>(),
            "summary": sub.get("summary").cloned().unwrap_or(serde_json::Value::Null)
        })
    }

    pub fn list_by_type(
        &self,
        node_type: &str,
        path_prefix: Option<&str>,
        lang: Option<&str>,
        limit: usize,
    ) -> Vec<SampleNode> {
        let want = NodeType::parse(node_type);
        let prefix = path_prefix.map(|s| s.replace('\\', "/").to_lowercase());
        let lang_l = lang.map(|s| s.to_lowercase());
        let Ok(entries) = self.db.scan("nodes") else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for (_k, val) in entries {
            let Ok(node) = serde_json::from_slice::<SampleNode>(&val) else {
                continue;
            };
            if let Some(ref t) = want {
                if &node.node_type != t {
                    continue;
                }
            } else if !node_type.is_empty()
                && !node.node_type.as_str().eq_ignore_ascii_case(node_type)
            {
                continue;
            }
            if let Some(ref p) = prefix {
                if !node.file_path.to_lowercase().starts_with(p)
                    && !node.file_path.to_lowercase().contains(p)
                {
                    continue;
                }
            }
            if let Some(ref l) = lang_l {
                if !node.language.eq_ignore_ascii_case(l) {
                    continue;
                }
            }
            out.push(node);
            if out.len() >= limit {
                break;
            }
        }
        out.sort_by(|a, b| a.id.cmp(&b.id));
        out
    }

    pub fn read_snippet(
        &self,
        seed: &str,
        before: usize,
        after: usize,
    ) -> serde_json::Value {
        let Some(node) = self.get_symbol(seed) else {
            return serde_json::json!({ "error": "symbol not found", "seed": seed });
        };
        let abs = self.target_dir.join(&node.file_path);
        let Ok(content) = fs::read_to_string(&abs) else {
            return serde_json::json!({
                "error": "cannot read file",
                "path": node.file_path,
                "node": node
            });
        };
        let lines: Vec<&str> = content.lines().collect();
        let center = node.line.unwrap_or(1).saturating_sub(1);
        let start = center.saturating_sub(before);
        let end = (center + after + 1).min(lines.len());
        let snippet: Vec<String> = lines[start..end]
            .iter()
            .enumerate()
            .map(|(i, l)| format!("{:>6}|{}", start + i + 1, l))
            .collect();
        serde_json::json!({
            "node": node,
            "start_line": start + 1,
            "end_line": end,
            "snippet": snippet.join("\n")
        })
    }

    pub fn detect_cycles(&self, max_cycles: usize) -> serde_json::Value {
        let edges = self.load_all_edges();
        let mut adj: HashMap<String, Vec<String>> = HashMap::new();
        let mut nodes: HashSet<String> = HashSet::new();
        for e in &edges {
            // Prefer structural edges for cycles
            if matches!(
                e.edge_type,
                EdgeType::Imports
                    | EdgeType::Includes
                    | EdgeType::Using
                    | EdgeType::Extends
                    | EdgeType::Implements
                    | EdgeType::Calls
            ) {
                adj.entry(e.source.clone())
                    .or_default()
                    .push(e.target.clone());
                nodes.insert(e.source.clone());
                nodes.insert(e.target.clone());
            }
        }

        let mut cycles: Vec<Vec<String>> = Vec::new();
        let mut visiting: HashSet<String> = HashSet::new();
        let mut visited: HashSet<String> = HashSet::new();
        let mut stack: Vec<String> = Vec::new();

        fn dfs(
            u: &str,
            adj: &HashMap<String, Vec<String>>,
            visiting: &mut HashSet<String>,
            visited: &mut HashSet<String>,
            stack: &mut Vec<String>,
            cycles: &mut Vec<Vec<String>>,
            max_cycles: usize,
        ) {
            if cycles.len() >= max_cycles {
                return;
            }
            visiting.insert(u.to_string());
            stack.push(u.to_string());
            if let Some(neis) = adj.get(u) {
                for v in neis {
                    if cycles.len() >= max_cycles {
                        break;
                    }
                    if visiting.contains(v) {
                        if let Some(pos) = stack.iter().position(|x| x == v) {
                            let mut cyc = stack[pos..].to_vec();
                            cyc.push(v.clone());
                            cycles.push(cyc);
                        }
                    } else if !visited.contains(v) {
                        dfs(v, adj, visiting, visited, stack, cycles, max_cycles);
                    }
                }
            }
            stack.pop();
            visiting.remove(u);
            visited.insert(u.to_string());
        }

        let mut sorted: Vec<String> = nodes.into_iter().collect();
        sorted.sort();
        for n in sorted {
            if !visited.contains(&n) {
                dfs(
                    &n,
                    &adj,
                    &mut visiting,
                    &mut visited,
                    &mut stack,
                    &mut cycles,
                    max_cycles,
                );
            }
        }

        serde_json::json!({
            "cycle_count": cycles.len(),
            "cycles": cycles,
            "note": "Directed cycles over imports/includes/extends/implements/calls (heuristic)"
        })
    }

    pub fn export_brdb(&self, path: &Path, format_version: u32) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
        }
        self.db
            .export_to_file_version(path, format_version)
            .map_err(|e| e.to_string())
    }

    pub fn import_brdb(&mut self, path: &Path) -> Result<(), String> {
        let db = Database::open_from_file(path).map_err(|e| e.to_string())?;
        self.db = Arc::new(db);
        Ok(())
    }

    pub fn default_brdb_path(&self) -> PathBuf {
        self.target_dir.join(".byterag").join("graph.brdb")
    }

    pub fn query_subgraph(
        &self,
        seed_ids: &[String],
        max_depth: usize,
        edge_types: Option<&[EdgeType]>,
    ) -> serde_json::Value {
        let resolved = self.resolve_seeds(seed_ids);
        if resolved.is_empty() {
            return serde_json::json!({
                "nodes": [],
                "edges": [],
                "query_seeds": seed_ids,
                "resolved_seeds": [],
                "max_depth": max_depth,
                "summary": { "node_count": 0, "edge_count": 0, "hubs": [] }
            });
        }

        let mut all_edges = self.load_all_edges();
        if let Some(types) = edge_types {
            if !types.is_empty() {
                all_edges.retain(|e| types.contains(&e.edge_type));
            }
        }

        let mut directed: Vec<(String, String)> = Vec::with_capacity(all_edges.len() * 2);
        let mut edge_lookup: HashMap<(String, String), GraphEdge> = HashMap::new();
        for edge in &all_edges {
            edge_lookup.insert((edge.source.clone(), edge.target.clone()), edge.clone());
            directed.push((edge.source.clone(), edge.target.clone()));
            directed.push((edge.target.clone(), edge.source.clone()));
        }
        let csr = CsrGraph::from_edges(&directed);

        let mut visited_edges: HashSet<String> = HashSet::new();
        let mut node_map: HashMap<String, SampleNode> = HashMap::new();
        let mut edge_list: Vec<GraphEdge> = Vec::new();

        for seed in &resolved {
            if let Some(node) = self.get_node(seed) {
                node_map.insert(seed.clone(), node);
            }
            let Some(sub) = csr.multi_hop_traversal(seed, max_depth) else {
                continue;
            };
            for &nid in &sub.nodes {
                let Some(name) = csr.get_node_name(nid).map(|s| s.to_string()) else {
                    continue;
                };
                if let Some(node) = self.get_node(&name) {
                    node_map.entry(name).or_insert(node);
                }
            }
            for &(u, v) in &sub.edges {
                let Some(src_name) = csr.get_node_name(u).map(|s| s.to_string()) else {
                    continue;
                };
                let Some(dst_name) = csr.get_node_name(v).map(|s| s.to_string()) else {
                    continue;
                };
                let canon = if let Some(e) = edge_lookup.get(&(src_name.clone(), dst_name.clone())) {
                    e.clone()
                } else if let Some(e) = edge_lookup.get(&(dst_name.clone(), src_name.clone())) {
                    e.clone()
                } else {
                    continue;
                };
                let edge_key = format!("{}->{}:{:?}", canon.source, canon.target, canon.edge_type);
                if visited_edges.insert(edge_key) {
                    edge_list.push(canon);
                }
            }
        }

        for id in &resolved {
            if let Some(node) = self.get_node(id) {
                node_map.entry(id.clone()).or_insert(node);
            }
        }

        let mut nodes: Vec<SampleNode> = node_map.into_values().collect();
        nodes.sort_by(|a, b| a.id.cmp(&b.id));
        edge_list.sort_by(|a, b| {
            a.source
                .cmp(&b.source)
                .then_with(|| a.target.cmp(&b.target))
        });

        let summary = build_summary(&nodes, &edge_list);

        serde_json::json!({
            "nodes": nodes,
            "edges": edge_list,
            "query_seeds": seed_ids,
            "resolved_seeds": resolved,
            "max_depth": max_depth,
            "summary": summary
        })
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

    fn load_all_nodes(&self) -> Vec<SampleNode> {
        let Ok(entries) = self.db.scan("nodes") else {
            return Vec::new();
        };
        entries
            .into_iter()
            .filter_map(|(_k, v)| serde_json::from_slice::<SampleNode>(&v).ok())
            .collect()
    }

    fn re_resolve_all_imports(&self) {
        let nodes = self.load_all_nodes();
        let Ok(entries) = self.db.scan("edges") else {
            return;
        };
        for (key, val) in entries {
            let Ok(mut edge) = serde_json::from_slice::<GraphEdge>(&val) else {
                continue;
            };
            let before = edge.target.clone();
            let mut one = [edge];
            resolve_import_edges(&nodes, &mut one);
            edge = one.into_iter().next().unwrap();
            if edge.target != before {
                let bytes = serde_json::to_vec(&edge).unwrap_or_default();
                let _ = self.db.insert("edges", &key, &bytes);
            }
        }
    }

    fn get_file_meta(&self, rel: &str) -> Option<FileMetaStored> {
        let val = self.db.get("file_meta", rel.as_bytes()).ok()??;
        serde_json::from_slice(&val).ok()
    }

    fn remove_file_artifacts(&self, meta: &FileMetaStored) {
        for id in &meta.node_ids {
            let _ = self.db.delete("nodes", id.as_bytes());
        }
        for key in &meta.edge_keys {
            let _ = self.db.delete("edges", key.as_bytes());
        }
    }

    fn count_table(&self, table: &str) -> usize {
        self.db.scan(table).map(|e| e.len()).unwrap_or(0)
    }

    fn path_excluded(path: &Path) -> bool {
        for comp in path.components() {
            if let Some(name) = comp.as_os_str().to_str() {
                if EXCLUDE_DIR_NAMES.iter().any(|x| *x == name) {
                    return true;
                }
            }
        }
        false
    }

    fn rel_path(&self, path: &Path) -> String {
        path.strip_prefix(&self.target_dir)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_else(|_| path.to_string_lossy().replace('\\', "/"))
    }
}

fn build_summary(nodes: &[SampleNode], edges: &[GraphEdge]) -> serde_json::Value {
    let mut degree: HashMap<String, usize> = HashMap::new();
    for e in edges {
        *degree.entry(e.source.clone()).or_default() += 1;
        *degree.entry(e.target.clone()).or_default() += 1;
    }
    let mut hubs: Vec<(String, usize)> = degree.into_iter().collect();
    hubs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    hubs.truncate(5);
    serde_json::json!({
        "node_count": nodes.len(),
        "edge_count": edges.len(),
        "hubs": hubs.into_iter().map(|(id, n)| json_obj_id_count(&id, n)).collect::<Vec<_>>()
    })
}

fn json_obj_id_count(id: &str, count: usize) -> serde_json::Value {
    serde_json::json!({ "id": id, "count": count })
}

fn file_mtime_secs(path: &Path) -> Option<u64> {
    let meta = fs::metadata(path).ok()?;
    let modified = meta.modified().ok()?;
    Some(
        modified
            .duration_since(UNIX_EPOCH)
            .ok()?
            .as_secs(),
    )
}

fn now_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
