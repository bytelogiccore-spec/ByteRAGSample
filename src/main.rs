mod parser;
mod store;
mod types;

use serde_json::{json, Value};
use std::env;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::thread;
use store::GraphStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = env::var("BYTERAG_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| env::current_dir().unwrap_or_default());

    // Open DB only — do not block MCP initialize on a full tree walk.
    let store = Arc::new(RwLock::new(GraphStore::open(target_dir)));

    {
        let store_bg = Arc::clone(&store);
        thread::spawn(move || {
            let Ok(guard) = store_bg.read() else {
                return;
            };
            guard.index_directory();
        });
    }

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        if line.trim().is_empty() {
            continue;
        }

        if let Ok(req) = serde_json::from_str::<Value>(&line) {
            let id = req.get("id").cloned();
            let method = req.get("method").and_then(|m| m.as_str()).unwrap_or("");

            match method {
                "initialize" => {
                    let resp = json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "protocolVersion": "2024-11-05",
                            "capabilities": { "tools": {} },
                            "serverInfo": { "name": "byterag-code-graph-rust", "version": "1.0.0" }
                        }
                    });
                    writeln!(stdout, "{}", serde_json::to_string(&resp)?)?;
                    stdout.flush()?;
                }
                "tools/list" => {
                    let resp = json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "tools": [
                                {
                                    "name": "byterag_query_graph",
                                    "description": "Query symbol dependency subgraph up to max_depth. Seeds accept exact ids (e.g. struct:Database@path/to/file.rs) or short names (struct:Database).",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": {
                                            "node_id": { "type": "string" },
                                            "node_ids": { "type": "array", "items": { "type": "string" } },
                                            "max_depth": { "type": "number" }
                                        }
                                    }
                                },
                                {
                                    "name": "byterag_search_symbols",
                                    "description": "Search symbols by id, name, or file path (case-insensitive substring). Returns up to `limit` matches, best first.",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": {
                                            "query": { "type": "string" },
                                            "limit": { "type": "number" }
                                        },
                                        "required": ["query"]
                                    }
                                },
                                {
                                    "name": "byterag_reindex",
                                    "description": "Re-index now. Optional target_dir switches the indexing root first. Prefer a crate/subtree, not a huge monorepo root.",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": { "target_dir": { "type": "string" } }
                                    }
                                }
                            ]
                        }
                    });
                    writeln!(stdout, "{}", serde_json::to_string(&resp)?)?;
                    stdout.flush()?;
                }
                "tools/call" => {
                    let params = req.get("params");
                    let tool_name = params
                        .and_then(|p| p.get("name"))
                        .and_then(|n| n.as_str())
                        .unwrap_or("");
                    let args = params.and_then(|p| p.get("arguments"));

                    let content_text = match tool_name {
                        "byterag_query_graph" => {
                            let single = args.and_then(|a| a.get("node_id")).and_then(|s| s.as_str());
                            let mut seeds = Vec::new();
                            if let Some(s) = single {
                                seeds.push(s.to_string());
                            }
                            if let Some(arr) = args
                                .and_then(|a| a.get("node_ids"))
                                .and_then(|arr| arr.as_array())
                            {
                                for item in arr {
                                    if let Some(s) = item.as_str() {
                                        seeds.push(s.to_string());
                                    }
                                }
                            }
                            let max_depth = args
                                .and_then(|a| a.get("max_depth"))
                                .and_then(|n| n.as_u64())
                                .unwrap_or(2) as usize;
                            let store = store.read().map_err(|e| e.to_string())?;
                            let sub_graph = store.query_subgraph(&seeds, max_depth);
                            serde_json::to_string_pretty(&sub_graph)?
                        }
                        "byterag_search_symbols" => {
                            let query = args
                                .and_then(|a| a.get("query"))
                                .and_then(|s| s.as_str())
                                .unwrap_or("");
                            let limit = args
                                .and_then(|a| a.get("limit"))
                                .and_then(|n| n.as_u64())
                                .unwrap_or(50)
                                .clamp(1, 200) as usize;
                            let store = store.read().map_err(|e| e.to_string())?;
                            let matches = store.search_symbols_with_limit(query, limit);
                            serde_json::to_string_pretty(&matches)?
                        }
                        "byterag_reindex" => {
                            let mut store = store.write().map_err(|e| e.to_string())?;
                            if let Some(dir) = args
                                .and_then(|a| a.get("target_dir"))
                                .and_then(|s| s.as_str())
                                .filter(|s| !s.is_empty())
                            {
                                store.set_target_dir(PathBuf::from(dir));
                            }
                            store.index_directory();
                            format!(
                                "Successfully re-indexed: {}",
                                store.target_dir().display()
                            )
                        }
                        _ => "Unknown tool".to_string(),
                    };

                    let resp = json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "content": [{ "type": "text", "text": content_text }]
                        }
                    });
                    writeln!(stdout, "{}", serde_json::to_string(&resp)?)?;
                    stdout.flush()?;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
