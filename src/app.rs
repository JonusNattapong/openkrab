//! OpenClaw Application
//!
//! Main application orchestrator that manages storage, gateway, and channels.

use openclaw_core::{ChannelRegistry, Config, OpenClawError, Result};
use openclaw_gateway::GatewayServer;
use openclaw_sessions::Storage;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::signal;
use tracing::{error, info, warn};

/// Configuration paths
pub const CONFIG_DIR_NAME: &str = "openclaw";
pub const CONFIG_FILE_NAME: &str = "openclaw.toml";
pub const DEFAULT_DB_NAME: &str = "openclaw.db";

/// Main application struct that orchestrates all components
pub struct Application {
    config: Config,
    storage: Option<Storage>,
    gateway: Option<GatewayServer>,
    channel_registry: Arc<ChannelRegistry>,
    shutdown_tx: Option<tokio::sync::broadcast::Sender<()>>,
}

impl Application {
    /// Create a new Application instance
    pub fn new(config: Config) -> Self {
        let channel_registry = Arc::new(ChannelRegistry::new());
        
        Self {
            config,
            storage: None,
            gateway: None,
            channel_registry,
            shutdown_tx: None,
        }
    }

    /// Initialize the application
    pub async fn init(&mut self) -> Result<()> {
        info!("Initializing OpenClaw application...");

        self.init_storage().await?;
        self.init_gateway().await?;
        self.init_channels().await?;

        let (shutdown_tx, _) = tokio::sync::broadcast::channel(1);
        self.shutdown_tx = Some(shutdown_tx);

        info!("Application initialized successfully");
        Ok(())
    }

    /// Initialize SQLite storage
    async fn init_storage(&mut self) -> Result<()> {
        let db_path = &self.config.database.path;
        
        info!("Initializing storage at: {}", db_path);
        
        let storage = Storage::new(db_path)
            .await
            .map_err(|e| OpenClawError::Internal {
                message: format!("Failed to initialize storage: {}", e),
            })?;
        
        self.storage = Some(storage);
        info!("Storage initialized");
        Ok(())
    }

    /// Initialize gateway server
    async fn init_gateway(&mut self) -> Result<()> {
        info!(
            "Initializing gateway on {}:{}",
            self.config.server.host, self.config.server.port
        );

        let gateway = GatewayServer::new(self.config.clone());
        self.gateway = Some(gateway);

        info!("Gateway initialized");
        Ok(())
    }

    /// Initialize channel registry
    async fn init_channels(&mut self) -> Result<()> {
        info!("Initializing channel registry");

        for channel_config in &self.config.channels {
            if channel_config.enabled {
                info!(
                    "Registering channel: {} ({})",
                    channel_config.name, channel_config.channel_type
                );
                // Channel implementations will be registered here
                // based on channel_config.channel_type
            }
        }

        info!("Channel registry initialized");
        Ok(())
    }

    /// Start the application
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting OpenClaw application...");

        let gateway = self
            .gateway
            .take()
            .ok_or_else(|| OpenClawError::Internal {
                message: "Gateway not initialized".to_string(),
            })?;

        let shutdown_tx = self
            .shutdown_tx
            .clone()
            .ok_or_else(|| OpenClawError::Internal {
                message: "Shutdown channel not initialized".to_string(),
            })?;

        let gateway_handle = tokio::spawn(async move {
            if let Err(e) = gateway.start().await {
                error!("Gateway error: {}", e);
            }
        });

        info!(
            "OpenClaw is running on {}:{}",
            self.config.server.host, self.config.server.port
        );
        info!("WebSocket endpoint: ws://{}:{}/ws", self.config.server.host, self.config.server.port);
        info!("Health check: http://{}:{}/health", self.config.server.host, self.config.server.port);
        info!("Press Ctrl+C to shutdown");

        let mut shutdown_rx = shutdown_tx.subscribe();

        tokio::select! {
            _ = self.handle_signals() => {
                info!("Shutdown signal received");
            }
            _ = shutdown_rx.recv() => {
                info!("Shutdown broadcast received");
            }
            _ = gateway_handle => {
                warn!("Gateway stopped unexpectedly");
            }
        }

        self.shutdown().await
    }

    /// Handle OS signals for graceful shutdown
    async fn handle_signals(&self) -> Result<()> {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("Failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("Failed to install SIGTERM handler")
                .recv()
                .await;
        };

        #[cfg(unix)]
        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }

        #[cfg(not(unix))]
        ctrl_c.await;

        Ok(())
    }

    /// Graceful shutdown
    async fn shutdown(&self) -> Result<()> {
        info!("Shutting down OpenClaw...");

        if let Some(ref shutdown_tx) = self.shutdown_tx {
            let _ = shutdown_tx.send(());
        }

        info!("Waiting for components to shutdown...");
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        info!("OpenClaw shutdown complete");
        Ok(())
    }

    /// Get the storage instance
    pub fn storage(&self) -> Option<&Storage> {
        self.storage.as_ref()
    }

    /// Get the channel registry
    pub fn channel_registry(&self) -> &Arc<ChannelRegistry> {
        &self.channel_registry
    }

    /// Get the config
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Trigger shutdown from external source
    pub fn request_shutdown(&self) {
        if let Some(ref shutdown_tx) = self.shutdown_tx {
            let _ = shutdown_tx.send(());
        }
    }
}

/// Get the default configuration directory
pub fn get_config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join(CONFIG_DIR_NAME))
}

/// Get the default configuration file path
pub fn get_config_path() -> Option<PathBuf> {
    get_config_dir().map(|dir| dir.join(CONFIG_FILE_NAME))
}

/// Get the default database path
pub fn get_default_db_path() -> Option<PathBuf> {
    get_config_dir().map(|dir| dir.join(DEFAULT_DB_NAME))
}

/// Create default configuration if it doesn't exist
pub async fn ensure_config() -> anyhow::Result<PathBuf> {
    let config_dir = get_config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
    
    let config_path = config_dir.join(CONFIG_FILE_NAME);
    
    if !config_dir.exists() {
        tokio::fs::create_dir_all(&config_dir).await?;
        info!("Created config directory: {}", config_dir.display());
    }
    
    if !config_path.exists() {
        let default_config = Config::default();
        let config_toml = toml::to_string_pretty(&default_config)?;
        tokio::fs::write(&config_path, config_toml).await?;
        info!("Created default config at: {}", config_path.display());
    }
    
    Ok(config_path)
}
