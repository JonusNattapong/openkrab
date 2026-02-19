//! Administrative and operational CLI commands.

use crate::daemon::daemon_status;
use crate::hooks::events;

pub fn update_command(manage_channels: bool) -> String {
    if manage_channels {
        "update: core binaries + channel adapters scheduled".to_string()
    } else {
        "update: core binaries scheduled (use --channels to include channel adapters)".to_string()
    }
}

pub fn skills_command(action: &str) -> String {
    format!("skills: action={}", action.trim())
}

pub fn sandbox_command(action: &str) -> String {
    format!("sandbox: action={} (docker sandbox control)", action.trim())
}

pub fn nodes_command(action: &str) -> String {
    format!("nodes: action={} (device node management)", action.trim())
}

pub fn browser_command(action: &str) -> String {
    format!(
        "browser: action={} (chrome/chromium control)",
        action.trim()
    )
}

pub fn hooks_command() -> String {
    let known = [
        events::MESSAGE_INBOUND,
        events::MESSAGE_OUTBOUND,
        events::AGENT_START,
        events::AGENT_COMPLETE,
        events::AGENT_ERROR,
        events::SESSION_CREATED,
        events::SESSION_CLOSED,
        events::MEMORY_INDEXED,
        events::CRON_FIRED,
    ];
    format!("hooks: {}", known.join(", "))
}

pub fn webhooks_command(action: &str) -> String {
    format!("webhooks: action={} (endpoint registration)", action.trim())
}

pub fn exec_approvals_command(action: &str) -> String {
    format!(
        "exec-approvals: action={} (bash approval policy)",
        action.trim()
    )
}

pub fn docs_command(topic: Option<&str>) -> String {
    let base = "https://docs.molt.bot";
    match topic.map(str::trim).filter(|s| !s.is_empty()) {
        Some(t) => format!("docs: {}/{}", base, t.trim_start_matches('/')),
        None => format!("docs: {}", base),
    }
}

pub fn dns_command(action: &str) -> String {
    format!("dns: action={} (service discovery)", action.trim())
}

pub fn directory_command(action: &str) -> String {
    format!("directory: action={} (service directory)", action.trim())
}

pub fn system_command() -> String {
    format!(
        "system: os={} arch={} version={}",
        std::env::consts::OS,
        std::env::consts::ARCH,
        crate::VERSION.as_str()
    )
}

pub fn devices_command(action: &str) -> String {
    format!(
        "devices: action={} (paired device management)",
        action.trim()
    )
}

pub fn daemon_command(action: &str) -> String {
    let status = daemon_status();
    match action.trim() {
        "status" | "" => format!("daemon: running={} pid={:?}", status.running, status.pid),
        other => format!(
            "daemon: action={} running={} pid={:?}",
            other, status.running, status.pid
        ),
    }
}
