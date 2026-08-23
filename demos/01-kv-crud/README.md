# demo-kv-crud — 임베디드 KV CRUD

## 왜 쓰는지

ByteRAG를 **로컬 임베디드 DB**로 쓸 때의 기본 경로입니다. 테이블(컬렉션)에 바이트 키·값을 넣고, `flush`로 디스크에 굳힙니다. codegraph MCP의 `nodes`/`edges` 저장도 이 API 위에 있습니다.

## 최소 개념

- `Database::open(path)` — 디렉터리에 WAL/WOS 유지
- `insert` / `get` / `delete` / `count`
- `flush()` — Delta → WOS (내구성)

## 실행

```bash
cargo run -p demo-kv-crud
```

데이터는 이 크레이트 아래 `.byterag-demo/`에 쓰입니다(워크스페이스 루트 `.byterag`와 분리).

## 핵심 API

```rust
let db = Database::open(&data_dir)?;
db.insert("users", b"user:1", b"Alice")?;
let v = db.get("users", b"user:1")?;
db.delete("users", b"user:3")?;
db.flush()?;
```

## 다음 샘플

SQL이 필요하면 → [02-sql-query](../02-sql-query)
