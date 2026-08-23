# byterag-codegraph — 코드 의존성 그래프 MCP

Rust MCP 서버입니다. [ByteRAG](https://crates.io/crates/byterag-core) (`byterag-core` **0.3.0**) 위에 **코드 의존성 그래프**를 쌓고, 에이전트용 분석 도구를 노출합니다.

- **ByteRAG** = 임베디드 DB / 그래프 저장 엔진  
- **byterag-codegraph** = 이 MCP (코드 그래프 도구)

기본 KV·SQL·CSR·`.brdb`만 보려면 워크스페이스 [루트 README](../../README.md)의 `demos/`를 먼저 보세요.

## 지원 언어

- **C++**: `*.cpp`, `*.h`, `*.hpp`, `*.cxx`, `*.cc`
- **C#**: `*.cs`
- **Rust**: `*.rs`
- **TypeScript / JS**: `*.ts`, `*.tsx`, `*.js`, `*.jsx`
- **Python**: `*.py`

## MCP 도구

**Core**
1. `byterag_query_graph` — CsrGraph BFS 서브그래프 (`max_depth`, 선택 `edge_types`, 응답 `summary`)
2. `byterag_search_symbols` — id / name / path 검색 (`limit` 기본 50)
3. `byterag_reindex` — 재빌드(+ `flush`); 선택 `target_dir`
4. `byterag_export_brdb` / `byterag_import_brdb` — 이식용 `.brdb` 팩

**Analysis**
5. `byterag_index_status` — 인덱싱 플래그, 카운트, 마지막 인덱스 시각
6. `byterag_get_symbol` / `byterag_get_neighbors` — 노드 + 1-hop (`edge_type` 필터)
7. `byterag_find_path` — 두 시드 사이 최단 경로
8. `byterag_blast_radius` — N-hop 도달 집합 + fan-in/out 허브
9. `byterag_list_by_type` — 노드 타입 / path / language 필터
10. `byterag_read_snippet` — 심볼 주변 소스 ±N줄
11. `byterag_detect_cycles` — 방향 사이클 샘플 (구조 냄새)

## 심볼 id 형식

```text
struct:Database@core/byterag-core/src/engine/database.rs
fn:open_in_memory@core/byterag-core/src/engine/constructors.rs
file:core/byterag-core/src/lib.rs
```

## 기동

- `<target>/.byterag/`를 연 뒤 MCP `initialize` / `tools/list`에 바로 응답합니다.
- 첫 인덱스는 **백그라운드 스레드** (`byterag: indexing start/done` on stderr).
- 인덱싱 끝에서 `flush()` (WAL 정리). 내구성 스냅샷이 필요하면 `byterag_reindex`를 선호하세요.
- 증분 재인덱스는 변경 없는 파일을 건너뜁니다(`file_meta` mtime). 제외: `target`, `.git`, `.byterag`, `node_modules`, `dist`, `build`, `__pycache__`.
- 유휴 `.brdb` 팩: `BYTERAG_IDLE_EXPORT_SECS` (기본 `0` = 끔). write 이후 해당 초 동안 추가 write가 없으면 `<target>/.byterag/graph.brdb`로 export.

## `BYTERAG_TARGET_DIR`

| 우선순위 | 출처 | 비고 |
| --- | --- | --- |
| 1 | env `BYTERAG_TARGET_DIR` | MCP / cargo |
| 2 | 프로세스 cwd | 미설정 시 |
| runtime | `byterag_reindex(target_dir)` | 재시작 없이 루트 전환 |

| Env | 기본값 | 비고 |
| --- | --- | --- |
| `BYTERAG_IDLE_EXPORT_SECS` | `0` (끔) | 마지막 write 후 자동 export까지 초 |

거대한 모노레포 루트보다 크레이트/서브트리를 권장합니다.

## Cursor MCP (전역)

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

워크스페이스 빌드 후 바이너리 경로:

```bash
cargo build -p byterag-codegraph
# → target/debug/byterag_sample(.exe)
```

다른 IDE 예:

```json
"byterag-codegraph": {
  "command": "d:/ByteLogicCore/ByteRAGSample/target/debug/byterag_sample.exe",
  "args": [],
  "env": {
    "BYTERAG_TARGET_DIR": "d:/ByteLogicCore/ByteRAG/core/byterag-core"
  }
}
```

## PowerShell 직접 attach

stdout/stderr를 **동시에** drain하지 않으면 파이프가 데드락날 수 있습니다.

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

## 빌드

```bash
cargo build -p byterag-codegraph
# 또는
cargo build -p byterag-codegraph --release
```

인덱스 데이터는 `<target>/.byterag/`에 있습니다. 이식용 스냅샷은 `byterag_export_brdb`를 사용하세요.
