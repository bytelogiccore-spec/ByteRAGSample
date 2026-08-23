# demo-sql-query — SQL / Query Builder

## 왜 쓰는지

KV만으로 부족할 때 **관계형 조회**가 필요하면 `execute_sql`과 fluent **Query Builder**를 씁니다. 스키마(DDL)와 SELECT를 같은 `Database` 핸들에서 다룹니다.

## 최소 개념

- `execute_sql("CREATE TABLE ...")` / `INSERT` / `SELECT`
- `db.query_builder().select(...).from(...).where_(...).execute()`
- 결과는 Arrow `RecordBatch` 벡터

## 실행

```bash
cargo run -p demo-sql-query
```

## 핵심 API

```rust
db.execute_sql("CREATE TABLE products (id INT, name TEXT, price INT)")?;
db.execute_sql("INSERT INTO products (id, name, price) VALUES (1, 'Laptop', 1200)")?;
let batches = db.execute_sql("SELECT id, name, price FROM products WHERE price > 50")?;

let batches = db
    .query_builder()
    .select(&["id", "name", "price"])
    .from("products")
    .where_("price", ">", "20")
    .order_by("price", "DESC")
    .limit(2)
    .execute()?;
```

## 다음 샘플

그래프 탐색이 필요하면 → [03-graph-csr](../03-graph-csr)
