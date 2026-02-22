//! role_routing — Role-based routing rule.
//! Ported from OpenClaw's `resolve-route.ts` memberRoleIds support.
//!
//! Allows routing decisions to be made based on the sender's roles
//! (e.g. admin, moderator) in a group context.

use std::collections::{HashMap, HashSet};

use crate::routing::{DeliveryTarget, RouteContext, RouteDecision, RoutingRule};

// ─── Role-based routing rule ──────────────────────────────────────────────────

/// Routes messages based on the sender's role in the group.
pub struct RoleRoutingRule {
    /// Map from role ID → target agent/delivery.
    role_targets: HashMap<String, DeliveryTarget>,
    /// Optional: sets of roles that are allowed to reach the agent at all.
    allowed_roles: Option<HashSet<String>>,
    /// Optional: roles that should be blocked.
    blocked_roles: HashSet<String>,
}

impl RoleRoutingRule {
    /// Create from a role → target mapping.
    pub fn new(role_targets: HashMap<String, DeliveryTarget>) -> Self {
        Self {
            role_targets,
            allowed_roles: None,
            blocked_roles: HashSet::new(),
        }
    }

    /// Only allow these roles (everything else is dropped).
    pub fn with_allowed_roles(mut self, roles: HashSet<String>) -> Self {
        self.allowed_roles = Some(roles);
        self
    }

    /// Block these roles (they get dropped).
    pub fn with_blocked_roles(mut self, roles: HashSet<String>) -> Self {
        self.blocked_roles = roles;
        self
    }

    /// Check if any of the given roles match this rule's target criteria.
    pub fn matches(&self, roles: &[String]) -> bool {
        // Block check
        for role in roles {
            if self.blocked_roles.contains(role) {
                return false;
            }
        }

        // Allowlist check
        if let Some(ref allowed) = self.allowed_roles {
            if !roles.iter().any(|r| allowed.contains(r)) {
                return false;
            }
        }

        // Role-specific targeting (at least one role must have a target)
        roles.iter().any(|r| self.role_targets.contains_key(r))
    }
}

impl RoutingRule for RoleRoutingRule {
    fn name(&self) -> &str {
        "role-routing"
    }

    fn evaluate(&self, ctx: &RouteContext) -> RouteDecision {
        let roles = &ctx.member_role_ids;

        // Block check
        for role in roles {
            if self.blocked_roles.contains(role) {
                return RouteDecision::Drop {
                    reason: format!("role '{}' is blocked", role),
                };
            }
        }

        // Allowlist check
        if let Some(ref allowed) = self.allowed_roles {
            if !roles.iter().any(|r| allowed.contains(r)) {
                return RouteDecision::Drop {
                    reason: "sender has no allowed roles".to_string(),
                };
            }
        }

        // Role-specific targeting
        for role in roles {
            if let Some(target) = self.role_targets.get(role) {
                return RouteDecision::Deliver(target.clone());
            }
        }

        RouteDecision::Fallthrough
    }
}

// ─── Priority role routing ────────────────────────────────────────────────────

/// A role-based routing rule that evaluates roles by priority.
/// Higher priority roles are checked first.
pub struct PriorityRoleRoutingRule {
    /// (priority, role_id, target) sorted by priority desc.
    entries: Vec<(i32, String, DeliveryTarget)>,
}

impl PriorityRoleRoutingRule {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Add a role with a numeric priority (higher = evaluated first).
    pub fn add_role(
        mut self,
        priority: i32,
        role_id: impl Into<String>,
        target: DeliveryTarget,
    ) -> Self {
        self.entries.push((priority, role_id.into(), target));
        self.entries.sort_by(|a, b| b.0.cmp(&a.0));
        self
    }
}

impl Default for PriorityRoleRoutingRule {
    fn default() -> Self {
        Self::new()
    }
}

impl RoutingRule for PriorityRoleRoutingRule {
    fn name(&self) -> &str {
        "priority-role-routing"
    }

    fn evaluate(&self, ctx: &RouteContext) -> RouteDecision {
        let roles: HashSet<&str> = ctx.member_role_ids.iter().map(|s| s.as_str()).collect();

        for (_, role_id, target) in &self.entries {
            if roles.contains(role_id.as_str()) {
                return RouteDecision::Deliver(target.clone());
            }
        }

        RouteDecision::Fallthrough
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx_with_roles(roles: Vec<&str>) -> RouteContext {
        RouteContext {
            connector: "discord".to_string(),
            from: "user123".to_string(),
            channel_id: Some("general".to_string()),
            chat_type: "group".to_string(),
            text: "hello".to_string(),
            member_role_ids: roles.into_iter().map(|s| s.to_string()).collect(),
            thread_id: None,
            parent_peer: None,
        }
    }

    #[test]
    fn role_target_match() {
        let mut targets = HashMap::new();
        targets.insert(
            "admin".to_string(),
            DeliveryTarget::new("discord", "admin-channel"),
        );
        let rule = RoleRoutingRule::new(targets);
        let ctx = ctx_with_roles(vec!["member", "admin"]);

        match rule.evaluate(&ctx) {
            RouteDecision::Deliver(t) => assert_eq!(t.to, "admin-channel"),
            other => panic!("expected Deliver, got {:?}", other),
        }
    }

    #[test]
    fn role_no_match_fallthrough() {
        let targets = HashMap::new();
        let rule = RoleRoutingRule::new(targets);
        let ctx = ctx_with_roles(vec!["member"]);

        assert!(matches!(rule.evaluate(&ctx), RouteDecision::Fallthrough));
    }

    #[test]
    fn role_blocked() {
        let mut blocked = HashSet::new();
        blocked.insert("spammer".to_string());
        let rule = RoleRoutingRule::new(HashMap::new()).with_blocked_roles(blocked);
        let ctx = ctx_with_roles(vec!["spammer"]);

        match rule.evaluate(&ctx) {
            RouteDecision::Drop { reason } => assert!(reason.contains("spammer")),
            other => panic!("expected Drop, got {:?}", other),
        }
    }

    #[test]
    fn role_allowlist() {
        let mut allowed = HashSet::new();
        allowed.insert("vip".to_string());
        let rule = RoleRoutingRule::new(HashMap::new()).with_allowed_roles(allowed);

        let allowed_ctx = ctx_with_roles(vec!["vip"]);
        assert!(matches!(
            rule.evaluate(&allowed_ctx),
            RouteDecision::Fallthrough
        ));

        let denied_ctx = ctx_with_roles(vec!["member"]);
        assert!(matches!(
            rule.evaluate(&denied_ctx),
            RouteDecision::Drop { .. }
        ));
    }

    #[test]
    fn priority_role_routing() {
        let rule = PriorityRoleRoutingRule::new()
            .add_role(10, "owner", DeliveryTarget::new("discord", "owner-channel"))
            .add_role(5, "admin", DeliveryTarget::new("discord", "admin-channel"))
            .add_role(
                1,
                "member",
                DeliveryTarget::new("discord", "member-channel"),
            );

        // User has both admin and member — should match admin (higher priority)
        let ctx = ctx_with_roles(vec!["member", "admin"]);
        match rule.evaluate(&ctx) {
            RouteDecision::Deliver(t) => assert_eq!(t.to, "admin-channel"),
            other => panic!("expected Deliver, got {:?}", other),
        }

        // User has owner — should match owner (highest)
        let ctx = ctx_with_roles(vec!["member", "owner"]);
        match rule.evaluate(&ctx) {
            RouteDecision::Deliver(t) => assert_eq!(t.to, "owner-channel"),
            other => panic!("expected Deliver, got {:?}", other),
        }
    }
}
