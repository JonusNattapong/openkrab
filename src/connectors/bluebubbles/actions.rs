//! BlueBubbles message actions (react, edit, unsend, reply, effects, group management).
//! Ported from openkrab/extensions/bluebubbles/src/actions.ts

use serde::{Deserialize, Serialize};

use super::chat::{
    add_participant, edit_message, leave_chat, remove_participant, rename_chat, set_group_icon,
    unsend_message,
};
use super::config::resolve_account;
use super::probe::is_private_api_enabled;
use super::reactions::{
    map_emoji_to_reaction, parse_reaction_action, remove_reaction, send_reaction,
};
use super::send::send_message;
use super::types::BlueBubblesConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub ok: bool,
    pub error: Option<String>,
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionContext {
    pub account_id: Option<String>,
    pub chat_guid: Option<String>,
    pub message_guid: Option<String>,
    pub address: Option<String>,
    pub params: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionParams {
    pub action: String,
    pub params: std::collections::HashMap<String, serde_json::Value>,
}

pub fn execute_action(
    action: &str,
    config: &BlueBubblesConfig,
    ctx: ActionContext,
) -> Result<ActionResult, String> {
    let account = resolve_account(config, ctx.account_id.as_deref());
    if !account.configured {
        return Err("BlueBubbles account not configured".to_string());
    }

    let base_url = account
        .config
        .server_url
        .as_ref()
        .ok_or("server_url required")?;
    let password = account
        .config
        .password
        .as_ref()
        .ok_or("password required")?;

    match action {
        "react" => execute_reaction(base_url, password, &ctx),
        "edit" => execute_edit(base_url, password, &ctx),
        "unsend" => execute_unsend(base_url, password, &ctx),
        "reply" => execute_reply(base_url, password, &ctx, config),
        "sendWithEffect" => execute_effect(base_url, password, &ctx),
        "renameGroup" => execute_rename_group(base_url, password, &ctx),
        "setGroupIcon" => execute_set_group_icon(base_url, password, &ctx),
        "addParticipant" => execute_add_participant(base_url, password, &ctx),
        "removeParticipant" => execute_remove_participant(base_url, password, &ctx),
        "leaveGroup" => execute_leave_group(base_url, password, &ctx),
        "sendAttachment" => Err("sendAttachment requires media upload".to_string()),
        _ => Err(format!("Unknown action: {}", action)),
    }
}

fn execute_reaction(
    base_url: &str,
    password: &str,
    ctx: &ActionContext,
) -> Result<ActionResult, String> {
    let chat_guid = ctx.chat_guid.as_ref().ok_or("chat_guid required")?;
    let message_guid = ctx.message_guid.as_ref().ok_or("message_guid required")?;

    let emoji = ctx
        .params
        .get("emoji")
        .and_then(|v| v.as_str())
        .ok_or("emoji required")?;

    let reaction =
        map_emoji_to_reaction(emoji).ok_or_else(|| format!("Unknown emoji: {}", emoji))?;

    send_reaction(base_url, password, message_guid, chat_guid, &reaction, None)
        .map(|r| ActionResult {
            ok: true,
            error: None,
            message_id: Some(r.message_id),
        })
        .map_err(|e| e.to_string())
}

fn execute_edit(
    base_url: &str,
    password: &str,
    ctx: &ActionContext,
) -> Result<ActionResult, String> {
    if !is_private_api_enabled("default") {
        return Err("Edit requires Private API to be enabled".to_string());
    }

    let message_guid = ctx.message_guid.as_ref().ok_or("message_guid required")?;

    let new_text = ctx
        .params
        .get("text")
        .or(ctx.params.get("message"))
        .and_then(|v| v.as_str())
        .ok_or("new text required")?;

    edit_message(base_url, password, message_guid, new_text, None)
        .map(|r| ActionResult {
            ok: true,
            error: None,
            message_id: Some(r.message_id),
        })
        .map_err(|e| e.to_string())
}

fn execute_unsend(
    base_url: &str,
    password: &str,
    ctx: &ActionContext,
) -> Result<ActionResult, String> {
    if !is_private_api_enabled("default") {
        return Err("Unsend requires Private API to be enabled".to_string());
    }

    let chat_guid = ctx.chat_guid.as_ref().ok_or("chat_guid required")?;
    let message_guid = ctx.message_guid.as_ref().ok_or("message_guid required")?;

    unsend_message(base_url, password, message_guid, chat_guid, None)
        .map(|r| ActionResult {
            ok: r.ok,
            error: r.error,
            message_id: None,
        })
        .map_err(|e| e.to_string())
}

fn execute_reply(
    base_url: &str,
    password: &str,
    ctx: &ActionContext,
    config: &BlueBubblesConfig,
) -> Result<ActionResult, String> {
    if !is_private_api_enabled("default") {
        return Err("Reply requires Private API to be enabled".to_string());
    }

    let chat_guid = ctx.chat_guid.as_ref().ok_or("chat_guid required")?;
    let message_guid = ctx.message_guid.as_ref().ok_or("message_guid required")?;

    let text = ctx
        .params
        .get("text")
        .or(ctx.params.get("message"))
        .and_then(|v| v.as_str())
        .ok_or("reply text required")?;

    let opts = super::send::SendOptions {
        server_url: None,
        password: None,
        account_id: ctx.account_id.clone(),
        timeout_ms: None,
        reply_to_message_guid: Some(message_guid.clone()),
        reply_to_part_index: Some(0),
        effect_id: None,
    };

    send_message(chat_guid, text, config, opts)
        .map(|r| ActionResult {
            ok: true,
            error: None,
            message_id: Some(r.message_id),
        })
        .map_err(|e| e.to_string())
}

fn execute_effect(
    base_url: &str,
    password: &str,
    ctx: &ActionContext,
) -> Result<ActionResult, String> {
    if !is_private_api_enabled("default") {
        return Err("Effects require Private API to be enabled".to_string());
    }

    let chat_guid = ctx.chat_guid.as_ref().ok_or("chat_guid required")?;

    let effect = ctx
        .params
        .get("effect")
        .or(ctx.params.get("effectId"))
        .and_then(|v| v.as_str())
        .ok_or("effect required")?;

    let effect_id = super::resolve_effect_id(Some(effect))
        .ok_or_else(|| format!("Unknown effect: {}", effect))?;

    let text = ctx
        .params
        .get("text")
        .or(ctx.params.get("message"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let opts = super::send::SendOptions {
        server_url: None,
        password: None,
        account_id: ctx.account_id.clone(),
        timeout_ms: None,
        reply_to_message_guid: None,
        reply_to_part_index: None,
        effect_id: Some(effect_id),
    };

    send_message(chat_guid, text, &BlueBubblesConfig::default(), opts)
        .map(|r| ActionResult {
            ok: true,
            error: None,
            message_id: Some(r.message_id),
        })
        .map_err(|e| e.to_string())
}

fn execute_rename_group(
    base_url: &str,
    password: &str,
    ctx: &ActionContext,
) -> Result<ActionResult, String> {
    let chat_guid = ctx.chat_guid.as_ref().ok_or("chat_guid required")?;

    let new_name = ctx
        .params
        .get("name")
        .or(ctx.params.get("newName"))
        .and_then(|v| v.as_str())
        .ok_or("new name required")?;

    rename_chat(base_url, password, chat_guid, new_name, None)
        .map(|r| ActionResult {
            ok: r.ok,
            error: r.error,
            message_id: None,
        })
        .map_err(|e| e.to_string())
}

fn execute_set_group_icon(
    base_url: &str,
    password: &str,
    ctx: &ActionContext,
) -> Result<ActionResult, String> {
    let chat_guid = ctx.chat_guid.as_ref().ok_or("chat_guid required")?;

    let image_path = ctx
        .params
        .get("imagePath")
        .or(ctx.params.get("path"))
        .and_then(|v| v.as_str())
        .ok_or("image path required")?;

    set_group_icon(base_url, password, chat_guid, image_path, None)
        .map(|r| ActionResult {
            ok: r.ok,
            error: r.error,
            message_id: None,
        })
        .map_err(|e| e.to_string())
}

fn execute_add_participant(
    base_url: &str,
    password: &str,
    ctx: &ActionContext,
) -> Result<ActionResult, String> {
    let chat_guid = ctx.chat_guid.as_ref().ok_or("chat_guid required")?;

    let address: &str = ctx
        .address
        .as_ref()
        .map(|s| s.as_str())
        .or_else(|| ctx.params.get("address").and_then(|v| v.as_str()))
        .ok_or("address required")?;

    add_participant(base_url, password, chat_guid, address, None)
        .map(|r| ActionResult {
            ok: r.ok,
            error: r.error,
            message_id: None,
        })
        .map_err(|e| e.to_string())
}

fn execute_remove_participant(
    base_url: &str,
    password: &str,
    ctx: &ActionContext,
) -> Result<ActionResult, String> {
    let chat_guid = ctx.chat_guid.as_ref().ok_or("chat_guid required")?;

    let address: &str = ctx
        .address
        .as_ref()
        .map(|s| s.as_str())
        .or_else(|| ctx.params.get("address").and_then(|v| v.as_str()))
        .ok_or("address required")?;

    remove_participant(base_url, password, chat_guid, address, None)
        .map(|r| ActionResult {
            ok: r.ok,
            error: r.error,
            message_id: None,
        })
        .map_err(|e| e.to_string())
}

fn execute_leave_group(
    base_url: &str,
    password: &str,
    ctx: &ActionContext,
) -> Result<ActionResult, String> {
    let chat_guid = ctx.chat_guid.as_ref().ok_or("chat_guid required")?;

    leave_chat(base_url, password, chat_guid, None)
        .map(|r| ActionResult {
            ok: r.ok,
            error: r.error,
            message_id: None,
        })
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_result_default() {
        let result = ActionResult {
            ok: false,
            error: None,
            message_id: None,
        };
        assert!(!result.ok);
    }

    #[test]
    fn test_action_context_default() {
        let ctx = ActionContext {
            account_id: None,
            chat_guid: None,
            message_guid: None,
            address: None,
        };
        assert!(ctx.account_id.is_none());
    }
}
