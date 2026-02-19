pub mod schema;
pub mod store;
pub mod embeddings;
pub mod manager;
pub mod config;
pub mod mmr;
pub mod temporal_decay;
pub mod query_expansion;
pub mod backend_config;

pub use store::MemoryStore;
pub use embeddings::{EmbeddingProvider, OpenAiProvider, GeminiProvider, OllamaProvider, VoyageProvider};
pub use manager::{MemoryManager, HybridSearchOptions};
pub use config::MemoryConfig;
pub use mmr::{MMRConfig, MMRItem, mmr_rerank, apply_mmr_to_results};
pub use temporal_decay::{TemporalDecayConfig, TemporalDecayItem, apply_temporal_decay_to_results};
pub use query_expansion::{extract_keywords, expand_query_for_fts};
pub use backend_config::{MemoryBackendConfig, QmdConfig, resolve_memory_backend_config};
