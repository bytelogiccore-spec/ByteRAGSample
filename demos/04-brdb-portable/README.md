# demo-brdb-portable — `.brdb` export / import

## 왜 쓰는지

인덱싱한 DB를 **한 파일 팩**으로 옮기거나 백업·공유할 때 사용합니다. MCP의 `byterag_export_brdb` / `byterag_import_brdb`와 동일한 `export_to_file` / `open_from_file` API입니다.

## 최소 개념

- `db.export_to_file("snapshot.brdb")`
- `Database::open_from_file("snapshot.brdb")` — 팩에서 복원
- 원본 디렉터리 없이도 키를 다시 읽을 수 있음

## 실행

```bash
cargo run -p demo-brdb-portable
```

산출물: 이 크레이트 디렉터리의 `snapshot.brdb` (`.gitignore` 대상).

## 핵심 API

```rust
db.export_to_file(&pack_path)?;
let restored = Database::open_from_file(&pack_path)?;
let v = restored.get("notes", b"n:1")?;
```

## 다음 샘플

에이전트용 코드 그래프 → [byterag-codegraph](../../crates/byterag-codegraph)
