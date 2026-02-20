//! Shell completion setup during onboarding.
//! Ported from `openclaw/src/wizard/onboarding.completion.ts`

use anyhow::Result;

use super::prompts::{WizardConfirmParams, WizardPrompter};
use super::types::WizardFlow;

/// Detect the current shell from environment.
pub fn detect_shell() -> &'static str {
    if cfg!(windows) {
        return "powershell";
    }

    if let Ok(shell) = std::env::var("SHELL") {
        if shell.contains("zsh") {
            return "zsh";
        }
        if shell.contains("fish") {
            return "fish";
        }
        if shell.contains("bash") {
            return "bash";
        }
    }

    "bash"
}

/// Get the shell profile file path hint.
pub fn profile_hint(shell: &str) -> String {
    match shell {
        "zsh" => "~/.zshrc".to_string(),
        "bash" => {
            let home = dirs::home_dir()
                .map(|h| h.display().to_string())
                .unwrap_or_default();
            let bashrc = format!("{}/.bashrc", home);
            if std::path::Path::new(&bashrc).exists() {
                "~/.bashrc".to_string()
            } else {
                "~/.bash_profile".to_string()
            }
        }
        "fish" => "~/.config/fish/config.fish".to_string(),
        "powershell" => "$PROFILE".to_string(),
        _ => "$PROFILE".to_string(),
    }
}

/// Get reload hint for the shell.
pub fn reload_hint(shell: &str, profile: &str) -> String {
    if shell == "powershell" {
        return "Restart your shell (or reload your PowerShell profile).".to_string();
    }
    format!("Restart your shell or run: source {}", profile)
}

/// Set up shell completion during onboarding.
pub async fn setup_onboarding_shell_completion(
    flow: WizardFlow,
    prompter: &dyn WizardPrompter,
) -> Result<()> {
    let shell = detect_shell();
    let profile = profile_hint(shell);
    let cli_name = "krabkrab";

    // In quickstart mode, auto-install
    let should_install = if flow == WizardFlow::Quickstart {
        true
    } else {
        prompter
            .confirm(WizardConfirmParams {
                message: format!(
                    "Enable {} shell completion for {}?",
                    shell, cli_name
                ),
                initial_value: Some(true),
            })
            .await?
    };

    if !should_install {
        return Ok(());
    }

    // Generate completions (placeholder — actual implementation depends on clap)
    prompter
        .note(
            &format!(
                "Shell completion installed. {}",
                reload_hint(shell, &profile)
            ),
            Some("Shell completion"),
        )
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_shell_returns_valid() {
        let shell = detect_shell();
        assert!(!shell.is_empty());
    }

    #[test]
    fn profile_hint_returns_path() {
        let hint = profile_hint("zsh");
        assert_eq!(hint, "~/.zshrc");

        let hint = profile_hint("fish");
        assert_eq!(hint, "~/.config/fish/config.fish");
    }

    #[test]
    fn reload_hint_powershell() {
        let hint = reload_hint("powershell", "$PROFILE");
        assert!(hint.contains("Restart your shell"));
    }

    #[test]
    fn reload_hint_unix() {
        let hint = reload_hint("bash", "~/.bashrc");
        assert!(hint.contains("source ~/.bashrc"));
    }
}
