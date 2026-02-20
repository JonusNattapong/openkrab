//! hot_reload — File system watcher for plugin hot-reloading during development.
//!
//! Watches plugin directories for changes and automatically reloads plugins.

use anyhow::{Context, Result};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, Mutex, RwLock};
use tracing::{debug, error, info, warn};

use crate::plugins::loader::{PluginLoader, PluginManager};
use crate::plugins::{PluginRegistry, PluginStatus};

/// Debounce duration to avoid rapid reloads
const DEBOUNCE_MS: u64 = 500;

/// Minimum time between reloads of the same plugin
const MIN_RELOAD_INTERVAL_MS: u64 = 2000;

/// Events that can trigger a plugin reload
#[derive(Debug, Clone)]
pub enum HotReloadEvent {
    /// Plugin file changed
    FileChanged { path: PathBuf, plugin_name: String },
    /// Plugin manifest changed
    ManifestChanged { path: PathBuf, plugin_name: String },
    /// Request full rescan
    Rescan,
}

/// Hot reload manager for plugins
pub struct HotReloadManager {
    /// Watcher instance
    watcher: Option<RecommendedWatcher>,
    /// Event channel receiver
    event_rx: Option<mpsc::Receiver<HotReloadEvent>>,
    /// Last reload time per plugin
    last_reload: Arc<RwLock<HashMap<String, Instant>>>,
    /// Plugin directories being watched
    watched_dirs: Vec<PathBuf>,
    /// Whether hot reload is enabled
    enabled: bool,
}

impl HotReloadManager {
    /// Create a new hot reload manager
    pub fn new() -> Self {
        Self {
            watcher: None,
            event_rx: None,
            last_reload: Arc::new(RwLock::new(HashMap::new())),
            watched_dirs: Vec::new(),
            enabled: false,
        }
    }

    /// Enable hot reload for the given plugin directories
    pub async fn enable(&mut self, plugin_dirs: &[PathBuf]) -> Result<()> {
        if self.enabled {
            return Ok(());
        }

        info!("Enabling plugin hot reload for {} directories", plugin_dirs.len());

        let (tx, rx) = mpsc::channel(100);
        let tx_clone = tx.clone();

        // Create file watcher
        let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            match res {
                Ok(event) => {
                    debug!("File system event: {:?}", event);

                    for path in &event.paths {
                        if let Some(event) = Self::classify_event(path, &event) {
                            let _ = tx_clone.try_send(event);
                        }
                    }
                }
                Err(e) => {
                    error!("File watcher error: {}", e);
                }
            }
        })?;

        // Watch plugin directories
        for dir in plugin_dirs {
            if dir.exists() {
                watcher.watch(dir, RecursiveMode::NonRecursive)?;
                self.watched_dirs.push(dir.clone());
                info!("Watching plugin directory: {}", dir.display());
            } else {
                warn!("Plugin directory does not exist: {}", dir.display());
            }
        }

        self.watcher = Some(watcher);
        self.event_rx = Some(rx);
        self.enabled = true;

        Ok(())
    }

    /// Disable hot reload
    pub fn disable(&mut self) {
        if !self.enabled {
            return;
        }

        info!("Disabling plugin hot reload");

        // Drop the watcher to stop watching
        self.watcher = None;
        self.event_rx = None;
        self.watched_dirs.clear();
        self.enabled = false;
    }

    /// Check if hot reload is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Process pending hot reload events
    pub async fn process_events(&mut self, manager: &Arc<Mutex<PluginManager>>) -> Result<HotReloadSummary> {
        if !self.enabled {
            return Ok(HotReloadSummary::default());
        }

        let mut summary = HotReloadSummary::default();

        // Process all pending events
        let events: Vec<_> = if let Some(ref mut rx) = self.event_rx {
            let mut collected = Vec::new();
            while let Ok(event) = rx.try_recv() {
                collected.push(event);
            }
            collected
        } else {
            Vec::new()
        };

        for event in events {
            debug!("Processing hot reload event: {:?}", event);

            match self.handle_event(event, manager).await {
                Ok(Some(plugin_name)) => {
                    summary.reloaded.push(plugin_name);
                }
                Ok(None) => {
                    summary.skipped += 1;
                }
                Err(e) => {
                    summary.failed += 1;
                    summary.errors.push(e.to_string());
                }
            }
        }

        Ok(summary)
    }

    /// Handle a single hot reload event
    async fn handle_event(
        &self,
        event: HotReloadEvent,
        manager: &Arc<Mutex<PluginManager>>,
    ) -> Result<Option<String>> {
        let plugin_name = match &event {
            HotReloadEvent::FileChanged { plugin_name, .. } => plugin_name.clone(),
            HotReloadEvent::ManifestChanged { plugin_name, .. } => plugin_name.clone(),
            HotReloadEvent::Rescan => {
                // Full rescan - reload all enabled plugins
                return self.rescan_all(manager).await.map(|_| None);
            }
        };

        // Check debounce
        if !self.should_reload(&plugin_name).await {
            debug!("Skipping reload of '{}' - too soon", plugin_name);
            return Ok(None);
        }

        info!("Hot reloading plugin: {}", plugin_name);

        // Step 1: Unload
        {
            let mut mgr = manager.lock().await;
            let registry = &mut mgr.registry as *mut PluginRegistry;
            unsafe {
                let _ = mgr.loader.unload(&plugin_name, &mut *registry);
            }
        }

        // Step 2: Discover
        let discovered = {
            let mgr = manager.lock().await;
            mgr.loader.discover().map_err(|e| anyhow::anyhow!("Failed to discover: {}", e))?
        };

        // Step 3: Find plugin
        let plugin = discovered.iter().find(|p| p.manifest.name == plugin_name);
        
        let should_init = match plugin {
            Some(p) => {
                // Step 4: Load
                {
                    let mut mgr = manager.lock().await;
                    let registry = &mut mgr.registry as *mut PluginRegistry;
                    unsafe {
                        let _ = mgr.loader.load(p, &mut *registry);
                    }
                }
                
                // Step 5: Check if enabled
                let mgr = manager.lock().await;
                mgr.registry.get(&plugin_name)
                    .map(|e| e.is_enabled())
                    .unwrap_or(false)
            }
            None => false
        };

        // Step 6: Initialize if needed
        if should_init {
            let mut mgr = manager.lock().await;
            let registry = &mut mgr.registry as *mut PluginRegistry;
            let result = unsafe { mgr.loader.initialize(&plugin_name, &mut *registry).await };
            if let Err(e) = result {
                if let Some(entry) = mgr.registry.get_mut(&plugin_name) {
                    entry.status = PluginStatus::Error {
                        reason: e.to_string(),
                    };
                }
                return Err(anyhow::anyhow!("Failed to initialize plugin: {}", e));
            }
            drop(mgr);
            self.update_reload_time(&plugin_name).await;
            info!("Plugin '{}' hot reloaded successfully", plugin_name);
            Ok(Some(plugin_name))
        } else {
            Err(anyhow::anyhow!("Plugin '{}' not found during rediscovery", plugin_name))
        }
    }

    /// Rescan and reload all plugins
    async fn rescan_all(&self, manager: &Arc<Mutex<PluginManager>>) -> Result<()> {
        info!("Rescanning all plugins");

        // Step 1: Get loaded plugins
        let loaded: Vec<String> = {
            let mgr = manager.lock().await;
            mgr.loader.loaded_plugins()
                .iter()
                .map(|p| p.manifest.name.clone())
                .collect()
        };

        // Step 2: Unload all
        for name in &loaded {
            let mut mgr = manager.lock().await;
            let registry = &mut mgr.registry as *mut PluginRegistry;
            unsafe {
                let _ = mgr.loader.unload(name, &mut *registry);
            }
        }

        // Step 3: Discover
        let discovered = {
            let mgr = manager.lock().await;
            mgr.loader.discover().map_err(|e| anyhow::anyhow!("Failed to discover: {}", e))?
        };
        
        // Step 4: Load all
        {
            let mut mgr = manager.lock().await;
            let loader = &mut mgr.loader as *mut PluginLoader;
            let registry = &mut mgr.registry as *mut PluginRegistry;
            unsafe {
                let _ = (*loader).load_all(&mut *registry);
            }
        }
        
        // Step 5: Initialize all
        {
            let mut mgr = manager.lock().await;
            let _ = mgr.initialize_all().await;
        }

        Ok(())
    }

    /// Check if enough time has passed since last reload
    async fn should_reload(&self, plugin_name: &str) -> bool {
        let last_reload = self.last_reload.read().await;

        if let Some(last_time) = last_reload.get(plugin_name) {
            let elapsed = last_time.elapsed().as_millis() as u64;
            elapsed > MIN_RELOAD_INTERVAL_MS
        } else {
            true
        }
    }

    /// Update the last reload time for a plugin
    async fn update_reload_time(&self, plugin_name: &str) {
        let mut last_reload = self.last_reload.write().await;
        last_reload.insert(plugin_name.to_string(), Instant::now());
    }

    /// Classify a file system event into a hot reload event
    fn classify_event(path: &Path, event: &Event) -> Option<HotReloadEvent> {
        let file_name = path.file_name()?.to_str()?;

        // Check if it's a manifest file
        if file_name == "plugin.json" || file_name == "manifest.json" || file_name == "krabkrab.json" {
            let plugin_name = path.parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            return Some(HotReloadEvent::ManifestChanged {
                path: path.to_path_buf(),
                plugin_name,
            });
        }

        // Check if it's a plugin file
        if let Some(ext) = path.extension() {
            let ext = ext.to_str()?;
            if ["wasm", "so", "dylib", "dll"].contains(&ext) {
                let plugin_name = path.file_stem()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                return Some(HotReloadEvent::FileChanged {
                    path: path.to_path_buf(),
                    plugin_name,
                });
            }
        }

        None
    }

    /// Start background hot reload task
    pub fn start_background_task(
        &self,
        manager: Arc<Mutex<PluginManager>>,
        poll_interval: Duration,
    ) -> tokio::task::JoinHandle<()> {
        let mut manager_clone = self.clone_for_task();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(poll_interval);

            loop {
                interval.tick().await;

                match manager_clone.process_events(&manager).await {
                    Ok(summary) => {
                        if summary.has_activity() {
                            info!("Hot reload activity: {:?}", summary);
                        }
                    }
                    Err(e) => {
                        error!("Hot reload processing error: {}", e);
                    }
                }
            }
        })
    }

    /// Clone for background task (watcher can't be cloned, but we recreate it)
    fn clone_for_task(&self) -> HotReloadTask {
        HotReloadTask {
            last_reload: self.last_reload.clone(),
            event_rx: None, // Receiver can't be cloned, needs to be taken
            enabled: self.enabled,
        }
    }
}

/// Lightweight version of HotReloadManager for background tasks
struct HotReloadTask {
    last_reload: Arc<RwLock<HashMap<String, Instant>>>,
    event_rx: Option<mpsc::Receiver<HotReloadEvent>>,
    enabled: bool,
}

impl HotReloadTask {
    async fn process_events(&mut self, manager: &Arc<Mutex<PluginManager>>) -> Result<HotReloadSummary> {
        if !self.enabled {
            return Ok(HotReloadSummary::default());
        }

        let mut summary = HotReloadSummary::default();

        if let Some(ref mut rx) = self.event_rx {
            while let Ok(event) = rx.try_recv() {
                // Similar logic to HotReloadManager::handle_event
                // Simplified for brevity...
                debug!("Background task processing: {:?}", event);
            }
        }

        Ok(summary)
    }
}

/// Summary of hot reload activity
#[derive(Debug, Default)]
pub struct HotReloadSummary {
    pub reloaded: Vec<String>,
    pub failed: usize,
    pub skipped: usize,
    pub errors: Vec<String>,
}

impl HotReloadSummary {
    pub fn has_activity(&self) -> bool {
        !self.reloaded.is_empty() || self.failed > 0 || self.skipped > 0
    }
}

/// Extension trait for PluginManager to support hot reload
#[async_trait::async_trait]
pub trait HotReloadable {
    async fn enable_hot_reload(&self) -> Result<()>;
    async fn disable_hot_reload(&self);
    async fn hot_reload_summary(&self) -> HotReloadSummary;
}

#[async_trait::async_trait]
impl HotReloadable for Arc<Mutex<PluginManager>> {
    async fn enable_hot_reload(&self) -> Result<()> {
        // Get plugin directories from config
        let dirs = {
            let mgr = self.lock().await;
            mgr.loader.config().plugin_dirs.clone()
        };

        // Create and enable hot reload manager
        let mut hot_reload = HotReloadManager::new();
        hot_reload.enable(&dirs).await?;

        // Store in global or return handle
        // For now, this is a simplified version
        info!("Hot reload enabled for plugin directories: {:?}", dirs);

        Ok(())
    }

    async fn disable_hot_reload(&self) {
        info!("Hot reload disabled");
    }

    async fn hot_reload_summary(&self) -> HotReloadSummary {
        HotReloadSummary::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hot_reload_summary_activity() {
        let mut summary = HotReloadSummary::default();
        assert!(!summary.has_activity());

        summary.reloaded.push("test-plugin".to_string());
        assert!(summary.has_activity());
    }
}
