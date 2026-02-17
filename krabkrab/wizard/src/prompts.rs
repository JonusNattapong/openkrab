use anyhow::Result;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct WizardSelectOption<T = String> {
    pub label: String,
    pub value: T,
}

pub trait WizardPrompter {
    fn note(&mut self, title: &str, message: &str) -> Result<()>;
    fn text(&mut self, title: &str, placeholder: Option<&str>) -> Result<String>;
    fn confirm(&mut self, title: &str, default: bool) -> Result<bool>;
    fn select(&mut self, title: &str, options: &[WizardSelectOption]) -> Result<String>;
}

#[derive(Default)]
pub struct ConsolePrompter;

impl ConsolePrompter {
    fn read_line(prompt: &str) -> Result<String> {
        print!("{prompt}");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }
}

impl WizardPrompter for ConsolePrompter {
    fn note(&mut self, title: &str, message: &str) -> Result<()> {
        println!("\n[{title}]\n{message}\n");
        Ok(())
    }

    fn text(&mut self, title: &str, placeholder: Option<&str>) -> Result<String> {
        let suffix = placeholder.map(|p| format!(" ({p})")).unwrap_or_default();
        Self::read_line(&format!("{title}{suffix}: "))
    }

    fn confirm(&mut self, title: &str, default: bool) -> Result<bool> {
        let hint = if default { "Y/n" } else { "y/N" };
        let input = Self::read_line(&format!("{title} [{hint}]: "))?;

        if input.is_empty() {
            return Ok(default);
        }

        let yes = matches!(input.to_ascii_lowercase().as_str(), "y" | "yes");
        Ok(yes)
    }

    fn select(&mut self, title: &str, options: &[WizardSelectOption]) -> Result<String> {
        println!("\n{title}");
        for (idx, option) in options.iter().enumerate() {
            println!("  {}. {}", idx + 1, option.label);
        }

        let picked = Self::read_line("Choose number: ")?;
        let idx: usize = picked.parse().unwrap_or(1);
        let idx = idx.saturating_sub(1).min(options.len().saturating_sub(1));
        Ok(options[idx].value.clone())
    }
}
