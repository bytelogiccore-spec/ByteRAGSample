use super::GraphStore;
use crate::types::{GraphNode as SampleNode, NodeType};
use std::collections::HashSet;
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct SearchScore(u8);

impl GraphStore {
    pub fn get_symbol(&self, seed: &str) -> Option<SampleNode> {
        let resolved = self.resolve_seeds(&[seed.to_string()]);
        let id = resolved.first()?;
        self.get_node(id)
    }

    pub fn list_by_type(
        &self,
        node_type: &str,
        path_prefix: Option<&str>,
        lang: Option<&str>,
        limit: usize,
    ) -> Vec<SampleNode> {
        let want = NodeType::parse(node_type);
        let prefix = path_prefix.map(|s| s.replace('\\', "/").to_lowercase());
        let lang_l = lang.map(|s| s.to_lowercase());
        let Ok(entries) = self.db.scan("nodes") else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for (_k, val) in entries {
            let Ok(node) = serde_json::from_slice::<SampleNode>(&val) else {
                continue;
            };
            if let Some(ref t) = want {
                if &node.node_type != t {
                    continue;
                }
            } else if !node_type.is_empty()
                && !node.node_type.as_str().eq_ignore_ascii_case(node_type)
            {
                continue;
            }
            if let Some(ref p) = prefix {
                if !node.file_path.to_lowercase().starts_with(p)
                    && !node.file_path.to_lowercase().contains(p)
                {
                    continue;
                }
            }
            if let Some(ref l) = lang_l {
                if !node.language.eq_ignore_ascii_case(l) {
                    continue;
                }
            }
            out.push(node);
            if out.len() >= limit {
                break;
            }
        }
        out.sort_by(|a, b| a.id.cmp(&b.id));
        out
    }

    pub fn read_snippet(
        &self,
        seed: &str,
        before: usize,
        after: usize,
    ) -> serde_json::Value {
        let Some(node) = self.get_symbol(seed) else {
            return serde_json::json!({ "error": "symbol not found", "seed": seed });
        };
        let abs = self.target_dir.join(&node.file_path);
        let Ok(content) = fs::read_to_string(&abs) else {
            return serde_json::json!({
                "error": "cannot read file",
                "path": node.file_path,
                "node": node
            });
        };
        let lines: Vec<&str> = content.lines().collect();
        let center = node.line.unwrap_or(1).saturating_sub(1);
        let start = center.saturating_sub(before);
        let end = (center + after + 1).min(lines.len());
        let snippet: Vec<String> = lines[start..end]
            .iter()
            .enumerate()
            .map(|(i, l)| format!("{:>6}|{}", start + i + 1, l))
            .collect();
        serde_json::json!({
            "node": node,
            "start_line": start + 1,
            "end_line": end,
            "snippet": snippet.join("\n")
        })
    }

    pub fn search_symbols_with_limit(&self, query: &str, limit: usize) -> Vec<SampleNode> {
        let q = query.trim();
        if q.is_empty() {
            return Vec::new();
        }

        let q_lower = q.to_lowercase();
        let mut scored: Vec<(SearchScore, SampleNode)> = Vec::new();

        if let Some(node) = self.get_node(q) {
            scored.push((SearchScore(100), node));
        }

        let Ok(entries) = self.db.scan("nodes") else {
            return Self::dedupe_and_limit(scored, limit);
        };

        for (_key, val) in entries {
            let Ok(node) = serde_json::from_slice::<SampleNode>(&val) else {
                continue;
            };
            if scored.iter().any(|(_, n)| n.id == node.id) {
                continue;
            }
            let id_lower = node.id.to_lowercase();
            let name_lower = node.name.to_lowercase();
            let path_lower = node.file_path.to_lowercase();

            let score = if id_lower == q_lower {
                SearchScore(95)
            } else if id_lower.starts_with(&format!("{q_lower}@"))
                || id_lower.starts_with(&format!("{q_lower}:"))
            {
                SearchScore(90)
            } else if name_lower == q_lower {
                SearchScore(85)
            } else if id_lower.contains(&q_lower) {
                SearchScore(75)
            } else if name_lower.contains(&q_lower) {
                SearchScore(70)
            } else if path_lower.contains(&q_lower) {
                SearchScore(60)
            } else {
                continue;
            };
            scored.push((score, node));
        }

        Self::dedupe_and_limit(scored, limit)
    }

    fn dedupe_and_limit(mut scored: Vec<(SearchScore, SampleNode)>, limit: usize) -> Vec<SampleNode> {
        scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.id.cmp(&b.1.id)));
        scored.into_iter().take(limit).map(|(_, n)| n).collect()
    }

    pub(crate) fn resolve_seeds(&self, seed_ids: &[String]) -> Vec<String> {
        let mut resolved = Vec::new();
        let mut seen = HashSet::new();
        for seed in seed_ids {
            if seed.is_empty() {
                continue;
            }
            if let Some(node) = self.get_node(seed) {
                if seen.insert(node.id.clone()) {
                    resolved.push(node.id);
                }
                continue;
            }
            let seed_lower = seed.to_lowercase();
            let Ok(entries) = self.db.scan("nodes") else {
                continue;
            };
            for (_key, val) in entries {
                let Ok(node) = serde_json::from_slice::<SampleNode>(&val) else {
                    continue;
                };
                let id_lower = node.id.to_lowercase();
                let name_lower = node.name.to_lowercase();
                let matches = id_lower == seed_lower
                    || id_lower.starts_with(&format!("{seed_lower}@"))
                    || (name_lower == seed_lower && !seed.contains('@'));
                if matches && seen.insert(node.id.clone()) {
                    resolved.push(node.id);
                }
            }
        }
        resolved.sort();
        resolved
    }
}
