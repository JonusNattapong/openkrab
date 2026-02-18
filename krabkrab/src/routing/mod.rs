//! routing — Message routing and dispatch logic.
//! Ported from `openclaw/src/routing/` (Phase 5).
//!
//! Responsible for deciding which connector / channel should handle an
//! inbound message and where the reply should be delivered.

use serde::{Deserialize, Serialize};

// ─── Delivery target ──────────────────────────────────────────────────────────

/// The resolved destination for a reply message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeliveryTarget {
    /// Connector name (e.g. "telegram", "slack", "discord").
    pub connector: String,
    /// Opaque channel/chat identifier within the connector.
    pub to: String,
    /// Optional thread/reply context.
    pub thread_id: Option<String>,
    /// Optional account identifier (for multi-account connectors).
    pub account_id: Option<String>,
}

impl DeliveryTarget {
    pub fn new(connector: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            connector: connector.into(),
            to: to.into(),
            thread_id: None,
            account_id: None,
        }
    }

    pub fn with_thread(mut self, thread_id: impl Into<String>) -> Self {
        self.thread_id = Some(thread_id.into());
        self
    }

    pub fn with_account(mut self, account_id: impl Into<String>) -> Self {
        self.account_id = Some(account_id.into());
        self
    }
}

// ─── Route decision ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteDecision {
    /// Deliver reply to the specified target.
    Deliver(DeliveryTarget),
    /// Drop the message — do not reply.
    Drop { reason: String },
    /// Fall through to the next routing rule.
    Fallthrough,
}

// ─── Routing rule trait ───────────────────────────────────────────────────────

pub trait RoutingRule: Send + Sync {
    fn name(&self) -> &str;
    fn evaluate(&self, ctx: &RouteContext) -> RouteDecision;
}

// ─── Routing context ──────────────────────────────────────────────────────────

/// Context provided to routing rules when evaluating a message.
#[derive(Debug, Clone)]
pub struct RouteContext {
    pub connector: String,
    pub from: String,
    pub channel_id: Option<String>,
    pub chat_type: String,
    pub text: String,
}

impl RouteContext {
    pub fn new(connector: impl Into<String>, from: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            connector: connector.into(),
            from: from.into(),
            channel_id: None,
            chat_type: "direct".to_string(),
            text: text.into(),
        }
    }
}

// ─── Router ───────────────────────────────────────────────────────────────────

/// Evaluates a chain of routing rules and returns the first non-fallthrough decision.
pub struct Router {
    rules: Vec<Box<dyn RoutingRule>>,
    default_target: Option<DeliveryTarget>,
}

impl Router {
    pub fn new() -> Self {
        Self { rules: Vec::new(), default_target: None }
    }

    pub fn with_default(mut self, target: DeliveryTarget) -> Self {
        self.default_target = Some(target);
        self
    }

    pub fn add_rule(&mut self, rule: Box<dyn RoutingRule>) {
        self.rules.push(rule);
    }

    /// Evaluate all rules in order. Returns the first decisive result.
    pub fn route(&self, ctx: &RouteContext) -> RouteDecision {
        for rule in &self.rules {
            match rule.evaluate(ctx) {
                RouteDecision::Fallthrough => continue,
                decision => return decision,
            }
        }
        match &self.default_target {
            Some(target) => RouteDecision::Deliver(target.clone()),
            None => RouteDecision::Drop { reason: "no matching route".to_string() },
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Built-in rules ───────────────────────────────────────────────────────────

/// Route back to the connector / channel that sent the message (echo routing).
pub struct EchoRoutingRule;

impl RoutingRule for EchoRoutingRule {
    fn name(&self) -> &str {
        "echo"
    }

    fn evaluate(&self, ctx: &RouteContext) -> RouteDecision {
        let target = DeliveryTarget::new(ctx.connector.clone(), ctx.from.clone());
        RouteDecision::Deliver(target)
    }
}

/// Drop any message matching a keyword blocklist.
pub struct BlocklistRule {
    blocked_keywords: Vec<String>,
}

impl BlocklistRule {
    pub fn new(keywords: Vec<String>) -> Self {
        Self { blocked_keywords: keywords }
    }
}

impl RoutingRule for BlocklistRule {
    fn name(&self) -> &str {
        "blocklist"
    }

    fn evaluate(&self, ctx: &RouteContext) -> RouteDecision {
        let text_lower = ctx.text.to_lowercase();
        for kw in &self.blocked_keywords {
            if text_lower.contains(&kw.to_lowercase()) {
                return RouteDecision::Drop {
                    reason: format!("blocked keyword: {}", kw),
                };
            }
        }
        RouteDecision::Fallthrough
    }
}

/// Allow only specific senders.
pub struct AllowlistRoutingRule {
    allowed_senders: Vec<String>,
}

impl AllowlistRoutingRule {
    pub fn new(allowed: Vec<String>) -> Self {
        Self { allowed_senders: allowed }
    }
}

impl RoutingRule for AllowlistRoutingRule {
    fn name(&self) -> &str {
        "allowlist"
    }

    fn evaluate(&self, ctx: &RouteContext) -> RouteDecision {
        if self.allowed_senders.iter().any(|s| s == "*" || s == &ctx.from) {
            RouteDecision::Fallthrough
        } else {
            RouteDecision::Drop { reason: format!("sender {} not allowlisted", ctx.from) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(connector: &str, from: &str, text: &str) -> RouteContext {
        RouteContext::new(connector, from, text)
    }

    #[test]
    fn echo_routing() {
        let rule = EchoRoutingRule;
        let ctx = ctx("telegram", "user123", "hello");
        match rule.evaluate(&ctx) {
            RouteDecision::Deliver(target) => {
                assert_eq!(target.connector, "telegram");
                assert_eq!(target.to, "user123");
            }
            other => panic!("expected Deliver, got {:?}", other),
        }
    }

    #[test]
    fn blocklist_rule_drops() {
        let rule = BlocklistRule::new(vec!["spam".to_string()]);
        assert_eq!(
            rule.evaluate(&ctx("slack", "u", "this is spam")),
            RouteDecision::Drop { reason: "blocked keyword: spam".to_string() }
        );
        assert_eq!(rule.evaluate(&ctx("slack", "u", "hello")), RouteDecision::Fallthrough);
    }

    #[test]
    fn allowlist_rule_drops_unknown() {
        let rule = AllowlistRoutingRule::new(vec!["alice".to_string()]);
        assert_eq!(rule.evaluate(&ctx("telegram", "alice", "hi")), RouteDecision::Fallthrough);
        match rule.evaluate(&ctx("telegram", "bob", "hi")) {
            RouteDecision::Drop { .. } => {}
            other => panic!("expected Drop, got {:?}", other),
        }
    }

    #[test]
    fn router_chain() {
        let mut router = Router::new();
        router.add_rule(Box::new(BlocklistRule::new(vec!["bad".to_string()])));
        router.add_rule(Box::new(EchoRoutingRule));

        let good = ctx("telegram", "u1", "hello");
        match router.route(&good) {
            RouteDecision::Deliver(t) => assert_eq!(t.to, "u1"),
            other => panic!("unexpected: {:?}", other),
        }

        let bad = ctx("telegram", "u1", "this is bad content");
        match router.route(&bad) {
            RouteDecision::Drop { .. } => {}
            other => panic!("unexpected: {:?}", other),
        }
    }

    #[test]
    fn delivery_target_builder() {
        let t = DeliveryTarget::new("slack", "C123")
            .with_thread("T456")
            .with_account("acct1");
        assert_eq!(t.connector, "slack");
        assert_eq!(t.to, "C123");
        assert_eq!(t.thread_id, Some("T456".to_string()));
        assert_eq!(t.account_id, Some("acct1".to_string()));
    }
}
