//! web_connector — Multi-platform web-based connector core.
//! Ported from `openclaw/src/web/` (Phase 9).
//!
//! Provides inbound/outbound message normalisation, session management,
//! account handling, and reconnection logic for web-socket based connectors
//! (WhatsApp Web, WeChat Web, etc.).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Account ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAccount {
    pub id: String,
    pub platform: String,
    pub phone_number: Option<String>,
    pub display_name: Option<String>,
    pub status: AccountStatus,
    pub session_data: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountStatus {
    Disconnected,
    Connecting,
    Connected,
    LoggedOut,
    Error,
}

impl WebAccount {
    pub fn new(id: impl Into<String>, platform: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            platform: platform.into(),
            phone_number: None,
            display_name: None,
            status: AccountStatus::Disconnected,
            session_data: None,
        }
    }

    pub fn is_connected(&self) -> bool {
        self.status == AccountStatus::Connected
    }
}

// ─── Inbound message ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundWebMessage {
    pub id: String,
    pub platform: String,
    pub account_id: String,
    pub from: String,
    pub chat_id: String,
    pub body: String,
    pub media_url: Option<String>,
    pub media_mime: Option<String>,
    pub is_group: bool,
    pub timestamp: i64,
    pub quoted_id: Option<String>,
}

impl InboundWebMessage {
    pub fn new(
        platform: impl Into<String>,
        account_id: impl Into<String>,
        from: impl Into<String>,
        chat_id: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            id: new_id(),
            platform: platform.into(),
            account_id: account_id.into(),
            from: from.into(),
            chat_id: chat_id.into(),
            body: body.into(),
            media_url: None,
            media_mime: None,
            is_group: false,
            timestamp: unix_now(),
            quoted_id: None,
        }
    }

    pub fn has_media(&self) -> bool {
        self.media_url.is_some()
    }
}

// ─── Outbound message ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboundWebMessage {
    pub account_id: String,
    pub to: String,
    pub body: String,
    pub media_url: Option<String>,
    pub media_caption: Option<String>,
    pub reply_to_id: Option<String>,
    pub platform: String,
}

impl OutboundWebMessage {
    pub fn text(account_id: impl Into<String>, to: impl Into<String>, body: impl Into<String>, platform: impl Into<String>) -> Self {
        Self {
            account_id: account_id.into(),
            to: to.into(),
            body: body.into(),
            media_url: None,
            media_caption: None,
            reply_to_id: None,
            platform: platform.into(),
        }
    }

    pub fn reply_to(mut self, msg_id: impl Into<String>) -> Self {
        self.reply_to_id = Some(msg_id.into());
        self
    }
}

// ─── Session ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSession {
    pub account_id: String,
    pub platform: String,
    pub ws_url: Option<String>,
    pub reconnect_count: u32,
    pub last_seen: i64,
}

impl WebSession {
    pub fn new(account_id: impl Into<String>, platform: impl Into<String>) -> Self {
        Self {
            account_id: account_id.into(),
            platform: platform.into(),
            ws_url: None,
            reconnect_count: 0,
            last_seen: unix_now(),
        }
    }

    pub fn record_reconnect(&mut self) {
        self.reconnect_count += 1;
        self.last_seen = unix_now();
    }
}

// ─── Reconnect strategy ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ReconnectConfig {
    pub max_attempts: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_factor: f64,
}

impl Default for ReconnectConfig {
    fn default() -> Self {
        Self {
            max_attempts: 10,
            initial_delay_ms: 1_000,
            max_delay_ms: 60_000,
            backoff_factor: 2.0,
        }
    }
}

impl ReconnectConfig {
    pub fn delay_for_attempt(&self, attempt: u32) -> u64 {
        if attempt == 0 { return self.initial_delay_ms; }
        let d = self.initial_delay_ms as f64 * self.backoff_factor.powi(attempt as i32);
        d.min(self.max_delay_ms as f64) as u64
    }

    pub fn should_retry(&self, attempt: u32) -> bool {
        attempt < self.max_attempts
    }
}

// ─── Account store ────────────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct AccountStore {
    accounts: HashMap<String, WebAccount>,
}

impl AccountStore {
    pub fn new() -> Self { Self::default() }

    pub fn upsert(&mut self, account: WebAccount) {
        self.accounts.insert(account.id.clone(), account);
    }

    pub fn get(&self, id: &str) -> Option<&WebAccount> {
        self.accounts.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut WebAccount> {
        self.accounts.get_mut(id)
    }

    pub fn connected(&self) -> Vec<&WebAccount> {
        self.accounts.values().filter(|a| a.is_connected()).collect()
    }

    pub fn list(&self) -> Vec<&WebAccount> {
        self.accounts.values().collect()
    }

    pub fn remove(&mut self, id: &str) -> bool {
        self.accounts.remove(id).is_some()
    }
}

// ─── QR login stub ────────────────────────────────────────────────────────────

/// A QR-code login token (placeholder for WhatsApp Web / WeChat QR flow).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrLoginToken {
    pub token: String,
    pub expires_at: i64,
    pub platform: String,
}

impl QrLoginToken {
    pub fn new(platform: impl Into<String>, ttl_secs: i64) -> Self {
        Self {
            token: new_id(),
            platform: platform.into(),
            expires_at: unix_now() + ttl_secs,
        }
    }

    pub fn is_expired(&self) -> bool {
        unix_now() >= self.expires_at
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn unix_now() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() as i64
}

fn new_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
    format!("{:X}", t as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn web_account_connected() {
        let mut a = WebAccount::new("acc-1", "whatsapp");
        assert!(!a.is_connected());
        a.status = AccountStatus::Connected;
        assert!(a.is_connected());
    }

    #[test]
    fn inbound_message_has_media() {
        let mut m = InboundWebMessage::new("whatsapp", "acc-1", "+66811234567", "chat-1", "hello");
        assert!(!m.has_media());
        m.media_url = Some("https://example.com/img.jpg".to_string());
        assert!(m.has_media());
    }

    #[test]
    fn outbound_message_reply() {
        let m = OutboundWebMessage::text("acc-1", "+66811234567", "hi", "whatsapp")
            .reply_to("msg-42");
        assert_eq!(m.reply_to_id.as_deref(), Some("msg-42"));
    }

    #[test]
    fn reconnect_config_backoff() {
        let cfg = ReconnectConfig::default();
        assert_eq!(cfg.delay_for_attempt(0), 1_000);
        assert!(cfg.delay_for_attempt(1) > 1_000);
        assert!(cfg.delay_for_attempt(100) <= 60_000);
        assert!(cfg.should_retry(5));
        assert!(!cfg.should_retry(10));
    }

    #[test]
    fn account_store_upsert_get() {
        let mut store = AccountStore::new();
        store.upsert(WebAccount::new("acc-1", "whatsapp"));
        assert!(store.get("acc-1").is_some());
        assert!(store.get("acc-x").is_none());
        store.remove("acc-1");
        assert!(store.get("acc-1").is_none());
    }

    #[test]
    fn qr_login_token_not_expired() {
        let tok = QrLoginToken::new("whatsapp", 60);
        assert!(!tok.is_expired());
    }

    #[test]
    fn session_reconnect_count() {
        let mut s = WebSession::new("acc-1", "whatsapp");
        assert_eq!(s.reconnect_count, 0);
        s.record_reconnect();
        s.record_reconnect();
        assert_eq!(s.reconnect_count, 2);
    }
}
