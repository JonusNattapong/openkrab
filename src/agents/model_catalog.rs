//! agents::model_catalog — Model catalog with capability detection.
//! Ported from `openclaw/src/agents/model-catalog.ts` (Phase 16).

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Model catalog entry with capabilities.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelCatalogEntry {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub context_window: Option<usize>,
    pub reasoning: Option<bool>,
    pub input_capabilities: Vec<ModelInputCapability>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ModelInputCapability {
    Text,
    Image,
    Audio,
    Video,
}

/// Static model catalog with known models and their capabilities.
static STATIC_MODEL_CATALOG: Lazy<Vec<ModelCatalogEntry>> = Lazy::new(|| {
    vec![
        // OpenAI models
        ModelCatalogEntry {
            id: "gpt-4o".to_string(),
            name: "GPT-4o".to_string(),
            provider: "openai".to_string(),
            context_window: Some(128000),
            reasoning: Some(false),
            input_capabilities: vec![ModelInputCapability::Text, ModelInputCapability::Image],
        },
        ModelCatalogEntry {
            id: "gpt-4o-mini".to_string(),
            name: "GPT-4o Mini".to_string(),
            provider: "openai".to_string(),
            context_window: Some(128000),
            reasoning: Some(false),
            input_capabilities: vec![ModelInputCapability::Text, ModelInputCapability::Image],
        },
        ModelCatalogEntry {
            id: "o1".to_string(),
            name: "o1".to_string(),
            provider: "openai".to_string(),
            context_window: Some(200000),
            reasoning: Some(true),
            input_capabilities: vec![ModelInputCapability::Text],
        },
        ModelCatalogEntry {
            id: "o3-mini".to_string(),
            name: "o3-mini".to_string(),
            provider: "openai".to_string(),
            context_window: Some(200000),
            reasoning: Some(true),
            input_capabilities: vec![ModelInputCapability::Text],
        },
        ModelCatalogEntry {
            id: "text-embedding-3-small".to_string(),
            name: "Text Embedding 3 Small".to_string(),
            provider: "openai".to_string(),
            context_window: Some(8191),
            reasoning: Some(false),
            input_capabilities: vec![ModelInputCapability::Text],
        },
        ModelCatalogEntry {
            id: "text-embedding-3-large".to_string(),
            name: "Text Embedding 3 Large".to_string(),
            provider: "openai".to_string(),
            context_window: Some(8191),
            reasoning: Some(false),
            input_capabilities: vec![ModelInputCapability::Text],
        },
        // Gemini models
        ModelCatalogEntry {
            id: "gemini-1.5-flash".to_string(),
            name: "Gemini 1.5 Flash".to_string(),
            provider: "gemini".to_string(),
            context_window: Some(1000000),
            reasoning: Some(false),
            input_capabilities: vec![ModelInputCapability::Text, ModelInputCapability::Image],
        },
        ModelCatalogEntry {
            id: "gemini-1.5-pro".to_string(),
            name: "Gemini 1.5 Pro".to_string(),
            provider: "gemini".to_string(),
            context_window: Some(2000000),
            reasoning: Some(false),
            input_capabilities: vec![ModelInputCapability::Text, ModelInputCapability::Image],
        },
        ModelCatalogEntry {
            id: "text-embedding-004".to_string(),
            name: "Text Embedding 004".to_string(),
            provider: "gemini".to_string(),
            context_window: Some(2048),
            reasoning: Some(false),
            input_capabilities: vec![ModelInputCapability::Text],
        },
        // Ollama models (common ones)
        ModelCatalogEntry {
            id: "llama3.1".to_string(),
            name: "Llama 3.1".to_string(),
            provider: "ollama".to_string(),
            context_window: Some(8192),
            reasoning: Some(false),
            input_capabilities: vec![ModelInputCapability::Text],
        },
        ModelCatalogEntry {
            id: "codellama".to_string(),
            name: "Code Llama".to_string(),
            provider: "ollama".to_string(),
            context_window: Some(4096),
            reasoning: Some(false),
            input_capabilities: vec![ModelInputCapability::Text],
        },
        ModelCatalogEntry {
            id: "nomic-embed-text".to_string(),
            name: "Nomic Embed Text".to_string(),
            provider: "ollama".to_string(),
            context_window: Some(8192),
            reasoning: Some(false),
            input_capabilities: vec![ModelInputCapability::Text],
        },
    ]
});

/// Load model catalog from static data.
/// In production, this could be extended to fetch from APIs.
pub fn load_model_catalog() -> Vec<ModelCatalogEntry> {
    STATIC_MODEL_CATALOG.clone()
}

/// Find a model in the catalog by provider and model ID.
pub fn find_model_in_catalog(
    catalog: &[ModelCatalogEntry],
    provider: &str,
    model_id: &str,
) -> Option<ModelCatalogEntry> {
    catalog
        .iter()
        .find(|entry| entry.provider == provider && entry.id == model_id)
        .cloned()
}

/// Check if a model supports vision (image input).
pub fn model_supports_vision(entry: &ModelCatalogEntry) -> bool {
    entry
        .input_capabilities
        .contains(&ModelInputCapability::Image)
}

/// Get all models for a specific provider.
pub fn get_models_for_provider(
    catalog: &[ModelCatalogEntry],
    provider: &str,
) -> Vec<ModelCatalogEntry> {
    catalog
        .iter()
        .filter(|entry| entry.provider == provider)
        .cloned()
        .collect()
}

/// Get all providers in the catalog.
pub fn get_providers(catalog: &[ModelCatalogEntry]) -> Vec<String> {
    let mut providers: Vec<String> = catalog
        .iter()
        .map(|entry| entry.provider.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    providers.sort();
    providers
}

/// Search models by name or ID.
pub fn search_models(catalog: &[ModelCatalogEntry], query: &str) -> Vec<ModelCatalogEntry> {
    let query_lower = query.to_lowercase();
    catalog
        .iter()
        .filter(|entry| {
            entry.name.to_lowercase().contains(&query_lower)
                || entry.id.to_lowercase().contains(&query_lower)
                || entry.provider.to_lowercase().contains(&query_lower)
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_catalog_has_entries() {
        let catalog = load_model_catalog();
        assert!(!catalog.is_empty());
        assert!(catalog.len() >= 10);
    }

    #[test]
    fn find_openai_gpt4o() {
        let catalog = load_model_catalog();
        let model = find_model_in_catalog(&catalog, "openai", "gpt-4o");
        assert!(model.is_some());
        let model = model.unwrap();
        assert_eq!(model.name, "GPT-4o");
        assert!(model_supports_vision(&model));
        assert_eq!(model.context_window, Some(128000));
    }

    #[test]
    fn find_gemini_flash() {
        let catalog = load_model_catalog();
        let model = find_model_in_catalog(&catalog, "gemini", "gemini-1.5-flash");
        assert!(model.is_some());
        let model = model.unwrap();
        assert_eq!(model.name, "Gemini 1.5 Flash");
        assert!(model_supports_vision(&model));
    }

    #[test]
    fn get_providers_includes_known_ones() {
        let catalog = load_model_catalog();
        let providers = get_providers(&catalog);
        assert!(providers.contains(&"openai".to_string()));
        assert!(providers.contains(&"gemini".to_string()));
        assert!(providers.contains(&"ollama".to_string()));
    }

    #[test]
    fn search_models_by_name() {
        let catalog = load_model_catalog();
        let results = search_models(&catalog, "gpt");
        assert!(!results.is_empty());
        assert!(results.iter().any(|m| m.name.contains("GPT")));
    }

    #[test]
    fn embedding_models_dont_support_vision() {
        let catalog = load_model_catalog();
        let embedding_model = find_model_in_catalog(&catalog, "openai", "text-embedding-3-small");
        assert!(embedding_model.is_some());
        assert!(!model_supports_vision(&embedding_model.unwrap()));
    }
}
