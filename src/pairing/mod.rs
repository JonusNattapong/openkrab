//! pairing — Device pairing / multi-device account management.
//! Ported from `openclaw/src/pairing/` (Phase 7).
//!
//! Manages short-lived pairing tokens so users can link a new device
//! (e.g. a phone) to an existing krabkrab agent account.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Pairing token ────────────────────────────────────────────────────────────

/// A short-lived pairing token issued to allow a new device to join.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingToken {
    /// The opaque token string.
    pub token: String,
    /// Which account / user ID this token is for.
    pub account_id: String,
    /// When this token expires.
    pub expires_at: DateTime<Utc>,
    /// Whether this token has been redeemed.
    pub redeemed: bool,
    /// The device that redeemed this token (filled on redemption).
    pub redeemed_by: Option<String>,
    /// When it was redeemed.
    pub redeemed_at: Option<DateTime<Utc>>,
}

impl PairingToken {
    pub fn new(account_id: impl Into<String>, ttl_secs: i64) -> Self {
        Self {
            token: generate_token(),
            account_id: account_id.into(),
            expires_at: Utc::now() + Duration::seconds(ttl_secs),
            redeemed: false,
            redeemed_by: None,
            redeemed_at: None,
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.redeemed && Utc::now() < self.expires_at
    }

    pub fn redeem(&mut self, device_id: impl Into<String>) -> bool {
        if !self.is_valid() {
            return false;
        }
        self.redeemed = true;
        self.redeemed_by = Some(device_id.into());
        self.redeemed_at = Some(Utc::now());
        true
    }

    /// Remaining seconds until expiry (0 if expired).
    pub fn remaining_secs(&self) -> i64 {
        (self.expires_at - Utc::now()).num_seconds().max(0)
    }
}

fn generate_token() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    // 8-char uppercase hex
    format!("{:08X}", (t ^ (t >> 32)) as u32)
}

// ─── Pairing code (human-readable) ───────────────────────────────────────────

/// A short human-readable numeric code for pairing (like WhatsApp's 8-digit code).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingCode {
    pub code: String,
    pub account_id: String,
    pub expires_at: DateTime<Utc>,
}

impl PairingCode {
    pub fn new(account_id: impl Into<String>, ttl_secs: i64) -> Self {
        Self {
            code: format!(
                "{:08}",
                (Utc::now().timestamp_nanos_opt().unwrap_or(0) % 100_000_000).unsigned_abs()
            ),
            account_id: account_id.into(),
            expires_at: Utc::now() + Duration::seconds(ttl_secs),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }

    pub fn display(&self) -> String {
        // Format as "XXXX-XXXX"
        if self.code.len() == 8 {
            format!("{}-{}", &self.code[..4], &self.code[4..])
        } else {
            self.code.clone()
        }
    }
}

// ─── Pairing service ──────────────────────────────────────────────────────────

pub struct PairingService {
    tokens: HashMap<String, PairingToken>,
    codes: HashMap<String, PairingCode>,
    token_ttl_secs: i64,
    code_ttl_secs: i64,
}

impl PairingService {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
            codes: HashMap::new(),
            token_ttl_secs: 300, // 5 minutes
            code_ttl_secs: 120,  // 2 minutes
        }
    }

    pub fn with_ttl(mut self, token_secs: i64, code_secs: i64) -> Self {
        self.token_ttl_secs = token_secs;
        self.code_ttl_secs = code_secs;
        self
    }

    /// Issue a new pairing token for the given account.
    pub fn issue_token(&mut self, account_id: &str) -> &PairingToken {
        let tok = PairingToken::new(account_id, self.token_ttl_secs);
        let key = tok.token.clone();
        self.tokens.insert(key.clone(), tok);
        self.tokens.get(&key).unwrap()
    }

    /// Issue a new human-readable pairing code.
    pub fn issue_code(&mut self, account_id: &str) -> &PairingCode {
        let code = PairingCode::new(account_id, self.code_ttl_secs);
        let key = code.code.clone();
        self.codes.insert(key.clone(), code);
        self.codes.get(&key).unwrap()
    }

    /// Redeem a token by its string value.
    pub fn redeem_token(&mut self, token: &str, device_id: &str) -> RedeemResult {
        match self.tokens.get_mut(token) {
            None => RedeemResult::NotFound,
            Some(tok) => {
                if tok.redeemed {
                    RedeemResult::AlreadyUsed
                } else if !tok.is_valid() {
                    RedeemResult::Expired
                } else {
                    tok.redeem(device_id);
                    RedeemResult::Success {
                        account_id: tok.account_id.clone(),
                    }
                }
            }
        }
    }

    /// Purge expired tokens and codes.
    pub fn gc(&mut self) -> usize {
        let before = self.tokens.len() + self.codes.len();
        self.tokens.retain(|_, t| t.is_valid());
        self.codes.retain(|_, c| !c.is_expired());
        before - (self.tokens.len() + self.codes.len())
    }
}

impl Default for PairingService {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RedeemResult {
    Success { account_id: String },
    NotFound,
    Expired,
    AlreadyUsed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_is_valid_initially() {
        let tok = PairingToken::new("acct1", 60);
        assert!(tok.is_valid());
        assert!(tok.remaining_secs() > 0);
    }

    #[test]
    fn token_redeem_once() {
        let mut tok = PairingToken::new("acct1", 60);
        assert!(tok.redeem("device-A"));
        assert!(!tok.is_valid());
        assert!(!tok.redeem("device-B")); // second redeem fails
    }

    #[test]
    fn pairing_code_display() {
        let code = PairingCode {
            code: "12345678".to_string(),
            account_id: "u1".to_string(),
            expires_at: Utc::now() + Duration::seconds(60),
        };
        assert_eq!(code.display(), "1234-5678");
    }

    #[test]
    fn service_issue_and_redeem() {
        let mut svc = PairingService::new();
        let tok = svc.issue_token("user1").token.clone();
        match svc.redeem_token(&tok, "phone-1") {
            RedeemResult::Success { account_id } => assert_eq!(account_id, "user1"),
            other => panic!("unexpected: {:?}", other),
        }
        // Second redeem fails
        match svc.redeem_token(&tok, "phone-2") {
            RedeemResult::AlreadyUsed => {}
            other => panic!("unexpected: {:?}", other),
        }
    }

    #[test]
    fn service_unknown_token() {
        let mut svc = PairingService::new();
        assert_eq!(svc.redeem_token("BADTOKEN", "d"), RedeemResult::NotFound);
    }
}
