pub mod export;
pub mod index;
pub mod search;
pub mod traversal;

use byterag_core::Database;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use crate::types::{GraphEdge, GraphNode as SampleNode};

pub use index::{file_mtime_secs, now_unix_secs};

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct FileMetaStored {
    pub mtime_secs: u64,
    pub node_ids: Vec<String>,
    pub edge_keys: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ByteRagDocStored {
    pub id: String,
    pub title: String,
    pub doc_type: String, // "plan" | "spec" | "manual" | "general"
    pub content: String,
    pub updated_at: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ByteRagTestResultStored {
    pub run_at: u64,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub duration_secs: f64,
    pub test_cases: Vec<ByteRagTestCaseItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ByteRagTestCaseItem {
    pub test_id: String,
    pub name: String,
    pub title: String,
    pub purpose: String,
    pub expected: String,
    pub passed: bool,
}

pub struct GraphStore {
    pub(crate) target_dir: PathBuf,
    pub(crate) db: Arc<Database>,
    pub(crate) indexing: Arc<AtomicBool>,
    /// True after a write (index/import) until a successful `.brdb` export clears it.
    pub(crate) dirty: Arc<AtomicBool>,
    /// Unix secs of the last write that set `dirty`.
    pub(crate) last_write_at: Arc<AtomicU64>,
    pub(crate) last_indexed_at: Arc<Mutex<Option<u64>>>,
    pub(crate) last_file_count: Arc<Mutex<usize>>,
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
            dirty: Arc::new(AtomicBool::new(false)),
            last_write_at: Arc::new(AtomicU64::new(0)),
            last_indexed_at: Arc::new(Mutex::new(None)),
            last_file_count: Arc::new(Mutex::new(0)),
        }
    }

    pub fn target_dir(&self) -> &PathBuf {
        &self.target_dir
    }

    pub fn is_indexing(&self) -> bool {
        self.indexing.load(Ordering::SeqCst)
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty.load(Ordering::SeqCst)
    }

    pub fn last_write_at(&self) -> u64 {
        self.last_write_at.load(Ordering::SeqCst)
    }

    pub(crate) fn mark_dirty(&self) {
        let now = now_unix_secs();
        self.last_write_at.store(now, Ordering::SeqCst);
        self.dirty.store(true, Ordering::SeqCst);
    }

    /// Clears dirty only if no newer write landed since `write_at`.
    pub fn clear_dirty_if_unchanged(&self, write_at: u64) {
        if self.last_write_at.load(Ordering::SeqCst) == write_at {
            self.dirty.store(false, Ordering::SeqCst);
        }
    }

    /// Cheap clone of Arc handles so background indexing need not hold `RwLock`.
    pub fn clone_arcs(&self) -> Self {
        Self {
            target_dir: self.target_dir.clone(),
            db: Arc::clone(&self.db),
            indexing: Arc::clone(&self.indexing),
            dirty: Arc::clone(&self.dirty),
            last_write_at: Arc::clone(&self.last_write_at),
            last_indexed_at: Arc::clone(&self.last_indexed_at),
            last_file_count: Arc::clone(&self.last_file_count),
        }
    }

    pub fn set_target_dir(&mut self, target_dir: PathBuf) {
        *self = Self::open(target_dir);
    }

    pub(crate) fn get_node(&self, id: &str) -> Option<SampleNode> {
        let val = self.db.get("nodes", id.as_bytes()).ok()??;
        serde_json::from_slice(&val).ok()
    }

    pub(crate) fn load_all_edges(&self) -> Vec<GraphEdge> {
        let Ok(entries) = self.db.scan("edges") else {
            return Vec::new();
        };
        entries
            .into_iter()
            .filter_map(|(_key, val)| serde_json::from_slice::<GraphEdge>(&val).ok())
            .collect()
    }

    pub(crate) fn load_all_nodes(&self) -> Vec<SampleNode> {
        let Ok(entries) = self.db.scan("nodes") else {
            return Vec::new();
        };
        entries
            .into_iter()
            .filter_map(|(_k, v)| serde_json::from_slice::<SampleNode>(&v).ok())
            .collect()
    }

    // --- ByteRAG Docs Persistence API ---

    pub fn list_docs(&self) -> Vec<ByteRagDocStored> {
        let Ok(entries) = self.db.scan("docs") else {
            return Vec::new();
        };
        entries
            .into_iter()
            .filter_map(|(_k, v)| serde_json::from_slice::<ByteRagDocStored>(&v).ok())
            .collect()
    }

    pub fn get_doc(&self, id: &str) -> Option<ByteRagDocStored> {
        let val = self.db.get("docs", id.as_bytes()).ok()??;
        serde_json::from_slice(&val).ok()
    }

    pub fn save_doc(&self, doc: &ByteRagDocStored) -> Result<(), String> {
        let bytes = serde_json::to_vec(doc).map_err(|e| e.to_string())?;
        self.db.insert("docs", doc.id.as_bytes(), &bytes).map_err(|e| e.to_string())?;
        let _ = self.db.flush();
        self.mark_dirty();
        Ok(())
    }

    pub fn delete_doc(&self, id: &str) -> Result<(), String> {
        self.db.delete("docs", id.as_bytes()).map_err(|e| e.to_string())?;
        let _ = self.db.flush();
        self.mark_dirty();
        Ok(())
    }

    // --- ByteRAG Verified Test History API (Successful cases only) ---

    pub fn save_test_result(&self, result: &ByteRagTestResultStored) -> Result<(), String> {
        // Only save when tests have passed
        if result.passed_tests == 0 {
            return Ok(());
        }
        let key = format!("run:{}", result.run_at);
        let bytes = serde_json::to_vec(result).map_err(|e| e.to_string())?;
        self.db.insert("test_history", key.as_bytes(), &bytes).map_err(|e| e.to_string())?;
        let _ = self.db.flush();
        self.mark_dirty();
        Ok(())
    }

    pub fn get_latest_test_result(&self) -> Option<ByteRagTestResultStored> {
        let Ok(entries) = self.db.scan("test_history") else {
            return None;
        };
        entries
            .into_iter()
            .filter_map(|(_k, v)| serde_json::from_slice::<ByteRagTestResultStored>(&v).ok())
            .max_by_key(|r| r.run_at)
    }
}
