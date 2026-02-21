//! Shell completion setup during onboarding.
//! Ported from `openkrab/src/wizard/onboarding.completion.ts`

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

    install_shell_completion(shell, cli_name)?;

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

fn install_shell_completion(shell: &str, cli_name: &str) -> Result<()> {
    match shell {
        "bash" => {
            let path = expand_home("~/.bashrc");
            append_managed_block(&path, &bash_completion_script(cli_name))?;
        }
        "zsh" => {
            let path = expand_home("~/.zshrc");
            append_managed_block(&path, &zsh_completion_script(cli_name))?;
        }
        "fish" => {
            let path = expand_home(&format!("~/.config/fish/completions/{}.fish", cli_name));
            write_file(&path, &fish_completion_script(cli_name))?;
        }
        "powershell" => {
            let path = powershell_profile_path();
            append_managed_block(&path, &powershell_completion_script(cli_name))?;
        }
        _ => {}
    }
    Ok(())
}

fn command_words() -> &'static [&'static str] {
    &[
        "hello", "status", "doctor", "onboard", "shell", "telegram", "slack", "discord",
        "whatsapp", "configure", "ask", "memory", "gateway", "models", "models-auth",
        "login", "bridge", "channels", "cron", "admin", "pairing", "sandbox", "sessions",
        "logs", "message", "health", "config",
    ]
}

fn bash_completion_script(cli_name: &str) -> String {
    let words = command_words().join(" ");
    format!(
        "_{}_completions() {{\n  local cur=\"${{COMP_WORDS[COMP_CWORD]}}\"\n  COMPREPLY=( $(compgen -W \"{}\" -- \"$cur\") )\n}}\ncomplete -F _{}_completions {}\n",
        cli_name, words, cli_name, cli_name
    )
}

fn zsh_completion_script(cli_name: &str) -> String {
    let words = command_words().join(" ");
    format!(
        "_{}_completions() {{\n  local -a commands\n  commands=({})\n  _describe 'command' commands\n}}\ncompdef _{}_completions {}\n",
        cli_name, words, cli_name, cli_name
    )
}

fn fish_completion_script(cli_name: &str) -> String {
    command_words()
        .iter()
        .map(|c| format!("complete -c {} -f -a '{}'", cli_name, c))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

fn powershell_completion_script(cli_name: &str) -> String {
    let words = command_words()
        .iter()
        .map(|w| format!("'{}'", w))
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "Register-ArgumentCompleter -Native -CommandName {0} -ScriptBlock {{\n  param($wordToComplete, $commandAst, $cursorPosition)\n  {1} | Where-Object {{ $_ -like \"$wordToComplete*\" }} | ForEach-Object {{\n    [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)\n  }}\n}}\n",
        cli_name, words
    )
}

fn append_managed_block(path: &std::path::Path, content: &str) -> Result<()> {
    const START: &str = "# >>> krabkrab completion >>>";
    const END: &str = "# <<< krabkrab completion <<<";

    let mut existing = if path.exists() {
        std::fs::read_to_string(path)?
    } else {
        String::new()
    };

    if let (Some(s), Some(e)) = (existing.find(START), existing.find(END)) {
        let end_idx = e + END.len();
        existing.replace_range(s..end_idx, &format!("{}\n{}{}", START, content, END));
    } else {
        if !existing.is_empty() && !existing.ends_with('\n') {
            existing.push('\n');
        }
        existing.push_str(START);
        existing.push('\n');
        existing.push_str(content);
        existing.push_str(END);
        existing.push('\n');
    }

    write_file(path, &existing)
}

fn write_file(path: &std::path::Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, content)?;
    Ok(())
}

fn expand_home(path: &str) -> std::path::PathBuf {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest);
        }
    }
    std::path::PathBuf::from(path)
}

fn powershell_profile_path() -> std::path::PathBuf {
    if let Ok(p) = std::env::var("PROFILE") {
        if !p.trim().is_empty() {
            return std::path::PathBuf::from(p);
        }
    }

    let base = std::env::var("USERPROFILE")
        .map(std::path::PathBuf::from)
        .or_else(|_| dirs::home_dir().ok_or(std::env::VarError::NotPresent))
        .unwrap_or_else(|_| std::path::PathBuf::from("."));

    base.join("Documents")
        .join("PowerShell")
        .join("Microsoft.PowerShell_profile.ps1")
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
