//! uninstall — Uninstall krabkrab command.
//! Ported from `openclaw/src/commands/uninstall.ts` (Phase 6).

use std::path::PathBuf;

/// Uninstall options.
#[derive(Debug, Clone, Default)]
pub struct UninstallOptions {
    pub purge: bool,
    pub force: bool,
}

/// Run uninstall command.
pub fn uninstall_command(opts: UninstallOptions) -> String {
    let mut lines = vec!["🦀 krabkrab Uninstall".to_string(), String::new()];

    if !opts.force {
        lines.push("⚠️  This will remove krabkrab from your system.".to_string());
        lines.push("Use --force to confirm uninstallation.".to_string());
        lines.push(String::new());
        lines.push("The following will be removed:".to_string());
    }

    let mut removed = Vec::new();

    // Binary
    let binary_paths = vec![
        "/usr/local/bin/krabkrab",
        "/usr/bin/krabkrab",
        "C:\\Program Files\\krabkrab\\krabkrab.exe",
    ];

    for path in &binary_paths {
        let path = PathBuf::from(path);
        if path.exists() {
            if opts.force {
                match std::fs::remove_file(&path) {
                    Ok(_) => removed.push(format!("Binary: {}", path.display())),
                    Err(_) => {}
                }
            } else {
                lines.push(format!("  - Binary: {}", path.display()));
            }
        }
    }

    // Config
    let config_dir = dirs::config_dir().map(|d| d.join("krabkrab"));

    if let Some(ref dir) = config_dir {
        if opts.purge && opts.force {
            match std::fs::remove_dir_all(dir) {
                Ok(_) => removed.push(format!("Config: {}", dir.display())),
                Err(_) => {}
            }
        } else if opts.purge {
            lines.push(format!("  - Config: {}", dir.display()));
        }
    }

    // Data
    let data_dir = dirs::data_dir().map(|d| d.join("krabkrab"));

    if let Some(ref dir) = data_dir {
        if opts.purge && opts.force {
            match std::fs::remove_dir_all(dir) {
                Ok(_) => removed.push(format!("Data: {}", dir.display())),
                Err(_) => {}
            }
        } else if opts.purge {
            lines.push(format!("  - Data: {}", dir.display()));
        }
    }

    if opts.force {
        lines.push(String::new());
        if removed.is_empty() {
            lines.push("No krabkrab installation found to remove.".to_string());
        } else {
            lines.push("Removed:".to_string());
            for item in removed {
                lines.push(format!("  ✓ {}", item));
            }
            lines.push(String::new());
            lines.push("krabkrab has been uninstalled.".to_string());
            lines.push("Sorry to see you go! 🦀".to_string());
        }
    }

    lines.join("\n")
}
