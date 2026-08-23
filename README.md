# ByteRAGSample — ByteRAG 사용법 데모 워크스페이스

[ByteRAG](https://crates.io/crates/byterag-core) (`byterag-core` **0.3.0**, crates.io)의 주요 사용 패턴을 **샘플별로 분리**한 저장소입니다.  
각 샘플은 한 가지 주제만 다루며, 실행 가능한 Rust 바이너리와 한국어 README를 제공합니다.

## 어떤 샘플을 볼까?

| 샘플 | 패키지 | 한 줄 요약 | 실행 |
| --- | --- | --- | --- |
| [demos/01-kv-crud](demos/01-kv-crud) | `demo-kv-crud` | 임베디드 KV: open / insert / get / delete / flush | `cargo run -p demo-kv-crud` |
| [demos/02-sql-query](demos/02-sql-query) | `demo-sql-query` | `execute_sql` + Query Builder | `cargo run -p demo-sql-query` |
| [demos/03-graph-csr](demos/03-graph-csr) | `demo-graph-csr` | `CsrGraph` 멀티홉 (파서/MCP 없음) | `cargo run -p demo-graph-csr` |
| [demos/04-brdb-portable](demos/04-brdb-portable) | `demo-brdb-portable` | `.brdb` export / import 왕복 | `cargo run -p demo-brdb-portable` |
| [crates/byterag-codegraph](crates/byterag-codegraph) | `byterag-codegraph` | 코드 의존성 그래프 **MCP** 서버 | `cargo build -p byterag-codegraph` |

추천 학습 순서: **KV → SQL → Graph → brdb → codegraph MCP**.

## 빌드

```bash
cargo build --workspace
# 릴리스
cargo build --workspace --release
```

MCP 바이너리 이름(Cursor `mcp.json` 호환): `target/debug/byterag_sample.exe` (Windows) / `target/debug/byterag_sample`.

## 다음에 추가 가능

Vector 검색, Encryption/WAL, 언어 바인딩(Python/Node/C#)은 의도적으로 제외했습니다. 필요하면 동일 패턴으로 `demos/`에 추가하면 됩니다.

## 엔진

공통 의존성: [`byterag-core = "0.3.0"`](https://crates.io/crates/byterag-core) ([workspace.dependencies](Cargo.toml)).
