//! broadcast — Broadcast group management.
//! Ported from `openkrab/src/web/auto-reply.broadcast-groups.*` (Phase 10).
//!
//! Allows the agent to fan-out a single message to multiple recipient groups
//! across multiple connectors in a configurable order.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Broadcast group ──────────────────────────────────────────────────────────

/// A named group of recipients that receive broadcast messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastGroup {
    pub id: String,
    pub name: String,
    /// Connector identifier (e.g. "telegram", "slack", "whatsapp").
    pub connector: String,
    /// List of recipient IDs for this connector.
    pub recipients: Vec<String>,
    /// Whether this group is active.
    pub enabled: bool,
    /// Optional label shown in status output.
    pub label: Option<String>,
}

impl BroadcastGroup {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        connector: impl Into<String>,
        recipients: Vec<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            connector: connector.into(),
            recipients,
            enabled: true,
            label: None,
        }
    }

    pub fn recipient_count(&self) -> usize {
        self.recipients.len()
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
}

// ─── Broadcast message ────────────────────────────────────────────────────────

/// A message to be broadcast to one or more groups.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastMessage {
    pub text: String,
    /// If set, only broadcast to these group IDs.
    pub target_groups: Option<Vec<String>>,
    /// Optional media URL to include.
    pub media_url: Option<String>,
    /// Whether to continue after a group fails.
    pub continue_on_error: bool,
}

impl BroadcastMessage {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            target_groups: None,
            media_url: None,
            continue_on_error: true,
        }
    }

    pub fn to_groups(mut self, groups: Vec<String>) -> Self {
        self.target_groups = Some(groups);
        self
    }
}

// ─── Broadcast result ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastDelivery {
    pub group_id: String,
    pub group_name: String,
    pub connector: String,
    pub recipient: String,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastResult {
    pub deliveries: Vec<BroadcastDelivery>,
    pub total: usize,
    pub succeeded: usize,
    pub failed: usize,
}

impl BroadcastResult {
    pub fn new() -> Self {
        Self {
            deliveries: Vec::new(),
            total: 0,
            succeeded: 0,
            failed: 0,
        }
    }

    pub fn add(&mut self, delivery: BroadcastDelivery) {
        self.total += 1;
        if delivery.success {
            self.succeeded += 1;
        } else {
            self.failed += 1;
        }
        self.deliveries.push(delivery);
    }

    pub fn all_ok(&self) -> bool {
        self.failed == 0
    }

    pub fn summary(&self) -> String {
        format!(
            "Broadcast: {}/{} delivered successfully{}",
            self.succeeded,
            self.total,
            if self.failed > 0 {
                format!(", {} failed", self.failed)
            } else {
                String::new()
            }
        )
    }
}

impl Default for BroadcastResult {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Broadcast registry ───────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct BroadcastRegistry {
    groups: HashMap<String, BroadcastGroup>,
    order: Vec<String>,
}

impl BroadcastRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a broadcast group. Groups are dispatched in registration order.
    pub fn register(&mut self, group: BroadcastGroup) {
        let id = group.id.clone();
        if !self.order.contains(&id) {
            self.order.push(id.clone());
        }
        self.groups.insert(id, group);
    }

    pub fn get(&self, id: &str) -> Option<&BroadcastGroup> {
        self.groups.get(id)
    }

    pub fn remove(&mut self, id: &str) -> bool {
        self.order.retain(|i| i != id);
        self.groups.remove(id).is_some()
    }

    /// Returns enabled groups in registration order, filtered by target_groups if set.
    pub fn resolve_targets<'a>(&'a self, msg: &'a BroadcastMessage) -> Vec<&'a BroadcastGroup> {
        self.order
            .iter()
            .filter_map(|id| self.groups.get(id))
            .filter(|g| g.enabled)
            .filter(|g| {
                msg.target_groups
                    .as_ref()
                    .map(|targets| targets.contains(&g.id))
                    .unwrap_or(true)
            })
            .collect()
    }

    pub fn len(&self) -> usize {
        self.groups.len()
    }
    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    /// Simulate a broadcast (for testing — does not actually send).
    pub fn simulate(&self, msg: &BroadcastMessage) -> BroadcastResult {
        let mut result = BroadcastResult::new();
        for group in self.resolve_targets(msg) {
            for recipient in &group.recipients {
                result.add(BroadcastDelivery {
                    group_id: group.id.clone(),
                    group_name: group.name.clone(),
                    connector: group.connector.clone(),
                    recipient: recipient.clone(),
                    success: true,
                    error: None,
                });
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_group(id: &str, connector: &str, recipients: &[&str]) -> BroadcastGroup {
        BroadcastGroup::new(
            id,
            id,
            connector,
            recipients.iter().map(|s| s.to_string()).collect(),
        )
    }

    #[test]
    fn group_recipient_count() {
        let g = make_group("g1", "telegram", &["@a", "@b", "@c"]);
        assert_eq!(g.recipient_count(), 3);
    }

    #[test]
    fn registry_order_preserved() {
        let mut reg = BroadcastRegistry::new();
        reg.register(make_group("g1", "telegram", &["@a"]));
        reg.register(make_group("g2", "slack", &["#ch"]));
        let msg = BroadcastMessage::new("hi");
        let targets = reg.resolve_targets(&msg);
        assert_eq!(targets[0].id, "g1");
        assert_eq!(targets[1].id, "g2");
    }

    #[test]
    fn registry_disabled_group_excluded() {
        let mut reg = BroadcastRegistry::new();
        reg.register(make_group("g1", "telegram", &["@a"]).disable());
        reg.register(make_group("g2", "slack", &["#ch"]));
        let msg = BroadcastMessage::new("hi");
        let targets = reg.resolve_targets(&msg);
        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0].id, "g2");
    }

    #[test]
    fn registry_target_filter() {
        let mut reg = BroadcastRegistry::new();
        reg.register(make_group("g1", "telegram", &["@a"]));
        reg.register(make_group("g2", "slack", &["#ch"]));
        let msg = BroadcastMessage::new("hi").to_groups(vec!["g2".into()]);
        let targets = reg.resolve_targets(&msg);
        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0].id, "g2");
    }

    #[test]
    fn simulate_broadcast() {
        let mut reg = BroadcastRegistry::new();
        reg.register(make_group("g1", "telegram", &["@a", "@b"]));
        let result = reg.simulate(&BroadcastMessage::new("hello"));
        assert_eq!(result.total, 2);
        assert!(result.all_ok());
        assert!(result.summary().contains("2/2"));
    }

    #[test]
    fn broadcast_result_summary_with_failures() {
        let mut r = BroadcastResult::new();
        r.add(BroadcastDelivery {
            group_id: "g1".into(),
            group_name: "G1".into(),
            connector: "tg".into(),
            recipient: "@a".into(),
            success: true,
            error: None,
        });
        r.add(BroadcastDelivery {
            group_id: "g1".into(),
            group_name: "G1".into(),
            connector: "tg".into(),
            recipient: "@b".into(),
            success: false,
            error: Some("timeout".into()),
        });
        assert!(!r.all_ok());
        assert!(r.summary().contains("failed"));
    }
}
