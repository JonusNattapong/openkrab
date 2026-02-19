//! doctor_gateway — Gateway health checks for doctor command.
//! Ported from `openclaw/src/commands/doctor-gateway-health.ts` (Phase 6).

use crate::config::AppConfig;

/// Gateway health check result.
#[derive(Debug, Clone)]
pub struct GatewayHealthResult {
    pub healthy: bool,
    pub message: String,
    pub channel_issues: Vec<ChannelIssue>,
}

/// Channel status issue.
#[derive(Debug, Clone)]
pub struct ChannelIssue {
    pub channel: String,
    pub account_id: String,
    pub message: String,
    pub fix: Option<String>,
}

/// Check gateway health.
pub async fn check_gateway_health(cfg: &AppConfig, timeout_ms: Option<u64>) -> GatewayHealthResult {
    let timeout_ms = timeout_ms.unwrap_or(10_000);

    // Try to connect to gateway
    let gateway_url = cfg
        .gateway
        .url
        .clone()
        .unwrap_or_else(|| format!("http://127.0.0.1:{}", crate::gateway::DEFAULT_PORT));

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(timeout_ms))
        .build();

    let client = match client {
        Ok(c) => c,
        Err(e) => {
            return GatewayHealthResult {
                healthy: false,
                message: format!("Failed to create HTTP client: {}", e),
                channel_issues: vec![],
            };
        }
    };

    // Try health endpoint
    let health_url = format!("{}/health", gateway_url);
    match client.get(&health_url).send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                GatewayHealthResult {
                    healthy: true,
                    message: "Gateway is healthy".to_string(),
                    channel_issues: vec![],
                }
            } else {
                GatewayHealthResult {
                    healthy: false,
                    message: format!("Gateway returned status {}", resp.status()),
                    channel_issues: vec![],
                }
            }
        }
        Err(e) => {
            if e.is_timeout() {
                GatewayHealthResult {
                    healthy: false,
                    message: format!("Gateway connection timed out after {}ms", timeout_ms),
                    channel_issues: vec![],
                }
            } else if e.is_connect() {
                GatewayHealthResult {
                    healthy: false,
                    message: "Gateway not running".to_string(),
                    channel_issues: vec![],
                }
            } else {
                GatewayHealthResult {
                    healthy: false,
                    message: format!("Gateway connection failed: {}", e),
                    channel_issues: vec![],
                }
            }
        }
    }
}

/// Collect channel status issues from gateway response.
pub fn collect_channel_status_issues(status: &serde_json::Value) -> Vec<ChannelIssue> {
    let mut issues = Vec::new();

    if let Some(channels) = status.get("channels").and_then(|c| c.as_array()) {
        for channel in channels {
            let name = channel
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("unknown");

            let account_id = channel
                .get("account_id")
                .and_then(|a| a.as_str())
                .unwrap_or("default");

            let healthy = channel
                .get("healthy")
                .and_then(|h| h.as_bool())
                .unwrap_or(true);

            if !healthy {
                let message = channel
                    .get("error")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Channel unhealthy")
                    .to_string();

                let fix = channel
                    .get("fix")
                    .and_then(|f| f.as_str())
                    .map(|s| s.to_string());

                issues.push(ChannelIssue {
                    channel: name.to_string(),
                    account_id: account_id.to_string(),
                    message,
                    fix,
                });
            }
        }
    }

    issues
}

/// Format gateway health result for display.
pub fn format_gateway_health(result: &GatewayHealthResult) -> String {
    let mut lines = vec![result.message.clone()];

    if !result.channel_issues.is_empty() {
        lines.push("\nChannel issues:".to_string());
        for issue in &result.channel_issues {
            let fix_str = issue
                .fix
                .as_ref()
                .map(|f| format!(" (fix: {})", f))
                .unwrap_or_default();
            lines.push(format!(
                "  - {} {}: {}{}",
                issue.channel, issue.account_id, issue.message, fix_str
            ));
        }
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_collect_channel_status_issues_empty() {
        let status = json!({ "channels": [] });
        let issues = collect_channel_status_issues(&status);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_collect_channel_status_issues_with_problems() {
        let status = json!({
            "channels": [
                {
                    "name": "slack",
                    "account_id": "default",
                    "healthy": false,
                    "error": "Token expired",
                    "fix": "Run 'krabkrab auth refresh slack'"
                },
                {
                    "name": "telegram",
                    "account_id": "default",
                    "healthy": true
                }
            ]
        });

        let issues = collect_channel_status_issues(&status);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].channel, "slack");
        assert_eq!(issues[0].message, "Token expired");
        assert!(issues[0].fix.is_some());
    }
}
