// Prevents black console window from appearing on Windows desktop launch
#![windows_subsystem = "windows"]

pub mod commands;
pub mod config;

use byterag_codegraph::GraphStore;
use commands::*;
use config::{ensure_workspace_in_config, load_config, save_config};
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::thread;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Manager, WindowEvent};

pub struct AppState {
    pub store: Arc<RwLock<GraphStore>>,
}

fn main() {
    let mut cfg = load_config();
    let initial_dir = crate::config::resolve_initial_workspace(&cfg);

    ensure_workspace_in_config(&mut cfg, &initial_dir);
    let _ = save_config(&cfg);

    let store = Arc::new(RwLock::new(GraphStore::open(initial_dir)));

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
            export_brdb_file,
            import_brdb_file,
            search_symbols,
            get_workspaces,
            add_workspace,
            remove_workspace,
            get_project_plan_status,
            get_ai_audit_logs,
            generate_ai_prompt_context,
            is_autostart_enabled,
            toggle_autostart,
            hide_to_tray,
        ])
        .run(tauri::generate_context!())
        .expect("error while running CodeOrbit desktop application");
}
