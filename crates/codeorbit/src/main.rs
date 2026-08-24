// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;

use byterag_codegraph::store::GraphStore;
use commands::*;
use config::{ensure_workspace_in_config, load_config, save_config};
use std::env;
use std::sync::{Arc, RwLock};
use std::thread;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
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
            let show_item = MenuItem::with_id(app, "show", "Open Dashboard", true, None::<&str>)?;
            let reindex_item = MenuItem::with_id(app, "reindex", "⚡ Trigger Reindex", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit CodeOrbit", true, None::<&str>)?;

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
                        let store_arc = Arc::clone(&state.store);
                        thread::spawn(move || {
                            if let Ok(store) = store_arc.read() {
                                let store_bg = store.clone_arcs();
                                drop(store);
                                store_bg.index_directory();
                            }
                        });
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        button_state: tauri::tray::MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.unminimize();
                            let _ = window.set_focus();
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
            get_blast_radius,
            get_symbol_neighbors,
            get_workspaces,
            add_workspace,
            remove_workspace,
            get_project_plan_status,
            get_ai_audit_logs,
            generate_ai_prompt_context,
            list_project_docs,
            get_doc_content,
            save_doc_to_byterag,
            delete_doc_from_byterag,
            create_doc_in_byterag,
            get_test_verification_report,
            is_autostart_enabled,
            toggle_autostart,
            hide_to_tray,
        ])
        .run(tauri::generate_context!())
        .expect("error while running CodeOrbit desktop application");
}
