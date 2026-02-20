//! Wizard prompter trait and common types.
//! Ported from `openclaw/src/wizard/prompts.ts`

use std::fmt;

/// A single option within a select/multiselect prompt.
#[derive(Debug, Clone)]
pub struct WizardSelectOption<T: Clone = String> {
    pub value: T,
    pub label: String,
    pub hint: Option<String>,
}

/// Parameters for a single-select prompt.
#[derive(Debug, Clone)]
pub struct WizardSelectParams<T: Clone = String> {
    pub message: String,
    pub options: Vec<WizardSelectOption<T>>,
    pub initial_value: Option<T>,
}

/// Parameters for a multi-select prompt.
#[derive(Debug, Clone)]
pub struct WizardMultiSelectParams<T: Clone = String> {
    pub message: String,
    pub options: Vec<WizardSelectOption<T>>,
    pub initial_values: Option<Vec<T>>,
    pub searchable: Option<bool>,
}

/// Parameters for a text-input prompt.
#[derive(Debug, Clone)]
pub struct WizardTextParams {
    pub message: String,
    pub initial_value: Option<String>,
    pub placeholder: Option<String>,
    // Validation is done at call-site via closures
}

/// Parameters for a yes/no confirmation prompt.
#[derive(Debug, Clone)]
pub struct WizardConfirmParams {
    pub message: String,
    pub initial_value: Option<bool>,
}

/// Progress tracker for long-running operations.
pub trait WizardProgress: Send + Sync {
    fn update(&self, message: &str);
    fn stop(&self, message: Option<&str>);
}

/// No-op progress tracker.
pub struct NoopProgress;

impl WizardProgress for NoopProgress {
    fn update(&self, _message: &str) {}
    fn stop(&self, _message: Option<&str>) {}
}

/// The prompter interface — abstracts over CLI, TUI, and gateway-driven prompts.
///
/// This is the core abstraction that allows the wizard to run in different
/// environments (terminal prompts, TUI, or remote gateway sessions).
#[async_trait::async_trait]
pub trait WizardPrompter: Send + Sync {
    /// Display a title/introduction.
    async fn intro(&self, title: &str) -> anyhow::Result<()>;

    /// Display a closing message.
    async fn outro(&self, message: &str) -> anyhow::Result<()>;

    /// Display a note with optional title.
    async fn note(&self, message: &str, title: Option<&str>) -> anyhow::Result<()>;

    /// Prompt the user to select one option from a list.
    async fn select(&self, params: WizardSelectParams<String>) -> anyhow::Result<String>;

    /// Prompt the user to select multiple options from a list.
    async fn multiselect(&self, params: WizardMultiSelectParams<String>) -> anyhow::Result<Vec<String>>;

    /// Prompt the user for text input.
    async fn text(&self, params: WizardTextParams) -> anyhow::Result<String>;

    /// Prompt the user for a yes/no confirmation.
    async fn confirm(&self, params: WizardConfirmParams) -> anyhow::Result<bool>;

    /// Start a progress indicator.
    fn progress(&self, label: &str) -> Box<dyn WizardProgress>;
}

/// Error type for when the user cancels the wizard.
#[derive(Debug, Clone)]
pub struct WizardCancelledError {
    pub message: String,
}

impl WizardCancelledError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Default for WizardCancelledError {
    fn default() -> Self {
        Self {
            message: "wizard cancelled".to_string(),
        }
    }
}

impl fmt::Display for WizardCancelledError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for WizardCancelledError {}

/// A CLI-based prompter that reads from stdin and writes to stdout.
/// This is the default prompter for terminal-based onboarding.
pub struct CliPrompter;

#[async_trait::async_trait]
impl WizardPrompter for CliPrompter {
    async fn intro(&self, title: &str) -> anyhow::Result<()> {
        println!();
        println!("┌  {}", title);
        println!("│");
        Ok(())
    }

    async fn outro(&self, message: &str) -> anyhow::Result<()> {
        println!("│");
        println!("└  {}", message);
        println!();
        Ok(())
    }

    async fn note(&self, message: &str, title: Option<&str>) -> anyhow::Result<()> {
        println!("│");
        if let Some(t) = title {
            println!("◆  {}", t);
        }
        for line in message.lines() {
            println!("│  {}", line);
        }
        Ok(())
    }

    async fn select(&self, params: WizardSelectParams<String>) -> anyhow::Result<String> {
        println!("│");
        println!("◆  {}", params.message);
        for (i, opt) in params.options.iter().enumerate() {
            let hint = opt.hint.as_deref().map(|h| format!(" ({})", h)).unwrap_or_default();
            println!("│  {}. {}{}", i + 1, opt.label, hint);
        }

        loop {
            print!("│  Enter choice [1-{}]: ", params.options.len());
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let trimmed = input.trim();
            if let Ok(n) = trimmed.parse::<usize>() {
                if n >= 1 && n <= params.options.len() {
                    return Ok(params.options[n - 1].value.clone());
                }
            }
            // Also check if they typed the value directly
            if let Some(opt) = params.options.iter().find(|o| o.value == trimmed || o.label.eq_ignore_ascii_case(trimmed)) {
                return Ok(opt.value.clone());
            }
            println!("│  Invalid selection, try again.");
        }
    }

    async fn multiselect(&self, params: WizardMultiSelectParams<String>) -> anyhow::Result<Vec<String>> {
        println!("│");
        println!("◆  {} (comma-separated numbers)", params.message);
        for (i, opt) in params.options.iter().enumerate() {
            let hint = opt.hint.as_deref().map(|h| format!(" ({})", h)).unwrap_or_default();
            println!("│  {}. {}{}", i + 1, opt.label, hint);
        }

        print!("│  Enter choices: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let mut result = Vec::new();
        for part in input.trim().split(',') {
            let part = part.trim();
            if let Ok(n) = part.parse::<usize>() {
                if n >= 1 && n <= params.options.len() {
                    result.push(params.options[n - 1].value.clone());
                }
            }
        }

        Ok(result)
    }

    async fn text(&self, params: WizardTextParams) -> anyhow::Result<String> {
        println!("│");
        let prompt = if let Some(ref ph) = params.placeholder {
            format!("◆  {} ({}): ", params.message, ph)
        } else {
            format!("◆  {}: ", params.message)
        };
        print!("{}", prompt);

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let trimmed = input.trim();

        if trimmed.is_empty() {
            if let Some(initial) = params.initial_value {
                return Ok(initial);
            }
        }

        Ok(trimmed.to_string())
    }

    async fn confirm(&self, params: WizardConfirmParams) -> anyhow::Result<bool> {
        println!("│");
        let default_hint = match params.initial_value {
            Some(true) => " [Y/n]",
            Some(false) => " [y/N]",
            None => " [y/n]",
        };
        print!("◆  {}{}: ", params.message, default_hint);

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let trimmed = input.trim().to_lowercase();

        if trimmed.is_empty() {
            return Ok(params.initial_value.unwrap_or(false));
        }

        Ok(trimmed == "y" || trimmed == "yes")
    }

    fn progress(&self, label: &str) -> Box<dyn WizardProgress> {
        println!("│  ⏳ {}", label);
        Box::new(NoopProgress)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancelled_error_display() {
        let err = WizardCancelledError::default();
        assert_eq!(err.to_string(), "wizard cancelled");

        let err = WizardCancelledError::new("risk not accepted");
        assert_eq!(err.to_string(), "risk not accepted");
    }

    #[test]
    fn noop_progress() {
        let p = NoopProgress;
        p.update("test");
        p.stop(Some("done"));
    }
}
