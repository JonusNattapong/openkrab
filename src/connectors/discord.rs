//! connectors::discord — Discord channel connector.
//! Ported from `openclaw/extensions/discord/src/channel.ts` (Phase 17).
//!
//! Provides Discord connector config types, capability declarations,
//! account resolution, DM policy helpers, action gates, message actions,
//! and the thread-safe runtime singleton with gateway lifecycle.

use crate::common::{Message, UserId};
pub use crate::connectors::discord_client::*;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use serenity::all::{Context, EventHandler, GatewayIntents, Message as SerenityMessage, Ready};
use std::sync::{Arc, OnceLock, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;

// ─── Runtime singleton ────────────────────────────────────────────────────────

/// Lightweight handle to the Discord runtime (injected at startup).
/// Mirrors `getDiscordRuntime()` / `setDiscordRuntime()` from `runtime.ts`.
#[derive(Clone)]
pub struct DiscordRuntime {
    pub label: String,
}

static DISCORD_RUNTIME: OnceLock<Arc<DiscordRuntime>> = OnceLock::new();
static DISCORD_STATUS: OnceLock<Arc<RwLock<DiscordStatusSnapshot>>> = OnceLock::new();

/// Install the Discord runtime. Must be called before the first `get_runtime()`.
pub fn set_runtime(rt: DiscordRuntime) {
    DISCORD_RUNTIME.get_or_init(|| Arc::new(rt));
}

/// Access the Discord runtime, or `Err` if not yet initialised.
pub fn get_runtime() -> Result<Arc<DiscordRuntime>> {
    DISCORD_RUNTIME
        .get()
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("Discord runtime not initialized"))
}

fn status_store() -> Arc<RwLock<DiscordStatusSnapshot>> {
    DISCORD_STATUS
        .get_or_init(|| {
            Arc::new(RwLock::new(DiscordStatusSnapshot {
                account_id: DEFAULT_ACCOUNT_ID.to_string(),
                ..Default::default()
            }))
        })
        .clone()
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

/// Return a copy of the latest Discord monitor status.
pub fn get_status_snapshot() -> DiscordStatusSnapshot {
    status_store()
        .read()
        .map(|s| s.clone())
        .unwrap_or_else(|_| DiscordStatusSnapshot::default())
}

fn set_running_status(running: bool) {
    if let Ok(mut s) = status_store().write() {
        s.running = running;
        if running {
            s.last_start_at = Some(now_ms());
            s.last_error = None;
        } else {
            s.last_stop_at = Some(now_ms());
        }
    }
}

fn set_error_status(message: impl Into<String>) {
    if let Ok(mut s) = status_store().write() {
        s.last_error = Some(message.into());
    }
}

fn mark_inbound() {
    if let Ok(mut s) = status_store().write() {
        s.last_inbound_at = Some(now_ms());
    }
}

fn mark_outbound() {
    if let Ok(mut s) = status_store().write() {
        s.last_outbound_at = Some(now_ms());
    }
}

fn initialize_monitor_status(account_id: &str, token_present: bool) {
    if let Ok(mut s) = status_store().write() {
        s.account_id = normalize_account_id(account_id);
        s.configured = token_present;
        s.token_source = Some("env".to_string());
    }
}

// ─── Constants ────────────────────────────────────────────────────────────────

pub const DEFAULT_ACCOUNT_ID: &str = "default";
pub const PAIRING_APPROVED_MESSAGE: &str = "✅ Pairing approved. You can now chat with me.";
/// Discord message character limit.
pub const TEXT_CHUNK_LIMIT: usize = 2000;
/// Maximum poll options Discord supports.
pub const POLL_MAX_OPTIONS: usize = 10;
/// Minimum chars before streaming is flushed.
pub const STREAM_MIN_CHARS: usize = 1500;
/// Streaming idle flush interval in ms.
pub const STREAM_IDLE_MS: u64 = 1000;

pub fn normalize_inbound(text: &str, channel_id: u64, user_id: u64, message_id: u64) -> Message {
    Message {
        id: format!("discord:{channel_id}:{user_id}:{message_id}"),
        text: text.to_string(),
        from: Some(UserId(format!("discord:{user_id}"))),
    }
}

pub fn format_outbound(text: &str) -> String {
    format!("[discord] {text}")
}

fn resolve_http_channel_target(normalized: &str) -> Result<String> {
    let lower = normalized.to_ascii_lowercase();
    if let Some(channel_id) = lower.strip_prefix("channel:") {
        let original = &normalized[8..];
        if channel_id.chars().all(|c| c.is_ascii_digit()) && !channel_id.is_empty() {
            return Ok(original.to_string());
        }
        bail!("Discord channel target must be numeric channel id (got: {normalized})");
    }
    if lower.starts_with("user:") {
        bail!(
            "Discord DM target is not supported in HTTP sender path yet. \
             Use gateway runtime send path or provide channel:<id>."
        );
    }
    bail!("Discord target must be channel:<id> or numeric id (got: {normalized})");
}

/// Send outbound Discord message via HTTP API path.
///
/// This helper validates and normalizes target before dispatch.
pub async fn send_outbound_message(
    client: &reqwest_middleware::ClientWithMiddleware,
    token: &str,
    to: Option<&str>,
    text: &str,
) -> Result<crate::connectors::discord_client::DiscordSendResult> {
    let normalized = normalize_outbound_target(to)?;
    let channel_id = resolve_http_channel_target(&normalized)?;
    crate::connectors::discord_client::send_message(client, token, &channel_id, text, None).await
}

// ─── Config types ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiscordDmConfig {
    pub policy: Option<DiscordDmPolicy>,
    pub allow_from: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DiscordDmPolicy {
    Open,
    Pairing,
    Closed,
}
impl Default for DiscordDmPolicy {
    fn default() -> Self {
        Self::Pairing
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DiscordGroupPolicy {
    Open,
    Allowlist,
    Closed,
}
impl Default for DiscordGroupPolicy {
    fn default() -> Self {
        Self::Open
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiscordGuildConfig {
    pub enabled: Option<bool>,
    pub channels: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiscordAccountConfig {
    pub token: Option<String>,
    pub name: Option<String>,
    pub enabled: Option<bool>,
    pub group_policy: Option<DiscordGroupPolicy>,
    pub dm: Option<DiscordDmConfig>,
    pub guilds: Option<std::collections::HashMap<String, DiscordGuildConfig>>,
    pub media_max_mb: Option<u64>,
    pub history_limit: Option<u64>,
    pub reply_to_mode: Option<ReplyToMode>,
    pub actions: Option<DiscordActionConfig>,
    pub text_chunk_limit: Option<usize>,
    pub max_lines_per_message: Option<usize>,
    pub allow_bots: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReplyToMode {
    Off,
    Reply,
    Thread,
}
impl Default for ReplyToMode {
    fn default() -> Self {
        Self::Off
    }
}

// ─── Action Config (port of openclaw/src/config/types.discord.ts:DiscordActionConfig) ───────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordActionConfig {
    #[serde(default = "default_true")]
    pub reactions: bool,
    #[serde(default = "default_true")]
    pub stickers: bool,
    #[serde(default = "default_true")]
    pub polls: bool,
    #[serde(default = "default_true")]
    pub permissions: bool,
    #[serde(default = "default_true")]
    pub messages: bool,
    #[serde(default = "default_true")]
    pub threads: bool,
    #[serde(default = "default_true")]
    pub pins: bool,
    #[serde(default = "default_true")]
    pub search: bool,
    #[serde(default = "default_true")]
    pub member_info: bool,
    #[serde(default = "default_true")]
    pub role_info: bool,
    #[serde(default)]
    pub roles: bool,
    #[serde(default = "default_true")]
    pub channel_info: bool,
    #[serde(default = "default_true")]
    pub voice_status: bool,
    #[serde(default = "default_true")]
    pub events: bool,
    #[serde(default)]
    pub moderation: bool,
    #[serde(default = "default_true")]
    pub emoji_uploads: bool,
    #[serde(default = "default_true")]
    pub sticker_uploads: bool,
    #[serde(default = "default_true")]
    pub channels: bool,
    #[serde(default)]
    pub presence: bool,
}

fn default_true() -> bool {
    true
}

impl Default for DiscordActionConfig {
    fn default() -> Self {
        Self {
            reactions: true,
            stickers: true,
            polls: true,
            permissions: true,
            messages: true,
            threads: true,
            pins: true,
            search: true,
            member_info: true,
            role_info: true,
            roles: false,
            channel_info: true,
            voice_status: true,
            events: true,
            moderation: false,
            emoji_uploads: true,
            sticker_uploads: true,
            channels: true,
            presence: false,
        }
    }
}

// ─── Action Gate (port of openclaw/src/discord/accounts.ts:createDiscordActionGate) ───────

pub fn create_action_gate(
    base_actions: Option<&DiscordActionConfig>,
    account_actions: Option<&DiscordActionConfig>,
) -> impl Fn(&str, bool) -> bool + Clone {
    let base = base_actions.cloned();
    let account = account_actions.cloned();

    move |key: &str, default_value: bool| -> bool {
        let check = |config: &DiscordActionConfig, k: &str| -> Option<bool> {
            match k {
                "reactions" => Some(config.reactions),
                "stickers" => Some(config.stickers),
                "polls" => Some(config.polls),
                "permissions" => Some(config.permissions),
                "messages" => Some(config.messages),
                "threads" => Some(config.threads),
                "pins" => Some(config.pins),
                "search" => Some(config.search),
                "memberInfo" => Some(config.member_info),
                "roleInfo" => Some(config.role_info),
                "roles" => Some(config.roles),
                "channelInfo" => Some(config.channel_info),
                "voiceStatus" => Some(config.voice_status),
                "events" => Some(config.events),
                "moderation" => Some(config.moderation),
                "emojiUploads" => Some(config.emoji_uploads),
                "stickerUploads" => Some(config.sticker_uploads),
                "channels" => Some(config.channels),
                "presence" => Some(config.presence),
                _ => None,
            }
        };

        // Check account first (if provided), then base (if provided), then default
        if let Some(ref acc) = account {
            if let Some(v) = check(acc, key) {
                return v;
            }
        }
        if let Some(ref base_cfg) = base {
            if let Some(v) = check(base_cfg, key) {
                return v;
            }
        }
        default_value
    }
}

// ─── Message Actions (port of openclaw/src/channels/plugins/actions/discord.ts) ─────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MessageAction {
    Send,
    Poll,
    React,
    Reactions,
    Read,
    Edit,
    Delete,
    Pin,
    Unpin,
    ListPins,
    Permissions,
    ThreadCreate,
    ThreadList,
    ThreadReply,
    Search,
    Sticker,
    MemberInfo,
    RoleInfo,
    EmojiList,
    EmojiUpload,
    StickerUpload,
    RoleAdd,
    RoleRemove,
    ChannelInfo,
    ChannelList,
    ChannelCreate,
    ChannelEdit,
    ChannelDelete,
    ChannelMove,
    CategoryCreate,
    CategoryEdit,
    CategoryDelete,
    VoiceStatus,
    EventList,
    EventCreate,
    Timeout,
    Kick,
    Ban,
    SetPresence,
}

pub fn list_message_actions(gate: &impl Fn(&str, bool) -> bool) -> Vec<MessageAction> {
    let mut actions = vec![MessageAction::Send];

    if gate("polls", true) {
        actions.push(MessageAction::Poll);
    }
    if gate("reactions", true) {
        actions.extend([
            MessageAction::React,
            MessageAction::Reactions,
            MessageAction::EmojiList,
        ]);
    }
    if gate("messages", true) {
        actions.extend([
            MessageAction::Read,
            MessageAction::Edit,
            MessageAction::Delete,
        ]);
    }
    if gate("pins", true) {
        actions.extend([
            MessageAction::Pin,
            MessageAction::Unpin,
            MessageAction::ListPins,
        ]);
    }
    if gate("permissions", true) {
        actions.push(MessageAction::Permissions);
    }
    if gate("threads", true) {
        actions.extend([
            MessageAction::ThreadCreate,
            MessageAction::ThreadList,
            MessageAction::ThreadReply,
        ]);
    }
    if gate("search", true) {
        actions.push(MessageAction::Search);
    }
    if gate("stickers", true) {
        actions.push(MessageAction::Sticker);
    }
    if gate("memberInfo", true) {
        actions.push(MessageAction::MemberInfo);
    }
    if gate("roleInfo", true) {
        actions.push(MessageAction::RoleInfo);
    }
    if gate("emojiUploads", true) {
        actions.push(MessageAction::EmojiUpload);
    }
    if gate("stickerUploads", true) {
        actions.push(MessageAction::StickerUpload);
    }
    if gate("roles", false) {
        actions.extend([MessageAction::RoleAdd, MessageAction::RoleRemove]);
    }
    if gate("channelInfo", true) {
        actions.extend([MessageAction::ChannelInfo, MessageAction::ChannelList]);
    }
    if gate("channels", true) {
        actions.extend([
            MessageAction::ChannelCreate,
            MessageAction::ChannelEdit,
            MessageAction::ChannelDelete,
            MessageAction::ChannelMove,
            MessageAction::CategoryCreate,
            MessageAction::CategoryEdit,
            MessageAction::CategoryDelete,
        ]);
    }
    if gate("voiceStatus", true) {
        actions.push(MessageAction::VoiceStatus);
    }
    if gate("events", true) {
        actions.extend([MessageAction::EventList, MessageAction::EventCreate]);
    }
    if gate("moderation", false) {
        actions.extend([
            MessageAction::Timeout,
            MessageAction::Kick,
            MessageAction::Ban,
        ]);
    }
    if gate("presence", false) {
        actions.push(MessageAction::SetPresence);
    }

    actions
}

/// Token source indicator (env-var vs explicit config).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TokenSource {
    Env,
    Config,
    None,
}

// ─── Resolved account ─────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ResolvedDiscordAccount {
    pub account_id: String,
    pub token: Option<String>,
    pub name: Option<String>,
    pub enabled: bool,
    pub token_source: TokenSource,
    pub config: DiscordAccountConfig,
}

impl ResolvedDiscordAccount {
    pub fn is_configured(&self) -> bool {
        self.token
            .as_deref()
            .map(|t| !t.trim().is_empty())
            .unwrap_or(false)
    }
}

// ─── Capability declarations ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct DiscordCapabilities {
    pub chat_types: Vec<&'static str>,
    pub polls: bool,
    pub reactions: bool,
    pub threads: bool,
    pub media: bool,
    pub native_commands: bool,
}

impl Default for DiscordCapabilities {
    fn default() -> Self {
        Self {
            chat_types: vec!["direct", "channel", "thread"],
            polls: true,
            reactions: true,
            threads: true,
            media: true,
            native_commands: true,
        }
    }
}

// ─── Target normalization ─────────────────────────────────────────────────────

/// Normalize a raw Discord target string to a canonical form.
/// Strips `discord:`, `user:`, `channel:` prefixes.
pub fn normalize_target(raw: &str) -> String {
    let s = raw.trim();
    // Strip mention format <@123> or <@!123>
    if s.starts_with("<@") && s.ends_with('>') {
        let inner = s
            .trim_start_matches("<@!")
            .trim_start_matches("<@")
            .trim_end_matches('>');
        return inner.to_string();
    }
    // Strip common prefixes
    for prefix in &["discord:", "user:", "channel:"] {
        if let Some(rest) = s.strip_prefix(prefix) {
            return rest.to_string();
        }
    }
    s.to_string()
}

/// Normalize target for messaging/routing.
/// Bare numeric IDs are treated as channels for stable routing parity.
pub fn normalize_messaging_target(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    if trimmed.chars().all(|c| c.is_ascii_digit()) {
        return Some(format!("channel:{trimmed}"));
    }
    if trimmed.starts_with("<@") && trimmed.ends_with('>') {
        return Some(format!("user:{}", normalize_target(trimmed)));
    }
    if trimmed.to_ascii_lowercase().starts_with("discord:") {
        return Some(format!("user:{}", normalize_target(trimmed)));
    }
    if trimmed.to_ascii_lowercase().starts_with("user:")
        || trimmed.to_ascii_lowercase().starts_with("channel:")
    {
        return Some(trimmed.to_string());
    }
    Some(format!("channel:{trimmed}"))
}

/// Normalize target for outbound sending.
/// Bare numeric IDs are converted to `channel:<id>`.
pub fn normalize_outbound_target(to: Option<&str>) -> Result<String> {
    let trimmed = to.unwrap_or_default().trim();
    if trimmed.is_empty() {
        bail!(
            "Discord recipient is required. Use \"channel:<id>\" for channels or \"user:<id>\" for DMs."
        );
    }
    if trimmed.chars().all(|c| c.is_ascii_digit()) {
        return Ok(format!("channel:{trimmed}"));
    }
    Ok(trimmed.to_string())
}

/// Return true if the input looks like a Discord snowflake ID or `user:`/`channel:` target.
pub fn looks_like_discord_target_id(input: &str) -> bool {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return false;
    }
    if trimmed.starts_with("<@") && trimmed.ends_with('>') {
        return true;
    }
    if trimmed.to_ascii_lowercase().starts_with("user:")
        || trimmed.to_ascii_lowercase().starts_with("channel:")
        || trimmed.to_ascii_lowercase().starts_with("discord:")
    {
        return true;
    }
    trimmed.chars().all(|c| c.is_ascii_digit()) && trimmed.len() >= 6
}

/// Normalize allow-from entry (strip provider prefix for storage).
pub fn normalize_allow_entry(entry: &str) -> String {
    let cleaned = entry
        .trim()
        .trim_start_matches("discord:")
        .trim_start_matches("user:")
        .to_string();
    if cleaned.starts_with("<@") && cleaned.ends_with('>') {
        cleaned
            .trim_start_matches("<@!")
            .trim_start_matches("<@")
            .trim_end_matches('>')
            .to_string()
    } else {
        cleaned
    }
}

// ─── DM policy helpers ────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct DmPolicyResolution {
    pub policy: DiscordDmPolicy,
    pub allow_from: Vec<String>,
    pub allow_from_path: String,
    pub approve_hint: String,
}

pub fn resolve_dm_policy(account: &ResolvedDiscordAccount) -> DmPolicyResolution {
    let policy = account
        .config
        .dm
        .as_ref()
        .and_then(|dm| dm.policy.clone())
        .unwrap_or_default();
    let allow_from = account
        .config
        .dm
        .as_ref()
        .and_then(|dm| dm.allow_from.clone())
        .unwrap_or_default()
        .into_iter()
        .map(|entry| normalize_allow_entry(&entry))
        .filter(|entry| !entry.trim().is_empty())
        .collect();
    let allow_from_path = format!("channels.discord.dm.");
    DmPolicyResolution {
        policy,
        allow_from,
        allow_from_path,
        approve_hint: format!("krabkrab pairing approve discord <discordUserId>"),
    }
}

/// Evaluate whether a DM sender is allowed to trigger the bot.
///
/// `pairing_approved` represents external pairing state lookup.
pub fn is_dm_allowed(sender_id: &str, policy: &DmPolicyResolution, pairing_approved: bool) -> bool {
    let normalized_sender = normalize_allow_entry(sender_id);
    match policy.policy {
        DiscordDmPolicy::Open => true,
        DiscordDmPolicy::Closed => false,
        DiscordDmPolicy::Pairing => {
            pairing_approved
                || policy
                    .allow_from
                    .iter()
                    .any(|allowed| allowed == &normalized_sender)
        }
    }
}

// ─── Group policy helpers ─────────────────────────────────────────────────────

pub fn resolve_group_policy(account: &ResolvedDiscordAccount) -> DiscordGroupPolicy {
    account.config.group_policy.clone().unwrap_or_default()
}

/// Collect warnings about insecure group configuration.
pub fn collect_config_warnings(account: &ResolvedDiscordAccount) -> Vec<String> {
    let mut warnings = Vec::new();
    let group_policy = resolve_group_policy(account);
    let guilds_configured = account
        .config
        .guilds
        .as_ref()
        .map(|g| !g.is_empty())
        .unwrap_or(false);

    if group_policy == DiscordGroupPolicy::Open {
        if guilds_configured {
            warnings.push(
                "- Discord guilds: groupPolicy=\"open\" allows any channel not explicitly denied \
                 to trigger (mention-gated). Set channels.discord.groupPolicy=\"allowlist\"."
                    .into(),
            );
        } else {
            warnings.push(
                "- Discord guilds: groupPolicy=\"open\" with no guild/channel allowlist; \
                 any channel can trigger (mention-gated). Set channels.discord.groupPolicy=\"allowlist\"."
                    .into(),
            );
        }
    }
    warnings
}

// ─── Account ID normalization ─────────────────────────────────────────────────

pub fn normalize_account_id(raw: &str) -> String {
    let s = raw.trim().to_lowercase();
    if s.is_empty() {
        DEFAULT_ACCOUNT_ID.to_string()
    } else {
        s
    }
}

// ─── Status snapshot ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiscordStatusSnapshot {
    pub account_id: String,
    pub configured: bool,
    pub token_source: Option<String>,
    pub running: bool,
    pub last_start_at: Option<u64>,
    pub last_stop_at: Option<u64>,
    pub last_error: Option<String>,
    pub last_inbound_at: Option<u64>,
    pub last_outbound_at: Option<u64>,
}

// ─── Gateway monitor lifecycle ────────────────────────────────────────────────

fn next_retry_delay_ms(attempt: u32) -> u64 {
    // 1s, 2s, 4s, ... up to 30s
    let shift = attempt.saturating_sub(1).min(5);
    let base = 1_000u64.saturating_mul(1u64 << shift);
    base.min(30_000)
}

fn chunk_text(text: &str, max_chars: usize) -> Vec<String> {
    if text.is_empty() {
        return vec![];
    }
    if text.chars().count() <= max_chars {
        return vec![text.to_string()];
    }

    let mut chunks = Vec::new();
    let mut current = String::new();
    let mut count = 0usize;

    for ch in text.chars() {
        current.push(ch);
        count += 1;
        if count >= max_chars {
            chunks.push(current);
            current = String::new();
            count = 0;
        }
    }
    if !current.is_empty() {
        chunks.push(current);
    }
    chunks
}

struct DiscordEventHandler {
    state: Arc<crate::gateway::GatewayState>,
}

#[serenity::async_trait]
impl EventHandler for DiscordEventHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        tracing::info!("[discord] connected as {}", ready.user.name);
        set_running_status(true);
    }

    async fn message(&self, ctx: Context, msg: SerenityMessage) {
        if msg.author.bot || msg.content.trim().is_empty() {
            return;
        }

        let inbound = normalize_inbound(
            &msg.content,
            msg.channel_id.get(),
            msg.author.id.get(),
            msg.id.get(),
        );
        tracing::debug!("[discord] inbound={:?}", inbound);

        mark_inbound();
        
        let agent = match &self.state.agent {
            Some(agent) => agent,
            None => {
                let _ = msg.channel_id.say(&ctx.http, "Agent not available").await;
                return;
            }
        };
        
        let answer = agent.answer(&inbound.text).await;
        match answer {
            Ok(text) => {
                let chunks = chunk_text(&text, TEXT_CHUNK_LIMIT);
                for part in chunks {
                    if part.trim().is_empty() {
                        continue;
                    }
                    if let Err(e) = msg.channel_id.say(&ctx.http, &part).await {
                        set_error_status(format!("send failed: {}", e));
                        break;
                    }
                    mark_outbound();
                }
            }
            Err(e) => {
                set_error_status(format!("agent failed: {}", e));
                let fallback = format!("unavailable: {}", e);
                if msg.channel_id.say(&ctx.http, &fallback).await.is_ok() {
                    mark_outbound();
                }
            }
        }
    }
}

async fn run_gateway_session(state: Arc<crate::gateway::GatewayState>, token: &str) -> Result<()> {
    let intents = GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let handler = DiscordEventHandler { state };
    let mut client = serenity::Client::builder(token, intents)
        .event_handler(handler)
        .await
        .map_err(|e| anyhow::anyhow!("discord client init failed: {}", e))?;

    set_running_status(true);
    let run = client.start().await;
    set_running_status(false);

    run.map_err(|e| anyhow::anyhow!("discord gateway stopped: {}", e))
}

/// Start the Discord gateway WebSocket monitor for a bot session.
///
/// Runs forever with reconnect/backoff until process shutdown.
pub async fn monitor(state: std::sync::Arc<crate::gateway::GatewayState>, token: String) {
    initialize_monitor_status(DEFAULT_ACCOUNT_ID, !token.trim().is_empty());
    let mut attempt = 0u32;
    loop {
        match run_gateway_session(state.clone(), &token).await {
            Ok(_) => {
                attempt = 0;
                sleep(Duration::from_millis(250)).await;
            }
            Err(e) => {
                attempt = attempt.saturating_add(1);
                let delay_ms = next_retry_delay_ms(attempt);
                set_error_status(e.to_string());
                tracing::warn!(
                    "[discord] gateway session failed (attempt {}), retry in {}ms: {}",
                    attempt,
                    delay_ms,
                    e
                );
                sleep(Duration::from_millis(delay_ms)).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_target_strips_mention() {
        assert_eq!(
            normalize_target("<@!123456789012345678>"),
            "123456789012345678"
        );
        assert_eq!(
            normalize_target("<@123456789012345678>"),
            "123456789012345678"
        );
    }

    #[test]
    fn normalize_target_strips_prefix() {
        assert_eq!(
            normalize_target("user:12345678901234567"),
            "12345678901234567"
        );
        assert_eq!(
            normalize_target("discord:12345678901234567"),
            "12345678901234567"
        );
        assert_eq!(
            normalize_target("channel:12345678901234567"),
            "12345678901234567"
        );
    }

    #[test]
    fn looks_like_discord_target_snowflake() {
        assert!(looks_like_discord_target_id("123456")); // 6 digits
        assert!(!looks_like_discord_target_id("12345")); // too short for heuristic
        assert!(looks_like_discord_target_id("<@123456789012345678>"));
        assert!(looks_like_discord_target_id("user:123456789012345678"));
        assert!(!looks_like_discord_target_id("abc"));
    }

    #[test]
    fn normalize_messaging_target_defaults_numeric_to_channel() {
        assert_eq!(
            normalize_messaging_target("123"),
            Some("channel:123".to_string())
        );
    }

    #[test]
    fn normalize_messaging_target_maps_mentions_and_prefixes() {
        assert_eq!(
            normalize_messaging_target("<@!456>"),
            Some("user:456".to_string())
        );
        assert_eq!(
            normalize_messaging_target("discord:987"),
            Some("user:987".to_string())
        );
        assert_eq!(
            normalize_messaging_target("general"),
            Some("channel:general".to_string())
        );
    }

    #[test]
    fn normalize_outbound_target_parity() {
        assert_eq!(
            normalize_outbound_target(Some("1470130713209602050")).unwrap(),
            "channel:1470130713209602050"
        );
        assert_eq!(
            normalize_outbound_target(Some("channel:123")).unwrap(),
            "channel:123"
        );
        assert_eq!(
            normalize_outbound_target(Some("user:123")).unwrap(),
            "user:123"
        );
        assert_eq!(
            normalize_outbound_target(Some("general")).unwrap(),
            "general"
        );
        assert_eq!(
            normalize_outbound_target(Some("  123  ")).unwrap(),
            "channel:123"
        );
        assert!(normalize_outbound_target(Some("")).is_err());
        assert!(normalize_outbound_target(None).is_err());
    }

    #[test]
    fn normalize_allow_entry_strips_prefix() {
        assert_eq!(normalize_allow_entry("discord:111222333"), "111222333");
        assert_eq!(normalize_allow_entry("user:111222333"), "111222333");
        assert_eq!(normalize_allow_entry("111222333"), "111222333");
        assert_eq!(normalize_allow_entry("<@!111222333>"), "111222333");
        assert_eq!(normalize_allow_entry("<@111222333>"), "111222333");
    }

    #[test]
    fn collect_config_warnings_open_policy() {
        let account = ResolvedDiscordAccount {
            account_id: "default".into(),
            token: None,
            name: None,
            enabled: true,
            token_source: TokenSource::None,
            config: DiscordAccountConfig {
                group_policy: Some(DiscordGroupPolicy::Open),
                ..Default::default()
            },
        };
        let warnings = collect_config_warnings(&account);
        assert!(!warnings.is_empty());
        assert!(warnings[0].contains("groupPolicy=\"open\""));
    }

    #[test]
    fn collect_config_warnings_allowlist_no_warn() {
        let account = ResolvedDiscordAccount {
            account_id: "default".into(),
            token: None,
            name: None,
            enabled: true,
            token_source: TokenSource::None,
            config: DiscordAccountConfig {
                group_policy: Some(DiscordGroupPolicy::Allowlist),
                ..Default::default()
            },
        };
        assert!(collect_config_warnings(&account).is_empty());
    }

    #[test]
    fn resolve_dm_policy_normalizes_allow_from_entries() {
        let account = ResolvedDiscordAccount {
            account_id: "default".into(),
            token: Some("token".into()),
            name: None,
            enabled: true,
            token_source: TokenSource::Config,
            config: DiscordAccountConfig {
                dm: Some(DiscordDmConfig {
                    policy: Some(DiscordDmPolicy::Pairing),
                    allow_from: Some(vec![
                        "discord:111222333".into(),
                        "user:444555666".into(),
                        "  ".into(),
                    ]),
                }),
                ..Default::default()
            },
        };

        let resolved = resolve_dm_policy(&account);
        assert_eq!(resolved.policy, DiscordDmPolicy::Pairing);
        assert_eq!(resolved.allow_from, vec!["111222333", "444555666"]);
    }

    #[test]
    fn is_dm_allowed_open_always_true() {
        let policy = DmPolicyResolution {
            policy: DiscordDmPolicy::Open,
            allow_from: vec![],
            allow_from_path: String::new(),
            approve_hint: String::new(),
        };
        assert!(is_dm_allowed("user:123", &policy, false));
    }

    #[test]
    fn is_dm_allowed_closed_always_false() {
        let policy = DmPolicyResolution {
            policy: DiscordDmPolicy::Closed,
            allow_from: vec!["123".into()],
            allow_from_path: String::new(),
            approve_hint: String::new(),
        };
        assert!(!is_dm_allowed("123", &policy, true));
        assert!(!is_dm_allowed("123", &policy, false));
    }

    #[test]
    fn is_dm_allowed_pairing_by_allowlist_or_pairing_state() {
        let policy = DmPolicyResolution {
            policy: DiscordDmPolicy::Pairing,
            allow_from: vec!["555".into()],
            allow_from_path: String::new(),
            approve_hint: String::new(),
        };
        assert!(is_dm_allowed("discord:555", &policy, false)); // allowlist hit
        assert!(!is_dm_allowed("discord:777", &policy, false)); // no allowlist and not paired
        assert!(is_dm_allowed("discord:777", &policy, true)); // paired externally
    }

    #[test]
    fn resolved_account_is_configured() {
        let mut acc = ResolvedDiscordAccount {
            account_id: "default".into(),
            token: Some("  ".into()), // whitespace only
            name: None,
            enabled: true,
            token_source: TokenSource::None,
            config: Default::default(),
        };
        assert!(!acc.is_configured());
        acc.token = Some("BOTTOKEN123".into());
        assert!(acc.is_configured());
    }

    #[test]
    fn discord_capabilities_default() {
        let caps = DiscordCapabilities::default();
        assert!(caps.polls);
        assert!(caps.threads);
        assert!(caps.chat_types.contains(&"direct"));
    }

    #[test]
    fn normalize_account_id_empty_returns_default() {
        assert_eq!(normalize_account_id("  "), DEFAULT_ACCOUNT_ID);
        assert_eq!(normalize_account_id("MyAccount"), "myaccount");
    }

    #[test]
    fn normalize_inbound_builds_common_message() {
        let msg = normalize_inbound("hello", 123, 456, 789);
        assert_eq!(msg.id, "discord:123:456:789");
        assert_eq!(msg.text, "hello");
        assert_eq!(msg.from.as_ref().map(|u| u.0.as_str()), Some("discord:456"));
    }

    #[test]
    fn format_outbound_prefixes_discord() {
        assert_eq!(format_outbound("hi"), "[discord] hi");
    }

    #[test]
    fn resolve_http_channel_target_accepts_numeric_channel() {
        assert_eq!(
            resolve_http_channel_target("channel:123456").unwrap(),
            "123456"
        );
    }

    #[test]
    fn resolve_http_channel_target_rejects_user_target() {
        let err = resolve_http_channel_target("user:123456").unwrap_err();
        assert!(err
            .to_string()
            .contains("not supported in HTTP sender path"));
    }

    #[test]
    fn resolve_http_channel_target_rejects_non_numeric_channel() {
        let err = resolve_http_channel_target("channel:general").unwrap_err();
        assert!(err.to_string().contains("must be numeric"));
    }

    #[test]
    fn retry_backoff_is_capped() {
        assert_eq!(next_retry_delay_ms(1), 1_000);
        assert_eq!(next_retry_delay_ms(2), 2_000);
        assert_eq!(next_retry_delay_ms(3), 4_000);
        assert_eq!(next_retry_delay_ms(10), 30_000);
    }

    #[test]
    fn chunk_text_respects_limit() {
        let chunks = chunk_text("abcdefghij", 4);
        assert_eq!(chunks, vec!["abcd", "efgh", "ij"]);
    }

    #[test]
    fn status_snapshot_running_transition() {
        set_running_status(true);
        let running = get_status_snapshot();
        assert!(running.running);
        assert!(running.last_start_at.is_some());

        set_running_status(false);
        let stopped = get_status_snapshot();
        assert!(!stopped.running);
        assert!(stopped.last_stop_at.is_some());
    }

    #[test]
    fn initialize_monitor_status_sets_env_source() {
        initialize_monitor_status("Default", true);
        let s = get_status_snapshot();
        assert_eq!(s.account_id, "default");
        assert!(s.configured);
        assert_eq!(s.token_source.as_deref(), Some("env"));
    }

    // ─── Action Gate Tests ──────────────────────────────────────────────────────

    #[test]
    fn action_gate_defaults() {
        let gate = create_action_gate(None, None);
        assert!(gate("reactions", true));
        assert!(gate("polls", true));
        assert!(!gate("roles", false));
        assert!(!gate("moderation", false));
        assert!(!gate("presence", false));
    }

    #[test]
    fn action_gate_account_overrides_base() {
        let base = DiscordActionConfig {
            reactions: true,
            polls: true,
            ..Default::default()
        };
        let account = DiscordActionConfig {
            reactions: false,
            polls: true,
            ..Default::default()
        };

        let gate = create_action_gate(Some(&base), Some(&account));
        assert!(!gate("reactions", true)); // account override
        assert!(gate("polls", true)); // both true
        assert!(!gate("roles", false)); // default
    }

    #[test]
    fn action_gate_base_used_when_account_missing() {
        let base = DiscordActionConfig {
            moderation: true,
            ..Default::default()
        };

        let gate = create_action_gate(Some(&base), None);
        assert!(gate("moderation", false)); // base override
        assert!(gate("reactions", true)); // default true
    }

    #[test]
    fn list_message_actions_basic() {
        let gate = |key: &str, default: bool| -> bool {
            matches!(key, "polls" | "reactions" | "messages" | "threads") || default
        };
        let actions = list_message_actions(&gate);

        assert!(actions.contains(&MessageAction::Send));
        assert!(actions.contains(&MessageAction::Poll));
        assert!(actions.contains(&MessageAction::React));
        assert!(actions.contains(&MessageAction::Read));
    }

    #[test]
    fn list_message_actions_moderation_disabled_by_default() {
        let gate = |_key: &str, default: bool| default;
        let actions = list_message_actions(&gate);

        assert!(!actions.contains(&MessageAction::Timeout));
        assert!(!actions.contains(&MessageAction::Kick));
        assert!(!actions.contains(&MessageAction::Ban));
    }

    #[test]
    fn list_message_actions_moderation_enabled() {
        let gate = |key: &str, _default: bool| key == "moderation";
        let actions = list_message_actions(&gate);

        assert!(actions.contains(&MessageAction::Timeout));
        assert!(actions.contains(&MessageAction::Kick));
        assert!(actions.contains(&MessageAction::Ban));
    }

    #[test]
    fn discord_action_config_default_values() {
        let config = DiscordActionConfig::default();
        assert!(config.reactions);
        assert!(config.polls);
        assert!(config.messages);
        assert!(config.threads);
        assert!(!config.roles);
        assert!(!config.moderation);
        assert!(!config.presence);
    }

    #[test]
    fn message_action_serialization() {
        let action = MessageAction::ThreadCreate;
        let json = serde_json::to_string(&action).unwrap();
        assert_eq!(json, "\"thread-create\"");

        let parsed: MessageAction = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, MessageAction::ThreadCreate);
    }
}
