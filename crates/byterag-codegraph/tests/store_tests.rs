use byterag_codegraph::GraphStore;
use std::fs;

#[test]
fn test_store_lifecycle_and_search() {
    let temp_dir = std::env::temp_dir().join(format!("byterag_test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    fs::create_dir_all(&temp_dir).unwrap();

    let src_dir = temp_dir.join("src");
    fs::create_dir_all(&src_dir).unwrap();

    // Create sample Rust files
    fs::write(
        src_dir.join("engine.rs"),
        r#"
        pub struct CoreEngine {
            running: bool,
        }
        pub fn start_engine() -> CoreEngine {
            CoreEngine { running: true }
        }
        "#,
    ).unwrap();

    fs::write(
        src_dir.join("main.rs"),
        r#"
        mod engine;
        use engine::{CoreEngine, start_engine};

        fn main() {
            let _e = start_engine();
        }
        "#,
    ).unwrap();

    let store = GraphStore::open(temp_dir.clone());
    store.index_directory();

    let status = store.index_status();
    assert_eq!(status["files"].as_u64().unwrap(), 2, "Should index 2 files");
    assert!(status["nodes"].as_u64().unwrap() >= 3, "Should have at least 3 nodes");

    // Symbol search test
    let results = store.search_symbols_with_limit("CoreEngine", 10);
    assert!(!results.is_empty(), "Search should find CoreEngine");
    assert_eq!(results[0].name, "CoreEngine");

    let fn_results = store.search_symbols_with_limit("start_engine", 10);
    assert!(!fn_results.is_empty(), "Search should find start_engine");

    // Subgraph query test
    let sub = store.query_subgraph(&["CoreEngine".to_string()], 2, None);
    assert!(sub["nodes"].as_array().unwrap().len() >= 1);

    // Path finding test
    let path = store.find_path("main", "start_engine", 3);
    assert!(path["found"].as_bool().unwrap_or(false) || !path["from_resolved"].as_array().unwrap().is_empty());

    // Blast radius test
    let blast = store.blast_radius("CoreEngine", 2);
    assert!(blast["reachable_nodes"].as_u64().unwrap() >= 1);

    // Export test
    let brdb_path = temp_dir.join("export.brdb");
    let exp_res = store.export_brdb(&brdb_path, 1);
    assert!(exp_res.is_ok(), "Export brdb must succeed");
    assert!(brdb_path.exists(), "BRDB file must exist");

    // Cleanup
    let _ = fs::remove_dir_all(&temp_dir);
}
