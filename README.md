# ByteRAGSample — `byterag-codegraph` MCP

Rust MCP server that builds a **code dependency graph** on top of the [ByteRAG](https://github.com/bytelogiccore-spec/ByteRAG) database engine (`byterag-core` **v0.3.0**).

- **ByteRAG** = embedded DB / graph storage engine  
- **`byterag-codegraph`** = this MCP (code-graph analysis tools for agents)

## Supported languages

- **C++**: `*.cpp`, `*.h`, `*.hpp`, `*.cxx`, `*.cc`
- **C#**: `*.cs`
- **Rust**: `*.rs`
- **TypeScript / JS**: `*.ts`, `*.tsx`, `*.js`, `*.jsx`
- **Python**: `*.py`

## MCP tools

**Core**
1. `byterag_query_graph` — CsrGraph BFS subgraph (`max_depth`, optional `edge_types`, response `summary`)
2. `byterag_search_symbols` — id / name / path search (`limit` default 50)
3. `byterag_reindex` — rebuild (+ `flush`); optional `target_dir`
4. `byterag_export_brdb` / `byterag_import_brdb` — portable `.brdb` packs

**Analysis**
5. `byterag_index_status` — indexing flag, counts, last indexed time
6. `byterag_get_symbol` / `byterag_get_neighbors` — node + 1-hop (edge_type filter)
7. `byterag_find_path` — shortest path between two seeds
8. `byterag_blast_radius` — N-hop reach set + fan-in/out hubs
9. `byterag_list_by_type` — filter by node type / path / language
10. `byterag_read_snippet` — source ±N lines around a symbol
11. `byterag_detect_cycles` — directed cycle samples (structural smell)

## Symbol id format

```text
struct:Database@core/byterag-core/src/engine/database.rs
fn:open_in_memory@core/byterag-core/src/engine/constructors.rs
file:core/byterag-core/src/lib.rs
```

## Startup

- Opens `.byterag/` then answers MCP `initialize` / `tools/list` immediately.
- First index runs on a **background thread** (`byterag: indexing start/done` on stderr).
- Indexing ends with `flush()` (WAL trim). Prefer `byterag_reindex` for a durable snapshot.
- Incremental reindex skips unchanged files (mtime in `file_meta`); excludes `target`, `.git`, `.byterag`, `node_modules`, `dist`, `build`, `__pycache__`.
- Optional idle `.brdb` pack: set `BYTERAG_IDLE_EXPORT_SECS` (default `0` = off). After a write (`reindex` / startup index / `import_brdb`), if no newer write for that many seconds, exports to `<target>/.byterag/graph.brdb`.

## `BYTERAG_TARGET_DIR`

| Priority | Source | Notes |
| --- | --- | --- |
| 1 | env `BYTERAG_TARGET_DIR` | MCP / cargo |
| 2 | process cwd | if unset |
| runtime | `byterag_reindex(target_dir)` | switch root without restart |

| Env | Default | Notes |
| --- | --- | --- |
| `BYTERAG_IDLE_EXPORT_SECS` | `0` (off) | Seconds after last write before auto-export `.brdb` |

Prefer a crate/subtree over a huge monorepo root.

## Cursor MCP (global)

`~/.cursor/mcp.json` (Windows: `%USERPROFILE%\.cursor\mcp.json`):

```json
{
  "mcpServers": {
    "byterag-codegraph": {
      "type": "stdio",
      "command": "d:/ByteLogicCore/ByteRAGSample/target/debug/byterag_sample.exe",
      "args": [],
      "env": {
        "BYTERAG_TARGET_DIR": "${workspaceFolder}"
      }
    }
  }
}
```

Other IDE example:

```json
"byterag-codegraph": {
  "command": "d:/ByteLogicCore/ByteRAGSample/target/debug/byterag_sample.exe",
  "args": [],
  "env": {
    "BYTERAG_TARGET_DIR": "d:/ByteLogicCore/ByteRAG/core/byterag-core"
  }
}
```

## PowerShell direct attach

Drain stdout/stderr **concurrently** or pipes deadlock:

```powershell
$env:BYTERAG_TARGET_DIR = "d:/ByteLogicCore/ByteRAG/core/byterag-core"
$psi = New-Object System.Diagnostics.ProcessStartInfo
$psi.FileName = "d:/ByteLogicCore/ByteRAGSample/target/debug/byterag_sample.exe"
$psi.UseShellExecute = $false
$psi.RedirectStandardInput = $true
$psi.RedirectStandardOutput = $true
$psi.RedirectStandardError = $true
$p = [Diagnostics.Process]::Start($psi)

$stdoutTask = $p.StandardOutput.ReadToEndAsync()
$stderrTask = $p.StandardError.ReadToEndAsync()

$p.StandardInput.WriteLine('{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}')
$p.StandardInput.Close()

[void][Threading.Tasks.Task]::WaitAll(@($stdoutTask, $stderrTask))
$p.WaitForExit(60000)
$stdoutTask.Result
```

## Build

```bash
cargo build
# or
cargo build --release

# Windows (PowerShell)
$env:BYTERAG_TARGET_DIR = "d:/ByteLogicCore/ByteRAG/core/byterag-core"
cargo run
```

Index data lives under `<target>/.byterag/`. Use `byterag_export_brdb` for portable snapshots.
