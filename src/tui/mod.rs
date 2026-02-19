//! TUI - Terminal User Interface for krabkrab
//! Provides interactive terminal experience with chat, sessions, and commands

pub mod app;

pub use app::TuiApp;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiConfig {
    pub gateway_url: String,
    pub session: String,
    pub theme: String,
}

impl Default for TuiConfig {
    fn default() -> Self {
        Self {
            gateway_url: "http://localhost:18789".to_string(),
            session: "main".to_string(),
            theme: "dark".to_string(),
        }
    }
}

pub fn run_tui(config: TuiConfig) -> anyhow::Result<()> {
    let mut app = TuiApp::new(config)?;
    app.run()?;
    Ok(())
}
