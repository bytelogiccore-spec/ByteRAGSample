//! ByteRAG SQL + Query Builder 데모
//!
//! 실행: cargo run -p demo-sql-query

use byterag_core::Database;
use std::fs;
use std::path::PathBuf;

fn main() -> byterag_core::ByteRagResult<()> {
    println!("=== ByteRAG SQL / Query Builder ===\n");

    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".byterag-demo");
    let _ = fs::remove_dir_all(&data_dir);
    fs::create_dir_all(&data_dir)?;

    let db = Database::open(&data_dir)?;

    // 1. DDL / DML via execute_sql
    println!("1. execute_sql — CREATE / INSERT");
    db.execute_sql("CREATE TABLE products (id INT, name TEXT, price INT)")?;
    db.execute_sql("INSERT INTO products (id, name, price) VALUES (1, 'Laptop', 1200)")?;
    db.execute_sql("INSERT INTO products (id, name, price) VALUES (2, 'Mouse', 25)")?;
    db.execute_sql("INSERT INTO products (id, name, price) VALUES (3, 'Keyboard', 75)")?;

    println!("2. execute_sql — SELECT");
    let batches = db.execute_sql("SELECT id, name, price FROM products WHERE price > 50")?;
    for batch in &batches {
        println!(
            "   rows={}, cols={}",
            batch.num_rows(),
            batch.num_columns()
        );
        println!("   {batch:?}");
    }

    // 3. Fluent Query Builder (SQL 문자열을 조립해 execute_sql 호출)
    println!("\n3. query_builder — SELECT with WHERE / ORDER / LIMIT");
    let qb_batches = db
        .query_builder()
        .select(&["id", "name", "price"])
        .from("products")
        .where_("price", ">", "20")
        .order_by("price", "DESC")
        .limit(2)
        .execute()?;
    for batch in &qb_batches {
        println!(
            "   rows={}, cols={}",
            batch.num_rows(),
            batch.num_columns()
        );
        println!("   {batch:?}");
    }

    db.flush()?;
    println!("\n=== 완료 ===");
    println!("다음: cargo run -p demo-graph-csr");
    Ok(())
}
