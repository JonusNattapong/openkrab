//! BlueBubbles connector - iMessage via macOS server.
//! Ported from openclaw/extensions/bluebubbles/
//!
//! This module provides full BlueBubbles integration including:
//! - Account configuration and resolution
//! - Server probing and Private API detection
//! - Target parsing and normalization
//! - Message sending (text, media, effects)
//! - Reactions, edits, unsend
//! - Group management (rename, add/remove participants, leave)
//! - Webhook monitoring for inbound messages

pub mod actions;
pub mod chat;
pub mod config;
pub mod media;
pub mod monitor;
pub mod probe;
pub mod reactions;
pub mod send;
pub mod targets;
pub mod types;

pub use actions::{execute_action, ActionContext, ActionResult};
pub use chat::{edit_message, unsend_message, ChatResult};
pub use config::{
    is_action_enabled, list_account_ids, resolve_account, resolve_default_account_id,
    resolve_dm_policy, resolve_group_allow_from, resolve_group_policy,
    resolve_group_require_mention, resolve_text_chunk_limit, resolve_webhook_path,
    ResolvedBlueBubblesAccount,
};
pub use media::send_media;
pub use monitor::{Monitor, RuntimeStatus, WebhookTarget};
pub use probe::{
    clear_all_cache, clear_cache, fetch_server_info, get_cached_server_info,
    get_private_api_status, is_macos_26_or_higher, is_private_api_enabled, probe_server,
    ProbeResult,
};
pub use reactions::{map_emoji_to_reaction, parse_reaction_action, remove_reaction, send_reaction};
pub use send::{create_new_chat, send_message, SendOptions};
pub use targets::{
    extract_handle_from_chat_guid, format_target_display, is_dm_chat_guid, is_group_chat_guid,
    normalize_handle, normalize_messaging_target, normalize_target, ParsedTarget,
};
pub use types::{
    build_api_url, extract_message_id, looks_like_target_id,
    normalize_handle as normalize_handle_util, normalize_server_url, BlueBubblesAccountConfig,
    BlueBubblesActionConfig, BlueBubblesAttachment, BlueBubblesConfig, BlueBubblesGroupConfig,
    BlueBubblesProbeResult, BlueBubblesSendResult, BlueBubblesSendTarget, BlueBubblesServerInfo,
    BlueBubblesToolPolicy, NormalizedWebhookMessage, NormalizedWebhookReaction, WebhookPayload,
    DEFAULT_TIMEOUT_MS,
};

use crate::common::{Message, UserId};

pub fn normalize_inbound(text: &str, chat_guid: &str, sender: &str, message_id: &str) -> Message {
    Message {
        id: format!("bluebubbles:{chat_guid}:{message_id}"),
        text: text.to_string(),
        from: Some(UserId(format!(
            "bluebubbles:{}",
            targets::normalize_target(sender)
        ))),
    }
}

pub use types::resolve_effect_id;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_effect_id() {
        assert_eq!(
            resolve_effect_id(Some("slam")),
            Some("com.apple.MobileSMS.expressivesend.impact".to_string())
        );
        assert_eq!(
            resolve_effect_id(Some("heart")),
            Some("com.apple.MobileSMS.expressivesend.love".to_string())
        );
    }

    #[test]
    fn test_normalize_inbound() {
        let msg = normalize_inbound(
            "hello",
            "iMessage;-;+15551234567",
            "+15551234567",
            "msg-123",
        );
        assert_eq!(msg.id, "bluebubbles:iMessage;-;+15551234567:msg-123");
        assert_eq!(msg.text, "hello");
    }

    #[test]
    fn test_list_account_ids() {
        let config = BlueBubblesConfig::default();
        let ids = list_account_ids(&config);
        assert!(ids.is_empty());
    }

    #[test]
    fn test_resolve_account_not_configured() {
        let config = BlueBubblesConfig::default();
        let account = resolve_account(&config, None);
        assert!(!account.configured);
    }

    #[test]
    fn test_targets() {
        assert!(looks_like_target_id("+15551234567"));
        assert!(looks_like_target_id("chat_guid:iMessage;-;+123"));
        assert!(!looks_like_target_id("hello world"));
    }
}
