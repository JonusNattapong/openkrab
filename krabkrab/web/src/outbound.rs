use crate::active_listener::{
    require_active_web_listener, ActiveWebSendOptions, PollInput,
};
use anyhow::{anyhow, Result};
use krabkrab_channels::whatsapp::normalize_whatsapp_target;

pub fn send_message_whatsapp(
    to: &str,
    body: &str,
    account_id: Option<&str>,
) -> Result<(String, String)> {
    let normalized = normalize_whatsapp_target(to)
        .ok_or_else(|| anyhow!("invalid WhatsApp target: expected E.164 or group JID"))?;

    let (_resolved_account_id, listener) = require_active_web_listener(account_id)?;
    listener.send_composing_to(&normalized)?;

    let message_id = listener.send_message(
        &normalized,
        body,
        None,
        None,
        Some(&ActiveWebSendOptions::default()),
    )?;

    Ok((message_id, normalized))
}

pub fn send_reaction_whatsapp(
    chat_jid: &str,
    message_id: &str,
    emoji: &str,
    from_me: bool,
    participant: Option<&str>,
    account_id: Option<&str>,
) -> Result<()> {
    let (_resolved_account_id, listener) = require_active_web_listener(account_id)?;
    listener.send_reaction(chat_jid, message_id, emoji, from_me, participant)
}

pub fn send_poll_whatsapp(
    to: &str,
    poll: PollInput,
    account_id: Option<&str>,
) -> Result<(String, String)> {
    let normalized = normalize_whatsapp_target(to)
        .ok_or_else(|| anyhow!("invalid WhatsApp target: expected E.164 or group JID"))?;

    let (_resolved_account_id, listener) = require_active_web_listener(account_id)?;
    let message_id = listener.send_poll(&normalized, &poll)?;

    Ok((message_id, normalized))
}
