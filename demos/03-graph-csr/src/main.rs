//! ByteRAG CsrGraph 멀티홉 데모 (파서/MCP 없음)
//!
//! 실행: cargo run -p demo-graph-csr

use byterag_core::graph::csr::CsrGraph;

fn main() {
    println!("=== ByteRAG CsrGraph ===\n");

    // 작은 의존 그래프: App → Service → Db, App → Auth
    let edges = vec![
        ("App".to_string(), "Service".to_string()),
        ("Service".to_string(), "Db".to_string()),
        ("App".to_string(), "Auth".to_string()),
        ("Auth".to_string(), "Db".to_string()),
    ];

    println!("1. CsrGraph::from_edges ({} directed edges)", edges.len());
    let graph = CsrGraph::from_edges(&edges);

    println!("2. neighbors of App");
    if let Some(app_id) = graph.get_node_id("App") {
        for &nid in graph.neighbors(app_id) {
            if let Some(name) = graph.get_node_name(nid) {
                println!("   App -> {name}");
            }
        }
    }

    println!("3. multi_hop_traversal(App, depth=2)");
    let Some(sub) = graph.multi_hop_traversal("App", 2) else {
        eprintln!("   App not found");
        return;
    };

    println!("   nodes:");
    for &nid in &sub.nodes {
        if let Some(name) = graph.get_node_name(nid) {
            println!("     - {name}");
        }
    }
    println!("   edges:");
    for &(u, v) in &sub.edges {
        let src = graph.get_node_name(u).unwrap_or("?");
        let dst = graph.get_node_name(v).unwrap_or("?");
        println!("     {src} -> {dst}");
    }

    println!("\n=== 완료 ===");
    println!("다음: cargo run -p demo-brdb-portable");
    println!("에이전트용 codegraph MCP: crates/byterag-codegraph");
}
