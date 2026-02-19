//! plugins — Plugin system: manifest, registry, loader, hooks.
//! Ported from `openclaw/src/plugins/` (Phase 9).

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Manifest ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// Plugin identifier (snake_case).
    pub name: String,
    /// Semver version string.
    pub version: String,
    /// Human-readable description.
    pub description: String,
    /// Author name or email.
    pub author: Option<String>,
    /// Whether this plugin is currently enabled.
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Plugin kind.
    #[serde(default)]
    pub kind: PluginKind,
    /// Required capabilities / permissions.
    #[serde(default)]
    pub requires: Vec<String>,
    /// Entry-point (path or URL).
    pub entry: Option<String>,
}

fn default_true() -> bool {
    true
}

impl PluginManifest {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            description: String::new(),
            author: None,
            enabled: true,
            kind: PluginKind::Extension,
            requires: Vec::new(),
            entry: None,
        }
    }

    /// Validate the manifest — name must be non-empty, version must look like semver.
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            bail!("plugin name must not be empty");
        }
        if self.name.contains(' ') {
            bail!("plugin name must not contain spaces: `{}`", self.name);
        }
        // Simple semver check: at least one dot
        if !self.version.contains('.') && !self.version.is_empty() {
            bail!(
                "plugin version `{}` does not look like semver",
                self.version
            );
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum PluginKind {
    #[default]
    Extension,
    Connector,
    Provider,
    Tool,
    Auth,
}

// ─── Plugin status ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginStatus {
    Installed,
    Enabled,
    Disabled,
    Error { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginEntry {
    pub manifest: PluginManifest,
    pub status: PluginStatus,
    pub install_path: Option<std::path::PathBuf>,
}

impl PluginEntry {
    pub fn new(manifest: PluginManifest) -> Self {
        let status = if manifest.enabled {
            PluginStatus::Enabled
        } else {
            PluginStatus::Disabled
        };
        Self {
            manifest,
            status,
            install_path: None,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.status == PluginStatus::Enabled
    }
}

// ─── Plugin registry ──────────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct PluginRegistry {
    plugins: HashMap<String, PluginEntry>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a plugin from its manifest.
    pub fn register(&mut self, manifest: PluginManifest) -> Result<()> {
        manifest.validate()?;
        let name = manifest.name.clone();
        self.plugins.insert(name, PluginEntry::new(manifest));
        Ok(())
    }

    /// Get a plugin entry by name.
    pub fn get(&self, name: &str) -> Option<&PluginEntry> {
        self.plugins.get(name)
    }

    /// Enable a plugin.
    pub fn enable(&mut self, name: &str) -> Result<()> {
        match self.plugins.get_mut(name) {
            None => bail!("plugin `{}` not found", name),
            Some(e) => {
                e.status = PluginStatus::Enabled;
                Ok(())
            }
        }
    }

    /// Disable a plugin.
    pub fn disable(&mut self, name: &str) -> Result<()> {
        match self.plugins.get_mut(name) {
            None => bail!("plugin `{}` not found", name),
            Some(e) => {
                e.status = PluginStatus::Disabled;
                Ok(())
            }
        }
    }

    /// Uninstall / remove a plugin.
    pub fn remove(&mut self, name: &str) -> bool {
        self.plugins.remove(name).is_some()
    }

    /// List all installed plugin names.
    pub fn list(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.plugins.keys().map(|k| k.as_str()).collect();
        names.sort_unstable();
        names
    }

    /// List all enabled plugins.
    pub fn enabled(&self) -> Vec<&PluginEntry> {
        self.plugins.values().filter(|e| e.is_enabled()).collect()
    }

    pub fn len(&self) -> usize {
        self.plugins.len()
    }
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }
}

// ─── Plugin hook slots ────────────────────────────────────────────────────────

/// Well-known hook phases that plugins can register on.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HookPhase {
    BeforeAgentStart,
    AfterAgentStart,
    BeforeToolCall,
    AfterToolCall,
    BeforeLlmRequest,
    AfterLlmResponse,
    BeforeReply,
    AfterReply,
    OnCompaction,
    OnSessionEnd,
}

/// A registered plugin hook.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginHook {
    pub plugin: String,
    pub phase: HookPhase,
    pub priority: i32,
}

/// Simple hook slot registry.
#[derive(Debug, Default)]
pub struct HookSlots {
    slots: HashMap<HookPhase, Vec<PluginHook>>,
}

impl HookSlots {
    pub fn register(&mut self, hook: PluginHook) {
        let phase = hook.phase.clone();
        let entry = self.slots.entry(phase).or_default();
        entry.push(hook);
        entry.sort_by_key(|h| h.priority);
    }

    pub fn hooks_for(&self, phase: &HookPhase) -> &[PluginHook] {
        self.slots.get(phase).map(|v| v.as_slice()).unwrap_or(&[])
    }

    pub fn clear_plugin(&mut self, plugin: &str) {
        for hooks in self.slots.values_mut() {
            hooks.retain(|h| h.plugin != plugin);
        }
    }
}

// ─── Discovery helpers ────────────────────────────────────────────────────────

/// Check if a given plugin name looks like a valid npm/package-style identifier.
pub fn is_valid_plugin_name(name: &str) -> bool {
    !name.is_empty()
        && !name.contains(' ')
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '-' | '_' | '.'))
}

/// Build a display name from a plugin slug (e.g. "my-plugin" → "My Plugin").
pub fn display_name(slug: &str) -> String {
    slug.replace('-', " ")
        .replace('_', " ")
        .split_whitespace()
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().to_string() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_validate_ok() {
        let m = PluginManifest::new("my-plugin", "1.0.0");
        assert!(m.validate().is_ok());
    }

    #[test]
    fn manifest_validate_empty_name() {
        let m = PluginManifest::new("", "1.0.0");
        assert!(m.validate().is_err());
    }

    #[test]
    fn manifest_validate_name_with_space() {
        let m = PluginManifest::new("my plugin", "1.0.0");
        assert!(m.validate().is_err());
    }

    #[test]
    fn registry_register_enable_disable() {
        let mut reg = PluginRegistry::new();
        reg.register(PluginManifest::new("test-plugin", "1.0.0"))
            .unwrap();
        assert_eq!(reg.len(), 1);
        assert!(reg.get("test-plugin").unwrap().is_enabled());
        reg.disable("test-plugin").unwrap();
        assert!(!reg.get("test-plugin").unwrap().is_enabled());
        reg.enable("test-plugin").unwrap();
        assert!(reg.get("test-plugin").unwrap().is_enabled());
    }

    #[test]
    fn registry_remove() {
        let mut reg = PluginRegistry::new();
        reg.register(PluginManifest::new("p1", "0.1.0")).unwrap();
        assert!(reg.remove("p1"));
        assert!(!reg.remove("p1")); // already gone
    }

    #[test]
    fn hook_slots() {
        let mut slots = HookSlots::default();
        slots.register(PluginHook {
            plugin: "p1".into(),
            phase: HookPhase::AfterReply,
            priority: 10,
        });
        slots.register(PluginHook {
            plugin: "p2".into(),
            phase: HookPhase::AfterReply,
            priority: 5,
        });
        let hooks = slots.hooks_for(&HookPhase::AfterReply);
        assert_eq!(hooks.len(), 2);
        assert_eq!(hooks[0].priority, 5); // sorted by priority
        slots.clear_plugin("p1");
        assert_eq!(slots.hooks_for(&HookPhase::AfterReply).len(), 1);
    }

    #[test]
    fn display_name_from_slug() {
        assert_eq!(display_name("my-plugin"), "My Plugin");
        assert_eq!(display_name("voice_call"), "Voice Call");
    }

    #[test]
    fn valid_plugin_name() {
        assert!(is_valid_plugin_name("my-plugin"));
        assert!(!is_valid_plugin_name("my plugin"));
        assert!(!is_valid_plugin_name(""));
    }
}
