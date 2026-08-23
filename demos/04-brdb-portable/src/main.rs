//! ByteRAG `.brdb` export / import 데모
//!
//! 실행: cargo run -p demo-brdb-portable

use byterag_core::Database;
use std::fs;
use std::path::PathBuf;

fn main() -> byterag_core::ByteRagResult<()> {
    println!("=== ByteRAG .brdb portable ===\n");

    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let data_dir = base.join(".byterag-demo");
    let pack_path = base.join("snapshot.brdb");

    let _ = fs::remove_dir_all(&data_dir);
    let _ = fs::remove_file(&pack_path);
    fs::create_dir_all(&data_dir)?;

    // 1. 원본 DB에 데이터 기록
    println!("1. open + insert + flush");
    {
        let db = Database::open(&data_dir)?;
        db.insert("notes", b"n:1", b"hello from source")?;
        db.insert("notes", b"n:2", b"portable pack")?;
        db.flush()?;

        println!("2. export_to_file({:?})", pack_path);
        db.export_to_file(&pack_path)?;
    }

    // 2. 원본 디렉터리 삭제 후 팩에서 복원
    println!("3. remove source dir, open_from_file");
    fs::remove_dir_all(&data_dir)?;
    let restored = Database::open_from_file(&pack_path)?;

    println!("4. verify keys");
    for key in [b"n:1".as_slice(), b"n:2".as_slice()] {
        match restored.get("notes", key)? {
            Some(v) => println!(
                "   {} = {}",
                String::from_utf8_lossy(key),
                String::from_utf8_lossy(&v)
            ),
            None => println!("   {} missing!", String::from_utf8_lossy(key)),
        }
    }

    println!("\n=== 완료 ===");
    println!("pack: {:?}", pack_path);
    println!("MCP codegraph도 export/import 도구로 동일 API를 사용합니다.");
    Ok(())
}
