# ByteRAGSample - Universal Code GraphRAG MCP Server (Rust)

Rust 기반의 고성능 **Universal Code GraphRAG MCP Server** 프로젝트입니다.

## 지원 언어

- **C++**: `*.cpp`, `*.h`, `*.hpp`, `*.cxx`, `*.cc`
- **C#**: `*.cs`
- **Rust**: `*.rs`
- **TypeScript / JS**: `*.ts`, `*.tsx`, `*.js`, `*.jsx`
- **Python**: `*.py`

## 제공 MCP Tools

1. `byterag_query_graph`: 시드 심볼 기준 **BFS 서브그래프** 조회 (`max_depth`, `edges` 포함).
2. `byterag_search_symbols`: id / 이름 / 파일 경로 **부분 일치 검색** (`limit` 기본 50).
3. `byterag_reindex`: 지식 그래프 재생성. `target_dir`을 넘기면 인덱싱 루트를 바꾼 뒤 동기 인덱싱합니다.

## 심볼 id 형식

파일 스코프 id로 충돌을 피합니다:

```text
struct:Database@core/byterag-core/src/engine/database.rs
fn:open_in_memory@core/byterag-core/src/engine/constructors.rs
file:core/byterag-core/src/lib.rs
```

- `byterag_search_symbols("Database")` → 이름/id/경로에 `Database`가 포함된 심볼들
- `byterag_query_graph(node_id: "struct:Database")` → 동일 짧은 이름의 **모든** 매칭 + outgoing edges 탐색

## 기동 동작

- DB(`.byterag/`)만 연 뒤 **즉시** MCP `initialize` / `tools/list`에 응답합니다.
- 첫 인덱싱은 **백그라운드 스레드**에서 수행합니다 (`stderr`에 `byterag: indexing start/done` 로그).
- 백그라운드 완료 전 조회는 부분 결과일 수 있습니다. 확실한 스냅샷이 필요하면 `byterag_reindex`를 호출하세요.

## `BYTERAG_TARGET_DIR` (동적 인덱싱 루트)

서버가 지식 그래프를 만들 대상 폴더입니다.

| 우선순위 | 출처 | 설명 |
| --- | --- | --- |
| 1 | 환경 변수 `BYTERAG_TARGET_DIR` | MCP/`cargo` 실행 시 주입 |
| 2 | 프로세스 cwd | 변수가 없으면 `current_dir()` 사용 |
| 런타임 | `byterag_reindex(target_dir)` | 서버를 재시작하지 않고 루트 변경 |

**범위:** 거대한 모노레포 루트 대신 `core/byterag-core`처럼 **필요한 크레이트/서브트리만** 지정하세요. 전체가 필요하면 크레이트 단위로 나눠 `byterag_reindex` 하세요.

Cursor에서는 `${workspaceFolder}`로 현재 워크스페이스를 넘깁니다.

## Cursor MCP 설정

프로젝트 설정 파일: [`.cursor/mcp.json`](.cursor/mcp.json)

```json
{
  "mcpServers": {
    "bytelogic": {
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

- `command`: 이 샘플을 빌드한 실행 파일 경로(서버 바이너리 위치).
- `BYTERAG_TARGET_DIR`: `${workspaceFolder}` → Cursor가 연 프로젝트 루트로 치환.
- Cursor는 stdio를 스트리밍하므로, PowerShell 직접 호출에서 나는 **파이프 데드락**은 보통 없습니다.

다른 IDE(예: Cascade) 예:

```json
"bytelogic": {
  "command": "d:/ByteLogicCore/ByteRAGSample/target/debug/byterag_sample.exe",
  "args": [],
  "env": {
    "BYTERAG_TARGET_DIR": "d:/ByteLogicCore/ByteRAG/core/byterag-core"
  }
}
```

## PowerShell로 exe를 직접 붙일 때

종료 후에야 `ReadToEnd` 하면 stdout/stderr 파이프가 가득 차 **데드락·타임아웃**이 납니다. 출력을 **먼저/동시에** 비우세요.

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

## 빌드 및 실행

```bash
cargo build
# 또는
cargo build --release

# Windows (PowerShell) — 작은 루트 권장
$env:BYTERAG_TARGET_DIR = "d:/ByteLogicCore/ByteRAG/core/byterag-core"
cargo run
```

인덱싱 결과는 대상 폴더 아래 `.byterag/`(`wal.log`, `wos/` 등)에 저장됩니다. 예전 `graph_store.json`은 쓰지 않으며, 있으면 기동 시 삭제합니다.
