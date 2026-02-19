use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Channel registry with metadata and helpers.
pub struct Registry {
    // mapping from channel id/name to some metadata
    entries: HashMap<String, String>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn register(&mut self, id: &str, meta: &str) {
        self.entries.insert(id.to_string(), meta.to_string());
    }

    pub fn get(&self, id: &str) -> Option<&String> {
        self.entries.get(id)
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

// Ported channel registry constants and helpers (partial parity)
pub const CHAT_CHANNEL_ORDER: &[&str] = &[
    "telegram",
    "whatsapp",
    "discord",
    "irc",
    "googlechat",
    "slack",
    "signal",
    "imessage",
];

pub const DEFAULT_CHAT_CHANNEL: &str = "whatsapp";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChannelMeta {
    pub id: String,
    pub label: String,
    pub selection_label: Option<String>,
    pub detail_label: Option<String>,
    pub docs_path: Option<String>,
    pub docs_label: Option<String>,
    pub blurb: Option<String>,
    pub system_image: Option<String>,
    pub selection_docs_prefix: Option<String>,
    pub selection_docs_omit_label: Option<bool>,
    pub selection_extras: Option<Vec<String>>,
}

fn chat_meta_map() -> HashMap<&'static str, ChatChannelMeta> {
    let mut m = HashMap::new();
    let website = "https://krabkrab.ai".to_string();

    m.insert(
        "telegram",
        ChatChannelMeta {
            id: "telegram".into(),
            label: "Telegram".into(),
            selection_label: Some("Telegram (Bot API)".into()),
            detail_label: Some("Telegram Bot".into()),
            docs_path: Some("/channels/telegram".into()),
            docs_label: Some("telegram".into()),
            blurb: Some(
                "simplest way to get started — register a bot with @BotFather and get going."
                    .into(),
            ),
            system_image: Some("paperplane".into()),
            selection_docs_prefix: Some("".into()),
            selection_docs_omit_label: Some(true),
            selection_extras: Some(vec![website.clone()]),
        },
    );
    m.insert(
        "whatsapp",
        ChatChannelMeta {
            id: "whatsapp".into(),
            label: "WhatsApp".into(),
            selection_label: Some("WhatsApp (QR link)".into()),
            detail_label: Some("WhatsApp Web".into()),
            docs_path: Some("/channels/whatsapp".into()),
            docs_label: Some("whatsapp".into()),
            blurb: Some("works with your own number; recommend a separate phone + eSIM.".into()),
            system_image: Some("message".into()),
            selection_docs_prefix: None,
            selection_docs_omit_label: None,
            selection_extras: None,
        },
    );
    m.insert(
        "slack",
        ChatChannelMeta {
            id: "slack".into(),
            label: "Slack".into(),
            selection_label: Some("Slack (Socket Mode)".into()),
            detail_label: Some("Slack Bot".into()),
            docs_path: Some("/channels/slack".into()),
            docs_label: Some("slack".into()),
            blurb: Some("supported (Socket Mode).".into()),
            system_image: Some("number".into()),
            selection_docs_prefix: None,
            selection_docs_omit_label: None,
            selection_extras: None,
        },
    );
    m.insert(
        "imessage",
        ChatChannelMeta {
            id: "imessage".into(),
            label: "iMessage".into(),
            selection_label: Some("iMessage (imsg)".into()),
            detail_label: Some("iMessage".into()),
            docs_path: Some("/channels/imessage".into()),
            docs_label: Some("imessage".into()),
            blurb: Some("this is still a work in progress.".into()),
            system_image: Some("message.fill".into()),
            selection_docs_prefix: None,
            selection_docs_omit_label: None,
            selection_extras: None,
        },
    );
    m
}

pub fn list_chat_channels() -> Vec<ChatChannelMeta> {
    CHAT_CHANNEL_ORDER
        .iter()
        .filter_map(|id| chat_meta_map().get(id).cloned())
        .collect()
}

pub fn list_chat_channel_aliases() -> Vec<String> {
    let aliases: HashMap<&str, &str> = vec![
        ("imsg", "imessage"),
        ("internet-relay-chat", "irc"),
        ("google-chat", "googlechat"),
        ("gchat", "googlechat"),
    ]
    .into_iter()
    .collect();
    aliases.keys().map(|s| s.to_string()).collect()
}

pub fn get_chat_channel_meta(id: &str) -> Option<ChatChannelMeta> {
    chat_meta_map().get(id).cloned()
}

fn normalize_channel_key(raw: Option<&str>) -> Option<String> {
    raw.and_then(|s| {
        let n = s.trim().to_lowercase();
        if n.is_empty() {
            None
        } else {
            Some(n)
        }
    })
}

pub fn normalize_chat_channel_id(raw: Option<&str>) -> Option<String> {
    let normalized = normalize_channel_key(raw)?;
    // map aliases
    let aliases: HashMap<&str, &str> = vec![
        ("imsg", "imessage"),
        ("internet-relay-chat", "irc"),
        ("google-chat", "googlechat"),
        ("gchat", "googlechat"),
    ]
    .into_iter()
    .collect();
    let resolved = aliases
        .get(normalized.as_str())
        .map(|s| s.to_string())
        .unwrap_or(normalized.clone());
    if CHAT_CHANNEL_ORDER.contains(&resolved.as_str()) {
        Some(resolved)
    } else {
        None
    }
}

pub fn normalize_channel_id(raw: Option<&str>) -> Option<String> {
    normalize_chat_channel_id(raw)
}

pub fn normalize_any_channel_id(raw: Option<&str>) -> Option<String> {
    // plugin registry omitted: fall back to known chat channel ids
    normalize_chat_channel_id(raw)
}

pub fn format_channel_primer_line(meta: &ChatChannelMeta) -> String {
    format!("{}: {}", meta.label, meta.blurb.clone().unwrap_or_default())
}

pub fn format_channel_selection_line<F>(meta: &ChatChannelMeta, docs_link: F) -> String
where
    F: Fn(&str, Option<&str>) -> String,
{
    let docs_prefix = meta.selection_docs_prefix.as_deref().unwrap_or("Docs:");
    let docs_label = meta.docs_label.as_deref().unwrap_or(meta.id.as_str());
    let docs = if meta.selection_docs_omit_label.unwrap_or(false) {
        docs_link(meta.docs_path.as_deref().unwrap_or(""), None)
    } else {
        docs_link(meta.docs_path.as_deref().unwrap_or(""), Some(docs_label))
    };
    let extras = meta
        .selection_extras
        .as_ref()
        .map(|v| v.join(" "))
        .unwrap_or_default();
    format!(
        "{} — {} {}{}",
        meta.label,
        meta.blurb.clone().unwrap_or_default(),
        if docs_prefix.is_empty() {
            "".to_string()
        } else {
            format!("{} ", docs_prefix)
        },
        docs
    )
}
