//! sessions — Session state management.
//! Ported from `openkrab/src/sessions/` (Phase 6).
//!
//! Tracks per-conversation state including model overrides, verbosity level
//! overrides, send policy, and a basic session transcript.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Verbosity level ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum VerbosityLevel {
    /// Minimal output (errors + key results only).
    Quiet,
    /// Standard assistant replies.
    #[default]
    Normal,
    /// Detailed reasoning + tool call info.
    Verbose,
    /// Everything including internal chain-of-thought.
    Debug,
}

impl VerbosityLevel {
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "quiet" | "q" => VerbosityLevel::Quiet,
            "verbose" | "v" => VerbosityLevel::Verbose,
            "debug" | "d" => VerbosityLevel::Debug,
            _ => VerbosityLevel::Normal,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            VerbosityLevel::Quiet => "quiet",
            VerbosityLevel::Normal => "normal",
            VerbosityLevel::Verbose => "verbose",
            VerbosityLevel::Debug => "debug",
        }
    }
}

// ─── Send policy ──────────────────────────────────────────────────────────────

/// Controls when/how replies are sent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SendPolicy {
    /// Send reply as soon as it is generated.
    #[default]
    Immediate,
    /// Collect all tokens, send as single message at end.
    Batch,
    /// Stream tokens as typing indicators.
    Stream,
}

// ─── Transcript entry ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptEntry {
    pub role: String,
    pub text: String,
    pub timestamp: DateTime<Utc>,
}

impl TranscriptEntry {
    pub fn user(text: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            text: text.into(),
            timestamp: Utc::now(),
        }
    }
    pub fn assistant(text: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            text: text.into(),
            timestamp: Utc::now(),
        }
    }
    pub fn system(text: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            text: text.into(),
            timestamp: Utc::now(),
        }
    }
}

// ─── Session ──────────────────────────────────────────────────────────────────

/// Per-conversation session state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Unique session ID (usually connector + chat_id).
    pub id: String,
    /// Session label for display (e.g. "@alice on telegram").
    pub label: Option<String>,
    /// Override the model for this session (None = use global default).
    pub model_override: Option<String>,
    /// Override the verbosity for this session.
    pub verbosity: VerbosityLevel,
    /// Send policy override.
    pub send_policy: SendPolicy,
    /// Whether elevated (admin) mode is active.
    pub elevated: bool,
    /// Conversation transcript (last N entries).
    pub transcript: Vec<TranscriptEntry>,
    /// Maximum transcript length to keep in memory.
    pub max_transcript: usize,
    /// When the session was created.
    pub created_at: DateTime<Utc>,
    /// When the session was last active.
    pub last_active: DateTime<Utc>,
    /// Arbitrary key-value metadata.
    pub metadata: HashMap<String, String>,
}

impl Session {
    pub fn new(id: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: id.into(),
            label: None,
            model_override: None,
            verbosity: VerbosityLevel::Normal,
            send_policy: SendPolicy::Immediate,
            elevated: false,
            transcript: Vec::new(),
            max_transcript: 50,
            created_at: now,
            last_active: now,
            metadata: HashMap::new(),
        }
    }

    /// Append a transcript entry and trim to `max_transcript`.
    pub fn append_transcript(&mut self, entry: TranscriptEntry) {
        self.transcript.push(entry);
        self.last_active = Utc::now();
        if self.transcript.len() > self.max_transcript {
            let drain_count = self.transcript.len() - self.max_transcript;
            self.transcript.drain(..drain_count);
        }
    }

    /// Get the effective model (override or None).
    pub fn effective_model(&self) -> Option<&str> {
        self.model_override.as_deref()
    }

    /// Recent messages formatted as (role, text) pairs.
    pub fn recent_messages(&self, n: usize) -> Vec<(&str, &str)> {
        self.transcript
            .iter()
            .rev()
            .take(n)
            .rev()
            .map(|e| (e.role.as_str(), e.text.as_str()))
            .collect()
    }

    pub fn set_meta(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    pub fn get_meta(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }
}

// ─── Session registry ─────────────────────────────────────────────────────────

/// Thread-safe registry of active sessions.
pub struct SessionRegistry {
    sessions: HashMap<String, Session>,
}

impl SessionRegistry {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    /// Get or create a session for the given ID.
    pub fn get_or_create(&mut self, id: &str) -> &mut Session {
        self.sessions
            .entry(id.to_string())
            .or_insert_with(|| Session::new(id))
    }

    pub fn get(&self, id: &str) -> Option<&Session> {
        self.sessions.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Session> {
        self.sessions.get_mut(id)
    }

    pub fn remove(&mut self, id: &str) -> Option<Session> {
        self.sessions.remove(id)
    }

    pub fn count(&self) -> usize {
        self.sessions.len()
    }

    /// Remove sessions that have been inactive for longer than `max_age_secs`.
    pub fn reap_stale(&mut self, max_age_secs: i64) -> usize {
        let cutoff = Utc::now() - chrono::Duration::seconds(max_age_secs);
        let before = self.sessions.len();
        self.sessions.retain(|_, s| s.last_active > cutoff);
        before - self.sessions.len()
    }
}

impl Default for SessionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_transcript_trim() {
        let mut s = Session::new("test");
        s.max_transcript = 3;
        for i in 0..5 {
            s.append_transcript(TranscriptEntry::user(format!("msg {}", i)));
        }
        assert_eq!(s.transcript.len(), 3);
        assert_eq!(s.transcript[0].text, "msg 2");
    }

    #[test]
    fn session_registry_get_or_create() {
        let mut reg = SessionRegistry::new();
        {
            let s = reg.get_or_create("abc");
            s.model_override = Some("gpt-4o".to_string());
        }
        assert_eq!(
            reg.get("abc").unwrap().model_override.as_deref(),
            Some("gpt-4o")
        );
        assert_eq!(reg.count(), 1);
    }

    #[test]
    fn verbosity_roundtrip() {
        for (s, v) in [
            ("quiet", VerbosityLevel::Quiet),
            ("verbose", VerbosityLevel::Verbose),
            ("debug", VerbosityLevel::Debug),
            ("normal", VerbosityLevel::Normal),
        ] {
            assert_eq!(VerbosityLevel::from_str(s), v);
            assert_eq!(v.as_str(), s);
        }
    }

    #[test]
    fn session_metadata() {
        let mut s = Session::new("s1");
        s.set_meta("key", "value");
        assert_eq!(s.get_meta("key"), Some("value"));
        assert_eq!(s.get_meta("missing"), None);
    }

    #[test]
    fn reap_stale_sessions() {
        let mut reg = SessionRegistry::new();
        // Create a session and manually age it
        let s = reg.get_or_create("old");
        s.last_active = Utc::now() - chrono::Duration::seconds(3600);
        let _s2 = reg.get_or_create("new"); // recent
        let reaped = reg.reap_stale(1800);
        assert_eq!(reaped, 1);
        assert_eq!(reg.count(), 1);
    }
}
