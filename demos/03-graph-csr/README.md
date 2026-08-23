# demo-graph-csr — CsrGraph 멀티홉

## 왜 쓰는지

의존성·호출 관계를 **CSR 그래프**로 두고 N-hop 이웃을 빠르게 보고 싶을 때 사용합니다. codegraph MCP의 `byterag_query_graph`도 내부에서 `CsrGraph::from_edges` + `multi_hop_traversal`을 씁니다. 이 데모는 **파서/MCP 없이** API만 보여 줍니다.

## 최소 개념

- 엣지 목록 `(from, to)` → `CsrGraph::from_edges`
- `neighbors` — 1-hop
- `multi_hop_traversal(seed, max_depth)` — 서브그래프

## 실행

```bash
cargo run -p demo-graph-csr
```

## 핵심 API

```rust
use byterag_core::graph::csr::CsrGraph;

let edges = vec![
    ("App".into(), "Service".into()),
    ("Service".into(), "Db".into()),
];
let graph = CsrGraph::from_edges(&edges);
let sub = graph.multi_hop_traversal("App", 2).unwrap();
```

## 다음 샘플

스냅샷 이동이 필요하면 → [04-brdb-portable](../04-brdb-portable)  
에이전트 도구가 필요하면 → [byterag-codegraph](../../crates/byterag-codegraph)
