//! ByteRAG 임베디드 KV CRUD 데모
//!
//! 실행: cargo run -p demo-kv-crud

use byterag_core::Database;
use std::fs;
use std::path::PathBuf;

fn main() -> byterag_core::ByteRagResult<()> {
    println!("=== ByteRAG KV CRUD ===\n");

    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".byterag-demo");
    let _ = fs::remove_dir_all(&data_dir);
    fs::create_dir_all(&data_dir)?;

    // 1. 디스크 디렉터리에 DB 열기 (WAL/WOS 유지)
    println!("1. Database::open({:?})", data_dir);
    let db = Database::open(&data_dir)?;

    // 2. Create
    println!("2. insert");
    db.insert("users", b"user:1", b"Alice")?;
    db.insert("users", b"user:2", b"Bob")?;
    db.insert("users", b"user:3", b"Charlie")?;

    // 3. Read
    println!("3. get");
    if let Some(value) = db.get("users", b"user:1")? {
        println!("   user:1 = {}", String::from_utf8_lossy(&value));
    }

    // 4. Update (동일 키 재삽입)
    println!("4. update (re-insert)");
    db.insert("users", b"user:1", b"Alice Updated")?;
    if let Some(value) = db.get("users", b"user:1")? {
        println!("   user:1 = {}", String::from_utf8_lossy(&value));
    }

    // 5. Delete
    println!("5. delete");
    db.delete("users", b"user:3")?;
    assert!(db.get("users", b"user:3")?.is_none());
    println!("   user:3 removed (get returns None)");

    // 6. Flush — Delta → WOS
    println!("6. flush");
    db.flush()?;
    println!("   durable snapshot under {:?}", data_dir.join("wos"));

    println!("\n=== 완료 ===");
    println!("다음: cargo run -p demo-sql-query");
    Ok(())
}
