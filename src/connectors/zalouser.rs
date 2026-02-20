//! connectors::zalouser — Zalo Personal Account connector via `zca` CLI.
//! Ported from `openkrab/extensions/zalouser/` (Phase 21).
//!
//! Wraps the `zca` CLI (https://zca-cli.dev) to provide Zalo personal account
//! messaging, presence monitoring, directory lookup, and agent tool support.
//! Unlike `zalo.rs` (OA API), this connector targets personal accounts via QR login.

use crate::common::{Message, UserId};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tokio::process::Command as TokioCommand;

// ─── Config ───────────────────────────────────────────────────────────────────

pub const DEFAULT_PROFILE: &str = "default";
pub const TEXT_CHUNK_LIMIT: usize = 2000;
pub const DEFAULT_TIMEOUT_MS: u64 = 30_000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZalouserConfig {
    pub enabled: bool,
    pub name: Option<String>,
    /// `zca` profile name (maps to `--profile` flag). Defaults to `"default"`.
    pub profile: Option<String>,
    pub default_account: Option<String>,
    pub dm_policy: DmPolicy,
    pub allow_from: Vec<String>,
    pub group_policy: GroupPolicy,
    pub groups: HashMap<String, GroupEntry>,
}

impl Default for ZalouserConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            name: None,
            profile: None,
            default_account: None,
            dm_policy: DmPolicy::Pairing,
            allow_from: vec![],
            group_policy: GroupPolicy::Open,
            groups: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DmPolicy {
    #[default]
    Pairing,
    Allowlist,
    Open,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum GroupPolicy {
    #[default]
    Open,
    Allowlist,
    Disabled,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupEntry {
    pub allow: Option<bool>,
    pub enabled: Option<bool>,
}

// ─── CLI result ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ZcaResult {
    pub ok: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

// ─── Wire types ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZcaFriend {
    #[serde(rename = "userId")]
    pub user_id: Option<serde_json::Value>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZcaGroup {
    #[serde(rename = "groupId")]
    pub group_id: Option<serde_json::Value>,
    pub name: Option<String>,
    #[serde(rename = "memberCount")]
    pub member_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZcaUserInfo {
    #[serde(rename = "userId")]
    pub user_id: Option<serde_json::Value>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub avatar: Option<String>,
}

/// Inbound message from `zca listen -r -k` (newline-delimited JSON).
#[derive(Debug, Clone, Deserialize)]
pub struct ZcaMessage {
    #[serde(rename = "threadId")]
    pub thread_id: String,
    #[serde(rename = "msgId")]
    pub msg_id: Option<String>,
    pub content: Option<String>,
    pub timestamp: Option<i64>,
    pub metadata: Option<ZcaMessageMeta>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ZcaMessageMeta {
    #[serde(rename = "isGroup")]
    pub is_group: Option<bool>,
    #[serde(rename = "threadName")]
    pub thread_name: Option<String>,
    #[serde(rename = "senderName")]
    pub sender_name: Option<String>,
    #[serde(rename = "fromId")]
    pub from_id: Option<String>,
}

// ─── CLI runner ───────────────────────────────────────────────────────────────

/// Build the full argument list, prepending `--profile <p>` when set.
fn build_args(args: &[&str], profile: Option<&str>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let env_profile = std::env::var("ZCA_PROFILE").ok();
    let effective = profile.or(env_profile.as_deref());
    if let Some(p) = effective {
        result.push("--profile".to_string());
        result.push(p.to_string());
    }
    for arg in args {
        result.push(arg.to_string());
    }
    result
}

/// Run `zca` with the given arguments and timeout.
pub async fn run_zca(args: &[&str], profile: Option<&str>, timeout_ms: Option<u64>) -> ZcaResult {
    let full_args = build_args(args, profile);
    let timeout = Duration::from_millis(timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS));

    let output = tokio::time::timeout(
        timeout,
        TokioCommand::new("zca")
            .args(&full_args)
            .output(),
    )
    .await;

    match output {
        Ok(Ok(out)) => ZcaResult {
            ok: out.status.success(),
            stdout: String::from_utf8_lossy(&out.stdout).trim().to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).trim().to_string(),
            exit_code: out.status.code().unwrap_or(1),
        },
        Ok(Err(e)) => ZcaResult {
            ok: false,
            stdout: String::new(),
            stderr: e.to_string(),
            exit_code: 1,
        },
        Err(_) => ZcaResult {
            ok: false,
            stdout: String::new(),
            stderr: "Command timed out".to_string(),
            exit_code: 124,
        },
    }
}

/// Check whether the `zca` binary is available in `$PATH`.
pub async fn check_zca_installed() -> bool {
    run_zca(&["--version"], None, Some(5000)).await.ok
}

/// Check whether the given profile is authenticated.
pub async fn check_zca_authenticated(profile: Option<&str>) -> bool {
    run_zca(&["auth", "status"], profile, Some(5000))
        .await
        .ok
}

/// Strip ANSI escape codes and attempt to parse JSON from stdout.
/// Handles cases where `zca` prefixes output with INFO/log lines.
pub fn parse_json_output<T: serde::de::DeserializeOwned>(stdout: &str) -> Option<T> {
    // Try raw parse
    if let Ok(v) = serde_json::from_str(stdout) {
        return Some(v);
    }
    // Strip ANSI and retry
    let cleaned = strip_ansi(stdout);
    if let Ok(v) = serde_json::from_str(&cleaned) {
        return Some(v);
    }
    // Scan line-by-line for first JSON token
    for (i, line) in cleaned.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with('{') || trimmed.starts_with('[') {
            let candidate: String = cleaned.lines().skip(i).collect::<Vec<_>>().join("\n");
            if let Ok(v) = serde_json::from_str(candidate.trim()) {
                return Some(v);
            }
        }
    }
    None
}

/// Naive ANSI escape code stripper (no regex crate dependency).
fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip escape sequence: ESC [ ... final-byte
            if chars.peek() == Some(&'[') {
                chars.next();
                for ch in chars.by_ref() {
                    if ch.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

// ─── Send helpers ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct SendOptions {
    pub profile: Option<String>,
    pub media_url: Option<String>,
    pub is_group: bool,
}

#[derive(Debug, Clone)]
pub struct SendResult {
    pub ok: bool,
    pub message_id: Option<String>,
    pub error: Option<String>,
}

/// Determine the media sub-command from a URL extension.
fn media_command_for_url(url: &str) -> &'static str {
    let lower = url.to_lowercase();
    if lower.ends_with(".mp4")
        || lower.ends_with(".mov")
        || lower.ends_with(".avi")
        || lower.ends_with(".webm")
    {
        "video"
    } else if lower.ends_with(".mp3")
        || lower.ends_with(".wav")
        || lower.ends_with(".ogg")
        || lower.ends_with(".m4a")
    {
        "voice"
    } else {
        "image"
    }
}

/// Extract a message ID from `zca` stdout (best-effort).
fn extract_message_id(stdout: &str) -> Option<String> {
    // Try `message_id: <id>` pattern
    let lower = stdout.to_lowercase();
    if let Some(pos) = lower.find("message_id") {
        let after = &stdout[pos..];
        if let Some(id_start) = after.find(':') {
            let candidate = after[id_start + 1..].trim().split_whitespace().next()?;
            if candidate
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
            {
                return Some(candidate.to_string());
            }
        }
    }
    // Fallback: first alphanumeric word
    stdout
        .trim()
        .split_whitespace()
        .next()
        .filter(|s| s.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_'))
        .map(|s| s.to_string())
}

/// Send a text (or media) message to a Zalo thread via `zca`.
pub async fn send_message(
    thread_id: &str,
    text: &str,
    opts: &SendOptions,
) -> SendResult {
    let profile = opts.profile.as_deref();

    if thread_id.trim().is_empty() {
        return SendResult {
            ok: false,
            message_id: None,
            error: Some("No threadId provided".to_string()),
        };
    }

    // Media send
    if let Some(media_url) = &opts.media_url {
        let cmd = media_command_for_url(media_url);
        let capped_text = &text[..text.len().min(TEXT_CHUNK_LIMIT)];
        let mut args: Vec<&str> = vec!["msg", cmd, thread_id.trim(), "-u", media_url.trim()];
        if !capped_text.is_empty() {
            args.extend_from_slice(&["-m", capped_text]);
        }
        if opts.is_group {
            args.push("-g");
        }
        let result = run_zca(&args, profile, None).await;
        return SendResult {
            ok: result.ok,
            message_id: if result.ok {
                extract_message_id(&result.stdout)
            } else {
                None
            },
            error: if result.ok {
                None
            } else {
                Some(
                    result
                        .stderr
                        .is_empty()
                        .then(|| format!("Failed to send {}", cmd))
                        .unwrap_or(result.stderr),
                )
            },
        };
    }

    // Text send
    let capped = &text[..text.len().min(TEXT_CHUNK_LIMIT)];
    let mut args = vec!["msg", "send", thread_id.trim(), capped];
    if opts.is_group {
        args.push("-g");
    }
    let result = run_zca(&args, profile, None).await;
    SendResult {
        ok: result.ok,
        message_id: if result.ok {
            extract_message_id(&result.stdout)
        } else {
            None
        },
        error: if result.ok {
            None
        } else {
            Some(
                result
                    .stderr
                    .is_empty()
                    .then(|| "Failed to send message".to_string())
                    .unwrap_or(result.stderr),
            )
        },
    }
}

// ─── Directory helpers ────────────────────────────────────────────────────────

/// Get the current user info for the given profile.
pub async fn get_user_info(profile: Option<&str>) -> Result<ZcaUserInfo> {
    let result = run_zca(&["me", "info", "-j"], profile, Some(10_000)).await;
    if !result.ok {
        bail!("zca me info failed: {}", result.stderr);
    }
    parse_json_output::<ZcaUserInfo>(&result.stdout)
        .ok_or_else(|| anyhow::anyhow!("Failed to parse user info"))
}

/// List friends, optionally filtered by query.
pub async fn list_friends(
    profile: Option<&str>,
    query: Option<&str>,
) -> Result<Vec<ZcaFriend>> {
    let args: &[&str] = if let Some(q) = query {
        &["friend", "find", q]
    } else {
        &["friend", "list", "-j"]
    };
    let result = run_zca(args, profile, Some(15_000)).await;
    if !result.ok {
        bail!("zca friend list/find failed: {}", result.stderr);
    }
    Ok(parse_json_output::<Vec<ZcaFriend>>(&result.stdout).unwrap_or_default())
}

/// List groups, optionally filtered by name/id query.
pub async fn list_groups(
    profile: Option<&str>,
    query: Option<&str>,
) -> Result<Vec<ZcaGroup>> {
    let result = run_zca(&["group", "list", "-j"], profile, Some(15_000)).await;
    if !result.ok {
        bail!("zca group list failed: {}", result.stderr);
    }
    let mut groups: Vec<ZcaGroup> =
        parse_json_output::<Vec<ZcaGroup>>(&result.stdout).unwrap_or_default();
    if let Some(q) = query {
        let ql = q.trim().to_lowercase();
        groups.retain(|g| {
            g.name
                .as_deref()
                .map(|n| n.to_lowercase().contains(&ql))
                .unwrap_or(false)
                || g.group_id
                    .as_ref()
                    .map(|id| id.to_string().contains(&ql))
                    .unwrap_or(false)
        });
    }
    Ok(groups)
}

// ─── Normalize helpers ────────────────────────────────────────────────────────

/// Strip `zalouser:` / `zlu:` prefix from an allow-from entry.
pub fn normalize_entry(entry: &str) -> &str {
    entry
        .strip_prefix("zalouser:")
        .or_else(|| entry.strip_prefix("zlu:"))
        .map(|s| s.trim_start_matches(':'))
        .unwrap_or(entry)
}

/// True when the entry looks like a plain numeric Zalo user/thread ID.
pub fn looks_like_numeric_id(s: &str) -> bool {
    let t = s.trim();
    t.len() >= 3 && t.chars().all(|c| c.is_ascii_digit())
}

/// Normalize a target string for outbound delivery.
pub fn normalize_target(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    Some(normalize_entry(trimmed).to_string())
}

// ─── Inbound normalization ────────────────────────────────────────────────────

/// Convert a `ZcaMessage` to the internal `common::Message` format.
pub fn normalize_inbound(msg: &ZcaMessage) -> Message {
    let sender_id = msg
        .metadata
        .as_ref()
        .and_then(|m| m.from_id.clone())
        .unwrap_or_else(|| msg.thread_id.clone());
    let msg_id = msg
        .msg_id
        .clone()
        .unwrap_or_else(|| msg.timestamp.unwrap_or(0).to_string());
    Message {
        id: format!("zalouser:{}", msg_id),
        text: msg.content.clone().unwrap_or_default(),
        from: Some(UserId(format!("zalouser:{}", sender_id))),
    }
}

// ─── Group policy helpers ─────────────────────────────────────────────────────

fn normalize_group_slug(raw: &str) -> String {
    raw.trim()
        .to_lowercase()
        .trim_start_matches('#')
        .to_string()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

/// Return `true` when the group is allowed per the groups config.
pub fn is_group_allowed(
    group_id: &str,
    group_name: Option<&str>,
    groups: &HashMap<String, GroupEntry>,
) -> bool {
    if groups.is_empty() {
        return false;
    }
    let slug = group_name.map(normalize_group_slug).unwrap_or_default();
    let candidates: &[&str] = &[
        group_id,
        group_name.unwrap_or(""),
        slug.as_str(),
        "*",
    ];
    for key in candidates {
        if key.is_empty() {
            continue;
        }
        if let Some(entry) = groups.get(*key) {
            return entry.allow.unwrap_or(true) && entry.enabled.unwrap_or(true);
        }
    }
    false
}

/// Return `true` when the sender ID is in the allow-from list (handles `*` wildcard).
pub fn is_sender_allowed(sender_id: &str, allow_from: &[String]) -> bool {
    if allow_from.iter().any(|e| e == "*") {
        return true;
    }
    let lower_id = sender_id.to_lowercase();
    allow_from.iter().any(|e| {
        e.to_lowercase()
            .trim_start_matches("zalouser:")
            .trim_start_matches("zlu:")
            .to_string()
            == lower_id
    })
}

// ─── Agent tool ───────────────────────────────────────────────────────────────

/// Actions supported by the `zalouser` agent tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ZalouserAction {
    Send,
    Image,
    Link,
    Friends,
    Groups,
    Me,
    Status,
}

/// Parameters for the `zalouser` agent tool call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZalouserToolParams {
    pub action: ZalouserAction,
    pub thread_id: Option<String>,
    pub message: Option<String>,
    pub is_group: Option<bool>,
    pub profile: Option<String>,
    pub query: Option<String>,
    pub url: Option<String>,
}

/// Execute the `zalouser` agent tool and return a JSON result.
pub async fn execute_tool(params: ZalouserToolParams) -> serde_json::Value {
    match params.action {
        ZalouserAction::Send => {
            let thread_id = match &params.thread_id {
                Some(id) if !id.trim().is_empty() => id.clone(),
                _ => {
                    return serde_json::json!({"error": "threadId and message required for send action"})
                }
            };
            let msg = params.message.as_deref().unwrap_or("");
            let result = send_message(
                &thread_id,
                msg,
                &SendOptions {
                    profile: params.profile.clone(),
                    is_group: params.is_group.unwrap_or(false),
                    ..Default::default()
                },
            )
            .await;
            if result.ok {
                serde_json::json!({"success": true, "messageId": result.message_id})
            } else {
                serde_json::json!({"error": result.error})
            }
        }

        ZalouserAction::Image => {
            let thread_id = match &params.thread_id {
                Some(id) if !id.trim().is_empty() => id.clone(),
                _ => return serde_json::json!({"error": "threadId required for image action"}),
            };
            let url = match &params.url {
                Some(u) if !u.trim().is_empty() => u.clone(),
                _ => return serde_json::json!({"error": "url required for image action"}),
            };
            let result = send_message(
                &thread_id,
                params.message.as_deref().unwrap_or(""),
                &SendOptions {
                    profile: params.profile.clone(),
                    media_url: Some(url),
                    is_group: params.is_group.unwrap_or(false),
                },
            )
            .await;
            if result.ok {
                serde_json::json!({"success": true, "messageId": result.message_id})
            } else {
                serde_json::json!({"error": result.error})
            }
        }

        ZalouserAction::Link => {
            let thread_id = match &params.thread_id {
                Some(id) if !id.trim().is_empty() => id.clone(),
                _ => return serde_json::json!({"error": "threadId and url required for link action"}),
            };
            let url = match &params.url {
                Some(u) if !u.trim().is_empty() => u.clone(),
                _ => return serde_json::json!({"error": "url required for link action"}),
            };
            let profile = params.profile.as_deref();
            let is_group = params.is_group.unwrap_or(false);
            let mut args = vec!["msg", "link", thread_id.trim()];
            let url_str = url.trim().to_string();
            args.push(&url_str);
            let group_flag = "-g";
            if is_group {
                args.push(group_flag);
            }
            let result = run_zca(&args, profile, None).await;
            if result.ok {
                serde_json::json!({"success": true})
            } else {
                serde_json::json!({"error": result.stderr})
            }
        }

        ZalouserAction::Friends => {
            let profile = params.profile.as_deref();
            match list_friends(profile, params.query.as_deref()).await {
                Ok(friends) => serde_json::json!(friends),
                Err(e) => serde_json::json!({"error": e.to_string()}),
            }
        }

        ZalouserAction::Groups => {
            let profile = params.profile.as_deref();
            match list_groups(profile, params.query.as_deref()).await {
                Ok(groups) => serde_json::json!(groups),
                Err(e) => serde_json::json!({"error": e.to_string()}),
            }
        }

        ZalouserAction::Me => {
            let profile = params.profile.as_deref();
            match get_user_info(profile).await {
                Ok(info) => serde_json::json!(info),
                Err(e) => serde_json::json!({"error": e.to_string()}),
            }
        }

        ZalouserAction::Status => {
            let profile = params.profile.as_deref();
            let result = run_zca(&["auth", "status"], profile, Some(5000)).await;
            serde_json::json!({
                "authenticated": result.ok,
                "output": if result.stdout.is_empty() { &result.stderr } else { &result.stdout }
            })
        }
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_entry_strips_prefix() {
        assert_eq!(normalize_entry("zalouser:12345"), "12345");
        assert_eq!(normalize_entry("zlu:12345"), "12345");
        assert_eq!(normalize_entry("12345"), "12345");
    }

    #[test]
    fn normalize_target_empty_returns_none() {
        assert!(normalize_target("").is_none());
        assert!(normalize_target("   ").is_none());
    }

    #[test]
    fn normalize_target_strips_prefix() {
        assert_eq!(normalize_target("zalouser:9999"), Some("9999".to_string()));
    }

    #[test]
    fn looks_like_numeric_id_valid() {
        assert!(looks_like_numeric_id("12345678"));
        assert!(looks_like_numeric_id("111"));
    }

    #[test]
    fn looks_like_numeric_id_invalid() {
        assert!(!looks_like_numeric_id("12")); // too short
        assert!(!looks_like_numeric_id("abc123"));
        assert!(!looks_like_numeric_id(""));
    }

    #[test]
    fn is_sender_allowed_wildcard() {
        assert!(is_sender_allowed("anyone", &["*".to_string()]));
    }

    #[test]
    fn is_sender_allowed_exact_match() {
        assert!(is_sender_allowed("12345", &["12345".to_string(), "67890".to_string()]));
        assert!(!is_sender_allowed("99999", &["12345".to_string()]));
    }

    #[test]
    fn is_sender_allowed_with_prefix() {
        assert!(is_sender_allowed("12345", &["zalouser:12345".to_string()]));
        assert!(is_sender_allowed("12345", &["zlu:12345".to_string()]));
    }

    #[test]
    fn is_group_allowed_exact_id() {
        let mut groups = HashMap::new();
        groups.insert("g001".to_string(), GroupEntry { allow: Some(true), enabled: Some(true) });
        assert!(is_group_allowed("g001", None, &groups));
        assert!(!is_group_allowed("g002", None, &groups));
    }

    #[test]
    fn is_group_allowed_wildcard() {
        let mut groups = HashMap::new();
        groups.insert("*".to_string(), GroupEntry { allow: Some(true), enabled: Some(true) });
        assert!(is_group_allowed("anything", Some("Any Group"), &groups));
    }

    #[test]
    fn is_group_allowed_empty_config() {
        assert!(!is_group_allowed("g001", None, &HashMap::new()));
    }

    #[test]
    fn normalize_inbound_maps_fields() {
        let msg = ZcaMessage {
            thread_id: "t1".to_string(),
            msg_id: Some("m1".to_string()),
            content: Some("hello".to_string()),
            timestamp: Some(1_000_000),
            metadata: Some(ZcaMessageMeta {
                is_group: Some(false),
                thread_name: None,
                sender_name: Some("Alice".to_string()),
                from_id: Some("u1".to_string()),
            }),
        };
        let m = normalize_inbound(&msg);
        assert!(m.id.starts_with("zalouser:m1"));
        assert_eq!(m.text, "hello");
        assert!(m.from.as_ref().unwrap().0.contains("u1"));
    }

    #[test]
    fn parse_json_output_valid() {
        let parsed: Option<Vec<serde_json::Value>> =
            parse_json_output(r#"[{"a":1}]"#);
        assert!(parsed.is_some());
    }

    #[test]
    fn parse_json_output_with_ansi() {
        let raw = "\x1b[32mINFO\x1b[0m some log\n[{\"a\":1}]";
        let parsed: Option<Vec<serde_json::Value>> = parse_json_output(raw);
        assert!(parsed.is_some());
    }

    #[test]
    fn parse_json_output_invalid() {
        let parsed: Option<serde_json::Value> = parse_json_output("not json");
        assert!(parsed.is_none());
    }

    #[test]
    fn media_command_for_url_video() {
        assert_eq!(media_command_for_url("https://example.com/foo.mp4"), "video");
        assert_eq!(media_command_for_url("https://example.com/foo.MOV"), "video");
    }

    #[test]
    fn media_command_for_url_voice() {
        assert_eq!(media_command_for_url("https://example.com/foo.mp3"), "voice");
    }

    #[test]
    fn media_command_for_url_image() {
        assert_eq!(media_command_for_url("https://example.com/foo.jpg"), "image");
        assert_eq!(media_command_for_url("https://example.com/foo.png"), "image");
    }

    #[test]
    fn strip_ansi_basic() {
        let input = "\x1b[32mHello\x1b[0m World";
        assert_eq!(strip_ansi(input), "Hello World");
    }

    #[test]
    fn build_args_with_profile() {
        let args = build_args(&["friend", "list"], Some("work"));
        assert_eq!(args[0], "--profile");
        assert_eq!(args[1], "work");
        assert_eq!(args[2], "friend");
    }

    #[test]
    fn build_args_without_profile() {
        // Ensure ZCA_PROFILE env var is not set in test
        std::env::remove_var("ZCA_PROFILE");
        let args = build_args(&["auth", "status"], None);
        assert_eq!(args, vec!["auth", "status"]);
    }
}
