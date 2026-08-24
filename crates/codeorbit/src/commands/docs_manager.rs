use crate::AppState;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::State;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocItem {
    pub name: String,
    pub path: String,
    pub doc_type: String, // "plan" | "spec" | "manual" | "general"
    pub size_bytes: u64,
    pub modified_str: String,
}

#[tauri::command]
pub fn list_project_docs(state: State<AppState>) -> Result<Vec<DocItem>, String> {
    let store = state.store.read().map_err(|e| format!("Lock error: {e:?}"))?;
    let target_dir = store.target_dir();
    let mut docs = Vec::new();

    // Scan root markdown files
    scan_markdown_in_dir(target_dir, &mut docs);

    // Scan docs/ folder
    let docs_dir = target_dir.join("docs");
    if docs_dir.exists() && docs_dir.is_dir() {
        scan_markdown_in_dir(&docs_dir, &mut docs);
    }

    Ok(docs)
}

fn scan_markdown_in_dir(dir: &std::path::Path, out: &mut Vec<DocItem>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                    let doc_type = if name.contains("plan") {
                        "plan".into()
                    } else if name.contains("manual") || name.contains("ui") {
                        "manual".into()
                    } else if name.contains("spec") || name.contains("design") || name.contains("arch") {
                        "spec".into()
                    } else {
                        "general".into()
                    };

                    let meta = fs::metadata(&path).ok();
                    let size_bytes = meta.as_ref().map(|m| m.len()).unwrap_or(0);
                    let modified_str = meta
                        .and_then(|m| m.modified().ok())
                        .map(|t| {
                            let dt = chrono_lite(t);
                            dt
                        })
                        .unwrap_or_else(|| "최근".into());

                    out.push(DocItem {
                        name,
                        path: path.to_string_lossy().to_string(),
                        doc_type,
                        size_bytes,
                        modified_str,
                    });
                }
            }
        }
    }
}

fn chrono_lite(time: std::time::SystemTime) -> String {
    let dur = std::time::SystemTime::now().duration_since(time).unwrap_or_default();
    let secs = dur.as_secs();
    if secs < 60 {
        "방금 전".into()
    } else if secs < 3600 {
        format!("{}분 전", secs / 60)
    } else if secs < 86400 {
        format!("{}시간 전", secs / 3600)
    } else {
        format!("{}일 전", secs / 86400)
    }
}

#[tauri::command]
pub fn read_doc_content(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_doc_content(path: String, content: String) -> Result<(), String> {
    fs::write(&path, content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_doc_file(path: String) -> Result<(), String> {
    fs::remove_file(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_doc_file(name: String, doc_type: String, state: State<AppState>) -> Result<String, String> {
    let store = state.store.read().map_err(|e| format!("Lock error: {e:?}"))?;
    let target_dir = store.target_dir();

    let docs_dir = target_dir.join("docs");
    if !docs_dir.exists() {
        let _ = fs::create_dir_all(&docs_dir);
    }

    let file_name = if name.ends_with(".md") { name } else { format!("{}.md", name) };
    let file_path = docs_dir.join(&file_name);

    let default_content = match doc_type.as_str() {
        "plan" => "# 📋 작업 계획서 (Plan)\n\n## 목표\n- \n\n## 작업 항목\n- [ ] Phase 1: \n- [ ] Phase 2: \n",
        "spec" => "# 📐 개발 사양서 (Specification)\n\n## 기능 개요\n- \n\n## 아키텍처 및 요구사항\n- \n",
        "manual" => "# 📖 기능 사용 매뉴얼 (Manual)\n\n## 시작하기\n- \n\n## 사용 방법\n- \n",
        _ => "# 📝 프로젝트 문서\n\n",
    };

    fs::write(&file_path, default_content).map_err(|e| e.to_string())?;
    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn open_in_ide(file_path: String, line: Option<usize>) -> Result<(), String> {
    let path_clean = file_path.replace('\\', "/");
    let line_suffix = line.map(|l| format!(":{}", l)).unwrap_or_default();
    
    // Try Cursor URL protocol first, fallback to code or default explorer
    let cursor_url = format!("cursor://file/{}{}", path_clean, line_suffix);
    
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", &cursor_url])
            .spawn();
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = std::process::Command::new("xdg-open")
            .arg(&cursor_url)
            .spawn();
    }

    Ok(())
}
