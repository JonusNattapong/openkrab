//! config_reload — Gateway config hot-reload.
//! Ported from `openclaw/src/gateway/config-reload.ts`.
//!
//! Watches config files for changes and builds a reload plan that determines
//! which parts of the gateway need to be restarted vs hot-reloaded.

use anyhow::Result;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;

// ─── Reload mode ──────────────────────────────────────────────────────────────

/// How the gateway should respond to config changes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum GatewayReloadMode {
    /// Do not reload on config changes.
    Off,
    /// Always restart the entire gateway.
    Restart,
    /// Hot-reload when possible, restart only when necessary.
    #[default]
    Hybrid,
    /// Only hot-reload; ignore changes that would require restart.
    Hot,
}

/// Gateway reload settings extracted from config.
#[derive(Debug, Clone)]
pub struct GatewayReloadSettings {
    pub mode: GatewayReloadMode,
    pub debounce_ms: u64,
}

impl Default for GatewayReloadSettings {
    fn default() -> Self {
        Self {
            mode: GatewayReloadMode::Hybrid,
            debounce_ms: 300,
        }
    }
}

// ─── Reload plan ──────────────────────────────────────────────────────────────

/// A plan describing what should happen when config changes.
#[derive(Debug, Clone, Default)]
pub struct GatewayReloadPlan {
    /// Config paths that changed.
    pub changed_paths: Vec<String>,
    /// Whether the gateway needs a full restart.
    pub restart_gateway: bool,
    /// Reasons for full restart.
    pub restart_reasons: Vec<String>,
    /// Reasons for hot reload.
    pub hot_reasons: Vec<String>,
    /// Whether hooks need to be reloaded.
    pub reload_hooks: bool,
    /// Whether browser control needs restart.
    pub restart_browser_control: bool,
    /// Whether cron needs restart.
    pub restart_cron: bool,
    /// Whether heartbeat needs restart.
    pub restart_heartbeat: bool,
    /// Channels that need to be restarted.
    pub restart_channels: HashSet<String>,
    /// Paths that don't require any action.
    pub noop_paths: Vec<String>,
}

// ─── Reload rules ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum ReloadKind {
    Restart,
    Hot,
    None,
}

#[derive(Debug, Clone)]
enum ReloadAction {
    ReloadHooks,
    RestartBrowserControl,
    RestartCron,
    RestartHeartbeat,
    #[allow(dead_code)]
    RestartChannel(String),
}

#[derive(Debug, Clone)]
struct ReloadRule {
    prefix: String,
    kind: ReloadKind,
    actions: Vec<ReloadAction>,
}

fn build_reload_rules() -> Vec<ReloadRule> {
    vec![
        // No-op prefixes (safe to ignore)
        ReloadRule {
            prefix: "gateway.remote".into(),
            kind: ReloadKind::None,
            actions: vec![],
        },
        ReloadRule {
            prefix: "gateway.reload".into(),
            kind: ReloadKind::None,
            actions: vec![],
        },
        // Hot-reload prefixes
        ReloadRule {
            prefix: "hooks".into(),
            kind: ReloadKind::Hot,
            actions: vec![ReloadAction::ReloadHooks],
        },
        ReloadRule {
            prefix: "agents.defaults.heartbeat".into(),
            kind: ReloadKind::Hot,
            actions: vec![ReloadAction::RestartHeartbeat],
        },
        ReloadRule {
            prefix: "agent.heartbeat".into(),
            kind: ReloadKind::Hot,
            actions: vec![ReloadAction::RestartHeartbeat],
        },
        ReloadRule {
            prefix: "cron".into(),
            kind: ReloadKind::Hot,
            actions: vec![ReloadAction::RestartCron],
        },
        ReloadRule {
            prefix: "browser".into(),
            kind: ReloadKind::Hot,
            actions: vec![ReloadAction::RestartBrowserControl],
        },
        // No-op prefixes (non-gateway changes)
        ReloadRule {
            prefix: "meta".into(),
            kind: ReloadKind::None,
            actions: vec![],
        },
        ReloadRule {
            prefix: "identity".into(),
            kind: ReloadKind::None,
            actions: vec![],
        },
        ReloadRule {
            prefix: "wizard".into(),
            kind: ReloadKind::None,
            actions: vec![],
        },
        ReloadRule {
            prefix: "logging".into(),
            kind: ReloadKind::None,
            actions: vec![],
        },
        ReloadRule {
            prefix: "models".into(),
            kind: ReloadKind::None,
            actions: vec![],
        },
        ReloadRule {
            prefix: "agents".into(),
            kind: ReloadKind::None,
            actions: vec![],
        },
        ReloadRule {
            prefix: "tools".into(),
            kind: ReloadKind::None,
            actions: vec![],
        },
        ReloadRule {
            prefix: "bindings".into(),
            kind: ReloadKind::None,
            actions: vec![],
        },
        ReloadRule {
            prefix: "routing".into(),
            kind: ReloadKind::None,
            actions: vec![],
        },
        ReloadRule {
            prefix: "session".into(),
            kind: ReloadKind::None,
            actions: vec![],
        },
        // Restart-requiring prefixes
        ReloadRule {
            prefix: "plugins".into(),
            kind: ReloadKind::Restart,
            actions: vec![],
        },
        ReloadRule {
            prefix: "gateway".into(),
            kind: ReloadKind::Restart,
            actions: vec![],
        },
        ReloadRule {
            prefix: "discovery".into(),
            kind: ReloadKind::Restart,
            actions: vec![],
        },
        ReloadRule {
            prefix: "canvasHost".into(),
            kind: ReloadKind::Restart,
            actions: vec![],
        },
    ]
}

fn match_rule<'a>(path: &str, rules: &'a [ReloadRule]) -> Option<&'a ReloadRule> {
    rules
        .iter()
        .find(|rule| path == rule.prefix || path.starts_with(&format!("{}.", rule.prefix)))
}

// ─── Config diffing ───────────────────────────────────────────────────────────

/// Diff two config values and return the list of changed dot-separated paths.
pub fn diff_config_paths(prev: &Value, next: &Value, prefix: &str) -> Vec<String> {
    if prev == next {
        return Vec::new();
    }

    match (prev, next) {
        (Value::Object(prev_map), Value::Object(next_map)) => {
            let mut all_keys: HashSet<&String> = prev_map.keys().collect();
            all_keys.extend(next_map.keys());

            let mut paths = Vec::new();
            for key in all_keys {
                let prev_val = prev_map.get(key).unwrap_or(&Value::Null);
                let next_val = next_map.get(key).unwrap_or(&Value::Null);
                if prev_val == next_val {
                    continue;
                }
                let child_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", prefix, key)
                };
                let child_paths = diff_config_paths(prev_val, next_val, &child_prefix);
                if child_paths.is_empty() && prev_val != next_val {
                    paths.push(child_prefix);
                } else {
                    paths.extend(child_paths);
                }
            }
            paths
        }
        _ => {
            vec![if prefix.is_empty() {
                "<root>".to_string()
            } else {
                prefix.to_string()
            }]
        }
    }
}

// ─── Build reload plan ────────────────────────────────────────────────────────

/// Build a reload plan from a list of changed config paths.
pub fn build_gateway_reload_plan(changed_paths: &[String]) -> GatewayReloadPlan {
    let rules = build_reload_rules();
    let mut plan = GatewayReloadPlan {
        changed_paths: changed_paths.to_vec(),
        ..Default::default()
    };

    for path in changed_paths {
        match match_rule(path, &rules) {
            Some(rule) => match rule.kind {
                ReloadKind::Restart => {
                    plan.restart_gateway = true;
                    plan.restart_reasons.push(path.clone());
                }
                ReloadKind::Hot => {
                    plan.hot_reasons.push(path.clone());
                    for action in &rule.actions {
                        match action {
                            ReloadAction::ReloadHooks => plan.reload_hooks = true,
                            ReloadAction::RestartBrowserControl => {
                                plan.restart_browser_control = true
                            }
                            ReloadAction::RestartCron => plan.restart_cron = true,
                            ReloadAction::RestartHeartbeat => plan.restart_heartbeat = true,
                            ReloadAction::RestartChannel(ch) => {
                                plan.restart_channels.insert(ch.clone());
                            }
                        }
                    }
                }
                ReloadKind::None => {
                    plan.noop_paths.push(path.clone());
                }
            },
            None => {
                // Unknown path → require restart for safety
                plan.restart_gateway = true;
                plan.restart_reasons.push(path.clone());
            }
        }
    }

    plan
}

// ─── Config reloader ──────────────────────────────────────────────────────────

/// Handle for stopping the config reloader.
pub struct ConfigReloaderHandle {
    stop_tx: mpsc::Sender<()>,
    watcher: Mutex<Option<RecommendedWatcher>>,
}

impl ConfigReloaderHandle {
    /// Stop watching for config changes.
    pub async fn stop(&self) {
        let _ = self.stop_tx.send(()).await;
        let _ = self.watcher.lock().unwrap().take();
    }
}

/// Callback types for config reload events.
pub type OnHotReload = Box<dyn Fn(&GatewayReloadPlan, &Value) + Send + Sync>;
pub type OnRestart = Box<dyn Fn(&GatewayReloadPlan, &Value) + Send + Sync>;

/// Start watching a config file for changes and building reload plans.
pub fn start_gateway_config_reloader(
    watch_path: PathBuf,
    initial_config: Value,
    read_config: Arc<dyn Fn() -> Result<Value> + Send + Sync>,
    on_hot_reload: OnHotReload,
    on_restart: OnRestart,
    settings: GatewayReloadSettings,
) -> Result<ConfigReloaderHandle> {
    let (stop_tx, mut stop_rx) = mpsc::channel::<()>(1);
    let (event_tx, mut event_rx) = mpsc::unbounded_channel::<()>();

    let current_config = Arc::new(Mutex::new(initial_config));

    // Set up file watcher
    let event_tx_clone = event_tx.clone();
    let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        if let Ok(_event) = res {
            let _ = event_tx_clone.send(());
        }
    })?;

    watcher.watch(&watch_path, RecursiveMode::NonRecursive)?;

    let debounce_ms = settings.debounce_ms;
    let mode = settings.mode.clone();
    let current_config_clone = current_config.clone();
    let read_config_clone = read_config.clone();

    // Spawn the reload handler
    tokio::spawn(async move {
        let mut restart_queued = false;

        loop {
            tokio::select! {
                _ = stop_rx.recv() => {
                    tracing::info!("config reloader stopped");
                    break;
                }
                _ = event_rx.recv() => {
                    // Debounce
                    tokio::time::sleep(Duration::from_millis(debounce_ms)).await;
                    // Drain any additional events
                    while event_rx.try_recv().is_ok() {}

                    if mode == GatewayReloadMode::Off {
                        continue;
                    }

                    match read_config_clone() {
                        Ok(next_config) => {
                            let prev_config = current_config_clone.lock().unwrap().clone();
                            let changed_paths = diff_config_paths(&prev_config, &next_config, "");

                            if changed_paths.is_empty() {
                                continue;
                            }

                            tracing::info!(
                                "config change detected: {}",
                                changed_paths.join(", ")
                            );

                            let plan = build_gateway_reload_plan(&changed_paths);
                            *current_config_clone.lock().unwrap() = next_config.clone();

                            match mode {
                                GatewayReloadMode::Restart => {
                                    if !restart_queued {
                                        restart_queued = true;
                                        on_restart(&plan, &next_config);
                                    }
                                }
                                GatewayReloadMode::Hybrid => {
                                    if plan.restart_gateway {
                                        if !restart_queued {
                                            restart_queued = true;
                                            on_restart(&plan, &next_config);
                                        }
                                    } else {
                                        on_hot_reload(&plan, &next_config);
                                    }
                                }
                                GatewayReloadMode::Hot => {
                                    if !plan.restart_gateway {
                                        on_hot_reload(&plan, &next_config);
                                    } else {
                                        tracing::warn!(
                                            "config change requires restart but mode=hot, ignoring: {}",
                                            plan.restart_reasons.join(", ")
                                        );
                                    }
                                }
                                GatewayReloadMode::Off => {}
                            }
                        }
                        Err(err) => {
                            tracing::warn!("failed to read config: {}", err);
                        }
                    }
                }
            }
        }
    });

    Ok(ConfigReloaderHandle {
        stop_tx,
        watcher: Mutex::new(Some(watcher)),
    })
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_empty() {
        let a = serde_json::json!({"a": 1});
        let b = serde_json::json!({"a": 1});
        assert!(diff_config_paths(&a, &b, "").is_empty());
    }

    #[test]
    fn diff_simple_change() {
        let a = serde_json::json!({"a": 1, "b": 2});
        let b = serde_json::json!({"a": 1, "b": 3});
        let paths = diff_config_paths(&a, &b, "");
        assert_eq!(paths, vec!["b"]);
    }

    #[test]
    fn diff_nested_change() {
        let a = serde_json::json!({"gateway": {"port": 8080}});
        let b = serde_json::json!({"gateway": {"port": 9090}});
        let paths = diff_config_paths(&a, &b, "");
        assert_eq!(paths, vec!["gateway.port"]);
    }

    #[test]
    fn diff_added_key() {
        let a = serde_json::json!({"a": 1});
        let b = serde_json::json!({"a": 1, "b": 2});
        let paths = diff_config_paths(&a, &b, "");
        assert_eq!(paths, vec!["b"]);
    }

    #[test]
    fn diff_removed_key() {
        let a = serde_json::json!({"a": 1, "b": 2});
        let b = serde_json::json!({"a": 1});
        let paths = diff_config_paths(&a, &b, "");
        assert_eq!(paths, vec!["b"]);
    }

    #[test]
    fn plan_hooks_hot_reload() {
        let plan = build_gateway_reload_plan(&["hooks.gmail".to_string()]);
        assert!(!plan.restart_gateway);
        assert!(plan.reload_hooks);
        assert_eq!(plan.hot_reasons, vec!["hooks.gmail"]);
    }

    #[test]
    fn plan_cron_hot_reload() {
        let plan = build_gateway_reload_plan(&["cron.schedule".to_string()]);
        assert!(!plan.restart_gateway);
        assert!(plan.restart_cron);
    }

    #[test]
    fn plan_gateway_restart() {
        let plan = build_gateway_reload_plan(&["gateway.port".to_string()]);
        assert!(plan.restart_gateway);
    }

    #[test]
    fn plan_plugins_restart() {
        let plan = build_gateway_reload_plan(&["plugins.enabled".to_string()]);
        assert!(plan.restart_gateway);
    }

    #[test]
    fn plan_noop() {
        let plan = build_gateway_reload_plan(&["models.default".to_string()]);
        assert!(!plan.restart_gateway);
        assert!(plan.noop_paths.contains(&"models.default".to_string()));
    }

    #[test]
    fn plan_unknown_requires_restart() {
        let plan = build_gateway_reload_plan(&["someNewThing.xyz".to_string()]);
        assert!(plan.restart_gateway);
    }

    #[test]
    fn plan_mixed() {
        let plan = build_gateway_reload_plan(&[
            "hooks.onMessage".to_string(),
            "gateway.port".to_string(),
            "models.default".to_string(),
        ]);
        assert!(plan.restart_gateway);
        assert!(plan.reload_hooks);
        assert!(!plan.noop_paths.is_empty());
    }

    #[test]
    fn plan_heartbeat_hot_reload() {
        let plan = build_gateway_reload_plan(&["agents.defaults.heartbeat.interval".to_string()]);
        assert!(!plan.restart_gateway);
        assert!(plan.restart_heartbeat);
    }
}
