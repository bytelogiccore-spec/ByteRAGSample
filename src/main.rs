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
use types::EdgeType;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = env::var("BYTERAG_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| env::current_dir().unwrap_or_default());

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
                            "serverInfo": { "name": "byterag-codegraph", "version": "0.3.1" }
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
                                    "description": "Query symbol dependency subgraph via CsrGraph BFS. Optional edge_types filter. Response includes summary (counts, hubs).",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": {
                                            "node_id": { "type": "string" },
                                            "node_ids": { "type": "array", "items": { "type": "string" } },
                                            "max_depth": { "type": "number" },
                                            "edge_types": { "type": "array", "items": { "type": "string" }, "description": "defines|imports|calls|extends|implements|..." }
                                        }
                                    }
                                },
                                {
                                    "name": "byterag_search_symbols",
                                    "description": "Search symbols by id, name, or file path (case-insensitive substring).",
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
                                    "description": "Re-index (incremental by mtime) then flush. Optional target_dir switches root.",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": { "target_dir": { "type": "string" } }
                                    }
                                },
                                {
                                    "name": "byterag_export_brdb",
                                    "description": "Export live DB to .brdb pack. Default <target>/.byterag/graph.brdb.",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": {
                                            "path": { "type": "string" },
                                            "format_version": { "type": "number" }
                                        }
                                    }
                                },
                                {
                                    "name": "byterag_import_brdb",
                                    "description": "Load .brdb into memory and replace live store.",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": { "path": { "type": "string" } },
                                        "required": ["path"]
                                    }
                                },
                                {
                                    "name": "byterag_index_status",
                                    "description": "Indexing flag, file/node/edge counts, last_indexed_at, target_dir.",
                                    "inputSchema": { "type": "object", "properties": {} }
                                },
                                {
                                    "name": "byterag_get_symbol",
                                    "description": "Resolve a seed to one symbol node (with signature when available).",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": { "node_id": { "type": "string" }, "query": { "type": "string" } }
                                    }
                                },
                                {
                                    "name": "byterag_get_neighbors",
                                    "description": "1-hop neighbors and edges for a seed. Optional edge_types filter.",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": {
                                            "node_id": { "type": "string" },
                                            "edge_types": { "type": "array", "items": { "type": "string" } }
                                        },
                                        "required": ["node_id"]
                                    }
                                },
                                {
                                    "name": "byterag_find_path",
                                    "description": "Shortest undirected path between two seeds.",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": {
                                            "from": { "type": "string" },
                                            "to": { "type": "string" },
                                            "max_depth": { "type": "number" }
                                        },
                                        "required": ["from", "to"]
                                    }
                                },
                                {
                                    "name": "byterag_blast_radius",
                                    "description": "N-hop reachable set plus fan-in/fan-out hubs.",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": {
                                            "node_id": { "type": "string" },
                                            "max_depth": { "type": "number" }
                                        },
                                        "required": ["node_id"]
                                    }
                                },
                                {
                                    "name": "byterag_list_by_type",
                                    "description": "List symbols by node type (struct|fn|file|...), optional path_prefix and language.",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": {
                                            "node_type": { "type": "string" },
                                            "path_prefix": { "type": "string" },
                                            "language": { "type": "string" },
                                            "limit": { "type": "number" }
                                        },
                                        "required": ["node_type"]
                                    }
                                },
                                {
                                    "name": "byterag_read_snippet",
                                    "description": "Read source lines around a symbol (before/after, default 5).",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": {
                                            "node_id": { "type": "string" },
                                            "before": { "type": "number" },
                                            "after": { "type": "number" }
                                        },
                                        "required": ["node_id"]
                                    }
                                },
                                {
                                    "name": "byterag_detect_cycles",
                                    "description": "Sample directed cycles over imports/extends/implements/calls.",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": { "max_cycles": { "type": "number" } }
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
                            let mut seeds = Vec::new();
                            if let Some(s) = args.and_then(|a| a.get("node_id")).and_then(|s| s.as_str())
                            {
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
                            let edge_types = parse_edge_types(args);
                            let store = store.read().map_err(|e| e.to_string())?;
                            let sub = store.query_subgraph(
                                &seeds,
                                max_depth,
                                if edge_types.is_empty() {
                                    None
                                } else {
                                    Some(&edge_types)
                                },
                            );
                            serde_json::to_string_pretty(&sub)?
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
                        "byterag_export_brdb" => {
                            let store = store.read().map_err(|e| e.to_string())?;
                            let path = args
                                .and_then(|a| a.get("path"))
                                .and_then(|s| s.as_str())
                                .filter(|s| !s.is_empty())
                                .map(PathBuf::from)
                                .unwrap_or_else(|| store.default_brdb_path());
                            let format_version = args
                                .and_then(|a| a.get("format_version"))
                                .and_then(|n| n.as_u64())
                                .unwrap_or(1)
                                .clamp(1, 2) as u32;
                            match store.export_brdb(&path, format_version) {
                                Ok(()) => format!(
                                    "Exported .brdb v{format_version} to {}",
                                    path.display()
                                ),
                                Err(e) => format!("export failed: {e}"),
                            }
                        }
                        "byterag_import_brdb" => {
                            let path = args
                                .and_then(|a| a.get("path"))
                                .and_then(|s| s.as_str())
                                .filter(|s| !s.is_empty())
                                .map(PathBuf::from);
                            match path {
                                Some(path) => {
                                    let mut store = store.write().map_err(|e| e.to_string())?;
                                    match store.import_brdb(&path) {
                                        Ok(()) => format!(
                                            "Imported .brdb into memory from {}",
                                            path.display()
                                        ),
                                        Err(e) => format!("import failed: {e}"),
                                    }
                                }
                                None => "import failed: path is required".to_string(),
                            }
                        }
                        "byterag_index_status" => {
                            let store = store.read().map_err(|e| e.to_string())?;
                            serde_json::to_string_pretty(&store.index_status())?
                        }
                        "byterag_get_symbol" => {
                            let seed = args
                                .and_then(|a| {
                                    a.get("node_id")
                                        .or_else(|| a.get("query"))
                                        .and_then(|s| s.as_str())
                                })
                                .unwrap_or("");
                            let store = store.read().map_err(|e| e.to_string())?;
                            match store.get_symbol(seed) {
                                Some(n) => serde_json::to_string_pretty(&n)?,
                                None => format!("symbol not found: {seed}"),
                            }
                        }
                        "byterag_get_neighbors" => {
                            let seed = args
                                .and_then(|a| a.get("node_id"))
                                .and_then(|s| s.as_str())
                                .unwrap_or("");
                            let edge_types = parse_edge_types(args);
                            let store = store.read().map_err(|e| e.to_string())?;
                            let v = store.get_neighbors(
                                seed,
                                if edge_types.is_empty() {
                                    None
                                } else {
                                    Some(&edge_types)
                                },
                            );
                            serde_json::to_string_pretty(&v)?
                        }
                        "byterag_find_path" => {
                            let from = args
                                .and_then(|a| a.get("from"))
                                .and_then(|s| s.as_str())
                                .unwrap_or("");
                            let to = args
                                .and_then(|a| a.get("to"))
                                .and_then(|s| s.as_str())
                                .unwrap_or("");
                            let max_depth = args
                                .and_then(|a| a.get("max_depth"))
                                .and_then(|n| n.as_u64())
                                .unwrap_or(8) as usize;
                            let store = store.read().map_err(|e| e.to_string())?;
                            serde_json::to_string_pretty(&store.find_path(from, to, max_depth))?
                        }
                        "byterag_blast_radius" => {
                            let seed = args
                                .and_then(|a| a.get("node_id"))
                                .and_then(|s| s.as_str())
                                .unwrap_or("");
                            let depth = args
                                .and_then(|a| a.get("max_depth"))
                                .and_then(|n| n.as_u64())
                                .unwrap_or(2) as usize;
                            let store = store.read().map_err(|e| e.to_string())?;
                            serde_json::to_string_pretty(&store.blast_radius(seed, depth))?
                        }
                        "byterag_list_by_type" => {
                            let node_type = args
                                .and_then(|a| a.get("node_type"))
                                .and_then(|s| s.as_str())
                                .unwrap_or("");
                            let path_prefix = args
                                .and_then(|a| a.get("path_prefix"))
                                .and_then(|s| s.as_str());
                            let language = args
                                .and_then(|a| a.get("language"))
                                .and_then(|s| s.as_str());
                            let limit = args
                                .and_then(|a| a.get("limit"))
                                .and_then(|n| n.as_u64())
                                .unwrap_or(100)
                                .clamp(1, 500) as usize;
                            let store = store.read().map_err(|e| e.to_string())?;
                            let list =
                                store.list_by_type(node_type, path_prefix, language, limit);
                            serde_json::to_string_pretty(&list)?
                        }
                        "byterag_read_snippet" => {
                            let seed = args
                                .and_then(|a| a.get("node_id"))
                                .and_then(|s| s.as_str())
                                .unwrap_or("");
                            let before = args
                                .and_then(|a| a.get("before"))
                                .and_then(|n| n.as_u64())
                                .unwrap_or(5) as usize;
                            let after = args
                                .and_then(|a| a.get("after"))
                                .and_then(|n| n.as_u64())
                                .unwrap_or(5) as usize;
                            let store = store.read().map_err(|e| e.to_string())?;
                            serde_json::to_string_pretty(&store.read_snippet(seed, before, after))?
                        }
                        "byterag_detect_cycles" => {
                            let max_cycles = args
                                .and_then(|a| a.get("max_cycles"))
                                .and_then(|n| n.as_u64())
                                .unwrap_or(20)
                                .clamp(1, 100) as usize;
                            let store = store.read().map_err(|e| e.to_string())?;
                            serde_json::to_string_pretty(&store.detect_cycles(max_cycles))?
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

fn parse_edge_types(args: Option<&Value>) -> Vec<EdgeType> {
    let Some(arr) = args
        .and_then(|a| a.get("edge_types"))
        .and_then(|v| v.as_array())
    else {
        return Vec::new();
    };
    arr.iter()
        .filter_map(|v| v.as_str())
        .filter_map(EdgeType::parse)
        .collect()
}
