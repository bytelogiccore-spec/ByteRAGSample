use crate::AppState;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use tauri::State;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlanMilestone {
    pub title: String,
    pub completed: bool,
    pub raw: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectPlanStatus {
    pub has_plan: bool,
    pub plan_file: String,
    pub title: String,
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub progress_percent: usize,
    pub milestones: Vec<PlanMilestone>,
    pub walkthrough_summary: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AiAuditEntry {
    pub timestamp: String,
    pub tool_name: String,
    pub query_target: String,
    pub duration_ms: u64,
    pub nodes_affected: usize,
    pub status: String,
}

fn parse_milestones_from_text(content: &str, plan_title: &mut String) -> Vec<PlanMilestone> {
    let mut milestones = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") && *plan_title == "No active plan found" {
            *plan_title = trimmed.trim_start_matches("# ").trim().to_string();
        } else if trimmed.starts_with("- [ ] ") || trimmed.starts_with("- [x] ") || trimmed.starts_with("- [X] ") {
            let is_done = trimmed.starts_with("- [x] ") || trimmed.starts_with("- [X] ");
            let task_text = trimmed[6..].trim().to_string();
            milestones.push(PlanMilestone {
                title: task_text,
                completed: is_done,
                raw: trimmed.to_string(),
            });
        }
    }
    milestones
}

#[tauri::command]
pub fn get_project_plan_status(state: State<AppState>) -> Result<ProjectPlanStatus, String> {
    let store = state.store.read().map_err(|e| format!("Lock error: {e:?}"))?;
    let target_dir = store.target_dir();

    let mut milestones = Vec::new();
    let mut plan_title = String::from("No active plan found");
    let mut has_plan = false;
    let mut active_plan_name = String::new();

    // 1. Check current workspace's ByteRAG DB stored plan (Zero Disk Mess per workspace)
    if let Some(plan_doc) = store.get_doc("plan:implementation_plan") {
        let parsed = parse_milestones_from_text(&plan_doc.content, &mut plan_title);
        if !parsed.is_empty() {
            milestones = parsed;
            has_plan = true;
            active_plan_name = "ByteRAG DB (plan:implementation_plan)".into();
        }
    }

    // 2. Check current workspace's physical implementation_plan.md
    if !has_plan {
        let plan_path = target_dir.join("implementation_plan.md");
        if plan_path.exists() {
            if let Ok(content) = fs::read_to_string(&plan_path) {
                let parsed = parse_milestones_from_text(&content, &mut plan_title);
                if !parsed.is_empty() {
                    milestones = parsed;
                    has_plan = true;
                    active_plan_name = "implementation_plan.md".into();
                }
            }
        }
    }

    // 3. Check current workspace's PLAN.md or docs/plan.md if exists
    if !has_plan {
        let alt_plans = [target_dir.join("PLAN.md"), target_dir.join("docs").join("plan.md")];
        for alt in &alt_plans {
            if alt.exists() {
                if let Ok(content) = fs::read_to_string(alt) {
                    let parsed = parse_milestones_from_text(&content, &mut plan_title);
                    if !parsed.is_empty() {
                        milestones = parsed;
                        has_plan = true;
                        active_plan_name = alt.file_name().unwrap_or_default().to_string_lossy().to_string();
                        break;
                    }
                }
            }
        }
    }

    let total = milestones.len();
    let completed = milestones.iter().filter(|m| m.completed).count();
    let percent = if total > 0 { (completed * 100) / total } else { 0 };

    let wt_path = target_dir.join("walkthrough.md");
    let wt_summary = if wt_path.exists() {
        fs::read_to_string(&wt_path).ok().map(|s| {
            s.lines().take(10).collect::<Vec<_>>().join("\n")
        })
    } else {
        None
    };

    Ok(ProjectPlanStatus {
        has_plan,
        plan_file: active_plan_name,
        title: plan_title,
        total_tasks: total,
        completed_tasks: completed,
        progress_percent: percent,
        milestones,
        walkthrough_summary: wt_summary,
    })
}

#[tauri::command]
pub fn get_ai_audit_logs(_state: State<AppState>) -> Result<Vec<AiAuditEntry>, String> {
    Ok(Vec::new())
}

#[tauri::command]
pub fn generate_ai_prompt_context(state: State<AppState>) -> Result<String, String> {
    let store = state.store.read().map_err(|e| format!("Lock error: {e:?}"))?;
    let status = store.index_status();
    let target = store.target_dir().display().to_string();
    let files = status.get("files").unwrap_or(&Value::Null);
    let nodes = status.get("nodes").unwrap_or(&Value::Null);
    let edges = status.get("edges").unwrap_or(&Value::Null);

    let prompt = format!(
r#"<project_architecture_context>
[Target Project Root]: {target}
[Code Graph Status]: {files} files, {nodes} knowledge symbols, {edges} relation edges
[Active Database]: ByteRAG 5T Embedded Engine (.byterag/graph.brdb)
[Available MCP Tools]: byterag_query_graph, byterag_blast_radius, byterag_find_path, byterag_search_symbols, byterag_read_snippet

You are equipped with CodeOrbit AST GraphRAG. Whenever you modify or analyze code in this project, proactively check blast radius and shortest dependency paths before making changes.
</project_architecture_context>"#
    );
    Ok(prompt)
}
