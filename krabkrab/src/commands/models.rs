use anyhow::{bail, Result};

/// List known model IDs for a provider.
///
/// This is a local catalog helper (no network call).
pub fn models_list_command(provider: &str) -> Result<String> {
    let normalized = provider.trim().to_lowercase();
    if normalized.is_empty() {
        bail!("provider is required");
    }

    let ids = crate::providers::known_model_ids(&normalized);
    if ids.is_empty() {
        bail!(
            "unknown or unsupported provider: {} (supported: openai, gemini, ollama, copilot)",
            normalized
        );
    }

    Ok(format!("provider={} models={}", normalized, ids.join(", ")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn models_list_command_for_copilot() {
        let out = models_list_command("copilot").unwrap();
        assert!(out.contains("provider=copilot"));
        assert!(out.contains("gpt-4o"));
        assert!(out.contains("o3-mini"));
    }

    #[test]
    fn models_list_command_unknown_provider() {
        let err = models_list_command("unknown").unwrap_err();
        assert!(err.to_string().contains("unsupported provider"));
    }
}
