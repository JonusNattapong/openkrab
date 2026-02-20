//! Connectors — messaging platform adapters

// ── Core connectors ───────────────────────────────────────────────────────────
pub mod slack;
pub mod slack_client;
pub mod telegram;
pub mod telegram_client;

// ── Extended connectors (Phase 4–10) ─────────────────────────────────────────
pub mod bluebubbles;
pub mod discord;
pub mod discord_client;
pub mod line;
pub mod line_client;
pub mod whatsapp;
pub mod whatsapp_client;
pub mod whatsapp_monitor;

// ── Phase 11 connectors ────────────────────────────────────────────────────────
pub mod irc;
pub mod mattermost;
pub mod msteams;
pub mod twitch;
pub mod zalo;

// ── Phase 12 connectors ────────────────────────────────────────────────────────
pub mod feishu;
pub mod googlechat;
pub mod nextcloud_talk;
pub mod nostr;

// ── Phase 13 connectors ────────────────────────────────────────────────────────
pub mod tlon;

// ── Phase 21 connectors ────────────────────────────────────────────────────────
pub mod zalouser;

// ── Phase 5-6 connectors (additional) ───────────────────────────────────────────
pub mod matrix;
pub mod signal;

// ── Public re-exports ─────────────────────────────────────────────────────────
pub use bluebubbles::{
    normalize_inbound as bluebubbles_normalize_inbound,
    normalize_target as bluebubbles_normalize_target,
};
pub use discord_client::build_discord_http_payload;
pub use feishu::{build_text_payload as feishu_build_text, parse_event as feishu_parse_event};
pub use googlechat::{
    build_text_message as googlechat_build_text, parse_event as googlechat_parse_event,
};
pub use irc::{build_privmsg as irc_build_privmsg, parse_privmsg as irc_parse_privmsg};
pub use line_client::{
    build_line_broadcast_payload, build_line_push_payload, build_line_reply_payload,
    build_line_rich_menu_payload,
};
pub use matrix::{
    build_html_message as matrix_build_html_message,
    build_text_message as matrix_build_text_message, extract_matrix_text,
    extract_messages_from_sync, format_outbound as matrix_format_outbound,
    normalize_inbound as matrix_normalize_inbound, parse_matrix_message, parse_sync_response,
};
pub use mattermost::{
    build_webhook_payload as mattermost_build_webhook, parse_webhook as mattermost_parse_webhook,
};
pub use msteams::{
    build_webhook_payload as msteams_build_webhook, parse_activity as msteams_parse_activity,
};
pub use nextcloud_talk::parse_message as nctalk_parse_message;
pub use nostr::{parse_event as nostr_parse_event, parse_relay_message as nostr_parse_relay};
pub use signal::{
    build_text_payload as signal_build_text_payload, extract_signal_text,
    format_outbound as signal_format_outbound, is_signal_cli_available,
    normalize_inbound as signal_normalize_inbound, parse_signal_message,
};
pub use slack::SlackConnector;
pub use slack_client::build_slack_http_payload;
pub use telegram_client::build_telegram_http_payload;
pub use tlon::{extract_text as tlon_extract_text, parse_graph_entry as tlon_parse_entry};
pub use twitch::{build_privmsg as twitch_build_privmsg, parse_privmsg as twitch_parse_privmsg};
pub use whatsapp_client::build_whatsapp_text_payload;
pub use zalo::{build_text_payload as zalo_build_text, parse_event as zalo_parse_event};
pub use zalouser::{
    normalize_inbound as zalouser_normalize_inbound,
    normalize_target as zalouser_normalize_target,
    is_sender_allowed as zalouser_is_sender_allowed,
};
