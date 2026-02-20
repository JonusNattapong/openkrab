//! Port of `openkrab/src/shared/node-list-types.ts` + `node-list-parse.ts`
//!
//! Types and parsers for the node (device) list used by the pairing protocol.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ─── Types (from node-list-types.ts) ─────────────────────────────────────────

/// A node/device in the network.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeListNode {
    pub node_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_env: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caps: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<HashMap<String, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paired: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_at_ms: Option<u64>,
}

/// A pending pairing request.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PendingRequest {
    pub request_id: String,
    pub node_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_repair: Option<bool>,
    pub ts: u64,
}

/// An approved/paired node.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PairedNode {
    pub node_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<HashMap<String, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_at_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_connected_at_ms: Option<u64>,
}

/// A pairing list containing pending requests and paired nodes.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PairingList {
    pub pending: Vec<PendingRequest>,
    pub paired: Vec<PairedNode>,
}

// ─── Parsers (from node-list-parse.ts) ──────────────────────────────────────

fn as_record(value: &Value) -> Option<&serde_json::Map<String, Value>> {
    value.as_object()
}

/// Parse a [`PairingList`] from a loose JSON value.
///
/// Expects `{ "pending": [...], "paired": [...] }`.
pub fn parse_pairing_list(value: &Value) -> PairingList {
    let obj = match as_record(value) {
        Some(o) => o,
        None => return PairingList::default(),
    };

    let pending = obj
        .get("pending")
        .and_then(|v| serde_json::from_value::<Vec<PendingRequest>>(v.clone()).ok())
        .unwrap_or_default();

    let paired = obj
        .get("paired")
        .and_then(|v| serde_json::from_value::<Vec<PairedNode>>(v.clone()).ok())
        .unwrap_or_default();

    PairingList { pending, paired }
}

/// Parse a node list from a loose JSON value.
///
/// Expects `{ "nodes": [...] }`.
pub fn parse_node_list(value: &Value) -> Vec<NodeListNode> {
    let obj = match as_record(value) {
        Some(o) => o,
        None => return Vec::new(),
    };

    obj.get("nodes")
        .and_then(|v| serde_json::from_value::<Vec<NodeListNode>>(v.clone()).ok())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_empty_pairing_list() {
        let list = parse_pairing_list(&json!({}));
        assert!(list.pending.is_empty());
        assert!(list.paired.is_empty());
    }

    #[test]
    fn parse_pairing_list_with_data() {
        let data = json!({
            "pending": [
                {"requestId": "r1", "nodeId": "n1", "ts": 1000}
            ],
            "paired": [
                {"nodeId": "n2", "token": "t2"}
            ]
        });
        let list = parse_pairing_list(&data);
        assert_eq!(list.pending.len(), 1);
        assert_eq!(list.pending[0].request_id, "r1");
        assert_eq!(list.paired.len(), 1);
        assert_eq!(list.paired[0].node_id, "n2");
    }

    #[test]
    fn parse_node_list_empty() {
        let nodes = parse_node_list(&json!({}));
        assert!(nodes.is_empty());
    }

    #[test]
    fn parse_node_list_with_nodes() {
        let data = json!({
            "nodes": [
                {"nodeId": "abc", "displayName": "My Device", "paired": true}
            ]
        });
        let nodes = parse_node_list(&data);
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].node_id, "abc");
        assert_eq!(nodes[0].display_name.as_deref(), Some("My Device"));
    }

    #[test]
    fn parse_from_non_object_returns_empty() {
        let nodes = parse_node_list(&json!("not an object"));
        assert!(nodes.is_empty());
        let list = parse_pairing_list(&json!(42));
        assert!(list.pending.is_empty());
    }

    #[test]
    fn node_roundtrip() {
        let node = NodeListNode {
            node_id: "n1".into(),
            display_name: Some("Test".into()),
            platform: Some("linux".into()),
            ..Default::default()
        };
        let json = serde_json::to_value(&node).unwrap();
        let back: NodeListNode = serde_json::from_value(json).unwrap();
        assert_eq!(back.node_id, "n1");
        assert_eq!(back.display_name.as_deref(), Some("Test"));
    }
}
