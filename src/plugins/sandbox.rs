//! sandbox — Security sandboxing for untrusted plugins.
//!
//! Provides resource limits, filesystem isolation, and network controls
//! for safely executing third-party plugins.

use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Sandbox security level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SandboxLevel {
    /// No sandboxing - full access (for trusted built-in plugins only)
    None,
    /// Light sandboxing - basic resource limits
    Light,
    /// Medium sandboxing - resource limits + filesystem restrictions
    Medium,
    /// Strict sandboxing - complete isolation
    Strict,
}

impl Default for SandboxLevel {
    fn default() -> Self {
        SandboxLevel::Medium
    }
}

/// Resource limits for sandboxed plugins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum memory in bytes (default: 128MB)
    #[serde(default = "default_max_memory")]
    pub max_memory: usize,
    /// Maximum CPU time per call in milliseconds (default: 30s)
    #[serde(default = "default_max_cpu_ms")]
    pub max_cpu_ms: u64,
    /// Maximum execution time per call (default: 60s)
    #[serde(default = "default_max_execution_time")]
    pub max_execution_time_secs: u64,
    /// Maximum stack size in bytes (default: 8MB)
    #[serde(default = "default_max_stack")]
    pub max_stack: usize,
    /// Maximum file size that can be read/written (default: 10MB)
    #[serde(default = "default_max_file_size")]
    pub max_file_size: usize,
}

fn default_max_memory() -> usize { 128 * 1024 * 1024 } // 128MB
fn default_max_cpu_ms() -> u64 { 30_000 } // 30s
fn default_max_execution_time() -> u64 { 60 } // 60s
fn default_max_stack() -> usize { 8 * 1024 * 1024 } // 8MB
fn default_max_file_size() -> usize { 10 * 1024 * 1024 } // 10MB

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory: default_max_memory(),
            max_cpu_ms: default_max_cpu_ms(),
            max_execution_time_secs: default_max_execution_time(),
            max_stack: default_max_stack(),
            max_file_size: default_max_file_size(),
        }
    }
}

/// Filesystem access permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemPolicy {
    /// Allow read access (default: false)
    #[serde(default)]
    pub allow_read: bool,
    /// Allow write access (default: false)
    #[serde(default)]
    pub allow_write: bool,
    /// List of allowed read paths
    #[serde(default)]
    pub read_paths: Vec<PathBuf>,
    /// List of allowed write paths
    #[serde(default)]
    pub write_paths: Vec<PathBuf>,
    /// List of blocked paths (takes precedence over allowed)
    #[serde(default)]
    pub blocked_paths: Vec<PathBuf>,
    /// Allow reading from plugin's own directory
    #[serde(default = "default_true")]
    pub allow_plugin_dir_read: bool,
    /// Allow writing to temp directory
    #[serde(default = "default_true")]
    pub allow_temp_write: bool,
}

fn default_true() -> bool { true }

impl Default for FilesystemPolicy {
    fn default() -> Self {
        Self {
            allow_read: false,
            allow_write: false,
            read_paths: Vec::new(),
            write_paths: Vec::new(),
            blocked_paths: Vec::new(),
            allow_plugin_dir_read: true,
            allow_temp_write: true,
        }
    }
}

impl FilesystemPolicy {
    /// Check if a path is allowed for reading
    pub fn can_read(&self, path: &Path) -> bool {
        // Check blocked paths first
        for blocked in &self.blocked_paths {
            if path.starts_with(blocked) {
                return false;
            }
        }

        if !self.allow_read {
            // Check explicit read paths
            return self.read_paths.iter().any(|allowed| path.starts_with(allowed));
        }

        true
    }

    /// Check if a path is allowed for writing
    pub fn can_write(&self, path: &Path) -> bool {
        // Check blocked paths first
        for blocked in &self.blocked_paths {
            if path.starts_with(blocked) {
                return false;
            }
        }

        if !self.allow_write {
            // Check explicit write paths
            return self.write_paths.iter().any(|allowed| path.starts_with(allowed));
        }

        true
    }
}

/// Network access policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPolicy {
    /// Allow network access (default: false)
    #[serde(default)]
    pub allow_network: bool,
    /// Allowed outbound hosts/domains
    #[serde(default)]
    pub allowed_hosts: Vec<String>,
    /// Blocked hosts/domains
    #[serde(default)]
    pub blocked_hosts: Vec<String>,
    /// Allowed ports (empty = all)
    #[serde(default)]
    pub allowed_ports: Vec<u16>,
    /// Allow localhost access
    #[serde(default)]
    pub allow_localhost: bool,
    /// Allow only HTTPS
    #[serde(default)]
    pub https_only: bool,
}

impl Default for NetworkPolicy {
    fn default() -> Self {
        Self {
            allow_network: false,
            allowed_hosts: Vec::new(),
            blocked_hosts: Vec::new(),
            allowed_ports: Vec::new(),
            allow_localhost: false,
            https_only: true,
        }
    }
}

impl NetworkPolicy {
    /// Check if a host is allowed
    pub fn can_connect(&self, host: &str, port: u16) -> bool {
        if !self.allow_network {
            return false;
        }

        // Check blocked hosts
        if self.blocked_hosts.iter().any(|h| host == h || host.ends_with(h)) {
            return false;
        }

        // Check localhost
        if host == "localhost" || host == "127.0.0.1" || host == "::1" {
            return self.allow_localhost;
        }

        // Check allowed hosts
        if !self.allowed_hosts.is_empty() {
            if !self.allowed_hosts.iter().any(|h| host == h || host.ends_with(h)) {
                return false;
            }
        }

        // Check ports
        if !self.allowed_ports.is_empty() {
            if !self.allowed_ports.contains(&port) {
                return false;
            }
        }

        true
    }
}

/// Environment variable policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentPolicy {
    /// Allow access to environment variables (default: false)
    #[serde(default)]
    pub allow_env_access: bool,
    /// List of allowed environment variable prefixes
    #[serde(default)]
    pub allowed_prefixes: Vec<String>,
    /// List of explicitly allowed variables
    #[serde(default)]
    pub allowed_vars: Vec<String>,
    /// List of blocked variables (takes precedence)
    #[serde(default)]
    pub blocked_vars: Vec<String>,
}

impl Default for EnvironmentPolicy {
    fn default() -> Self {
        Self {
            allow_env_access: false,
            allowed_prefixes: vec!["KRABKRAB_".to_string()],
            allowed_vars: Vec::new(),
            blocked_vars: vec![
                "HOME".to_string(),
                "USER".to_string(),
                "PATH".to_string(),
                "SHELL".to_string(),
                "API_KEY".to_string(),
                "SECRET".to_string(),
                "TOKEN".to_string(),
                "PASSWORD".to_string(),
            ],
        }
    }
}

impl EnvironmentPolicy {
    /// Check if an environment variable can be accessed
    pub fn can_access(&self, var: &str) -> bool {
        // Check blocked first
        if self.blocked_vars.contains(&var.to_string()) {
            return false;
        }

        if !self.allow_env_access {
            // Check explicit allows
            if self.allowed_vars.contains(&var.to_string()) {
                return true;
            }
            // Check prefixes
            return self.allowed_prefixes.iter().any(|prefix| var.starts_with(prefix));
        }

        true
    }
}

/// Complete sandbox configuration for a plugin
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxConfig {
    /// Security level
    #[serde(default)]
    pub level: SandboxLevel,
    /// Resource limits
    #[serde(default)]
    pub resources: ResourceLimits,
    /// Filesystem policy
    #[serde(default)]
    pub filesystem: FilesystemPolicy,
    /// Network policy
    #[serde(default)]
    pub network: NetworkPolicy,
    /// Environment policy
    #[serde(default)]
    pub environment: EnvironmentPolicy,
    /// Additional capabilities to grant
    #[serde(default)]
    pub capabilities: Vec<String>,
}

impl SandboxConfig {
    /// Create config for a given security level
    pub fn for_level(level: SandboxLevel) -> Self {
        match level {
            SandboxLevel::None => Self::none(),
            SandboxLevel::Light => Self::light(),
            SandboxLevel::Medium => Self::medium(),
            SandboxLevel::Strict => Self::strict(),
        }
    }

    /// No sandboxing (trusted plugins only)
    pub fn none() -> Self {
        Self {
            level: SandboxLevel::None,
            resources: ResourceLimits {
                max_memory: 512 * 1024 * 1024, // 512MB
                max_cpu_ms: 120_000, // 2 min
                max_execution_time_secs: 300, // 5 min
                max_stack: 16 * 1024 * 1024, // 16MB
                max_file_size: 100 * 1024 * 1024, // 100MB
            },
            filesystem: FilesystemPolicy {
                allow_read: true,
                allow_write: true,
                ..Default::default()
            },
            network: NetworkPolicy {
                allow_network: true,
                ..Default::default()
            },
            environment: EnvironmentPolicy {
                allow_env_access: true,
                ..Default::default()
            },
            capabilities: Vec::new(),
        }
    }

    /// Light sandboxing
    pub fn light() -> Self {
        Self {
            level: SandboxLevel::Light,
            resources: ResourceLimits::default(),
            filesystem: FilesystemPolicy {
                allow_read: true,
                allow_write: false,
                allow_temp_write: true,
                ..Default::default()
            },
            network: NetworkPolicy {
                allow_network: true,
                allowed_hosts: vec![],
                https_only: true,
                ..Default::default()
            },
            environment: EnvironmentPolicy::default(),
            capabilities: Vec::new(),
        }
    }

    /// Medium sandboxing (default)
    pub fn medium() -> Self {
        Self {
            level: SandboxLevel::Medium,
            resources: ResourceLimits::default(),
            filesystem: FilesystemPolicy::default(),
            network: NetworkPolicy::default(),
            environment: EnvironmentPolicy::default(),
            capabilities: Vec::new(),
        }
    }

    /// Strict sandboxing
    pub fn strict() -> Self {
        Self {
            level: SandboxLevel::Strict,
            resources: ResourceLimits {
                max_memory: 64 * 1024 * 1024, // 64MB
                max_cpu_ms: 10_000, // 10s
                max_execution_time_secs: 30, // 30s
                max_stack: 4 * 1024 * 1024, // 4MB
                max_file_size: 1 * 1024 * 1024, // 1MB
            },
            filesystem: FilesystemPolicy {
                allow_read: false,
                allow_write: false,
                allow_plugin_dir_read: true,
                allow_temp_write: false,
                ..Default::default()
            },
            network: NetworkPolicy {
                allow_network: false,
                ..Default::default()
            },
            environment: EnvironmentPolicy {
                allow_env_access: false,
                allowed_prefixes: vec![],
                ..Default::default()
            },
            capabilities: Vec::new(),
        }
    }
}

/// Sandbox enforcement engine
pub struct Sandbox {
    config: SandboxConfig,
    plugin_name: String,
    plugin_dir: Option<PathBuf>,
    temp_dir: PathBuf,
}

impl Sandbox {
    /// Create a new sandbox for a plugin
    pub fn new(plugin_name: impl Into<String>, config: SandboxConfig) -> Self {
        let temp_dir = std::env::temp_dir().join("krabkrab").join("sandbox");
        Self {
            config,
            plugin_name: plugin_name.into(),
            plugin_dir: None,
            temp_dir,
        }
    }

    /// Set the plugin directory
    pub fn with_plugin_dir(mut self, dir: PathBuf) -> Self {
        self.plugin_dir = Some(dir);
        self
    }

    /// Get resource limits
    pub fn resources(&self) -> &ResourceLimits {
        &self.config.resources
    }

    /// Check if a file can be read
    pub fn can_read_file(&self, path: &Path) -> bool {
        // Always allow reading from plugin's own directory
        if self.config.filesystem.allow_plugin_dir_read {
            if let Some(ref plugin_dir) = self.plugin_dir {
                if path.starts_with(plugin_dir) {
                    return true;
                }
            }
        }

        self.config.filesystem.can_read(path)
    }

    /// Check if a file can be written
    pub fn can_write_file(&self, path: &Path) -> bool {
        // Allow writing to temp directory
        if self.config.filesystem.allow_temp_write {
            if path.starts_with(&self.temp_dir) {
                return true;
            }
        }

        self.config.filesystem.can_write(path)
    }

    /// Check if can connect to a host
    pub fn can_connect(&self, host: &str, port: u16) -> bool {
        self.config.network.can_connect(host, port)
    }

    /// Check if can access environment variable
    pub fn can_access_env(&self, var: &str) -> bool {
        self.config.environment.can_access(var)
    }

    /// Get filtered environment variables
    pub fn filtered_env(&self) -> Vec<(String, String)> {
        std::env::vars()
            .filter(|(k, _)| self.can_access_env(k))
            .collect()
    }

    /// Create a temp directory for this sandbox
    pub fn create_temp_dir(&self) -> Result<PathBuf> {
        let plugin_temp = self.temp_dir.join(&self.plugin_name);
        std::fs::create_dir_all(&plugin_temp)?;
        Ok(plugin_temp)
    }

    /// Clean up temp directory
    pub fn cleanup_temp(&self) -> Result<()> {
        let plugin_temp = self.temp_dir.join(&self.plugin_name);
        if plugin_temp.exists() {
            std::fs::remove_dir_all(&plugin_temp)?;
        }
        Ok(())
    }

    /// Validate that an operation is allowed
    pub fn validate_operation(&self, op: SandboxOperation) -> Result<()> {
        match op {
            SandboxOperation::ReadFile(ref path) => {
                if !self.can_read_file(path) {
                    bail!("Sandbox: File read not allowed: {}", path.display());
                }
            }
            SandboxOperation::WriteFile(ref path) => {
                if !self.can_write_file(path) {
                    bail!("Sandbox: File write not allowed: {}", path.display());
                }
            }
            SandboxOperation::NetworkConnect { host, port } => {
                if !self.can_connect(&host, port) {
                    bail!("Sandbox: Network connection not allowed: {}:{}", host, port);
                }
            }
            SandboxOperation::ReadEnv(ref var) => {
                if !self.can_access_env(var) {
                    bail!("Sandbox: Environment variable access not allowed: {}", var);
                }
            }
        }
        Ok(())
    }
}

/// Operations that can be sandboxed
#[derive(Debug, Clone)]
pub enum SandboxOperation {
    ReadFile(PathBuf),
    WriteFile(PathBuf),
    NetworkConnect { host: String, port: u16 },
    ReadEnv(String),
}

/// Global sandbox manager
pub struct SandboxManager {
    default_config: SandboxConfig,
    plugin_configs: Arc<RwLock<HashMap<String, SandboxConfig>>>,
}

impl SandboxManager {
    /// Create a new sandbox manager
    pub fn new() -> Self {
        Self {
            default_config: SandboxConfig::medium(),
            plugin_configs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Set default sandbox configuration
    pub fn set_default_config(&mut self, config: SandboxConfig) {
        self.default_config = config;
    }

    /// Set sandbox config for a specific plugin
    pub async fn set_plugin_config(&self, plugin_name: &str, config: SandboxConfig) {
        let mut configs = self.plugin_configs.write().await;
        configs.insert(plugin_name.to_string(), config);
    }

    /// Get sandbox config for a plugin
    pub async fn get_config(&self, plugin_name: &str, level: Option<SandboxLevel>) -> SandboxConfig {
        let configs = self.plugin_configs.read().await;

        if let Some(config) = configs.get(plugin_name) {
            return config.clone();
        }

        if let Some(lvl) = level {
            return SandboxConfig::for_level(lvl);
        }

        self.default_config.clone()
    }

    /// Create a sandbox for a plugin
    pub async fn create_sandbox(&self, plugin_name: &str, plugin_dir: Option<PathBuf>) -> Sandbox {
        let config = self.get_config(plugin_name, None).await;
        let mut sandbox = Sandbox::new(plugin_name, config);

        if let Some(dir) = plugin_dir {
            sandbox = sandbox.with_plugin_dir(dir);
        }

        sandbox
    }
}

impl Default for SandboxManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Extension trait for WASI context to apply sandbox policies
pub trait ApplySandbox {
    fn apply_sandbox(&mut self, sandbox: &Sandbox) -> Result<()>;
}

#[cfg(feature = "wasm-plugins")]
impl ApplySandbox for wasmtime_wasi::WasiCtxBuilder {
    fn apply_sandbox(&mut self, sandbox: &Sandbox) -> Result<()> {
        let config = &sandbox.config;

        // Apply filesystem restrictions
        if !config.filesystem.allow_read {
            // Preopen only allowed paths
            for path in &config.filesystem.read_paths {
                self.preopen_dir(path, path)?;
            }

            // Preopen plugin directory
            if let Some(ref plugin_dir) = sandbox.plugin_dir {
                self.preopen_dir(plugin_dir, plugin_dir)?;
            }

            // Preopen temp directory
            if config.filesystem.allow_temp_write {
                let temp = sandbox.create_temp_dir()?;
                self.preopen_dir(&temp, &temp)?;
            }
        }

        // Apply environment restrictions
        let filtered_env: Vec<(&str, &str)> = sandbox.filtered_env()
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();

        self.envs(&filtered_env);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sandbox_level_configs() {
        let none = SandboxConfig::none();
        assert!(none.filesystem.allow_read);
        assert!(none.network.allow_network);

        let strict = SandboxConfig::strict();
        assert!(!strict.filesystem.allow_read);
        assert!(!strict.network.allow_network);
        assert!(strict.resources.max_memory < none.resources.max_memory);
    }

    #[test]
    fn filesystem_policy() {
        let policy = FilesystemPolicy {
            allow_read: false,
            read_paths: vec![PathBuf::from("/allowed")],
            blocked_paths: vec![PathBuf::from("/allowed/secret")],
            ..Default::default()
        };

        assert!(!policy.can_read(Path::new("/other")));
        assert!(policy.can_read(Path::new("/allowed/file.txt")));
        assert!(!policy.can_read(Path::new("/allowed/secret/data.txt")));
    }

    #[test]
    fn network_policy() {
        let policy = NetworkPolicy {
            allow_network: true,
            allowed_hosts: vec!["api.example.com".to_string()],
            blocked_hosts: vec!["evil.com".to_string()],
            allow_localhost: false,
            ..Default::default()
        };

        assert!(policy.can_connect("api.example.com", 443));
        assert!(!policy.can_connect("evil.com", 80));
        assert!(!policy.can_connect("localhost", 8080));
        assert!(!policy.can_connect("other.com", 80));
    }

    #[test]
    fn env_policy() {
        let policy = EnvironmentPolicy {
            allow_env_access: false,
            allowed_prefixes: vec!["KRABKRAB_".to_string()],
            blocked_vars: vec!["KRABKRAB_SECRET".to_string()],
            ..Default::default()
        };

        assert!(policy.can_access("KRABKRAB_SETTING"));
        assert!(!policy.can_access("KRABKRAB_SECRET"));
        assert!(!policy.can_access("HOME"));
    }
}
