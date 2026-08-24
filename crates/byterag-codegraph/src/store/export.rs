use super::GraphStore;
use byterag_core::Database;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
use std::sync::Arc;

impl GraphStore {
    pub fn export_brdb(&self, path: &Path, format_version: u32) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
        }
        let write_at = self.last_write_at.load(Ordering::SeqCst);
        self.db
            .export_to_file_version(path, format_version)
            .map_err(|e| e.to_string())?;
        self.clear_dirty_if_unchanged(write_at);

        // Clean up temporary live WAL and WOS cache files, leaving only the clean .brdb single file
        let db_dir = self.target_dir.join(".byterag");
        if db_dir.exists() {
            let wos_dir = db_dir.join("wos");
            if wos_dir.exists() {
                let _ = fs::remove_dir_all(&wos_dir);
            }
            let wal_log = db_dir.join("wal.log");
            if wal_log.exists() {
                let _ = fs::remove_file(&wal_log);
            }
        }

        Ok(())
    }

    pub fn import_brdb(&mut self, path: &Path) -> Result<(), String> {
        let db = Database::open_from_file(path).map_err(|e| e.to_string())?;
        self.db = Arc::new(db);
        self.mark_dirty();
        Ok(())
    }

    pub fn default_brdb_path(&self) -> PathBuf {
        self.target_dir.join(".byterag").join("graph.brdb")
    }
}
