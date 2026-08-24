use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const CONFIG_FILE_NAME: &str = "codeorbit_config.json";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkspaceItem {
    pub name: String,
    pub path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub active_path: String,
    pub workspaces: Vec<WorkspaceItem>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            active_path: String::new(),
            workspaces: Vec::new(),
        }
    }
}

pub fn get_config_dir() -> PathBuf {
    let home = dirs_or_current();
    let dir = home.join(".codeorbit");
    if !dir.exists() {
        let _ = fs::create_dir_all(&dir);
    }
    dir
}

fn dirs_or_current() -> PathBuf {
    if let Ok(path) = std::env::var("USERPROFILE") {
        return PathBuf::from(path);
    }
    if let Ok(path) = std::env::var("HOME") {
        return PathBuf::from(path);
    }
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

pub fn load_config() -> AppConfig {
    let file = get_config_dir().join(CONFIG_FILE_NAME);
    if let Ok(bytes) = fs::read(&file) {
        if let Ok(cfg) = serde_json::from_slice::<AppConfig>(&bytes) {
            return cfg;
        }
    }
    AppConfig::default()
}

pub fn save_config(cfg: &AppConfig) -> Result<(), String> {
    let file = get_config_dir().join(CONFIG_FILE_NAME);
    let bytes = serde_json::to_vec_pretty(cfg).map_err(|e| e.to_string())?;
    fs::write(file, bytes).map_err(|e| e.to_string())
}

pub fn ensure_workspace_in_config(cfg: &mut AppConfig, target_path: &Path) {
    let path_str = target_path.to_string_lossy().to_string();
    if path_str.is_empty() {
        return;
    }
    let name = target_path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| path_str.clone());

    if !cfg.workspaces.iter().any(|w| w.path == path_str) {
        cfg.workspaces.push(WorkspaceItem {
            name,
            path: path_str.clone(),
        });
    }
    cfg.active_path = path_str;
}

pub fn resolve_initial_workspace(cfg: &AppConfig) -> PathBuf {
    if let Ok(target) = std::env::var("BYTERAG_TARGET_DIR") {
        return PathBuf::from(target);
    }
    if !cfg.active_path.is_empty() && PathBuf::from(&cfg.active_path).exists() {
        return PathBuf::from(&cfg.active_path);
    }
    // If running inside target/debug or crates/codeorbit, find workspace root
    let cur = std::env::current_dir().unwrap_or_default();
    let mut p = cur.as_path();
    while let Some(parent) = p.parent() {
        if p.join("Cargo.lock").exists() && p.join("crates").exists() {
            return p.to_path_buf();
        }
        p = parent;
    }
    cur
}
