//! providers::copilot_models — GitHub Copilot model list and definition builder.
//! Ported from `openkrab/src/providers/github-copilot-models.ts` (Phase 16).
//!
//! Provides the default Copilot model IDs and a builder for OpenAI-compatible
//! model definitions with Copilot-specific cost/context defaults.

use serde::{Deserialize, Serialize};

// ─── Defaults ─────────────────────────────────────────────────────────────────

pub const DEFAULT_CONTEXT_WINDOW: u64 = 128_000;
pub const DEFAULT_MAX_TOKENS: u64 = 8_192;

/// Default Copilot model IDs as of Phase 16.
/// Intentionally broad — unavailable models return errors from the API.
pub const DEFAULT_MODEL_IDS: &[&str] = &[
    "gpt-4o",
    "gpt-4.1",
    "gpt-4.1-mini",
    "gpt-4.1-nano",
    "o1",
    "o1-mini",
    "o3-mini",
];

/// Return a Vec of the default Copilot model IDs.
pub fn get_default_model_ids() -> Vec<&'static str> {
    DEFAULT_MODEL_IDS.to_vec()
}

// ─── Model definition ─────────────────────────────────────────────────────────

/// Supported API style for a model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModelApi {
    OpenaiResponses,
    OpenaiChat,
    Custom(String),
}

/// Input modality supported by a model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InputModality {
    Text,
    Image,
    Audio,
}

/// Cost structure (all zero for Copilot — billed via subscription).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelCost {
    pub input: u64,
    pub output: u64,
    pub cache_read: u64,
    pub cache_write: u64,
}

/// Full model definition (mirrors `ModelDefinitionConfig` in pi-ai).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDefinition {
    pub id: String,
    pub name: String,
    pub api: ModelApi,
    pub reasoning: bool,
    pub input: Vec<InputModality>,
    pub cost: ModelCost,
    pub context_window: u64,
    pub max_tokens: u64,
}

/// Build a `ModelDefinition` for a Copilot model by ID.
///
/// Uses the OpenAI Responses API style and zero-cost (subscription-billed).
pub fn build_copilot_model_definition(model_id: &str) -> anyhow::Result<ModelDefinition> {
    let id = model_id.trim().to_string();
    if id.is_empty() {
        anyhow::bail!("Model id required");
    }
    Ok(ModelDefinition {
        name: id.clone(),
        id,
        api: ModelApi::OpenaiResponses,
        reasoning: false,
        input: vec![InputModality::Text, InputModality::Image],
        cost: ModelCost::default(),
        context_window: DEFAULT_CONTEXT_WINDOW,
        max_tokens: DEFAULT_MAX_TOKENS,
    })
}

/// Build definitions for all default Copilot models.
pub fn build_default_copilot_models() -> Vec<ModelDefinition> {
    DEFAULT_MODEL_IDS
        .iter()
        .filter_map(|id| build_copilot_model_definition(id).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_model_ids_non_empty() {
        let ids = get_default_model_ids();
        assert!(!ids.is_empty());
        assert!(ids.contains(&"gpt-4o"));
        assert!(ids.contains(&"o3-mini"));
    }

    #[test]
    fn build_copilot_model_definition_ok() {
        let def = build_copilot_model_definition("gpt-4o").unwrap();
        assert_eq!(def.id, "gpt-4o");
        assert_eq!(def.name, "gpt-4o");
        assert_eq!(def.api, ModelApi::OpenaiResponses);
        assert_eq!(def.context_window, DEFAULT_CONTEXT_WINDOW);
        assert_eq!(def.max_tokens, DEFAULT_MAX_TOKENS);
        assert!(!def.reasoning);
        assert!(def.input.contains(&InputModality::Text));
        assert!(def.input.contains(&InputModality::Image));
    }

    #[test]
    fn build_copilot_model_definition_empty_id() {
        assert!(build_copilot_model_definition("  ").is_err());
        assert!(build_copilot_model_definition("").is_err());
    }

    #[test]
    fn build_default_copilot_models_count() {
        let models = build_default_copilot_models();
        assert_eq!(models.len(), DEFAULT_MODEL_IDS.len());
    }

    #[test]
    fn model_cost_default_zero() {
        let def = build_copilot_model_definition("gpt-4o").unwrap();
        assert_eq!(def.cost.input, 0);
        assert_eq!(def.cost.output, 0);
    }
}
