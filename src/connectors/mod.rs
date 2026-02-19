//! Connectors — messaging platform adapters

// ── Core connectors ───────────────────────────────────────────────────────────
pub mod slack;
pub mod slack_client;
pub mod telegram;
pub mod telegram_client;

// ── Extended connectors (Phase 4–10) ─────────────────────────────────────────
pub mod discord;
pub mod discord_client;
pub mod bluebubbles;
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

// ── Phase 5-6 connectors (additional) ───────────────────────────────────────────
pub mod signal;
pub mod matrix;

// ── Public re-exports ─────────────────────────────────────────────────────────
pub use slack::SlackConnector;
pub use slack_client::build_slack_http_payload;
pub use telegram_client::build_telegram_http_payload;
pub use discord_client::build_discord_http_payload;
pub use bluebubbles::{normalize_inbound as bluebubbles_normalize_inbound, normalize_target as bluebubbles_normalize_target};
pub use line_client::{
    build_line_reply_payload,
    build_line_push_payload,
    build_line_broadcast_payload,
    build_line_rich_menu_payload,
};
pub use whatsapp_client::build_whatsapp_text_payload;
pub use irc::{parse_privmsg as irc_parse_privmsg, build_privmsg as irc_build_privmsg};
pub use msteams::{parse_activity as msteams_parse_activity, build_webhook_payload as msteams_build_webhook};
pub use mattermost::{parse_webhook as mattermost_parse_webhook, build_webhook_payload as mattermost_build_webhook};
pub use twitch::{parse_privmsg as twitch_parse_privmsg, build_privmsg as twitch_build_privmsg};
pub use zalo::{parse_event as zalo_parse_event, build_text_payload as zalo_build_text};
pub use googlechat::{parse_event as googlechat_parse_event, build_text_message as googlechat_build_text};
pub use feishu::{parse_event as feishu_parse_event, build_text_payload as feishu_build_text};
pub use nextcloud_talk::{parse_message as nctalk_parse_message};
pub use nostr::{parse_event as nostr_parse_event, parse_relay_message as nostr_parse_relay};
pub use tlon::{parse_graph_entry as tlon_parse_entry, extract_text as tlon_extract_text};
pub use signal::{parse_signal_message, extract_signal_text, normalize_inbound as signal_normalize_inbound, format_outbound as signal_format_outbound, build_text_payload as signal_build_text_payload, is_signal_cli_available};
pub use matrix::{parse_matrix_message, extract_matrix_text, normalize_inbound as matrix_normalize_inbound, format_outbound as matrix_format_outbound, build_text_message as matrix_build_text_message, build_html_message as matrix_build_html_message, parse_sync_response, extract_messages_from_sync};
