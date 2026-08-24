use crate::config::{load_config, save_config, WorkspaceItem};
use crate::AppState;
use serde_json::Value;
use std::path::PathBuf;
use std::thread;
use tauri::State;

#[tauri::command]
pub fn get_index_status(state: State<AppState>) -> Result<Value, String> {
    let store = state.store.read().map_err(|e| format!("Lock error: {e:?}"))?;
    Ok(store.index_status())
}

#[tauri::command]
pub fn trigger_reindex(target_dir: Option<String>, state: State<AppState>) -> Result<String, String> {
    let mut store = state.store.write().map_err(|e| format!("Lock error: {e:?}"))?;
    if let Some(dir) = target_dir.filter(|s| !s.trim().is_empty()) {
        let pb = PathBuf::from(&dir);
        store.set_target_dir(pb.clone());
        let mut cfg = load_config();
        crate::config::ensure_workspace_in_config(&mut cfg, &pb);
        let _ = save_config(&cfg);
    }
    let store_bg = store.clone_arcs();
    thread::spawn(move || {
        store_bg.index_directory();
    });
    Ok(format!("Started indexing for {}", store.target_dir().display()))
}

#[tauri::command]
pub fn export_brdb_file(path: Option<String>, state: State<AppState>) -> Result<String, String> {
    let store = state.store.read().map_err(|e| format!("Lock error: {e:?}"))?;
    let target_path = path
        .map(PathBuf::from)
        .unwrap_or_else(|| store.default_brdb_path());
    store.export_brdb(&target_path, 1)?;
    Ok(format!("Successfully exported to .brdb archive: {}", target_path.display()))
}

#[tauri::command]
pub fn import_brdb_file(path: String, state: State<AppState>) -> Result<String, String> {
    let mut store = state.store.write().map_err(|e| format!("Lock error: {e:?}"))?;
    let target_path = PathBuf::from(&path);
    store.import_brdb(&target_path)?;
    Ok(format!("Successfully loaded .brdb file: {}", target_path.display()))
}

#[tauri::command]
pub fn search_symbols(query: String, limit: Option<usize>, state: State<AppState>) -> Result<Value, String> {
    let store = state.store.read().map_err(|e| format!("Lock error: {e:?}"))?;
    let matches = store.search_symbols_with_limit(&query, limit.unwrap_or(50));
    serde_json::to_value(matches).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_blast_radius(seed: String, depth: Option<usize>, state: State<AppState>) -> Result<Value, String> {
    let store = state.store.read().map_err(|e| format!("Lock error: {e:?}"))?;
    Ok(store.blast_radius(&seed, depth.unwrap_or(2)))
}

#[tauri::command]
pub fn get_symbol_neighbors(seed: String, state: State<AppState>) -> Result<Value, String> {
    let store = state.store.read().map_err(|e| format!("Lock error: {e:?}"))?;
    Ok(store.get_neighbors(&seed, None))
}

#[tauri::command]
pub fn get_workspaces() -> Result<Value, String> {
    let cfg = load_config();
    Ok(serde_json::json!({
        "active_path": cfg.active_path,
        "workspaces": cfg.workspaces
    }))
}

#[tauri::command]
pub fn add_workspace(path: String, name: Option<String>) -> Result<Value, String> {
    let mut cfg = load_config();
    let pb = PathBuf::from(&path);
    let item_name = name.unwrap_or_else(|| {
        pb.file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| path.clone())
    });

    if let Some(existing) = cfg.workspaces.iter_mut().find(|w| w.path == path) {
        existing.name = item_name;
    } else {
        cfg.workspaces.push(WorkspaceItem {
            name: item_name,
            path: path.clone(),
        });
    }
    cfg.active_path = path;
    save_config(&cfg)?;
    Ok(serde_json::json!({
        "active_path": cfg.active_path,
        "workspaces": cfg.workspaces
    }))
}

#[tauri::command]
pub fn remove_workspace(path: String) -> Result<Value, String> {
    let mut cfg = load_config();
    cfg.workspaces.retain(|w| w.path != path);
    if cfg.active_path == path {
        cfg.active_path = cfg
            .workspaces
            .first()
            .map(|w| w.path.clone())
            .unwrap_or_default();
    }
    save_config(&cfg)?;
    Ok(serde_json::json!({
        "active_path": cfg.active_path,
        "workspaces": cfg.workspaces
    }))
}
