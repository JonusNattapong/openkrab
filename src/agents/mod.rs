pub mod identity;
pub mod chat;
pub mod core;
pub mod tool;
pub mod model_catalog;
pub mod provider_auth;

pub use identity::AgentIdentity;
pub use chat::{ChatProvider, ChatMessage, OpenAiChatProvider};
pub use core::Agent;
pub use tool::{Tool, SearchMemoryTool, ReadFileTool, ListFilesTool, WriteFileTool, ExecCommandTool, RememberTool, TaskTool, SpeakTool, ScheduleTool, BrowserTool, CodeInterpreterTool};
pub use model_catalog::{ModelCatalogEntry, ModelInputCapability, load_model_catalog, find_model_in_catalog, model_supports_vision};
pub use provider_auth::{ProviderAuthConfig, get_provider_config, is_provider_configured, get_configured_providers};
