//! sessions — Session management commands.
//! Ported from `openkrab/src/commands/sessions.ts` (Phase 6).

use crate::memory::MemoryStore;
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub id: String,
    pub label: Option<String>,
    pub channel: Option<String>,
    pub elevated: bool,
    pub transcript_len: usize,
}

#[derive(Debug, Clone)]
pub struct SessionStats {
    pub total_sessions: usize,
    pub elevated_sessions: usize,
    pub total_transcript_entries: usize,
    pub by_channel: HashMap<String, usize>,
}

fn get_db_path() -> PathBuf {
    let home = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    home.join("krabkrab").join("memory.db")
}

fn open_store() -> Result<MemoryStore, String> {
    let path = get_db_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    MemoryStore::open(path).map_err(|e| format!("Failed to open database: {}", e))
}

/// Session list options.
#[derive(Debug, Clone, Default)]
pub struct SessionListOptions {
    pub channel: Option<String>,
    pub active_only: bool,
    pub limit: Option<usize>,
}

/// List sessions.
pub fn sessions_list_command(opts: SessionListOptions) -> String {
    let store = match open_store() {
        Ok(s) => s,
        Err(e) => return e,
    };

    match store.list_sessions() {
        Ok(sessions) => {
            let mut lines = vec!["Active Sessions:".to_string()];
            let mut filtered: Vec<_> = sessions
                .into_iter()
                .filter(|s| {
                    opts.channel
                        .as_ref()
                        .map(|c| s.channel.as_deref() == Some(c))
                        .unwrap_or(true)
                })
                .filter(|s| {
                    if opts.active_only {
                        s.get_meta("archived") != Some("true")
                    } else {
                        true
                    }
                })
                .collect();

            if let Some(limit) = opts.limit {
                filtered.truncate(limit);
            }

            if filtered.is_empty() {
                lines.push("  No sessions found".to_string());
            } else {
                for session in filtered {
                    let lock_icon = if session.elevated { "🔒" } else { "  " };
                    let archived_suffix = if session.get_meta("archived") == Some("true") {
                        " [archived]"
                    } else {
                        ""
                    };
                    let channel = session.channel.as_deref().unwrap_or("?");
                    let label = session.display_name.as_deref().or(session.label.as_deref()).unwrap_or(&session.id);
                    let tokens = if session.total_tokens > 0 {
                        format!(" | {} tok", session.total_tokens)
                    } else {
                        "".to_string()
                    };

                    lines.push(format!(
                        "  {} {} | {} | {} | {} msgs{}{} | Last: {}",
                        lock_icon,
                        session.id,
                        label,
                        channel,
                        session.transcript.len(),
                        tokens,
                        archived_suffix,
                        session.last_active.format("%Y-%m-%d %H:%M")
                    ));
                }
            }
            lines.join("\n")
        }
        Err(e) => format!("Failed to list sessions: {}", e),
    }
}

/// Get session details.
pub fn sessions_get_command(session_id: &str) -> String {
    let store = match open_store() {
        Ok(s) => s,
        Err(e) => return e,
    };

    match store.load_session(session_id) {
        Ok(Some(s)) => {
            let mut out = format!("Session: {}\n", s.id);
            out.push_str(&format!(
                "  Display Name: {}\n",
                s.display_name.as_deref().unwrap_or("none")
            ));
            out.push_str(&format!(
                "  Label: {}\n",
                s.label.as_deref().unwrap_or("none")
            ));
            out.push_str(&format!(
                "  Channel: {}\n",
                s.channel.as_deref().unwrap_or("unknown")
            ));
            out.push_str(&format!(
                "  Chat Type: {}\n",
                s.chat_type.as_deref().unwrap_or("unknown")
            ));
            out.push_str(&format!(
                "  Model Override: {}\n",
                s.model_override.as_deref().unwrap_or("none")
            ));
            out.push_str(&format!("  Verbosity: {}\n", s.verbosity.as_str()));
            if let Some(dm) = s.delivery_mode {
                out.push_str(&format!("  Delivery Mode: {:?}\n", dm));
            }
            if let Some(sp) = s.send_policy {
                out.push_str(&format!("  Send Policy: {:?}\n", sp));
            }
            if let Some(tl) = s.thinking_level {
                out.push_str(&format!("  Thinking Level: {:?}\n", tl));
            }
            out.push_str(&format!("  Elevated: {}\n", s.elevated));
            out.push_str(&format!("  Usage: {} in / {} out / {} total ({} ctx)\n", 
                s.input_tokens, s.output_tokens, s.total_tokens, s.context_tokens));
            out.push_str(&format!("  Created: {}\n", s.created_at));
            out.push_str(&format!("  Last Active: {}\n", s.last_active));

            if !s.metadata.is_empty() {
                out.push_str("  Metadata:\n");
                for (k, v) in &s.metadata {
                    out.push_str(&format!("    {}: {}\n", k, v));
                }
            }

            out.push_str("\nTranscript:\n");
            if s.transcript.is_empty() {
                out.push_str("  (no messages)\n");
            } else {
                for entry in s.transcript {
                    out.push_str(&format!(
                        "    [{}] {}: {}\n",
                        entry.timestamp.format("%H:%M:%S"),
                        entry.role,
                        entry.text
                    ));
                }
            }
            out
        }
        Ok(None) => format!("Session not found: {}", session_id),
        Err(e) => format!("Error loading session: {}", e),
    }
}

/// Lock a session (elevate).
pub fn sessions_lock_command(session_id: &str) -> String {
    let store = match open_store() {
        Ok(s) => s,
        Err(e) => return e,
    };

    match store.load_session(session_id) {
        Ok(Some(mut s)) => {
            s.elevated = true;
            if let Err(e) = store.save_session(&s) {
                return format!("Failed to save session: {}", e);
            }
            format!("🔒 Locked (elevated) session {}", session_id)
        }
        Ok(None) => format!("Session not found: {}", session_id),
        Err(e) => format!("Error: {}", e),
    }
}

/// Unlock a session (de-elevate).
pub fn sessions_unlock_command(session_id: &str) -> String {
    let store = match open_store() {
        Ok(s) => s,
        Err(e) => return e,
    };

    match store.load_session(session_id) {
        Ok(Some(mut s)) => {
            s.elevated = false;
            if let Err(e) = store.save_session(&s) {
                return format!("Failed to save session: {}", e);
            }
            format!("🔓 Unlocked (de-elevated) session {}", session_id)
        }
        Ok(None) => format!("Session not found: {}", session_id),
        Err(e) => format!("Error: {}", e),
    }
}

/// Archive a session.
pub fn sessions_archive_command(session_id: &str) -> String {
    let store = match open_store() {
        Ok(s) => s,
        Err(e) => return e,
    };

    match store.load_session(session_id) {
        Ok(Some(mut s)) => {
            s.set_meta("archived", "true");
            if let Err(e) = store.save_session(&s) {
                return format!("Failed to save session: {}", e);
            }
            format!("📦 Archived session {}", session_id)
        }
        Ok(None) => format!("Session not found: {}", session_id),
        Err(e) => format!("Error: {}", e),
    }
}

/// Delete a session.
pub fn sessions_delete_command(session_id: &str, force: bool) -> String {
    if !force {
        return format!("⚠️  Use --force to delete session {}", session_id);
    }

    let store = match open_store() {
        Ok(s) => s,
        Err(e) => return e,
    };

    match store.delete_session(session_id) {
        Ok(_) => format!("🗑️  Deleted session {}", session_id),
        Err(e) => format!("Failed to delete session: {}", e),
    }
}

/// Get session statistics.
pub fn sessions_stats_command() -> String {
    let store = match open_store() {
        Ok(s) => s,
        Err(e) => return e,
    };

    match store.list_sessions() {
        Ok(sessions) => {
            let total = sessions.len();
            let elevated = sessions.iter().filter(|s| s.elevated).count();
            let total_msgs: usize = sessions.iter().map(|s| s.transcript.len()).sum();
            let total_tokens: u32 = sessions.iter().map(|s| s.total_tokens).sum();

            let mut by_channel: HashMap<String, usize> = HashMap::new();
            for s in &sessions {
                let chan = s.channel.clone().unwrap_or_else(|| "unknown".to_string());
                *by_channel.entry(chan).or_insert(0) += 1;
            }

            let mut out = format!("Session Statistics:\n");
            out.push_str(&format!("  Total sessions: {}\n", total));
            out.push_str(&format!("  Elevated sessions: {}\n", elevated));
            out.push_str(&format!("  Total transcript entries: {}\n", total_msgs));
            out.push_str(&format!("  Total tokens processed: {}\n", total_tokens));
            out.push_str("  By channel:\n");
            for (chan, count) in by_channel {
                out.push_str(&format!("    {}: {}\n", chan, count));
            }
            out
        }
        Err(e) => format!("Failed to get stats: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::MemoryStore;
    use crate::sessions::{Session, TranscriptEntry};

    #[test]
    fn test_sessions_db_roundtrip() {
        // Use a temp DB for testing
        let store = MemoryStore::open_in_memory().unwrap();
        let mut s = Session::new("test-123");
        s.label = Some("Test Session".to_string());
        s.append_transcript(TranscriptEntry::user("hello"));

        store.save_session(&s).unwrap();
        let loaded = store.load_session("test-123").unwrap().unwrap();
        assert_eq!(loaded.id, "test-123");
        assert_eq!(loaded.label.as_deref(), Some("Test Session"));
        assert_eq!(loaded.transcript.len(), 1);
    }
}
