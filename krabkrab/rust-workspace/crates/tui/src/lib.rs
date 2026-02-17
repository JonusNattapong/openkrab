//! TUI crate skeleton - port TUI code here using `tui`/`crossterm`.

pub fn tui_ready() -> &'static str { "tui ready" }

#[derive(Clone, Debug)]
pub struct GatewayConnectionOptions {
    pub url: Option<String>,
    pub token: Option<String>,
    pub password: Option<String>,
}

#[derive(Clone, Debug)]
pub struct GatewayConfig {
    pub mode: Option<String>, // "local" | "remote"
    pub bind: Option<String>, // "tailnet" | "lan"
    pub remote: Option<GatewayRemote>,
    pub auth_token: Option<String>,
}

#[derive(Clone, Debug)]
pub struct GatewayRemote {
    pub url: Option<String>,
    pub token: Option<String>,
    pub password: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedConnection {
    pub url: String,
    pub token: Option<String>,
    pub password: Option<String>,
}

fn resolve_explicit_gateway_auth(opts: &GatewayConnectionOptions) -> (Option<String>, Option<String>) {
    (opts.token.clone(), opts.password.clone())
}

fn ensure_explicit_gateway_auth(url_override: Option<&str>, explicit_auth: &(Option<String>, Option<String>)) -> Result<(), String> {
    if url_override.is_some() {
        if explicit_auth.0.is_none() && explicit_auth.1.is_none() {
            return Err("explicit credentials required: pass token or password".to_string());
        }
    }
    Ok(())
}

pub fn resolve_gateway_connection<F1, F2, F3>(
    opts: GatewayConnectionOptions,
    config: GatewayConfig,
    resolve_gateway_port: F1,
    pick_primary_tailnet_ipv4: F2,
    pick_primary_lan_ipv4: F3,
) -> Result<ResolvedConnection, String>
where
    F1: Fn() -> u16,
    F2: Fn() -> Option<String>,
    F3: Fn() -> Option<String>,
{
    let is_remote_mode = config.mode.as_deref() == Some("remote");

    let url_override = opts.url.as_ref().and_then(|s| {
        let t = s.trim();
        if t.is_empty() { None } else { Some(t) }
    });

    let explicit_auth = resolve_explicit_gateway_auth(&opts);
    ensure_explicit_gateway_auth(url_override, &explicit_auth)?;

    // Build URL
    let url = if let Some(override_url) = url_override {
        override_url.to_string()
    } else if !is_remote_mode {
        // local mode
        let port = resolve_gateway_port();
        let host = match config.bind.as_deref() {
            Some("tailnet") => pick_primary_tailnet_ipv4().ok_or_else(|| "no tailnet host".to_string())?,
            Some("lan") => pick_primary_lan_ipv4().ok_or_else(|| "no lan host".to_string())?,
            _ => "127.0.0.1".to_string(),
        };
        format!("ws://{}:{}", host, port)
    } else {
        config.remote.and_then(|r| r.url).unwrap_or_else(|| "wss://127.0.0.1/".to_string())
    };

    // Token selection
    let token = if explicit_auth.0.is_some() {
        explicit_auth.0.clone()
    } else if url_override.is_none() {
        if is_remote_mode {
            config.remote.and_then(|r| r.token)
        } else {
            opts.token.or(config.auth_token)
        }
    } else {
        None
    };

    let password = if explicit_auth.1.is_some() {
        explicit_auth.1.clone()
    } else if url_override.is_none() {
        if is_remote_mode {
            config.remote.and_then(|r| r.password)
        } else {
            opts.password
        }
    } else {
        None
    };

    Ok(ResolvedConnection { url, token, password })
}

mod gateway_chat;
pub use gateway_chat::GatewayChatClient;

// --- Editor submit handler
pub type SetTextFn = Box<dyn Fn(&str) + Send + Sync>;
pub type AddToHistoryFn = Box<dyn Fn(&str) + Send + Sync>;
pub type HandleFn = Box<dyn Fn(&str) + Result<(), String> + Send + Sync>;

pub fn create_editor_submit_handler(
    set_text: SetTextFn,
    add_to_history: AddToHistoryFn,
    handle_command: Box<dyn Fn(&str) -> Result<(), String> + Send + Sync>,
    send_message: Box<dyn Fn(&str) -> Result<(), String> + Send + Sync>,
    handle_bang_line: Box<dyn Fn(&str) -> Result<(), String> + Send + Sync>,
) -> Box<dyn Fn(&str) + Send + Sync> {
    Box::new(move |text: &str| {
        let raw = text;
        let value = raw.trim();
        set_text("");

        if value.is_empty() {
            return;
        }

        if raw.starts_with('!') && raw != "!" {
            add_to_history(raw);
            let _ = handle_bang_line(raw);
            return;
        }

        add_to_history(value);
        if value.starts_with('/') {
            let _ = handle_command(value);
            return;
        }
        let _ = send_message(value);
    })
}

// --- Windows Git Bash paste fallback detection
pub fn should_enable_windows_git_bash_paste_fallback(platform: &str, env: &std::collections::HashMap<String, String>) -> bool {
    if platform != "win32" {
        return false;
    }
    let msystem = env.get("MSYSTEM").map(|s| s.to_uppercase()).unwrap_or_default();
    let shell = env.get("SHELL").cloned().unwrap_or_default();
    let term_program = env.get("TERM_PROGRAM").map(|s| s.to_lowercase()).unwrap_or_default();
    if msystem.starts_with("MINGW") || msystem.starts_with("MSYS") {
        return true;
    }
    if shell.to_lowercase().contains("bash") {
        return true;
    }
    term_program.contains("mintty")
}

// --- Submit burst coalescer
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn create_submit_burst_coalescer(
    submit: Box<dyn Fn(&str) + Send + Sync>,
    enabled: bool,
    burst_window_ms: Option<u64>,
) -> Box<dyn Fn(&str) + Send + Sync> {
    let window_ms = std::cmp::max(1, burst_window_ms.unwrap_or(50));
    let state = Arc::new(Mutex::new((None::<String>, 0u128)));
    let submit_clone = submit.clone();
    Box::new(move |value: &str| {
        if !enabled {
            submit_clone(value);
            return;
        }
        if value.contains('\n') {
            let s = submit_clone.clone();
            s(value);
            return;
        }
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_millis(0)).as_millis();
        let mut s = state.lock().unwrap();
        if s.0.is_none() {
            s.0 = Some(value.to_string());
            s.1 = now;
            // No async timer here; flush immediately after window to keep behavior simple for now.
            // For the purposes of porting logic, we flush inline if new incoming within window.
            return;
        }
        if now - s.1 <= window_ms as u128 {
            if let Some(ref mut pending) = s.0 {
                pending.push('\n');
                pending.push_str(value);
                s.1 = now;
            }
            return;
        }
        if let Some(pending) = s.0.take() {
            submit_clone(&pending);
        }
        s.0 = Some(value.to_string());
        s.1 = now;
    })
}

// --- resolve tui session key
pub fn resolve_tui_session_key(raw: Option<&str>, session_scope: &str, current_agent_id: &str, session_main_key: &str) -> String {
    let trimmed = raw.unwrap_or("").trim();
    if trimmed.is_empty() {
        if session_scope == "global" {
            return "global".to_string();
        }
        return format!("agent:{}:{}", current_agent_id, session_main_key);
    }
    if trimmed == "global" || trimmed == "unknown" {
        return trimmed.to_string();
    }
    if trimmed.starts_with("agent:") {
        return trimmed.to_string();
    }
    format!("agent:{}:{}", current_agent_id, trimmed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(tui_ready(), "tui ready");
    }
}
