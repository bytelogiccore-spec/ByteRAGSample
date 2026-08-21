# ByteRAGSample - Universal Code GraphRAG MCP Server (Rust)

Rust 기반의 고성능 **Universal Code GraphRAG MCP Server** 프로젝트입니다.

## 🌟 지원 언어

- **C++**: `*.cpp`, `*.h`, `*.hpp`, `*.cxx`, `*.cc`
- **C#**: `*.cs`
- **Rust**: `*.rs`
- **TypeScript / JS**: `*.ts`, `*.tsx`, `*.js`, `*.jsx`
- **Python**: `*.py`

## 🛠️ 제공 MCP Tools

1. `byterag_query_graph`: 시드 심볼 및 탐색 깊이 기준 의존성 서브그래프 조회.
2. `byterag_search_symbols`: 키워드 기반 심볼 검색.
3. `byterag_reindex`: 프로젝트 폴더 지식 그래프 즉시 재생성.

## 🚀 빌드 및 실행

```bash
# 1. 빌드
cargo build --release

# 2. 실행 (Stdio MCP Server)
cargo run --release
```
