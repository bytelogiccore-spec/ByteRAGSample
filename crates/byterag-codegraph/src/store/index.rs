use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::atomic::Ordering;
use std::time::{SystemTime, UNIX_EPOCH};

use super::GraphStore;
use crate::parser::{parse_file, resolve_import_edges};
use crate::types::GraphEdge;

const EXCLUDE_DIR_NAMES: &[&str] = &[
    "target",
    ".git",
    ".byterag",
    "node_modules",
    "dist",
    "build",
    "__pycache__",
];

impl GraphStore {
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
            let meta = super::FileMetaStored {
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
                if let Ok(meta) = serde_json::from_slice::<super::FileMetaStored>(&val) {
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

        // Auto export to single portable graph.brdb
        let default_brdb = self.default_brdb_path();
        if let Err(err) = self.export_brdb(&default_brdb, 1) {
            eprintln!("byterag: auto export brdb notice: {err}");
        }

        let now = now_unix_secs();
        if let Ok(mut g) = self.last_indexed_at.lock() {
            *g = Some(now);
        }
        if let Ok(mut g) = self.last_file_count.lock() {
            *g = file_count;
        }
        self.mark_dirty();
        self.clear_dirty_if_unchanged(now);
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
        let last_write = self.last_write_at.load(Ordering::SeqCst);
        serde_json::json!({
            "target_dir": self.target_dir.display().to_string(),
            "indexing": self.indexing.load(Ordering::SeqCst),
            "dirty": self.dirty.load(Ordering::SeqCst),
            "last_write_at": if last_write == 0 {
                serde_json::Value::Null
            } else {
                serde_json::json!(last_write)
            },
            "files": files,
            "nodes": nodes,
            "edges": edges,
            "last_indexed_at": last
        })
    }

    pub(crate) fn re_resolve_all_imports(&self) {
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

    pub(crate) fn get_file_meta(&self, rel: &str) -> Option<super::FileMetaStored> {
        let val = self.db.get("file_meta", rel.as_bytes()).ok()??;
        serde_json::from_slice(&val).ok()
    }

    pub(crate) fn remove_file_artifacts(&self, meta: &super::FileMetaStored) {
        for id in &meta.node_ids {
            let _ = self.db.delete("nodes", id.as_bytes());
        }
        for key in &meta.edge_keys {
            let _ = self.db.delete("edges", key.as_bytes());
        }
    }

    pub(crate) fn count_table(&self, table: &str) -> usize {
        self.db.scan(table).map(|e| e.len()).unwrap_or(0)
    }

    pub(crate) fn path_excluded(path: &Path) -> bool {
        for comp in path.components() {
            if let Some(name) = comp.as_os_str().to_str() {
                if EXCLUDE_DIR_NAMES.iter().any(|x| *x == name) {
                    return true;
                }
            }
        }
        false
    }

    pub(crate) fn rel_path(&self, path: &Path) -> String {
        path.strip_prefix(&self.target_dir)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_else(|_| path.to_string_lossy().replace('\\', "/"))
    }
}

pub fn file_mtime_secs(path: &Path) -> Option<u64> {
    let meta = fs::metadata(path).ok()?;
    let modified = meta.modified().ok()?;
    Some(
        modified
            .duration_since(UNIX_EPOCH)
            .ok()?
            .as_secs(),
    )
}

pub fn now_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
