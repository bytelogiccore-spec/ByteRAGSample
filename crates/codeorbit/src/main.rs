// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use byterag_codegraph::GraphStore;
use serde_json::Value;
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::thread;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, State, WindowEvent};
use tauri_plugin_autostart::ManagerExt;

struct AppState {
    store: Arc<RwLock<GraphStore>>,
}

#[tauri::command]
fn get_index_status(state: State<AppState>) -> Result<Value, String> {
    let store = state.store.read().map_err(|e| e.to_string())?;
    Ok(store.index_status())
}

#[tauri::command]
fn trigger_reindex(target_dir: Option<String>, state: State<AppState>) -> Result<String, String> {
    let mut store = state.store.write().map_err(|e| e.to_string())?;
    if let Some(dir) = target_dir.filter(|s| !s.trim().is_empty()) {
        store.set_target_dir(PathBuf::from(dir));
    }
    let store_bg = store.clone_arcs();
    thread::spawn(move || {
        store_bg.index_directory();
    });
    Ok(format!("Started indexing for {}", store.target_dir().display()))
}

#[tauri::command]
fn search_symbols(query: String, limit: Option<usize>, state: State<AppState>) -> Result<Value, String> {
    let store = state.store.read().map_err(|e| e.to_string())?;
    let matches = store.search_symbols_with_limit(&query, limit.unwrap_or(50));
    serde_json::to_value(matches).map_err(|e| e.to_string())
}

#[tauri::command]
fn is_autostart_enabled(app: AppHandle) -> Result<bool, String> {
    let autostart = app.autolaunch();
    autostart.is_enabled().map_err(|e| e.to_string())
}

#[tauri::command]
fn toggle_autostart(enabled: bool, app: AppHandle) -> Result<(), String> {
    let autostart = app.autolaunch();
    if enabled {
        autostart.enable().map_err(|e| e.to_string())?;
    } else {
        autostart.disable().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn hide_to_tray(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
    Ok(())
}

fn main() {
    let target_dir = env::var("BYTERAG_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| env::current_dir().unwrap_or_default());

    let store = Arc::new(RwLock::new(GraphStore::open(target_dir)));

    // Initial background indexing
    {
        let store_bg = Arc::clone(&store);
        thread::spawn(move || {
            let worker = {
                let Ok(guard) = store_bg.read() else {
                    return;
                };
                guard.clone_arcs()
            };
            worker.index_directory();
        });
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            store: Arc::clone(&store),
        })
        .setup(|app| {
            let show_item = MenuItem::with_id(app, "show", "대시보드 열기", true, None::<&str>)?;
            let reindex_item = MenuItem::with_id(app, "reindex", "⚡ 재인덱싱 실행", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "종료", true, None::<&str>)?;

            let tray_menu = Menu::with_items(app, &[&show_item, &reindex_item, &quit_item])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.unminimize();
                            let _ = window.set_focus();
                        }
                    }
                    "reindex" => {
                        let state = app.state::<AppState>();
                        let worker_opt = {
                            state.store.read().ok().map(|guard| guard.clone_arcs())
                        };
                        if let Some(worker) = worker_opt {
                            thread::spawn(move || {
                                worker.index_directory();
                            });
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.unminimize();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            // Start minimized if launched with --minimized (e.g. from Autostart)
            if env::args().any(|arg| arg == "--minimized") {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                // Prevent window from closing the entire app, hide to tray instead!
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_index_status,
            trigger_reindex,
            search_symbols,
            is_autostart_enabled,
            toggle_autostart,
            hide_to_tray,
        ])
        .run(tauri::generate_context!())
        .expect("error while running CodeOrbit desktop application");
}
