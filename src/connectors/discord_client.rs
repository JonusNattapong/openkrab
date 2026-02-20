//! Discord HTTP API client — ported from `openkrab/src/discord/send.*.ts`.
//!
//! Provides comprehensive Discord API functionality:
//! - Message send/edit/delete/pin/fetch
//! - Polls, stickers, reactions
//! - Thread management
//! - Guild actions (channels, members, roles)
//! - Typing indicator, voice messages

use anyhow::{bail, Result};
use reqwest_middleware::ClientWithMiddleware as Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

const DISCORD_API_BASE: &str = "https://discord.com/api/v10";
const SUPPRESS_NOTIFICATIONS_FLAG: u64 = 1 << 12;

// ─── Send Result Types ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordSendResult {
    pub message_id: String,
    pub channel_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordChannelInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: i32,
    pub name: Option<String>,
    pub guild_id: Option<String>,
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordMemberInfo {
    pub user: DiscordUserInfo,
    pub roles: Vec<String>,
    pub nick: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordUserInfo {
    pub id: String,
    pub username: String,
    pub discriminator: Option<String>,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordRoleInfo {
    pub id: String,
    pub name: String,
    pub color: i32,
    pub position: i32,
    pub permissions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordMessage {
    pub id: String,
    pub channel_id: String,
    pub content: Option<String>,
    pub author: DiscordUserInfo,
    pub timestamp: String,
    pub referenced_message: Option<Box<DiscordMessage>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordReaction {
    pub emoji: DiscordEmoji,
    pub count: i32,
    pub me: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordEmoji {
    pub id: Option<String>,
    pub name: Option<String>,
    pub animated: Option<bool>,
}

// ─── Poll Types ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollInput {
    pub question: String,
    pub answers: Vec<PollAnswer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_hours: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_multiselect: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollAnswer {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<PollEmoji>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollEmoji {
    pub id: Option<String>,
    pub name: Option<String>,
    pub animated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiscordPollMedia {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji: Option<PollEmoji>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiscordPollAnswer {
    poll_media: DiscordPollMedia,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiscordPollPayload {
    question: DiscordPollMedia,
    answers: Vec<DiscordPollAnswer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_multiselect: Option<bool>,
}

// ─── Embed Types ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiscordEmbed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<DiscordEmbedFooter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<DiscordEmbedImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<DiscordEmbedThumbnail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<DiscordEmbedAuthor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<DiscordEmbedField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordEmbedFooter {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordEmbedImage {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordEmbedThumbnail {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordEmbedAuthor {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordEmbedField {
    pub name: String,
    pub value: String,
    #[serde(default)]
    pub inline: bool,
}

// ─── Send Options ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct SendOptions {
    pub reply_to: Option<String>,
    pub embeds: Option<Vec<DiscordEmbed>>,
    pub silent: bool,
}

// ─── HTTP Payload Builders ─────────────────────────────────────────────────────

fn build_message_payload(content: &str, opts: &SendOptions) -> serde_json::Value {
    let mut payload = json!({
        "content": content,
    });

    if let Some(ref reply_to) = opts.reply_to {
        payload["message_reference"] = json!({
            "message_id": reply_to,
        });
    }

    if let Some(ref embeds) = opts.embeds {
        if !embeds.is_empty() {
            payload["embeds"] = serde_json::to_value(embeds).unwrap();
        }
    }

    if opts.silent {
        payload["flags"] = json!(SUPPRESS_NOTIFICATIONS_FLAG);
    }

    payload
}

/// Build a Discord JSON payload for `POST /channels/{channel_id}/messages`.
pub fn build_discord_http_payload(
    content: &str,
    reply_to_message_id: Option<&str>,
    silent: bool,
) -> serde_json::Value {
    let opts = SendOptions {
        reply_to: reply_to_message_id.map(ToString::to_string),
        embeds: None,
        silent,
    };
    build_message_payload(content, &opts)
}

fn build_poll_payload(poll: &PollInput, content: Option<&str>, silent: bool) -> serde_json::Value {
    let discord_poll = DiscordPollPayload {
        question: DiscordPollMedia {
            text: poll.question.clone(),
            emoji: None,
        },
        answers: poll
            .answers
            .iter()
            .map(|a| DiscordPollAnswer {
                poll_media: DiscordPollMedia {
                    text: a.text.clone(),
                    emoji: a.emoji.clone(),
                },
            })
            .collect(),
        duration: poll.duration_hours,
        allow_multiselect: poll.allow_multiselect,
    };

    let mut payload = json!({
        "poll": discord_poll,
    });

    if let Some(text) = content {
        payload["content"] = json!(text);
    }

    if silent {
        payload["flags"] = json!(SUPPRESS_NOTIFICATIONS_FLAG);
    }

    payload
}

// ─── Core Send Functions ───────────────────────────────────────────────────────

pub async fn send_message(
    client: &Client,
    token: &str,
    channel_id: &str,
    content: &str,
    opts: Option<SendOptions>,
) -> Result<DiscordSendResult> {
    let url = format!("{}/channels/{}/messages", DISCORD_API_BASE, channel_id);
    let opts = opts.unwrap_or_default();
    let payload = build_message_payload(content, &opts);

    let resp = client
        .post(&url)
        .header("Authorization", format!("Bot {}", token))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Discord API error ({}): {}", status, body);
    }

    let data: DiscordMessage = resp.json().await?;
    Ok(DiscordSendResult {
        message_id: data.id,
        channel_id: data.channel_id,
    })
}

pub async fn send_poll(
    client: &Client,
    token: &str,
    channel_id: &str,
    poll: &PollInput,
    content: Option<&str>,
    silent: bool,
) -> Result<DiscordSendResult> {
    let url = format!("{}/channels/{}/messages", DISCORD_API_BASE, channel_id);
    let payload = build_poll_payload(poll, content, silent);

    let resp = client
        .post(&url)
        .header("Authorization", format!("Bot {}", token))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Discord API error ({}): {}", status, body);
    }

    let data: DiscordMessage = resp.json().await?;
    Ok(DiscordSendResult {
        message_id: data.id,
        channel_id: data.channel_id,
    })
}

pub async fn send_typing(client: &Client, token: &str, channel_id: &str) -> Result<()> {
    let url = format!("{}/channels/{}/typing", DISCORD_API_BASE, channel_id);
    let resp = client
        .post(&url)
        .header("Authorization", format!("Bot {}", token))
        .send()
        .await?;

    if !resp.status().is_success() {
        bail!("Failed to trigger typing indicator");
    }
    Ok(())
}

// ─── Message Actions ───────────────────────────────────────────────────────────

pub async fn edit_message(
    client: &Client,
    token: &str,
    channel_id: &str,
    message_id: &str,
    content: &str,
) -> Result<DiscordSendResult> {
    let url = format!(
        "{}/channels/{}/messages/{}",
        DISCORD_API_BASE, channel_id, message_id
    );
    let payload = json!({ "content": content });

    let resp = client
        .patch(&url)
        .header("Authorization", format!("Bot {}", token))
        .json(&payload)
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Discord API error ({}): {}", status, body);
    }

    let data: DiscordMessage = resp.json().await?;
    Ok(DiscordSendResult {
        message_id: data.id,
        channel_id: data.channel_id,
    })
}

pub async fn delete_message(
    client: &Client,
    token: &str,
    channel_id: &str,
    message_id: &str,
) -> Result<()> {
    let url = format!(
        "{}/channels/{}/messages/{}",
        DISCORD_API_BASE, channel_id, message_id
    );

    let resp = client
        .delete(&url)
        .header("Authorization", format!("Bot {}", token))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Discord API error ({}): {}", status, body);
    }
    Ok(())
}

pub async fn fetch_message(
    client: &Client,
    token: &str,
    channel_id: &str,
    message_id: &str,
) -> Result<DiscordMessage> {
    let url = format!(
        "{}/channels/{}/messages/{}",
        DISCORD_API_BASE, channel_id, message_id
    );

    let resp = client
        .get(&url)
        .header("Authorization", format!("Bot {}", token))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Discord API error ({}): {}", status, body);
    }

    Ok(resp.json().await?)
}

pub async fn pin_message(
    client: &Client,
    token: &str,
    channel_id: &str,
    message_id: &str,
) -> Result<()> {
    let url = format!(
        "{}/channels/{}/pins/{}",
        DISCORD_API_BASE, channel_id, message_id
    );

    let resp = client
        .put(&url)
        .header("Authorization", format!("Bot {}", token))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Discord API error ({}): {}", status, body);
    }
    Ok(())
}

pub async fn unpin_message(
    client: &Client,
    token: &str,
    channel_id: &str,
    message_id: &str,
) -> Result<()> {
    let url = format!(
        "{}/channels/{}/pins/{}",
        DISCORD_API_BASE, channel_id, message_id
    );

    let resp = client
        .delete(&url)
        .header("Authorization", format!("Bot {}", token))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Discord API error ({}): {}", status, body);
    }
    Ok(())
}

pub async fn list_pins(
    client: &Client,
    token: &str,
    channel_id: &str,
) -> Result<Vec<DiscordMessage>> {
    let url = format!("{}/channels/{}/pins", DISCORD_API_BASE, channel_id);

    let resp = client
        .get(&url)
        .header("Authorization", format!("Bot {}", token))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Discord API error ({}): {}", status, body);
    }

    Ok(resp.json().await?)
}

// ─── Reactions ─────────────────────────────────────────────────────────────────

pub async fn add_reaction(
    client: &Client,
    token: &str,
    channel_id: &str,
    message_id: &str,
    emoji: &str,
) -> Result<()> {
    let encoded_emoji = urlencoding::encode(emoji);
    let url = format!(
        "{}/channels/{}/messages/{}/reactions/{}/@me",
        DISCORD_API_BASE, channel_id, message_id, encoded_emoji
    );

    let resp = client
        .put(&url)
        .header("Authorization", format!("Bot {}", token))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Discord API error ({}): {}", status, body);
    }
    Ok(())
}

pub async fn remove_own_reaction(
    client: &Client,
    token: &str,
    channel_id: &str,
    message_id: &str,
    emoji: &str,
) -> Result<()> {
    let encoded_emoji = urlencoding::encode(emoji);
    let url = format!(
        "{}/channels/{}/messages/{}/reactions/{}/@me",
        DISCORD_API_BASE, channel_id, message_id, encoded_emoji
    );

    let resp = client
        .delete(&url)
        .header("Authorization", format!("Bot {}", token))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Discord API error ({}): {}", status, body);
    }
    Ok(())
}

pub async fn fetch_reactions(
    client: &Client,
    token: &str,
    channel_id: &str,
    message_id: &str,
    emoji: &str,
) -> Result<Vec<DiscordUserInfo>> {
    let encoded_emoji = urlencoding::encode(emoji);
    let url = format!(
        "{}/channels/{}/messages/{}/reactions/{}",
        DISCORD_API_BASE, channel_id, message_id, encoded_emoji
    );

    let resp = client
        .get(&url)
        .header("Authorization", format!("Bot {}", token))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Discord API error ({}): {}", status, body);
    }

    Ok(resp.json().await?)
}

// ─── Threads ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct ThreadCreate {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DiscordThread {
    pub id: String,
    pub guild_id: Option<String>,
    pub parent_id: Option<String>,
    pub owner_id: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: i32,
    pub message_count: Option<i32>,
    pub member_count: Option<i32>,
}

pub async fn create_thread(
    client: &Client,
    token: &str,
    channel_id: &str,
    message_id: &str,
    thread: &ThreadCreate,
) -> Result<DiscordThread> {
    let url = format!(
        "{}/channels/{}/messages/{}/threads",
        DISCORD_API_BASE, channel_id, message_id
    );

    let resp = client
        .post(&url)
        .header("Authorization", format!("Bot {}", token))
        .json(thread)
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Discord API error ({}): {}", status, body);
    }

    Ok(resp.json().await?)
}

pub async fn list_threads(
    client: &Client,
    token: &str,
    channel_id: &str,
) -> Result<Vec<DiscordThread>> {
    let url = format!(
        "{}/channels/{}/threads/active",
        DISCORD_API_BASE, channel_id
    );

    let resp = client
        .get(&url)
        .header("Authorization", format!("Bot {}", token))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Discord API error ({}): {}", status, body);
    }

    #[derive(Deserialize)]
    struct ThreadListResponse {
        threads: Vec<DiscordThread>,
    }

    let data: ThreadListResponse = resp.json().await?;
    Ok(data.threads)
}

// ─── Guild Actions ─────────────────────────────────────────────────────────────

pub async fn fetch_channel_info(
    client: &Client,
    token: &str,
    channel_id: &str,
) -> Result<DiscordChannelInfo> {
    let url = format!("{}/channels/{}", DISCORD_API_BASE, channel_id);

    let resp = client
        .get(&url)
        .header("Authorization", format!("Bot {}", token))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Discord API error ({}): {}", status, body);
    }

    Ok(resp.json().await?)
}

pub async fn fetch_member_info(
    client: &Client,
    token: &str,
    guild_id: &str,
    user_id: &str,
) -> Result<DiscordMemberInfo> {
    let url = format!(
        "{}/guilds/{}/members/{}",
        DISCORD_API_BASE, guild_id, user_id
    );

    let resp = client
        .get(&url)
        .header("Authorization", format!("Bot {}", token))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Discord API error ({}): {}", status, body);
    }

    Ok(resp.json().await?)
}

pub async fn fetch_role_info(
    client: &Client,
    token: &str,
    guild_id: &str,
    role_id: &str,
) -> Result<DiscordRoleInfo> {
    let url = format!("{}/guilds/{}/roles", DISCORD_API_BASE, guild_id);

    let resp = client
        .get(&url)
        .header("Authorization", format!("Bot {}", token))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Discord API error ({}): {}", status, body);
    }

    #[derive(Deserialize)]
    struct RoleListResponse {
        roles: Vec<DiscordRoleInfo>,
    }

    let data: RoleListResponse = resp.json().await?;
    data.roles
        .into_iter()
        .find(|r| r.id == role_id)
        .ok_or_else(|| anyhow::anyhow!("Role not found: {}", role_id))
}

// ─── Moderation Actions ────────────────────────────────────────────────────────

pub async fn timeout_member(
    client: &Client,
    token: &str,
    guild_id: &str,
    user_id: &str,
    duration_seconds: u32,
) -> Result<()> {
    let url = format!(
        "{}/guilds/{}/members/{}",
        DISCORD_API_BASE, guild_id, user_id
    );

    let timeout_until = chrono::Utc::now() + chrono::Duration::seconds(duration_seconds as i64);
    let payload = json!({
        "communication_disabled_until": timeout_until.to_rfc3339()
    });

    let resp = client
        .patch(&url)
        .header("Authorization", format!("Bot {}", token))
        .json(&payload)
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Discord API error ({}): {}", status, body);
    }
    Ok(())
}

pub async fn kick_member(
    client: &Client,
    token: &str,
    guild_id: &str,
    user_id: &str,
) -> Result<()> {
    let url = format!(
        "{}/guilds/{}/members/{}",
        DISCORD_API_BASE, guild_id, user_id
    );

    let resp = client
        .delete(&url)
        .header("Authorization", format!("Bot {}", token))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Discord API error ({}): {}", status, body);
    }
    Ok(())
}

pub async fn ban_member(
    client: &Client,
    token: &str,
    guild_id: &str,
    user_id: &str,
    reason: Option<&str>,
) -> Result<()> {
    let url = format!("{}/guilds/{}/bans/{}", DISCORD_API_BASE, guild_id, user_id);

    let payload = json!({
        "delete_message_seconds": 86400,
        "reason": reason
    });

    let resp = client
        .put(&url)
        .header("Authorization", format!("Bot {}", token))
        .json(&payload)
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Discord API error ({}): {}", status, body);
    }
    Ok(())
}

// ─── Role Management ───────────────────────────────────────────────────────────

pub async fn add_role(
    client: &Client,
    token: &str,
    guild_id: &str,
    user_id: &str,
    role_id: &str,
) -> Result<()> {
    let url = format!(
        "{}/guilds/{}/members/{}/roles/{}",
        DISCORD_API_BASE, guild_id, user_id, role_id
    );

    let resp = client
        .put(&url)
        .header("Authorization", format!("Bot {}", token))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Discord API error ({}): {}", status, body);
    }
    Ok(())
}

pub async fn remove_role(
    client: &Client,
    token: &str,
    guild_id: &str,
    user_id: &str,
    role_id: &str,
) -> Result<()> {
    let url = format!(
        "{}/guilds/{}/members/{}/roles/{}",
        DISCORD_API_BASE, guild_id, user_id, role_id
    );

    let resp = client
        .delete(&url)
        .header("Authorization", format!("Bot {}", token))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("Discord API error ({}): {}", status, body);
    }
    Ok(())
}

// ─── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discord_payload_has_content() {
        let p = build_message_payload("hello world", &SendOptions::default());
        assert_eq!(p["content"], "hello world");
    }

    #[test]
    fn discord_http_payload_builder_public_api() {
        let p = build_discord_http_payload("hello world", Some("123456789"), true);
        assert_eq!(p["content"], "hello world");
        assert_eq!(p["message_reference"]["message_id"], "123456789");
        assert_eq!(p["flags"], SUPPRESS_NOTIFICATIONS_FLAG);
    }

    #[test]
    fn discord_payload_with_reply() {
        let opts = SendOptions {
            reply_to: Some("123456789".to_string()),
            ..Default::default()
        };
        let p = build_message_payload("reply text", &opts);
        assert_eq!(p["message_reference"]["message_id"], "123456789");
    }

    #[test]
    fn discord_payload_silent_flag() {
        let opts = SendOptions {
            silent: true,
            ..Default::default()
        };
        let p = build_message_payload("silent message", &opts);
        assert_eq!(p["flags"], SUPPRESS_NOTIFICATIONS_FLAG);
    }

    #[test]
    fn discord_poll_payload_builds() {
        let poll = PollInput {
            question: "What?".to_string(),
            answers: vec![
                PollAnswer {
                    text: "A".to_string(),
                    emoji: None,
                },
                PollAnswer {
                    text: "B".to_string(),
                    emoji: None,
                },
            ],
            duration_hours: Some(24),
            allow_multiselect: Some(true),
        };
        let p = build_poll_payload(&poll, Some("Poll time!"), false);
        assert_eq!(p["poll"]["question"]["text"], "What?");
        assert_eq!(p["poll"]["duration"], 24);
        assert_eq!(p["poll"]["allow_multiselect"], true);
        assert_eq!(p["content"], "Poll time!");
    }

    #[test]
    fn discord_api_base_is_v10() {
        assert!(DISCORD_API_BASE.contains("v10"));
    }

    #[test]
    fn embed_serialization() {
        let embed = DiscordEmbed {
            title: Some("Test".to_string()),
            description: Some("Description".to_string()),
            color: Some(0xFF0000),
            fields: Some(vec![DiscordEmbedField {
                name: "Field".to_string(),
                value: "Value".to_string(),
                inline: true,
            }]),
            ..Default::default()
        };

        let json = serde_json::to_value(&embed).unwrap();
        assert_eq!(json["title"], "Test");
        assert_eq!(json["color"], 0xFF0000);
        assert_eq!(json["fields"][0]["inline"], true);
    }
}
