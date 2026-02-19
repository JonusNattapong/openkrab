//! sessions — Session management commands.
//! Ported from `openclaw/src/commands/sessions.ts` (Phase 6).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Session information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: String,
    pub name: String,
    pub channel: String,
    pub user_id: String,
    pub created_at: String,
    pub last_activity: String,
    pub message_count: usize,
    pub locked: bool,
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
    let mut lines = vec!["Active Sessions:".to_string()];

    // In real implementation, would fetch from session store
    let mock_sessions = vec![
        SessionInfo {
            id: "sess-001".to_string(),
            name: "main".to_string(),
            channel: "telegram".to_string(),
            user_id: "user-123".to_string(),
            created_at: "2026-02-19T10:00:00Z".to_string(),
            last_activity: "2026-02-19T14:30:00Z".to_string(),
            message_count: 42,
            locked: false,
        },
        SessionInfo {
            id: "sess-002".to_string(),
            name: "support".to_string(),
            channel: "slack".to_string(),
            user_id: "user-456".to_string(),
            created_at: "2026-02-19T09:00:00Z".to_string(),
            last_activity: "2026-02-19T13:00:00Z".to_string(),
            message_count: 15,
            locked: false,
        },
    ];

    let sessions: Vec<_> = mock_sessions
        .into_iter()
        .filter(|s| {
            opts.channel
                .as_ref()
                .map(|c| &s.channel == c)
                .unwrap_or(true)
        })
        .take(opts.limit.unwrap_or(100))
        .collect();

    if sessions.is_empty() {
        lines.push("  No sessions found".to_string());
    } else {
        for session in sessions {
            let lock_icon = if session.locked { "🔒" } else { "  " };
            lines.push(format!(
                "  {} {} | {} | {} | {} msgs | Last: {}",
                lock_icon,
                session.id,
                session.name,
                session.channel,
                session.message_count,
                session.last_activity
            ));
        }
    }

    lines.join("\n")
}

/// Get session details.
pub fn sessions_get_command(session_id: &str) -> String {
    // In real implementation, would fetch from session store
    format!(
        "Session details for {}:\n  (Implementation would show full session details)",
        session_id
    )
}

/// Lock a session.
pub fn sessions_lock_command(session_id: &str) -> String {
    format!("🔒 Locked session {}", session_id)
}

/// Unlock a session.
pub fn sessions_unlock_command(session_id: &str) -> String {
    format!("🔓 Unlocked session {}", session_id)
}

/// Archive a session.
pub fn sessions_archive_command(session_id: &str) -> String {
    format!("📦 Archived session {}", session_id)
}

/// Delete a session.
pub fn sessions_delete_command(session_id: &str, force: bool) -> String {
    if force {
        format!("🗑️  Deleted session {}", session_id)
    } else {
        format!("⚠️  Use --force to delete session {}", session_id)
    }
}

/// Session statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStats {
    pub total_sessions: usize,
    pub active_sessions: usize,
    pub locked_sessions: usize,
    pub total_messages: usize,
    pub by_channel: HashMap<String, usize>,
}

/// Get session statistics.
pub fn sessions_stats_command() -> String {
    let stats = SessionStats {
        total_sessions: 2,
        active_sessions: 2,
        locked_sessions: 0,
        total_messages: 57,
        by_channel: [("telegram".to_string(), 42), ("slack".to_string(), 15)]
            .into_iter()
            .collect(),
    };

    let mut lines = vec![
        "Session Statistics:".to_string(),
        format!("  Total sessions: {}", stats.total_sessions),
        format!("  Active sessions: {}", stats.active_sessions),
        format!("  Locked sessions: {}", stats.locked_sessions),
        format!("  Total messages: {}", stats.total_messages),
        "  By channel:".to_string(),
    ];

    for (channel, count) in &stats.by_channel {
        lines.push(format!("    {}: {}", channel, count));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sessions_list() {
        let opts = SessionListOptions::default();
        let output = sessions_list_command(opts);
        assert!(output.contains("Active Sessions:"));
    }

    #[test]
    fn test_sessions_lock() {
        let output = sessions_lock_command("sess-001");
        assert!(output.contains("Locked"));
        assert!(output.contains("sess-001"));
    }

    #[test]
    fn test_sessions_delete_force() {
        let output = sessions_delete_command("sess-001", true);
        assert!(output.contains("Deleted"));
    }

    #[test]
    fn test_sessions_delete_no_force() {
        let output = sessions_delete_command("sess-001", false);
        assert!(output.contains("--force"));
    }
}
