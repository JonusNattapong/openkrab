use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

pub const DEFAULT_ACCOUNT_ID: &str = "default";

#[derive(Debug, Clone, Default)]
pub struct ActiveWebSendOptions {
    pub gif_playback: bool,
    pub account_id: Option<String>,
    pub file_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PollInput {
    pub question: String,
    pub options: Vec<String>,
    pub max_selections: usize,
}

pub trait ActiveWebListener: Send + Sync {
    fn send_message(
        &self,
        to: &str,
        text: &str,
        media_buffer: Option<&[u8]>,
        media_type: Option<&str>,
        options: Option<&ActiveWebSendOptions>,
    ) -> Result<String>;

    fn send_poll(&self, to: &str, poll: &PollInput) -> Result<String>;

    fn send_reaction(
        &self,
        chat_jid: &str,
        message_id: &str,
        emoji: &str,
        from_me: bool,
        participant: Option<&str>,
    ) -> Result<()>;

    fn send_composing_to(&self, to: &str) -> Result<()>;

    fn close(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Default)]
struct ListenerRegistry {
    listeners: HashMap<String, Arc<dyn ActiveWebListener>>,
}

fn registry() -> &'static Mutex<ListenerRegistry> {
    static REGISTRY: OnceLock<Mutex<ListenerRegistry>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(ListenerRegistry::default()))
}

pub fn resolve_web_account_id(account_id: Option<&str>) -> String {
    account_id
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(DEFAULT_ACCOUNT_ID)
        .to_string()
}

pub fn set_active_web_listener(account_id: Option<&str>, listener: Option<Arc<dyn ActiveWebListener>>) {
    let id = resolve_web_account_id(account_id);
    let mut guard = registry().lock().expect("listener registry lock poisoned");

    if let Some(listener) = listener {
        guard.listeners.insert(id, listener);
    } else {
        guard.listeners.remove(&id);
    }
}

pub fn get_active_web_listener(account_id: Option<&str>) -> Option<Arc<dyn ActiveWebListener>> {
    let id = resolve_web_account_id(account_id);
    let guard = registry().lock().expect("listener registry lock poisoned");
    guard.listeners.get(&id).cloned()
}

pub fn require_active_web_listener(account_id: Option<&str>) -> Result<(String, Arc<dyn ActiveWebListener>)> {
    let id = resolve_web_account_id(account_id);
    let listener = get_active_web_listener(Some(&id)).ok_or_else(|| {
        anyhow!(
            "No active WhatsApp Web listener (account: {}). Start gateway then login this account.",
            id
        )
    })?;

    Ok((id, listener))
}
