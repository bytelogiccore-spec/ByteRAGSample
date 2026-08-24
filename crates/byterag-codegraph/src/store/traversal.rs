use super::GraphStore;
use crate::types::{EdgeType, GraphEdge, GraphNode as SampleNode};
use byterag_core::graph::csr::CsrGraph;
use std::collections::{HashMap, HashSet, VecDeque};

impl GraphStore {
    pub fn get_neighbors(
        &self,
        seed: &str,
        edge_types: Option<&[EdgeType]>,
    ) -> serde_json::Value {
        let resolved = self.resolve_seeds(&[seed.to_string()]);
        if resolved.is_empty() {
            return serde_json::json!({
                "seed": seed,
                "resolved": [],
                "node": null,
                "neighbors": [],
                "edges": []
            });
        }
        let id = &resolved[0];
        let node = self.get_node(id);
        let mut neighbor_ids: HashSet<String> = HashSet::new();
        let mut edges_out: Vec<GraphEdge> = Vec::new();
        for edge in self.load_all_edges() {
            if let Some(types) = edge_types {
                if !types.is_empty() && !types.contains(&edge.edge_type) {
                    continue;
                }
            }
            if &edge.source == id {
                neighbor_ids.insert(edge.target.clone());
                edges_out.push(edge);
            } else if &edge.target == id {
                neighbor_ids.insert(edge.source.clone());
                edges_out.push(edge);
            }
        }
        let mut neighbors: Vec<SampleNode> = neighbor_ids
            .into_iter()
            .filter_map(|nid| self.get_node(&nid))
            .collect();
        neighbors.sort_by(|a, b| a.id.cmp(&b.id));
        serde_json::json!({
            "seed": seed,
            "resolved": resolved,
            "node": node,
            "neighbors": neighbors,
            "edges": edges_out
        })
    }

    pub fn find_path(&self, from: &str, to: &str, max_depth: usize) -> serde_json::Value {
        let from_ids = self.resolve_seeds(&[from.to_string()]);
        let to_ids = self.resolve_seeds(&[to.to_string()]);
        if from_ids.is_empty() || to_ids.is_empty() {
            return serde_json::json!({
                "found": false,
                "path": [],
                "edges": [],
                "from": from,
                "to": to
            });
        }
        let start = from_ids[0].clone();
        let goals: HashSet<String> = to_ids.into_iter().collect();

        let all_edges = self.load_all_edges();
        let mut adj: HashMap<String, Vec<(String, GraphEdge)>> = HashMap::new();
        for e in &all_edges {
            adj.entry(e.source.clone())
                .or_default()
                .push((e.target.clone(), e.clone()));
            adj.entry(e.target.clone())
                .or_default()
                .push((e.source.clone(), e.clone()));
        }

        let mut prev: HashMap<String, (String, GraphEdge)> = HashMap::new();
        let mut visited = HashSet::new();
        let mut q = VecDeque::new();
        visited.insert(start.clone());
        q.push_back((start.clone(), 0usize));

        let mut found_goal: Option<String> = None;
        while let Some((cur, depth)) = q.pop_front() {
            if goals.contains(&cur) && cur != start {
                found_goal = Some(cur);
                break;
            }
            if depth >= max_depth {
                continue;
            }
            if let Some(neis) = adj.get(&cur) {
                for (nxt, edge) in neis {
                    if visited.insert(nxt.clone()) {
                        prev.insert(nxt.clone(), (cur.clone(), edge.clone()));
                        q.push_back((nxt.clone(), depth + 1));
                    }
                }
            }
        }

        let Some(goal) = found_goal.or_else(|| {
            if goals.contains(&start) {
                Some(start.clone())
            } else {
                None
            }
        }) else {
            return serde_json::json!({
                "found": false,
                "path": [],
                "edges": [],
                "from_resolved": from_ids,
                "to_resolved": goals.into_iter().collect::<Vec<_>>(),
            });
        };

        let mut path_nodes = vec![goal.clone()];
        let mut path_edges = Vec::new();
        let mut cur = goal;
        while let Some((p, e)) = prev.get(&cur) {
            path_edges.push(e.clone());
            path_nodes.push(p.clone());
            cur = p.clone();
            if cur == start {
                break;
            }
        }
        path_nodes.reverse();
        path_edges.reverse();

        let nodes: Vec<SampleNode> = path_nodes
            .iter()
            .filter_map(|id| self.get_node(id))
            .collect();

        serde_json::json!({
            "found": true,
            "path": path_nodes,
            "nodes": nodes,
            "edges": path_edges,
            "from_resolved": from_ids,
        })
    }

    pub fn blast_radius(&self, seed: &str, depth: usize) -> serde_json::Value {
        let sub = self.query_subgraph(&[seed.to_string()], depth, None);
        let nodes = sub
            .get("nodes")
            .and_then(|n| n.as_array())
            .cloned()
            .unwrap_or_default();
        let edges = sub
            .get("edges")
            .and_then(|n| n.as_array())
            .cloned()
            .unwrap_or_default();

        let mut fan_out: HashMap<String, usize> = HashMap::new();
        let mut fan_in: HashMap<String, usize> = HashMap::new();
        for e in &edges {
            if let (Some(s), Some(t)) = (
                e.get("source").and_then(|v| v.as_str()),
                e.get("target").and_then(|v| v.as_str()),
            ) {
                *fan_out.entry(s.to_string()).or_default() += 1;
                *fan_in.entry(t.to_string()).or_default() += 1;
            }
        }
        let mut hubs_out: Vec<(String, usize)> = fan_out.into_iter().collect();
        let mut hubs_in: Vec<(String, usize)> = fan_in.into_iter().collect();
        hubs_out.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        hubs_in.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        hubs_out.truncate(10);
        hubs_in.truncate(10);

        serde_json::json!({
            "seed": seed,
            "depth": depth,
            "reachable_nodes": nodes.len(),
            "edges": edges.len(),
            "nodes": nodes,
            "fan_out_hubs": hubs_out.into_iter().map(|(id, n)| json_obj_id_count(&id, n)).collect::<Vec<_>>(),
            "fan_in_hubs": hubs_in.into_iter().map(|(id, n)| json_obj_id_count(&id, n)).collect::<Vec<_>>(),
            "summary": sub.get("summary").cloned().unwrap_or(serde_json::Value::Null)
        })
    }

    pub fn detect_cycles(&self, max_cycles: usize) -> serde_json::Value {
        let edges = self.load_all_edges();
        let mut adj: HashMap<String, Vec<String>> = HashMap::new();
        let mut nodes: HashSet<String> = HashSet::new();
        for e in &edges {
            if matches!(
                e.edge_type,
                EdgeType::Imports
                    | EdgeType::Includes
                    | EdgeType::Using
                    | EdgeType::Extends
                    | EdgeType::Implements
                    | EdgeType::Calls
            ) {
                adj.entry(e.source.clone())
                    .or_default()
                    .push(e.target.clone());
                nodes.insert(e.source.clone());
                nodes.insert(e.target.clone());
            }
        }

        let mut cycles: Vec<Vec<String>> = Vec::new();
        let mut visiting: HashSet<String> = HashSet::new();
        let mut visited: HashSet<String> = HashSet::new();
        let mut stack: Vec<String> = Vec::new();

        fn dfs(
            u: &str,
            adj: &HashMap<String, Vec<String>>,
            visiting: &mut HashSet<String>,
            visited: &mut HashSet<String>,
            stack: &mut Vec<String>,
            cycles: &mut Vec<Vec<String>>,
            max_cycles: usize,
        ) {
            if cycles.len() >= max_cycles {
                return;
            }
            visiting.insert(u.to_string());
            stack.push(u.to_string());
            if let Some(neis) = adj.get(u) {
                for v in neis {
                    if cycles.len() >= max_cycles {
                        break;
                    }
                    if visiting.contains(v) {
                        if let Some(pos) = stack.iter().position(|x| x == v) {
                            let mut cyc = stack[pos..].to_vec();
                            cyc.push(v.clone());
                            cycles.push(cyc);
                        }
                    } else if !visited.contains(v) {
                        dfs(v, adj, visiting, visited, stack, cycles, max_cycles);
                    }
                }
            }
            stack.pop();
            visiting.remove(u);
            visited.insert(u.to_string());
        }

        let mut sorted: Vec<String> = nodes.into_iter().collect();
        sorted.sort();
        for n in sorted {
            if !visited.contains(&n) {
                dfs(
                    &n,
                    &adj,
                    &mut visiting,
                    &mut visited,
                    &mut stack,
                    &mut cycles,
                    max_cycles,
                );
            }
        }

        serde_json::json!({
            "cycle_count": cycles.len(),
            "cycles": cycles,
            "note": "Directed cycles over imports/includes/extends/implements/calls (heuristic)"
        })
    }

    pub fn query_subgraph(
        &self,
        seed_ids: &[String],
        max_depth: usize,
        edge_types: Option<&[EdgeType]>,
    ) -> serde_json::Value {
        let resolved = self.resolve_seeds(seed_ids);
        if resolved.is_empty() {
            return serde_json::json!({
                "nodes": [],
                "edges": [],
                "query_seeds": seed_ids,
                "resolved_seeds": [],
                "max_depth": max_depth,
                "summary": { "node_count": 0, "edge_count": 0, "hubs": [] }
            });
        }

        let mut all_edges = self.load_all_edges();
        if let Some(types) = edge_types {
            if !types.is_empty() {
                all_edges.retain(|e| types.contains(&e.edge_type));
            }
        }

        let mut directed: Vec<(String, String)> = Vec::with_capacity(all_edges.len() * 2);
        let mut edge_lookup: HashMap<(String, String), GraphEdge> = HashMap::new();
        for edge in &all_edges {
            edge_lookup.insert((edge.source.clone(), edge.target.clone()), edge.clone());
            directed.push((edge.source.clone(), edge.target.clone()));
            directed.push((edge.target.clone(), edge.source.clone()));
        }
        let csr = CsrGraph::from_edges(&directed);

        let mut visited_edges: HashSet<String> = HashSet::new();
        let mut node_map: HashMap<String, SampleNode> = HashMap::new();
        let mut edge_list: Vec<GraphEdge> = Vec::new();

        for seed in &resolved {
            if let Some(node) = self.get_node(seed) {
                node_map.insert(seed.clone(), node);
            }
            let Some(sub) = csr.multi_hop_traversal(seed, max_depth) else {
                continue;
            };
            for &nid in &sub.nodes {
                let Some(name) = csr.get_node_name(nid).map(|s| s.to_string()) else {
                    continue;
                };
                if let Some(node) = self.get_node(&name) {
                    node_map.entry(name).or_insert(node);
                }
            }
            for &(u, v) in &sub.edges {
                let Some(src_name) = csr.get_node_name(u).map(|s| s.to_string()) else {
                    continue;
                };
                let Some(dst_name) = csr.get_node_name(v).map(|s| s.to_string()) else {
                    continue;
                };
                let canon = if let Some(e) = edge_lookup.get(&(src_name.clone(), dst_name.clone())) {
                    e.clone()
                } else if let Some(e) = edge_lookup.get(&(dst_name.clone(), src_name.clone())) {
                    e.clone()
                } else {
                    continue;
                };
                let edge_key = format!("{}->{}:{:?}", canon.source, canon.target, canon.edge_type);
                if visited_edges.insert(edge_key) {
                    edge_list.push(canon);
                }
            }
        }

        for id in &resolved {
            if let Some(node) = self.get_node(id) {
                node_map.entry(id.clone()).or_insert(node);
            }
        }

        let mut nodes: Vec<SampleNode> = node_map.into_values().collect();
        nodes.sort_by(|a, b| a.id.cmp(&b.id));
        edge_list.sort_by(|a, b| {
            a.source
                .cmp(&b.source)
                .then_with(|| a.target.cmp(&b.target))
        });

        let summary = build_summary(&nodes, &edge_list);

        serde_json::json!({
            "nodes": nodes,
            "edges": edge_list,
            "query_seeds": seed_ids,
            "resolved_seeds": resolved,
            "max_depth": max_depth,
            "summary": summary
        })
    }
}

fn build_summary(nodes: &[SampleNode], edges: &[GraphEdge]) -> serde_json::Value {
    let mut degree: HashMap<String, usize> = HashMap::new();
    for e in edges {
        *degree.entry(e.source.clone()).or_default() += 1;
        *degree.entry(e.target.clone()).or_default() += 1;
    }
    let mut hubs: Vec<(String, usize)> = degree.into_iter().collect();
    hubs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    hubs.truncate(5);
    serde_json::json!({
        "node_count": nodes.len(),
        "edge_count": edges.len(),
        "hubs": hubs.into_iter().map(|(id, n)| json_obj_id_count(&id, n)).collect::<Vec<_>>()
    })
}

fn json_obj_id_count(id: &str, count: usize) -> serde_json::Value {
    serde_json::json!({ "id": id, "count": count })
}
