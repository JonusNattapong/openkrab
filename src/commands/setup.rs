//! setup — Initial setup command.
//! Ported from `openkrab/src/commands/setup.ts` (Phase 6).

use std::path::PathBuf;

/// Setup options.
#[derive(Debug, Clone, Default)]
pub struct SetupOptions {
    pub skip_config: bool,
    pub skip_deps: bool,
    pub force: bool,
}

/// Run setup command.
pub fn setup_command(opts: SetupOptions) -> String {
    let mut lines = vec!["🦀 openkrab Setup".to_string(), String::new()];

    // Check directories
    lines.push("Checking directories...".to_string());
    let config_dir = dirs::config_dir()
        .map(|d| d.join("openkrab"))
        .unwrap_or_else(|| PathBuf::from(".openkrab"));

    if !opts.skip_config {
        match std::fs::create_dir_all(&config_dir) {
            Ok(_) => lines.push(format!("  ✓ Config directory: {}", config_dir.display())),
            Err(e) => lines.push(format!("  ✗ Config directory: {}", e)),
        }
    }

    // Check data directory
    let data_dir = dirs::data_dir()
        .map(|d| d.join("openkrab"))
        .unwrap_or_else(|| PathBuf::from(".openkrab-data"));

    match std::fs::create_dir_all(&data_dir) {
        Ok(_) => lines.push(format!("  ✓ Data directory: {}", data_dir.display())),
        Err(e) => lines.push(format!("  ✗ Data directory: {}", e)),
    }

    // Check dependencies
    if !opts.skip_deps {
        lines.push(String::new());
        lines.push("Checking dependencies...".to_string());

        // Check Docker
        let docker_available = crate::commands::doctor_sandbox::is_docker_available();
        lines.push(format!(
            "  {} Docker",
            if docker_available { "✓" } else { "✗" }
        ));

        // Check Signal CLI
        let signal_available = crate::connectors::signal::is_signal_cli_available();
        lines.push(format!(
            "  {} Signal CLI",
            if signal_available { "✓" } else { "○" }
        ));
    }

    lines.push(String::new());
    lines.push("Setup complete!".to_string());
    lines.push("Run 'openkrab onboard' to configure your assistant.".to_string());

    lines.join("\n")
}

