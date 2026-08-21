# ByteRAGSample - Universal Code GraphRAG MCP Server (Rust)

Rust 기반의 고성능 **Universal Code GraphRAG MCP Server** 프로젝트입니다.

## 지원 언어

- **C++**: `*.cpp`, `*.h`, `*.hpp`, `*.cxx`, `*.cc`
- **C#**: `*.cs`
- **Rust**: `*.rs`
- **TypeScript / JS**: `*.ts`, `*.tsx`, `*.js`, `*.jsx`
- **Python**: `*.py`

## 제공 MCP Tools

1. `byterag_query_graph`: 시드 심볼 및 탐색 깊이 기준 의존성 서브그래프 조회.
2. `byterag_search_symbols`: 키워드 기반 심볼 검색.
3. `byterag_reindex`: 프로젝트 폴더 지식 그래프 즉시 재생성. `target_dir`을 넘기면 인덱싱 루트를 그 경로로 전환합니다.

## `BYTERAG_TARGET_DIR` (동적 인덱싱 루트)

서버가 지식 그래프를 만들 대상 폴더입니다.

| 우선순위 | 출처 | 설명 |
| --- | --- | --- |
| 1 | 환경 변수 `BYTERAG_TARGET_DIR` | MCP/`cargo` 실행 시 주입 |
| 2 | 프로세스 cwd | 변수가 없으면 `current_dir()` 사용 |
| 런타임 | `byterag_reindex(target_dir)` | 서버를 재시작하지 않고 루트 변경 |

하드코딩 경로 대신 Cursor의 `${workspaceFolder}`를 쓰면, **현재 연 워크스페이스**가 자동으로 인덱싱 대상이 됩니다.

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
- 다른 폴더만 인덱싱하려면 `byterag_reindex`에 `target_dir`을 넘기면 됩니다.

다른 IDE(예: Cascade)에서도 동일하게 환경 변수만 넘기면 됩니다.

```json
"bytelogic": {
  "command": "d:/ByteLogicCore/ByteRAGSample/target/debug/byterag_sample.exe",
  "args": [],
  "env": {
    "BYTERAG_TARGET_DIR": "d:/Your/Project"
  }
}
```

## 빌드 및 실행

```bash
# 1. 빌드
cargo build
# 또는
cargo build --release

# 2. 실행 (Stdio MCP Server) — 대상 디렉터리 지정
# Windows (PowerShell)
$env:BYTERAG_TARGET_DIR = "d:/Your/Project"
cargo run

# 미지정 시 현재 작업 디렉터리를 인덱싱합니다.
cargo run --release
```

인덱싱 결과는 대상 폴더 아래 `.byterag/`에 저장됩니다.
