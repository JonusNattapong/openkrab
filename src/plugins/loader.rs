//! plugin_loader — Dynamic plugin loading from filesystem.
//!
//! Provides functionality to:
//! - Discover plugins from a directory
//! - Load plugin manifests
//! - Initialize plugins (WASM or dynamic libraries)
//! - Manage plugin lifecycle

use crate::plugin_sdk::{PluginDeclaration, PluginTool};
use crate::plugins::{HookPhase, HookSlots, PluginHook, PluginManifest, PluginRegistry, PluginStatus};
use anyhow::{bail, Context, Result};
use once_cell::sync::Lazy;
use std::collections::HashMap;
#[cfg(feature = "native-plugins")]
use std::ffi::CStr;
#[cfg(feature = "native-plugins")]
use std::os::raw::c_char;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as AsyncMutex;
use tracing::{debug, error, info, warn};

// ─── Plugin loader configuration ──────────────────────────────────────────────

/// Configuration for the plugin loader.
#[derive(Debug, Clone)]
pub struct PluginLoaderConfig {
    /// Directories to search for plugins.
    pub plugin_dirs: Vec<PathBuf>,
    /// Whether to enable hot-reloading.
    pub hot_reload: bool,
    /// File extensions to consider as plugins.
    pub extensions: Vec<String>,
    /// SECURITY: Require code signatures for all plugins
    pub require_signatures: bool,
    /// SECURITY: Allow native plugins (default: false, WASM only)
    pub allow_native_plugins: bool,
    /// SECURITY: Trusted public keys for signature verification
    pub trusted_keys: Vec<String>,
    /// SECURITY: Default sandbox level for plugins
    pub default_sandbox_level: crate::plugins::sandbox::SandboxLevel,
    /// SECURITY: Maximum plugin file size (default: 50MB)
    pub max_plugin_size: usize,
}

impl Default for PluginLoaderConfig {
    fn default() -> Self {
        Self {
            plugin_dirs: vec![
                PathBuf::from("./plugins"),
                dirs::home_dir()
                    .map(|h| h.join(".krabkrab/plugins"))
                    .unwrap_or_else(|| PathBuf::from("./plugins")),
            ],
            hot_reload: false,
            // SECURITY: WASM only by default, native plugins require explicit opt-in
            extensions: vec![
                #[cfg(feature = "wasm-plugins")]
                "wasm".to_string(),
                // Native extensions only loaded if allow_native_plugins = true
            ],
            require_signatures: true, // SECURITY: Require signatures by default
            allow_native_plugins: false, // SECURITY: WASM only by default
            trusted_keys: Vec::new(),
            default_sandbox_level: crate::plugins::sandbox::SandboxLevel::Strict,
            max_plugin_size: 50 * 1024 * 1024, // 50MB
        }
    }
}

// ─── Loaded plugin ────────────────────────────────────────────────────────────

/// A loaded plugin with its instance and metadata.
#[derive(Debug)]
pub struct LoadedPlugin {
    pub manifest: PluginManifest,
    pub path: PathBuf,
    pub instance: PluginInstance,
    pub declaration: Option<PluginDeclaration>,
}

/// Plugin instance type.
#[derive(Debug)]
pub enum PluginInstance {
    /// Native dynamic library plugin.
    #[cfg(feature = "native-plugins")]
    Native(libloading::Library),
    /// WASM plugin.
    #[cfg(feature = "wasm-plugins")]
    Wasm(crate::plugins::wasm_runtime::WasmPlugin),
    /// Statically linked plugin (built-in).
    Static,
}

// ─── Plugin loader ────────────────────────────────────────────────────────────

/// Dynamic plugin loader.
pub struct PluginLoader {
    config: PluginLoaderConfig,
    /// Cache of loaded plugin instances.
    instances: HashMap<String, LoadedPlugin>,
    /// Tool declarations provided by each plugin.
    plugin_tools: HashMap<String, Vec<PluginTool>>,
    /// Hook slots registered by plugins.
    hook_slots: HookSlots,
}

impl PluginLoader {
    /// Create a new plugin loader with default configuration.
    pub fn new() -> Self {
        Self::with_config(PluginLoaderConfig::default())
    }

    /// Create a new plugin loader with custom configuration.
    pub fn with_config(config: PluginLoaderConfig) -> Self {
        Self {
            config,
            instances: HashMap::new(),
            plugin_tools: HashMap::new(),
            hook_slots: HookSlots::default(),
        }
    }

    /// Get the loader configuration.
    pub fn config(&self) -> &PluginLoaderConfig {
        &self.config
    }

    /// Discover plugins in all configured directories.
    pub fn discover(&self) -> Result<Vec<DiscoveredPlugin>> {
        let mut discovered = Vec::new();

        for dir in &self.config.plugin_dirs {
            if !dir.exists() {
                debug!("Plugin directory does not exist: {}", dir.display());
                continue;
            }

            match self.discover_in_dir(dir) {
                Ok(plugins) => discovered.extend(plugins),
                Err(e) => warn!("Failed to discover plugins in {}: {}", dir.display(), e),
            }
        }

        Ok(discovered)
    }

    /// Discover plugins in a specific directory.
    fn discover_in_dir(&self, dir: &Path) -> Result<Vec<DiscoveredPlugin>> {
        let mut discovered = Vec::new();

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Look for plugin.json or manifest.json in subdirectory
                if let Some(plugin) = self.try_load_manifest(&path)? {
                    discovered.push(plugin);
                }
            } else if let Some(ext) = path.extension() {
                // Check if it's a plugin file
                let ext = ext.to_string_lossy().to_string();
                if self.config.extensions.contains(&ext) {
                    if let Some(plugin) = self.try_load_plugin_file(&path)? {
                        discovered.push(plugin);
                    }
                }
            }
        }

        Ok(discovered)
    }

    /// Try to load a plugin manifest from a directory.
    fn try_load_manifest(&self, dir: &Path) -> Result<Option<DiscoveredPlugin>> {
        let manifest_paths = ["plugin.json", "manifest.json", "krabkrab.json"];

        for manifest_name in &manifest_paths {
            let manifest_path = dir.join(manifest_name);
            if manifest_path.exists() {
                let content = std::fs::read_to_string(&manifest_path)?;
                let manifest: PluginManifest = serde_json::from_str(&content)
                    .with_context(|| format!("Failed to parse {}", manifest_path.display()))?;

                // Look for entry point
                let entry_path = manifest
                    .entry
                    .as_ref()
                    .map(|e| dir.join(e))
                    .filter(|p| p.exists())
                    .or_else(|| {
                        // Try to find a plugin file with matching name
                        self.find_plugin_file(dir, &manifest.name)
                    });

                return Ok(Some(DiscoveredPlugin {
                    manifest,
                    manifest_path,
                    entry_path,
                    plugin_dir: Some(dir.to_path_buf()),
                }));
            }
        }

        Ok(None)
    }

    /// Try to load a plugin directly from a file.
    fn try_load_plugin_file(&self, path: &Path) -> Result<Option<DiscoveredPlugin>> {
        // For standalone plugin files, we might extract metadata from the file itself
        // For now, create a basic manifest from filename
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let manifest = PluginManifest::new(&name, "1.0.0");

        Ok(Some(DiscoveredPlugin {
            manifest,
            manifest_path: path.to_path_buf(),
            entry_path: Some(path.to_path_buf()),
            plugin_dir: path.parent().map(|p| p.to_path_buf()),
        }))
    }

    /// Find a plugin file in a directory.
    fn find_plugin_file(&self, dir: &Path, name: &str) -> Option<PathBuf> {
        for ext in &self.config.extensions {
            let path = dir.join(format!("{}.{}" , name, ext));
            if path.exists() {
                return Some(path);
            }
        }

        // Also check for index files
        for ext in &self.config.extensions {
            let path = dir.join(format!("index.{}" , ext));
            if path.exists() {
                return Some(path);
            }
        }

        None
    }

    /// Load a discovered plugin into the registry.
    pub fn load(&mut self, discovered: &DiscoveredPlugin, registry: &mut PluginRegistry) -> Result<()> {
        let name = discovered.manifest.name.clone();

        // Validate manifest
        discovered.manifest.validate()?;

        // Check if already registered
        if registry.get(&name).is_some() {
            bail!("Plugin '{}' is already registered", name);
        }

        // Register in registry
        registry.register(discovered.manifest.clone())?;

        info!("Loaded plugin '{}' v{}", name, discovered.manifest.version);

        Ok(())
    }

    /// Initialize a plugin (load its code).
    pub async fn initialize(&mut self, name: &str, registry: &mut PluginRegistry) -> Result<()> {
        let entry = registry
            .get(name)
            .with_context(|| format!("Plugin '{}' not found in registry", name))?;

        if !entry.is_enabled() {
            bail!("Plugin '{}' is disabled", name);
        }

        // Check if already initialized
        if self.instances.contains_key(name) {
            return Ok(());
        }

        // Find the discovered plugin info
        let discovered = self
            .find_discovered_plugin(name)
            .with_context(|| format!("Plugin '{}' not found on disk", name))?;

        // Load the instance based on entry type
        let (instance, declaration) = self.create_instance(&discovered).await?;

        let loaded = LoadedPlugin {
            manifest: discovered.manifest.clone(),
            path: discovered.entry_path.clone().unwrap_or_default(),
            instance,
            declaration: declaration.clone(),
        };

        if let Some(decl) = declaration {
            self.register_declaration(name, &decl);
        }

        self.instances.insert(name.to_string(), loaded);

        info!("Initialized plugin '{}'", name);

        Ok(())
    }

    /// Create a plugin instance based on the entry type.
    async fn create_instance(
        &self,
        discovered: &DiscoveredPlugin,
    ) -> Result<(PluginInstance, Option<PluginDeclaration>)> {
        match &discovered.entry_path {
            Some(path) => {
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("");

                match ext {
                    #[cfg(feature = "native-plugins")]
                    "so" | "dylib" | "dll" => self.load_native_plugin(path).await,
                    #[cfg(feature = "wasm-plugins")]
                    "wasm" => self.load_wasm_plugin(path, discovered.plugin_dir.as_deref()).await,
                    _ => {
                        warn!("Unknown plugin extension: {}", ext);
                        let declaration = self.read_declaration_json(path)?;
                        Ok((PluginInstance::Static, declaration))
                    }
                }
            }
            None => Ok((
                PluginInstance::Static,
                self.read_declaration_in_dir(discovered.plugin_dir.as_deref())?,
            )),
        }
    }

    /// Load a native dynamic library plugin.
    /// 
    /// SECURITY: This requires explicit opt-in via config.allow_native_plugins
    #[cfg(feature = "native-plugins")]
    async fn load_native_plugin(
        &self,
        path: &Path,
    ) -> Result<(PluginInstance, Option<PluginDeclaration>)> {
        // SECURITY: Check if native plugins are allowed
        if !self.config.allow_native_plugins {
            bail!(
                "Native plugins are disabled. Set allow_native_plugins=true to enable (not recommended). \
                 Consider using WASM plugins instead for better security."
            );
        }

        // SECURITY: Log native plugin load attempt
        crate::security_audit::audit().log(
            crate::security_audit::SecurityEvent::new(
                crate::security_audit::SecurityEventType::PluginLoadAttempt,
                crate::security_audit::SecuritySeverity::Warning,
                "plugin_loader",
                format!("Loading native plugin: {}", path.display()),
            )
            .with_subject(path.to_string_lossy().to_string())
        ).await;

        unsafe {
            let lib = libloading::Library::new(path)
                .with_context(|| format!("Failed to load native library from {}", path.display()))?;
            let native_manifest =
                read_json_symbol::<PluginManifest>(&lib, crate::plugin_sdk::ABI_MANIFEST_SYMBOL).ok();
            if let Some(m) = native_manifest {
                debug!(
                    "Native plugin manifest from ABI: {}@{}",
                    m.name,
                    m.version
                );
            }
            let declaration = read_json_symbol::<PluginDeclaration>(
                &lib,
                crate::plugin_sdk::ABI_DECLARATION_SYMBOL,
            )
            .ok();
            
            // SECURITY: Log successful load
            crate::security_audit::audit().log(
                crate::security_audit::SecurityEvent::new(
                    crate::security_audit::SecurityEventType::PluginLoadSuccess,
                    crate::security_audit::SecuritySeverity::Warning,
                    "plugin_loader",
                    format!("Native plugin loaded: {}", path.display()),
                )
                .with_subject(path.to_string_lossy().to_string())
            ).await;
            
            Ok((PluginInstance::Native(lib), declaration))
        }
    }

    /// Load a WASM plugin with sandboxing.
    #[cfg(feature = "wasm-plugins")]
    async fn load_wasm_plugin(
        &self,
        path: &Path,
        plugin_dir: Option<&Path>,
    ) -> Result<(PluginInstance, Option<PluginDeclaration>)> {
        use crate::plugins::sandbox::{Sandbox, SandboxManager};

        // Create sandbox for the plugin
        let name = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("wasm-plugin");

        let manager = SandboxManager::new();
        let sandbox = manager.create_sandbox(name, plugin_dir.map(|p| p.to_path_buf())).await;

        let wasm_plugin = crate::plugins::wasm_runtime::WasmPlugin::load_with_sandbox(path, sandbox).await
            .with_context(|| format!("Failed to load WASM plugin from {}", path.display()))?;

        let declaration = wasm_plugin.declaration.clone();

        Ok((PluginInstance::Wasm(wasm_plugin), declaration))
    }

    fn read_declaration_json(&self, entry_path: &Path) -> Result<Option<PluginDeclaration>> {
        let plugin_dir = match entry_path.parent() {
            Some(dir) => dir,
            None => return Ok(None),
        };
        let declaration_path = plugin_dir.join("plugin.declaration.json");
        if !declaration_path.exists() {
            return Ok(None);
        }
        let content = std::fs::read_to_string(&declaration_path)
            .with_context(|| format!("Failed to read {}", declaration_path.display()))?;
        let declaration: PluginDeclaration = serde_json::from_str(&content)
            .with_context(|| format!("Invalid declaration JSON in {}", declaration_path.display()))?;
        Ok(Some(declaration))
    }

    fn read_declaration_in_dir(&self, plugin_dir: Option<&Path>) -> Result<Option<PluginDeclaration>> {
        let Some(dir) = plugin_dir else {
            return Ok(None);
        };
        let declaration_path = dir.join("plugin.declaration.json");
        if !declaration_path.exists() {
            return Ok(None);
        }
        let content = std::fs::read_to_string(&declaration_path)
            .with_context(|| format!("Failed to read {}", declaration_path.display()))?;
        let declaration: PluginDeclaration = serde_json::from_str(&content)
            .with_context(|| format!("Invalid declaration JSON in {}", declaration_path.display()))?;
        Ok(Some(declaration))
    }

    fn register_declaration(&mut self, plugin_name: &str, declaration: &PluginDeclaration) {
        if !declaration.tools.is_empty() {
            self.plugin_tools
                .insert(plugin_name.to_string(), declaration.tools.clone());
        }
        for phase in &declaration.hook_phases {
            if let Some(h) = parse_hook_phase(phase) {
                self.hook_slots.register(PluginHook {
                    plugin: plugin_name.to_string(),
                    phase: h,
                    priority: 100,
                });
            } else {
                warn!("Unknown hook phase '{}' from plugin '{}'", phase, plugin_name);
            }
        }
    }

    /// Find a discovered plugin by name.
    fn find_discovered_plugin(&self, name: &str) -> Result<DiscoveredPlugin> {
        // Re-discover to find the plugin
        let discovered = self.discover()?;
        discovered
            .into_iter()
            .find(|p| p.manifest.name == name)
            .context("Plugin not found")
    }

    /// Unload a plugin.
    pub fn unload(&mut self, name: &str, registry: &mut PluginRegistry) -> Result<()> {
        // Remove instance
        if let Some(loaded) = self.instances.remove(name) {
            info!("Unloaded plugin '{}' from {}", name, loaded.path.display());
        }

        // Remove from registry
        registry.remove(name);
        self.plugin_tools.remove(name);
        self.hook_slots.clear_plugin(name);

        Ok(())
    }

    /// Get a loaded plugin instance.
    pub fn get_instance(&self, name: &str) -> Option<&LoadedPlugin> {
        self.instances.get(name)
    }

    /// List all loaded plugin instances.
    pub fn loaded_plugins(&self) -> Vec<&LoadedPlugin> {
        self.instances.values().collect()
    }

    /// List plugin-provided tool declarations.
    pub fn plugin_tools(&self) -> &HashMap<String, Vec<PluginTool>> {
        &self.plugin_tools
    }

    /// Get registered hook slots from plugin declarations.
    pub fn hook_slots(&self) -> &HookSlots {
        &self.hook_slots
    }

    /// Load all discovered plugins into the registry.
    pub fn load_all(&mut self, registry: &mut PluginRegistry) -> Result<LoadSummary> {
        let discovered = self.discover()?;
        let mut summary = LoadSummary::default();

        for plugin in discovered {
            let name = plugin.manifest.name.clone();
            match self.load(&plugin, registry) {
                Ok(()) => {
                    summary.loaded += 1;
                    debug!("Registered plugin '{}'", name);
                }
                Err(e) => {
                    summary.failed += 1;
                    summary.errors.push((name, e.to_string()));
                    error!("Failed to load plugin: {}", e);
                }
            }
        }

        Ok(summary)
    }

    /// Initialize all enabled plugins.
    pub async fn initialize_all(&mut self, registry: &mut PluginRegistry) -> Result<InitSummary> {
        let enabled: Vec<String> = registry
            .enabled()
            .iter()
            .map(|e| e.manifest.name.clone())
            .collect();

        let mut summary = InitSummary::default();

        for name in enabled {
            match self.initialize(&name, registry).await {
                Ok(()) => {
                    summary.initialized += 1;
                }
                Err(e) => {
                    summary.failed += 1;
                    summary.errors.push((name.clone(), e.to_string()));
                    error!("Failed to initialize plugin '{}': {}", name, e);

                    // Update status to error
                    if let Some(entry) = registry.get_mut(&name) {
                        entry.status = PluginStatus::Error {
                            reason: e.to_string(),
                        };
                    }
                }
            }
        }

        Ok(summary)
    }
}

impl Default for PluginLoader {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_hook_phase(phase: &str) -> Option<HookPhase> {
    match phase {
        "before-agent-start" => Some(HookPhase::BeforeAgentStart),
        "after-agent-start" => Some(HookPhase::AfterAgentStart),
        "before-tool-call" => Some(HookPhase::BeforeToolCall),
        "after-tool-call" => Some(HookPhase::AfterToolCall),
        "before-llm-request" => Some(HookPhase::BeforeLlmRequest),
        "after-llm-response" => Some(HookPhase::AfterLlmResponse),
        "before-reply" => Some(HookPhase::BeforeReply),
        "after-reply" => Some(HookPhase::AfterReply),
        "on-compaction" => Some(HookPhase::OnCompaction),
        "on-session-end" => Some(HookPhase::OnSessionEnd),
        _ => None,
    }
}

#[cfg(feature = "native-plugins")]
unsafe fn read_json_symbol<T: serde::de::DeserializeOwned>(
    lib: &libloading::Library,
    symbol_name: &'static [u8],
) -> Result<T> {
    let symbol: libloading::Symbol<unsafe extern "C" fn() -> *const c_char> =
        lib.get(symbol_name)
            .with_context(|| format!("Missing symbol {:?}", symbol_name))?;
    let ptr = symbol();
    if ptr.is_null() {
        bail!("Symbol {:?} returned null pointer", symbol_name);
    }
    let text = CStr::from_ptr(ptr)
        .to_str()
        .with_context(|| format!("Invalid UTF-8 in symbol {:?}", symbol_name))?;
    let parsed = serde_json::from_str::<T>(text)
        .with_context(|| format!("Invalid JSON from symbol {:?}", symbol_name))?;
    Ok(parsed)
}

// ─── Discovered plugin ────────────────────────────────────────────────────────

/// A discovered plugin on the filesystem.
#[derive(Debug, Clone)]
pub struct DiscoveredPlugin {
    pub manifest: PluginManifest,
    pub manifest_path: PathBuf,
    pub entry_path: Option<PathBuf>,
    pub plugin_dir: Option<PathBuf>,
}

// ─── Load summaries ───────────────────────────────────────────────────────────

/// Summary of plugin loading operation.
#[derive(Debug, Default)]
pub struct LoadSummary {
    pub loaded: usize,
    pub failed: usize,
    pub errors: Vec<(String, String)>,
}

/// Summary of plugin initialization operation.
#[derive(Debug, Default)]
pub struct InitSummary {
    pub initialized: usize,
    pub failed: usize,
    pub errors: Vec<(String, String)>,
}

// ─── Plugin manager ───────────────────────────────────────────────────────────

/// High-level plugin manager that combines registry and loader.
pub struct PluginManager {
    pub registry: PluginRegistry,
    pub loader: PluginLoader,
}

static GLOBAL_PLUGIN_MANAGER: Lazy<Mutex<Option<Arc<AsyncMutex<PluginManager>>>>> =
    Lazy::new(|| Mutex::new(None));

impl PluginManager {
    /// Create a new plugin manager.
    pub fn new() -> Self {
        Self {
            registry: PluginRegistry::new(),
            loader: PluginLoader::new(),
        }
    }

    /// Create with custom configuration.
    pub fn with_config(config: PluginLoaderConfig) -> Self {
        Self {
            registry: PluginRegistry::new(),
            loader: PluginLoader::with_config(config),
        }
    }

    /// Create loader config from app plugins config.
    pub fn config_from_plugins_config(cfg: Option<&crate::openkrab_config::PluginsConfig>) -> PluginLoaderConfig {
        let mut out = PluginLoaderConfig::default();
        if let Some(c) = cfg {
            if let Some(dirs) = &c.plugin_dirs {
                out.plugin_dirs = dirs.iter().map(PathBuf::from).collect();
            }
        }
        out
    }

    /// Discover and load all plugins.
    pub fn load_all(&mut self) -> Result<LoadSummary> {
        self.loader.load_all(&mut self.registry)
    }

    /// Initialize all enabled plugins.
    pub async fn initialize_all(&mut self) -> Result<InitSummary> {
        self.loader.initialize_all(&mut self.registry).await
    }

    /// Bootstrap plugin manager from configuration and keep it globally alive.
    pub async fn bootstrap_from_config(
        cfg: Option<&crate::openkrab_config::PluginsConfig>,
    ) -> Result<Option<PluginBootstrapSummary>> {
        let Some(plugin_cfg) = cfg else {
            return Ok(None);
        };
        if !plugin_cfg.enabled {
            return Ok(None);
        }

        let mut manager = PluginManager::with_config(Self::config_from_plugins_config(Some(plugin_cfg)));
        let load = manager.load_all()?;
        let init = manager.initialize_all().await?;
        let arc = Arc::new(AsyncMutex::new(manager));
        let mut guard = GLOBAL_PLUGIN_MANAGER.lock().expect("plugin manager mutex poisoned");
        *guard = Some(arc);
        Ok(Some(PluginBootstrapSummary { load, init }))
    }

    /// Access globally bootstrapped plugin manager.
    pub fn global() -> Option<Arc<AsyncMutex<PluginManager>>> {
        GLOBAL_PLUGIN_MANAGER
            .lock()
            .expect("plugin manager mutex poisoned")
            .as_ref()
            .cloned()
    }

    /// Get plugin registry.
    pub fn registry(&self) -> &PluginRegistry {
        &self.registry
    }

    /// Get mutable plugin registry.
    pub fn registry_mut(&mut self) -> &mut PluginRegistry {
        &mut self.registry
    }

    /// Get plugin loader.
    pub fn loader(&self) -> &PluginLoader {
        &self.loader
    }

    /// Get mutable plugin loader.
    pub fn loader_mut(&mut self) -> &mut PluginLoader {
        &mut self.loader
    }

    /// Flatten all plugin-provided tool declarations.
    pub fn all_plugin_tools(&self) -> Vec<PluginTool> {
        self.loader
            .plugin_tools()
            .values()
            .flat_map(|tools| tools.iter().cloned())
            .collect()
    }

    /// Access registered plugin hook slots.
    pub fn plugin_hook_slots(&self) -> &HookSlots {
        self.loader.hook_slots()
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct PluginBootstrapSummary {
    pub load: LoadSummary,
    pub init: InitSummary,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_discover_manifest() {
        let temp_dir = TempDir::new().unwrap();
        let plugin_dir = temp_dir.path().join("test-plugin");
        std::fs::create_dir(&plugin_dir).unwrap();

        let manifest = r#"{
            "name": "test-plugin",
            "version": "1.0.0",
            "description": "A test plugin"
        }"#;

        let mut file = std::fs::File::create(plugin_dir.join("plugin.json")).unwrap();
        file.write_all(manifest.as_bytes()).unwrap();

        let loader = PluginLoader::with_config(PluginLoaderConfig {
            plugin_dirs: vec![temp_dir.path().to_path_buf()],
            hot_reload: false,
            extensions: vec![],
        });

        let discovered = loader.discover().unwrap();
        assert_eq!(discovered.len(), 1);
        assert_eq!(discovered[0].manifest.name, "test-plugin");
    }

    #[test]
    fn test_load_plugin() {
        let temp_dir = TempDir::new().unwrap();
        let plugin_dir = temp_dir.path().join("my-plugin");
        std::fs::create_dir(&plugin_dir).unwrap();

        let manifest = r#"{
            "name": "my-plugin",
            "version": "1.0.0",
            "description": "My test plugin"
        }"#;

        let mut file = std::fs::File::create(plugin_dir.join("plugin.json")).unwrap();
        file.write_all(manifest.as_bytes()).unwrap();

        let mut loader = PluginLoader::with_config(PluginLoaderConfig {
            plugin_dirs: vec![temp_dir.path().to_path_buf()],
            hot_reload: false,
            extensions: vec![],
        });

        let mut registry = PluginRegistry::new();
        let discovered = loader.discover().unwrap();

        assert_eq!(discovered.len(), 1);
        loader.load(&discovered[0], &mut registry).unwrap();

        assert_eq!(registry.len(), 1);
        assert!(registry.get("my-plugin").is_some());
    }

    #[test]
    fn test_plugin_manager() {
        let temp_dir = TempDir::new().unwrap();
        let plugin_dir = temp_dir.path().join("managed-plugin");
        std::fs::create_dir(&plugin_dir).unwrap();

        let manifest = r#"{
            "name": "managed-plugin",
            "version": "2.0.0",
            "description": "Managed plugin test"
        }"#;

        let mut file = std::fs::File::create(plugin_dir.join("plugin.json")).unwrap();
        file.write_all(manifest.as_bytes()).unwrap();

        let mut manager = PluginManager::with_config(PluginLoaderConfig {
            plugin_dirs: vec![temp_dir.path().to_path_buf()],
            hot_reload: false,
            extensions: vec![],
        });

        let summary = manager.load_all().unwrap();
        assert_eq!(summary.loaded, 1);
        assert_eq!(manager.registry().len(), 1);
    }
}
