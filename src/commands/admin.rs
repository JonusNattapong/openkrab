//! Administrative and operational CLI commands.

use crate::daemon::daemon_status;
use crate::hooks::events;
use crate::skills::{build_skills_report, format_skill_info, format_skills_list};

pub fn update_command(manage_channels: bool) -> String {
    if manage_channels {
        "update: core binaries + channel adapters scheduled".to_string()
    } else {
        "update: core binaries scheduled (use --channels to include channel adapters)".to_string()
    }
}

pub fn skills_command(action: &str) -> String {
    let action = action.trim();
    let report = build_skills_report();

    match action {
        "list" | "" => format_skills_list(&report, false),
        "eligible" => format_skills_list(&report, true),
        "check" => {
            let eligible = report.skills.iter().filter(|s| s.eligible).count();
            let total = report.skills.len();
            format!(
                "Skills Status Check\n\nTotal: {}\n✓ Eligible: {}\n✗ Not eligible: {}",
                total,
                eligible,
                total - eligible
            )
        }
        _ => {
            if action.starts_with("info ") || action.starts_with("show ") {
                let name = action.splitn(2, ' ').nth(1).unwrap_or("");
                format_skill_info(&report, name)
            } else {
                format_skills_list(&report, false)
            }
        }
    }
}

pub fn sandbox_command(action: &str) -> String {
    let cfg = match crate::config_io::load_config() {
        Ok(c) => crate::config::openkrab_to_app_config(&c),
        Err(_) => crate::config::AppConfig::default(),
    };
    match action.trim() {
        "list" => crate::commands::sandbox::sandbox_list_command(&cfg),
        "build" => crate::commands::sandbox::sandbox_build_command(true, true),
        "recreate" => crate::commands::sandbox::sandbox_recreate_command(&cfg, true),
        "explain" => crate::commands::sandbox::sandbox_explain_command(&cfg),
        _ => crate::commands::sandbox::sandbox_list_command(&cfg),
    }
}

pub fn nodes_command(action: &str) -> String {
    let action = action.trim();

    match action {
        "list" | "" => match crate::node_host::run_action("list", &serde_json::json!({})) {
            Ok(result) => {
                if let Some(nodes) = result.get("nodes").and_then(|n| n.as_array()) {
                    let mut output = String::from("Nodes:\n");
                    output.push_str(&"=".repeat(50));
                    output.push('\n');
                    for node in nodes {
                        let id = node.get("id").and_then(|v| v.as_str()).unwrap_or("unknown");
                        let platform = node
                            .get("platform")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown");
                        let status = node
                            .get("status")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown");
                        output.push_str(&format!("• {} [{}] - {}\n", id, platform, status));
                    }
                    output
                } else {
                    "No nodes found.".to_string()
                }
            }
            Err(e) => format!("Failed to list nodes: {}", e),
        },
        "status" => match crate::node_host::run_action("status", &serde_json::json!({})) {
            Ok(result) => format!("{:#}", result),
            Err(e) => format!("Failed to get node status: {}", e),
        },
        _ => {
            let parts: Vec<&str> = action.splitn(2, ' ').collect();
            match parts[0] {
                "invoke" | "run" => {
                    let cmd = parts.get(1).unwrap_or(&"");
                    format!("nodes run: {} (requires gateway connection)", cmd)
                }
                "camera" => {
                    "nodes camera: Use 'krabkrab nodes status' to list nodes, then invoke camera action".to_string()
                }
                "screen" => {
                    "nodes screen: Use 'krabkrab nodes status' to list nodes".to_string()
                }
                "pairing" => {
                    "nodes pairing: Use 'krabkrab pairing' commands instead".to_string()
                }
                "notify" => {
                    "nodes notify: Use 'krabkrab nodes status' to list nodes".to_string()
                }
                _ => {
                    format!(
                        "Nodes management\n\nAvailable actions:\n\
                         • list - List all nodes\n\
                         • status - Show node status\n\
                         • pairing - Node pairing (use 'krabkrab pairing')\n\
                         • invoke - Run commands on nodes\n\
                         • camera - Capture from node camera\n\
                         • screen - Screen capture from nodes\n\n\
                         Usage: krabkrab nodes <action>"
                    )
                }
            }
        }
    }
}

pub fn browser_command(action: &str) -> String {
    let action = action.trim();

    match action {
        "list" | "" => {
            let profiles = crate::browser::list_profiles();
            if profiles.is_empty() {
                "No browser profiles configured.".to_string()
            } else {
                let mut output = String::from("Browser Profiles:\n");
                output.push_str(&"=".repeat(50));
                output.push('\n');
                for profile in profiles {
                    output.push_str(&format!(
                        "• {} (CDP: {})\n",
                        profile.name, profile.cdp_http_url
                    ));
                }
                output
            }
        }
        "status" => "Browser status: Use CDP connection to query live browser state".to_string(),
        _ => {
            let parts: Vec<&str> = action.splitn(2, ' ').collect();
            match parts[0] {
                "open" | "launch" => {
                    format!("browser open: (requires gateway running with browser plugin)")
                }
                "screenshot" => {
                    "browser screenshot: Use 'krabkrab browser list' to see profiles".to_string()
                }
                "tabs" => "browser tabs: Requires active CDP connection".to_string(),
                "create" | "add" => {
                    "browser create: Use 'krabkrab browser list' to see existing profiles"
                        .to_string()
                }
                _ => {
                    format!(
                        "Browser management (Chrome/Chromium CDP)\n\n\
                         Available actions:\n\
                         • list - List browser profiles\n\
                         • status - Show browser status\n\
                         • open <url> - Open URL in browser\n\
                         • screenshot - Take screenshot\n\
                         • tabs - List open tabs\n\n\
                         Usage: krabkrab browser <action> [args]"
                    )
                }
            }
        }
    }
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
    let action = action.trim();

    match action {
        "list" | "" => {
            let config = match crate::config_io::load_config() {
                Ok(c) => c,
                Err(_) => return "No configuration found.".to_string(),
            };

            let mut output = String::from("Configured Webhooks:\n");
            output.push_str(&"=".repeat(50));
            output.push('\n');

            let mut found = false;

            if let Some(channels) = &config.channels {
                for (name, account) in &channels.googlechat {
                    if !account.allowlist.is_empty() || account.enabled {
                        found = true;
                        output.push_str(&format!(
                            "✓ GoogleChat [{}]: enabled={}\n",
                            name, account.enabled
                        ));
                    }
                }
                for (name, account) in &channels.msteams {
                    if !account.allowlist.is_empty() || account.enabled {
                        found = true;
                        output.push_str(&format!(
                            "✓ MSTeams [{}]: enabled={}\n",
                            name, account.enabled
                        ));
                    }
                }
            }

            if !found {
                output.push_str("No webhooks configured.\n");
                output.push_str("\nTo add webhooks:\n");
                output.push_str("  krabkrab channels add googlechat --token <token>\n");
                output.push_str("  krabkrab channels add msteams --token <token>\n");
            }

            output
        }
        "test" => "webhooks test: Use curl to test your webhook endpoint manually".to_string(),
        _ => {
            format!(
                "Webhooks management\n\n\
                 Available actions:\n\
                 • list - List configured webhooks\n\
                 • test - Test webhook endpoint\n\n\
                 Note: Webhooks are configured per-channel.\n\
                 Use: krabkrab channels add <channel>"
            )
        }
    }
}

pub fn exec_approvals_command(action: &str) -> String {
    let action = action.trim();

    match action {
        "list" | "" => {
            let config = match crate::config_io::load_config() {
                Ok(c) => c,
                Err(_) => return "No configuration found.".to_string(),
            };

            let mut output = String::from("Execution Approvals Policy:\n");
            output.push_str(&"=".repeat(50));
            output.push('\n');

            let mut found = false;

            if let Some(channels) = &config.channels {
                if let Some(discord) = &channels.discord {
                    if let Some(exec_approvals) = &discord.exec_approvals {
                        found = true;
                        output.push_str(&format!("Discord:\n"));
                        output.push_str(&format!("  Enabled: {}\n", exec_approvals.enabled));
                        if let Some(timeout) = exec_approvals.timeout_seconds {
                            output.push_str(&format!("  Timeout: {} seconds\n", timeout));
                        }
                        if !exec_approvals.allowed_users.is_empty() {
                            output.push_str(&format!(
                                "  Allowed users: {}\n",
                                exec_approvals.allowed_users.join(", ")
                            ));
                        }
                        if !exec_approvals.allowed_roles.is_empty() {
                            output.push_str(&format!(
                                "  Allowed roles: {}\n",
                                exec_approvals.allowed_roles.join(", ")
                            ));
                        }
                    }
                }
            }

            if !found {
                output.push_str("No execution approval policies configured.\n");
                output.push_str("\nTo configure exec approvals, edit config and add:\n");
                output.push_str("  [channels.discord.exec_approvals]\n");
                output.push_str("  enabled = true\n");
                output.push_str("  allowed_users = [\"user1\", \"user2\"]\n");
            }

            output
        }
        "allow" => "exec-approvals allow: Modify config to add users to allowed list".to_string(),
        "deny" => {
            "exec-approvals deny: Modify config to remove users from allowed list".to_string()
        }
        _ => {
            format!(
                "Execution Approvals Policy\n\n\
                 Available actions:\n\
                 • list - Show current approval policies\n\
                 • allow <user> - Add user to allowed list\n\
                 • deny <user> - Remove user from allowed list\n\n\
                 Note: Execution approvals control which users can approve\n\
                 dangerous commands (bash, run) in channels like Discord."
            )
        }
    }
}

pub fn docs_command(topic: Option<&str>) -> String {
    let base = "https://docs.molt.bot";
    match topic.map(str::trim).filter(|s| !s.is_empty()) {
        Some(t) => format!("docs: {}/{}", base, t.trim_start_matches('/')),
        None => format!("docs: {}", base),
    }
}

pub fn dns_command(action: &str) -> String {
    let action = action.trim();

    match action {
        "list" | "" => {
            let mut output = String::from("DNS Service Discovery:\n");
            output.push_str(&"=".repeat(50));
            output.push('\n');
            output.push_str("No DNS services configured.\n");
            output.push_str("\nDNS discovery helps find services on local network.\n");
            output
        }
        "status" => "DNS discovery: No active DNS services running".to_string(),
        _ => {
            format!(
                "DNS Service Discovery\n\n\
                 Available actions:\n\
                 • list - List known DNS services\n\
                 • status - Show DNS discovery status\n\n\
                 DNS discovery helps detect services like gateways,\n\
                 nodes, and other devices on the local network."
            )
        }
    }
}

pub fn directory_command(action: &str) -> String {
    let action = action.trim();

    match action {
        "list" | "" => {
            let config_dir = crate::utils::CONFIG_DIR.as_str();
            let mut output = String::from("Service Directory:\n");
            output.push_str(&"=".repeat(50));
            output.push('\n');
            output.push_str(&format!("Config directory: {}\n", config_dir));
            output.push_str("\nNo registered services.\n");
            output
        }
        "status" => "Service directory: No services registered".to_string(),
        _ => {
            format!(
                "Service Directory\n\n\
                 Available actions:\n\
                 • list - List registered services\n\
                 • status - Show directory status\n\n\
                 Service directory tracks registered services and their\n\
                 connection details for the gateway."
            )
        }
    }
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
    let action = action.trim();

    match action {
        "list" | "" => {
            let config = match crate::config_io::load_config() {
                Ok(c) => c,
                Err(_) => return "No configuration found.".to_string(),
            };

            let mut output = String::from("Paired Devices:\n");
            output.push_str(&"=".repeat(50));
            output.push('\n');

            let mut found = false;

            if let Some(channels) = &config.channels {
                if let Some(telegram) = &channels.telegram {
                    for (acc_name, acc) in &telegram.accounts {
                        for id in &acc.allowlist {
                            found = true;
                            output.push_str(&format!("✓ Telegram [{}]: {}\n", acc_name, id));
                        }
                    }
                }
                if let Some(discord) = &channels.discord {
                    for (acc_name, acc) in &discord.accounts {
                        for id in &acc.allowlist {
                            found = true;
                            output.push_str(&format!("✓ Discord [{}]: {}\n", acc_name, id));
                        }
                    }
                }
                for (acc_name, acc) in &channels.slack {
                    for id in &acc.allowlist {
                        found = true;
                        output.push_str(&format!("✓ Slack [{}]: {}\n", acc_name, id));
                    }
                }
                for (acc_name, acc) in &channels.whatsapp {
                    for id in &acc.allowlist {
                        found = true;
                        output.push_str(&format!("✓ WhatsApp [{}]: {}\n", acc_name, id));
                    }
                }
            }

            if !found {
                output.push_str("No paired devices.\n");
                output.push_str("\nUse 'krabkrab pairing' to manage device pairing.\n");
            }

            output
        }
        "revoke" => "devices revoke: Use 'krabkrab pairing revoke <device-id>' instead".to_string(),
        _ => {
            format!(
                "Paired Devices Management\n\n\
                 Available actions:\n\
                 • list - List paired devices\n\
                 • revoke - Revoke a device (use pairing revoke)\n\n\
                 Note: Paired devices are managed via the pairing system.\n\
                 Use: krabkrab pairing"
            )
        }
    }
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
