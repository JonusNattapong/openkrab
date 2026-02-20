//! health — Health check commands.
//! Ported from `openkrab/src/commands/health.ts` (Phase 6).

use crate::config::AppConfig;
use serde::{Deserialize, Serialize};

/// Health check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResult {
    pub healthy: bool,
    pub checks: Vec<HealthCheck>,
    pub timestamp: String,
}

/// Individual health check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub name: String,
    pub status: CheckStatus,
    pub message: String,
    pub duration_ms: u64,
}

/// Check status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckStatus {
    Pass,
    Fail,
    Warn,
}

/// Run health checks.
pub async fn health_command(cfg: &AppConfig, timeout_ms: u64) -> HealthResult {
    let _start = std::time::Instant::now();
    let mut checks = Vec::new();

    // Check 1: Config load
    let config_check = HealthCheck {
        name: "config".to_string(),
        status: CheckStatus::Pass,
        message: "Configuration loaded successfully".to_string(),
        duration_ms: 0,
    };
    checks.push(config_check);

    // Check 2: Gateway connectivity
    let gateway_start = std::time::Instant::now();
    let gateway_status = check_gateway_health(cfg, timeout_ms).await;
    let gateway_check = HealthCheck {
        name: "gateway".to_string(),
        status: if gateway_status {
            CheckStatus::Pass
        } else {
            CheckStatus::Fail
        },
        message: if gateway_status {
            "Gateway is reachable".to_string()
        } else {
            "Gateway is not reachable".to_string()
        },
        duration_ms: gateway_start.elapsed().as_millis() as u64,
    };
    checks.push(gateway_check);

    // Check 3: Memory system
    let memory_enabled = cfg.memory.enabled.unwrap_or(true);
    let memory_check = HealthCheck {
        name: "memory".to_string(),
        status: if memory_enabled {
            CheckStatus::Pass
        } else {
            CheckStatus::Warn
        },
        message: if memory_enabled {
            "Memory system enabled".to_string()
        } else {
            "Memory system disabled".to_string()
        },
        duration_ms: 0,
    };
    checks.push(memory_check);

    // Check 4: Auth profiles
    let auth_count = cfg.auth.profiles.len();
    let auth_check = HealthCheck {
        name: "auth".to_string(),
        status: if auth_count > 0 {
            CheckStatus::Pass
        } else {
            CheckStatus::Warn
        },
        message: format!("{} auth profile(s) configured", auth_count),
        duration_ms: 0,
    };
    checks.push(auth_check);

    let healthy = checks
        .iter()
        .all(|c| !matches!(c.status, CheckStatus::Fail));

    HealthResult {
        healthy,
        checks,
        timestamp: chrono::Utc::now().to_rfc3339(),
    }
}

/// Check gateway health.
async fn check_gateway_health(cfg: &AppConfig, timeout_ms: u64) -> bool {
    let gateway_url = cfg
        .gateway
        .url
        .clone()
        .unwrap_or_else(|| format!("http://127.0.0.1:{}", crate::gateway::DEFAULT_PORT));

    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(timeout_ms))
        .build()
    {
        Ok(c) => c,
        Err(_) => return false,
    };

    match client.get(format!("{}/health", gateway_url)).send().await {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}

/// Format health result for display.
pub fn format_health_result(result: &HealthResult, json: bool) -> String {
    if json {
        return serde_json::to_string_pretty(result).unwrap_or_default();
    }

    let mut lines = vec![
        format!("Health Check - {}", result.timestamp),
        String::new(),
    ];

    for check in &result.checks {
        let icon = match check.status {
            CheckStatus::Pass => "✓",
            CheckStatus::Fail => "✗",
            CheckStatus::Warn => "⚠",
        };
        lines.push(format!(
            "{} {}: {} ({}ms)",
            icon, check.name, check.message, check.duration_ms
        ));
    }

    lines.push(String::new());
    if result.healthy {
        lines.push("✅ All checks passed".to_string());
    } else {
        lines.push("❌ Some checks failed".to_string());
    }

    lines.join("\n")
}

/// Format health check failure.
pub fn format_health_check_failure(error: &anyhow::Error) -> String {
    format!("Health check failed: {}", error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_health_result_text() {
        let result = HealthResult {
            healthy: true,
            checks: vec![HealthCheck {
                name: "test".to_string(),
                status: CheckStatus::Pass,
                message: "OK".to_string(),
                duration_ms: 10,
            }],
            timestamp: "2026-02-19T00:00:00Z".to_string(),
        };

        let output = format_health_result(&result, false);
        assert!(output.contains("✓ test"));
        assert!(output.contains("All checks passed"));
    }

    #[test]
    fn test_format_health_result_json() {
        let result = HealthResult {
            healthy: true,
            checks: vec![],
            timestamp: "2026-02-19T00:00:00Z".to_string(),
        };

        let output = format_health_result(&result, true);
        assert!(output.contains("healthy"));
        assert!(output.contains("timestamp"));
    }
}
