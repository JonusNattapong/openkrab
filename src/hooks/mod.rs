//! hooks — Plugin / hook registry.
//! Ported from `openclaw/src/hooks/` (Phase 6).
//!
//! Provides a lightweight synchronous hook system that allows modules to
//! register callbacks for named lifecycle events without tight coupling.

use std::collections::HashMap;

// ─── Hook event ───────────────────────────────────────────────────────────────

/// Well-known lifecycle event names (mirrors openclaw hook events).
pub mod events {
    pub const MESSAGE_INBOUND: &str = "message:inbound";
    pub const MESSAGE_OUTBOUND: &str = "message:outbound";
    pub const AGENT_START: &str = "agent:start";
    pub const AGENT_COMPLETE: &str = "agent:complete";
    pub const AGENT_ERROR: &str = "agent:error";
    pub const SESSION_CREATED: &str = "session:created";
    pub const SESSION_CLOSED: &str = "session:closed";
    pub const MEMORY_INDEXED: &str = "memory:indexed";
    pub const CRON_FIRED: &str = "cron:fired";
    pub const CONNECTOR_CONNECTED: &str = "connector:connected";
    pub const CONNECTOR_DISCONNECTED: &str = "connector:disconnected";
}

// ─── Hook payload ─────────────────────────────────────────────────────────────

/// Payload passed to hook callbacks — a simple key-value bag.
#[derive(Debug, Clone, Default)]
pub struct HookPayload {
    pub data: HashMap<String, serde_json::Value>,
}

impl HookPayload {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) {
        self.data.insert(key.into(), value.into());
    }

    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.data.get(key)?.as_str()
    }

    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.data.get(key)?.as_i64()
    }
}

// ─── Hook callback ────────────────────────────────────────────────────────────

pub type HookFn = Box<dyn Fn(&HookPayload) + Send + Sync + 'static>;

// ─── Hook registry ────────────────────────────────────────────────────────────

/// Registry mapping event names to a list of callbacks.
pub struct HookRegistry {
    hooks: HashMap<String, Vec<HookFn>>,
}

impl HookRegistry {
    pub fn new() -> Self {
        Self {
            hooks: HashMap::new(),
        }
    }

    /// Register a callback for the given event name.
    pub fn on(&mut self, event: impl Into<String>, cb: HookFn) {
        self.hooks.entry(event.into()).or_default().push(cb);
    }

    /// Fire all callbacks registered for the given event.
    pub fn emit(&self, event: &str, payload: &HookPayload) {
        if let Some(cbs) = self.hooks.get(event) {
            for cb in cbs {
                cb(payload);
            }
        }
    }

    /// Number of registered listeners for an event.
    pub fn listener_count(&self, event: &str) -> usize {
        self.hooks.get(event).map(|v| v.len()).unwrap_or(0)
    }

    /// Remove all listeners for an event.
    pub fn clear(&mut self, event: &str) {
        self.hooks.remove(event);
    }
}

impl Default for HookRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Internal hooks (built-in) ────────────────────────────────────────────────

/// Built-in hook that logs every emitted event to stdout (for debugging).
pub fn debug_logger_hook(event: &str) -> HookFn {
    let event = event.to_string();
    Box::new(move |payload| {
        println!(
            "[hook:{}] payload keys: {:?}",
            event,
            payload.data.keys().collect::<Vec<_>>()
        );
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn emit_fires_callbacks() {
        let mut reg = HookRegistry::new();
        let fired = Arc::new(Mutex::new(false));
        let fired2 = fired.clone();

        reg.on(
            events::AGENT_COMPLETE,
            Box::new(move |_payload| {
                *fired2.lock().unwrap() = true;
            }),
        );

        let mut p = HookPayload::new();
        p.set("response", serde_json::Value::String("ok".to_string()));

        reg.emit(events::AGENT_COMPLETE, &p);
        assert!(*fired.lock().unwrap());
    }

    #[test]
    fn listener_count() {
        let mut reg = HookRegistry::new();
        assert_eq!(reg.listener_count(events::MESSAGE_INBOUND), 0);
        reg.on(events::MESSAGE_INBOUND, Box::new(|_| {}));
        reg.on(events::MESSAGE_INBOUND, Box::new(|_| {}));
        assert_eq!(reg.listener_count(events::MESSAGE_INBOUND), 2);
        reg.clear(events::MESSAGE_INBOUND);
        assert_eq!(reg.listener_count(events::MESSAGE_INBOUND), 0);
    }

    #[test]
    fn payload_accessors() {
        let mut p = HookPayload::new();
        p.set("name", serde_json::Value::String("alice".to_string()));
        p.set("count", serde_json::Value::Number(42.into()));
        assert_eq!(p.get_str("name"), Some("alice"));
        assert_eq!(p.get_i64("count"), Some(42));
        assert_eq!(p.get_str("missing"), None);
    }

    #[test]
    fn no_panic_on_empty_event() {
        let reg = HookRegistry::new();
        let p = HookPayload::new();
        reg.emit("unknown:event", &p); // should not panic
    }
}
