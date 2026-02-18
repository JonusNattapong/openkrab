//! compat — Compatibility layer: legacy name mappings and feature flags.
//! Ported from `openclaw/src/compat/legacy-names.ts` (Phase 8).
//!
//! Provides canonical name resolution for connectors and providers
//! that shipped under different names in earlier openclaw versions,
//! plus runtime feature-flag helpers.

use std::collections::HashMap;

// ─── Legacy name mappings ─────────────────────────────────────────────────────

/// Canonical name → set of accepted aliases (including itself).
static CONNECTOR_ALIASES: &[(&str, &[&str])] = &[
    ("telegram",  &["telegram", "tg", "tlg"]),
    ("slack",     &["slack", "sl"]),
    ("discord",   &["discord", "dc", "ds"]),
    ("whatsapp",  &["whatsapp", "wa", "whats"]),
    ("line",      &["line", "ln"]),
    ("signal",    &["signal", "sig"]),
    ("imessage",  &["imessage", "im", "imsg"]),
    ("matrix",    &["matrix", "mx"]),
    ("mattermost",&["mattermost", "mm"]),
    ("irc",       &["irc"]),
    ("nostr",     &["nostr"]),
];

static PROVIDER_ALIASES: &[(&str, &[&str])] = &[
    ("openai",    &["openai", "oai", "gpt"]),
    ("gemini",    &["gemini", "google", "bard"]),
    ("ollama",    &["ollama", "local"]),
    ("anthropic", &["anthropic", "claude"]),
    ("groq",      &["groq"]),
    ("mistral",   &["mistral"]),
    ("cohere",    &["cohere"]),
];

/// Resolve a connector name (possibly a legacy alias) to its canonical form.
/// Returns `None` if the name is not recognised.
pub fn canonical_connector(name: &str) -> Option<&'static str> {
    let lower = name.to_lowercase();
    for (canonical, aliases) in CONNECTOR_ALIASES {
        if aliases.iter().any(|&a| a == lower.as_str()) {
            return Some(canonical);
        }
    }
    None
}

/// Resolve a provider name (possibly a legacy alias) to its canonical form.
pub fn canonical_provider(name: &str) -> Option<&'static str> {
    let lower = name.to_lowercase();
    for (canonical, aliases) in PROVIDER_ALIASES {
        if aliases.iter().any(|&a| a == lower.as_str()) {
            return Some(canonical);
        }
    }
    None
}

/// Returns all known canonical connector names.
pub fn known_connectors() -> Vec<&'static str> {
    CONNECTOR_ALIASES.iter().map(|(c, _)| *c).collect()
}

/// Returns all known canonical provider names.
pub fn known_providers() -> Vec<&'static str> {
    PROVIDER_ALIASES.iter().map(|(p, _)| *p).collect()
}

// ─── Feature flags ────────────────────────────────────────────────────────────

/// Runtime feature flags read from environment variables.
#[derive(Debug, Clone, Default)]
pub struct FeatureFlags {
    flags: HashMap<String, bool>,
}

impl FeatureFlags {
    /// Load flags from `KRABKRAB_FEATURES` env var (comma-separated list of `flag` or `!flag`).
    pub fn from_env() -> Self {
        let mut flags = HashMap::new();
        if let Ok(val) = std::env::var("KRABKRAB_FEATURES") {
            for part in val.split(',') {
                let part = part.trim();
                if part.starts_with('!') {
                    flags.insert(part[1..].to_lowercase(), false);
                } else if !part.is_empty() {
                    flags.insert(part.to_lowercase(), true);
                }
            }
        }
        Self { flags }
    }

    /// Returns true if `flag` is enabled.
    pub fn is_enabled(&self, flag: &str) -> bool {
        *self.flags.get(&flag.to_lowercase()).unwrap_or(&false)
    }

    /// Enable a flag programmatically.
    pub fn enable(&mut self, flag: &str) {
        self.flags.insert(flag.to_lowercase(), true);
    }

    /// Disable a flag programmatically.
    pub fn disable(&mut self, flag: &str) {
        self.flags.insert(flag.to_lowercase(), false);
    }

    pub fn enabled_flags(&self) -> Vec<&str> {
        self.flags.iter().filter(|(_, &v)| v).map(|(k, _)| k.as_str()).collect()
    }
}

// ─── Config key migration ─────────────────────────────────────────────────────

/// Maps old config key names to new ones (for migrating stored configs).
static CONFIG_KEY_MIGRATIONS: &[(&str, &str)] = &[
    ("agent.name",          "agent.identity.name"),
    ("agent.emoji",         "agent.identity.emoji"),
    ("agent.personality",   "agent.identity.system_prompt"),
    ("memory.provider",     "memory.embedding_provider"),
    ("memory.path",         "memory.dir"),
    ("gateway.port",        "server.port"),
    ("telegram.bot_token",  "connectors.telegram.token"),
    ("slack.bot_token",     "connectors.slack.bot_token"),
];

/// Migrate an old config key to its new name. Returns `None` if no migration needed.
pub fn migrate_config_key(old_key: &str) -> Option<&'static str> {
    CONFIG_KEY_MIGRATIONS
        .iter()
        .find(|(old, _)| *old == old_key)
        .map(|(_, new)| *new)
}

/// Apply all key migrations to a flat `HashMap<String,String>` config map.
pub fn migrate_config_map(config: &mut HashMap<String, String>) {
    let migrations: Vec<(String, String)> = config
        .keys()
        .filter_map(|k| migrate_config_key(k).map(|new| (k.clone(), new.to_string())))
        .collect();

    for (old, new) in migrations {
        if let Some(val) = config.remove(&old) {
            config.entry(new).or_insert(val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_connector_basic() {
        assert_eq!(canonical_connector("tg"), Some("telegram"));
        assert_eq!(canonical_connector("Telegram"), Some("telegram"));
        assert_eq!(canonical_connector("wa"), Some("whatsapp"));
        assert_eq!(canonical_connector("unknown_xyz"), None);
    }

    #[test]
    fn canonical_provider_basic() {
        assert_eq!(canonical_provider("gpt"), Some("openai"));
        assert_eq!(canonical_provider("Claude"), Some("anthropic"));
        assert_eq!(canonical_provider("bard"), Some("gemini"));
        assert_eq!(canonical_provider("nope"), None);
    }

    #[test]
    fn known_connectors_list() {
        let c = known_connectors();
        assert!(c.contains(&"telegram"));
        assert!(c.contains(&"slack"));
        assert!(c.contains(&"discord"));
    }

    #[test]
    fn feature_flags_programmatic() {
        let mut ff = FeatureFlags::default();
        assert!(!ff.is_enabled("vision"));
        ff.enable("vision");
        assert!(ff.is_enabled("vision"));
        ff.disable("vision");
        assert!(!ff.is_enabled("vision"));
    }

    #[test]
    fn config_key_migration() {
        assert_eq!(migrate_config_key("agent.name"), Some("agent.identity.name"));
        assert_eq!(migrate_config_key("no.such.key"), None);
    }

    #[test]
    fn migrate_config_map_test() {
        let mut map = HashMap::new();
        map.insert("agent.name".to_string(), "KrabBot".to_string());
        map.insert("gateway.port".to_string(), "3000".to_string());
        migrate_config_map(&mut map);
        assert!(map.contains_key("agent.identity.name"));
        assert!(map.contains_key("server.port"));
        assert!(!map.contains_key("agent.name"));
    }
}
