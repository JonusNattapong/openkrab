//! signal_install — Signal CLI installation helper.
//! Ported from `openkrab/src/commands/signal-install.ts` (Phase 6).

use std::process::Command;

/// Signal CLI installation result.
#[derive(Debug, Clone)]
pub struct SignalInstallResult {
    pub success: bool,
    pub version: Option<String>,
    pub message: String,
}

/// Check if Signal CLI is installed.
pub fn is_signal_cli_installed() -> bool {
    Command::new("signal-cli")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Get Signal CLI version if installed.
pub fn get_signal_cli_version() -> Option<String> {
    Command::new("signal-cli")
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
}

/// Install Signal CLI (platform-specific).
pub async fn install_signal_cli() -> SignalInstallResult {
    #[cfg(target_os = "macos")]
    return install_signal_cli_macos().await;

    #[cfg(target_os = "linux")]
    return install_signal_cli_linux().await;

    #[cfg(target_os = "windows")]
    return install_signal_cli_windows().await;

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    SignalInstallResult {
        success: false,
        version: None,
        message: "Unsupported platform for automatic Signal CLI installation".to_string(),
    }
}

#[cfg(target_os = "macos")]
async fn install_signal_cli_macos() -> SignalInstallResult {
    // Try Homebrew first
    match Command::new("brew")
        .args(["install", "signal-cli"])
        .status()
    {
        Ok(status) if status.success() => SignalInstallResult {
            success: true,
            version: get_signal_cli_version(),
            message: "Signal CLI installed via Homebrew".to_string(),
        },
        _ => SignalInstallResult {
            success: false,
            version: None,
            message: "Failed to install Signal CLI via Homebrew. Try: brew install signal-cli"
                .to_string(),
        },
    }
}

#[cfg(target_os = "linux")]
async fn install_signal_cli_linux() -> SignalInstallResult {
    // Try apt (Debian/Ubuntu)
    match Command::new("apt-get")
        .args(["install", "-y", "signal-cli"])
        .status()
    {
        Ok(status) if status.success() => SignalInstallResult {
            success: true,
            version: get_signal_cli_version(),
            message: "Signal CLI installed via apt".to_string(),
        },
        _ => SignalInstallResult {
            success: false,
            version: None,
            message:
                "Failed to install Signal CLI via apt. See: https://github.com/AsamK/signal-cli"
                    .to_string(),
        },
    }
}

#[cfg(target_os = "windows")]
async fn install_signal_cli_windows() -> SignalInstallResult {
    SignalInstallResult {
        success: false,
        version: None,
        message: "Automatic installation not supported on Windows. Download from: https://github.com/AsamK/signal-cli/releases".to_string(),
    }
}

/// Signal CLI installation command.
pub async fn signal_install_command() -> String {
    if is_signal_cli_installed() {
        let version = get_signal_cli_version().unwrap_or_else(|| "unknown".to_string());
        return format!("Signal CLI is already installed: {}", version);
    }

    println!("Installing Signal CLI...");
    let result = install_signal_cli().await;

    if result.success {
        format!("✅ {}", result.message)
    } else {
        format!("❌ {}\n\nManual installation:\n  https://github.com/AsamK/signal-cli/blob/master/README.md", result.message)
    }
}

/// Check if archive file looks like Signal CLI distribution.
pub fn looks_like_signal_archive(name: &str) -> bool {
    name.contains("signal-cli") && (name.ends_with(".tar.gz") || name.ends_with(".zip"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_looks_like_signal_archive() {
        assert!(looks_like_signal_archive("signal-cli-0.12.0.tar.gz"));
        assert!(looks_like_signal_archive("signal-cli-0.12.0.zip"));
        assert!(!looks_like_signal_archive("other-file.tar.gz"));
        assert!(!looks_like_signal_archive("signal-cli.txt"));
    }
}
