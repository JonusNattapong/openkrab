use crate::memory::embeddings::{
    EmbeddingProvider, GeminiProvider, OllamaProvider, OpenAiProvider, VoyageProvider,
};
use crate::memory::manager::HybridSearchOptions;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

pub const SUPPORTED_EMBEDDING_PROVIDERS: &[&str] = &["openai", "gemini", "ollama", "voyage"];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MemoryConfig {
    pub enabled: Option<bool>,
    pub provider: String,
    pub model: Option<String>,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub vector_weight: Option<f64>,
    pub text_weight: Option<f64>,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            enabled: Some(true),
            provider: "openai".to_string(),
            model: None,
            api_key: None,
            base_url: None,
            vector_weight: Some(0.7),
            text_weight: Some(0.3),
        }
    }
}

impl MemoryConfig {
    pub fn supported_embedding_providers() -> Vec<&'static str> {
        SUPPORTED_EMBEDDING_PROVIDERS.to_vec()
    }

    pub fn is_supported_embedding_provider(provider: &str) -> bool {
        let p = provider.trim().to_lowercase();
        p == "google" || SUPPORTED_EMBEDDING_PROVIDERS.contains(&p.as_str())
    }

    pub fn create_provider(&self) -> Result<Box<dyn EmbeddingProvider>> {
        match self.provider.trim().to_lowercase().as_str() {
            "openai" => {
                let api_key = self
                    .api_key
                    .clone()
                    .or_else(|| std::env::var("OPENAI_API_KEY").ok())
                    .ok_or_else(|| anyhow!("Missing OpenAI API Key"))?;
                Ok(Box::new(OpenAiProvider::new(
                    api_key,
                    self.base_url.clone(),
                    self.model.clone(),
                )))
            }
            "gemini" | "google" => {
                let api_key = self
                    .api_key
                    .clone()
                    .or_else(|| std::env::var("GOOGLE_API_KEY").ok())
                    .ok_or_else(|| anyhow!("Missing Gemini/Google API Key"))?;
                Ok(Box::new(GeminiProvider::new(
                    api_key,
                    self.base_url.clone(),
                    self.model.clone(),
                )))
            }
            "ollama" => Ok(Box::new(OllamaProvider::new(
                self.base_url.clone(),
                self.model.clone(),
            ))),
            "voyage" => {
                let api_key = self
                    .api_key
                    .clone()
                    .or_else(|| std::env::var("VOYAGE_API_KEY").ok())
                    .ok_or_else(|| anyhow!("Missing Voyage API Key"))?;
                Ok(Box::new(VoyageProvider::new(
                    api_key,
                    self.base_url.clone(),
                    self.model.clone(),
                )))
            }
            _ => Err(anyhow!("Unsupported embedding provider: {}", self.provider)),
        }
    }

    pub fn hybrid_search_options(&self) -> HybridSearchOptions {
        HybridSearchOptions {
            max_results: 10,
            vector_weight: self.vector_weight.unwrap_or(0.7),
            text_weight: self.text_weight.unwrap_or(0.3),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supported_embedding_providers_non_empty() {
        let p = MemoryConfig::supported_embedding_providers();
        assert!(p.contains(&"openai"));
        assert!(p.contains(&"gemini"));
        assert!(p.contains(&"ollama"));
    }

    #[test]
    fn is_supported_embedding_provider_alias_and_unknown() {
        assert!(MemoryConfig::is_supported_embedding_provider("openai"));
        assert!(MemoryConfig::is_supported_embedding_provider("google"));
        assert!(!MemoryConfig::is_supported_embedding_provider("copilot"));
        assert!(!MemoryConfig::is_supported_embedding_provider("unknown"));
    }
}
