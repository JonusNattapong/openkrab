//! Port of `openkrab/src/shared/node-match.ts`
//!
//! Node matching utilities for the pairing / device management layer.
//! Given a query string, find matching nodes by ID, display name, or IP.

use regex::Regex;

/// Minimal trait for a node that can be matched.
#[derive(Debug, Clone)]
pub struct NodeMatchCandidate {
    pub node_id: String,
    pub display_name: Option<String>,
    pub remote_ip: Option<String>,
}

/// Normalize a node key for comparison: lowercase, replace non-alphanumeric
/// runs with hyphens, strip leading/trailing hyphens.
pub fn normalize_node_key(value: &str) -> String {
    lazy_static::lazy_static! {
        static ref NON_ALNUM: Regex = Regex::new(r"[^a-z0-9]+").unwrap();
    }
    let lower = value.to_lowercase();
    let replaced = NON_ALNUM.replace_all(&lower, "-");
    replaced.trim_matches('-').to_string()
}

/// List known nodes as a comma-separated display string.
fn list_known_nodes(nodes: &[NodeMatchCandidate]) -> String {
    nodes
        .iter()
        .filter_map(|n| {
            n.display_name
                .as_deref()
                .or(n.remote_ip.as_deref())
                .or(Some(&n.node_id))
                .filter(|s| !s.is_empty())
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// Find all nodes matching a query string.
///
/// Matching rules (any match counts):
/// 1. Exact `node_id` match
/// 2. Exact `remote_ip` match
/// 3. Normalized `display_name` equals normalized query
/// 4. `node_id` starts with the query (if query length ≥ 6, prefix match)
pub fn resolve_node_matches<'a>(
    nodes: &'a [NodeMatchCandidate],
    query: &str,
) -> Vec<&'a NodeMatchCandidate> {
    let q = query.trim();
    if q.is_empty() {
        return Vec::new();
    }

    let q_norm = normalize_node_key(q);
    nodes
        .iter()
        .filter(|n| {
            // Exact node_id
            if n.node_id == q {
                return true;
            }
            // Exact remote_ip
            if let Some(ref ip) = n.remote_ip {
                if ip == q {
                    return true;
                }
            }
            // Normalized display_name
            if let Some(ref name) = n.display_name {
                if !name.is_empty() && normalize_node_key(name) == q_norm {
                    return true;
                }
            }
            // Prefix match on node_id (≥ 6 chars)
            if q.len() >= 6 && n.node_id.starts_with(q) {
                return true;
            }
            false
        })
        .collect()
}

/// Resolve a query to exactly one node ID.
///
/// Returns `Ok(node_id)` if exactly one node matches, or `Err` with a
/// descriptive message if zero or multiple nodes match.
pub fn resolve_node_id_from_candidates(
    nodes: &[NodeMatchCandidate],
    query: &str,
) -> Result<String, String> {
    let q = query.trim();
    if q.is_empty() {
        return Err("node required".to_string());
    }

    let matches = resolve_node_matches(nodes, q);
    match matches.len() {
        1 => Ok(matches[0].node_id.clone()),
        0 => {
            let known = list_known_nodes(nodes);
            if known.is_empty() {
                Err(format!("unknown node: {}", q))
            } else {
                Err(format!("unknown node: {} (known: {})", q, known))
            }
        }
        _ => {
            let names: Vec<&str> = matches
                .iter()
                .map(|n| {
                    n.display_name
                        .as_deref()
                        .or(n.remote_ip.as_deref())
                        .unwrap_or(&n.node_id)
                })
                .collect();
            Err(format!(
                "ambiguous node: {} (matches: {})",
                q,
                names.join(", ")
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_nodes() -> Vec<NodeMatchCandidate> {
        vec![
            NodeMatchCandidate {
                node_id: "abc123def456".to_string(),
                display_name: Some("My Laptop".to_string()),
                remote_ip: Some("192.168.1.10".to_string()),
            },
            NodeMatchCandidate {
                node_id: "xyz789ghi012".to_string(),
                display_name: Some("Phone".to_string()),
                remote_ip: None,
            },
        ]
    }

    #[test]
    fn normalize_key() {
        assert_eq!(normalize_node_key("Hello World!"), "hello-world");
        assert_eq!(normalize_node_key("---test---"), "test");
    }

    #[test]
    fn match_by_node_id() {
        let nodes = make_nodes();
        let matches = resolve_node_matches(&nodes, "abc123def456");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].node_id, "abc123def456");
    }

    #[test]
    fn match_by_ip() {
        let nodes = make_nodes();
        let matches = resolve_node_matches(&nodes, "192.168.1.10");
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn match_by_display_name() {
        let nodes = make_nodes();
        let matches = resolve_node_matches(&nodes, "my laptop");
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn prefix_match() {
        let nodes = make_nodes();
        let matches = resolve_node_matches(&nodes, "abc123");
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn prefix_too_short() {
        let nodes = make_nodes();
        let matches = resolve_node_matches(&nodes, "abc");
        assert!(matches.is_empty());
    }

    #[test]
    fn empty_query_returns_empty() {
        let nodes = make_nodes();
        assert!(resolve_node_matches(&nodes, "").is_empty());
    }

    #[test]
    fn resolve_single() {
        let nodes = make_nodes();
        assert_eq!(
            resolve_node_id_from_candidates(&nodes, "Phone").unwrap(),
            "xyz789ghi012"
        );
    }

    #[test]
    fn resolve_unknown() {
        let nodes = make_nodes();
        let err = resolve_node_id_from_candidates(&nodes, "nope").unwrap_err();
        assert!(err.contains("unknown node"));
    }

    #[test]
    fn resolve_empty_query() {
        let nodes = make_nodes();
        let err = resolve_node_id_from_candidates(&nodes, "").unwrap_err();
        assert_eq!(err, "node required");
    }
}
