//! reset — Reset/cleanup command.
//! Ported from `openclaw/src/commands/reset.ts` (Phase 6).

use std::path::PathBuf;

/// Reset options.
#[derive(Debug, Clone, Default)]
pub struct ResetOptions {
    pub config: bool,
    pub data: bool,
    pub cache: bool,
    pub all: bool,
    pub force: bool,
}

/// Run reset command.
pub fn reset_command(opts: ResetOptions) -> String {
    let mut lines = vec!["🦀 krabkrab Reset".to_string(), String::new()];

    if !opts.force {
        lines.push("⚠️  This will delete data. Use --force to confirm.".to_string());
        lines.push(String::new());
        lines.push("What would be deleted:".to_string());
    }

    let mut deleted = Vec::new();

    // Reset config
    if opts.config || opts.all {
        let config_dir = dirs::config_dir()
            .map(|d| d.join("krabkrab"))
            .unwrap_or_else(|| PathBuf::from(".krabkrab"));

        if opts.force {
            match std::fs::remove_dir_all(&config_dir) {
                Ok(_) => deleted.push(format!("Config: {}", config_dir.display())),
                Err(e) => lines.push(format!("  ✗ Config: {}", e)),
            }
        } else {
            lines.push(format!("  - Config: {}", config_dir.display()));
        }
    }

    // Reset data
    if opts.data || opts.all {
        let data_dir = dirs::data_dir()
            .map(|d| d.join("krabkrab"))
            .unwrap_or_else(|| PathBuf::from(".krabkrab-data"));

        if opts.force {
            match std::fs::remove_dir_all(&data_dir) {
                Ok(_) => deleted.push(format!("Data: {}", data_dir.display())),
                Err(e) => lines.push(format!("  ✗ Data: {}", e)),
            }
        } else {
            lines.push(format!("  - Data: {}", data_dir.display()));
        }
    }

    // Reset cache
    if opts.cache || opts.all {
        let cache_dir = dirs::cache_dir()
            .map(|d| d.join("krabkrab"))
            .unwrap_or_else(|| PathBuf::from(".krabkrab-cache"));

        if opts.force {
            match std::fs::remove_dir_all(&cache_dir) {
                Ok(_) => deleted.push(format!("Cache: {}", cache_dir.display())),
                Err(e) => lines.push(format!("  ✗ Cache: {}", e)),
            }
        } else {
            lines.push(format!("  - Cache: {}", cache_dir.display()));
        }
    }

    if opts.force && !deleted.is_empty() {
        lines.push(String::new());
        lines.push("Deleted:".to_string());
        for item in deleted {
            lines.push(format!("  ✓ {}", item));
        }
    }

    lines.join("\n")
}
