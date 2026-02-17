use async_trait::async_trait;
use openclaw_core::{ChannelId, Message, Result, Session, SessionId, User};

pub mod backends;

pub use backends::sqlite::SqliteStorage;

/// Storage backend trait for persistent data
#[async_trait]
pub trait Storage: Send + Sync {
    // Sessions
    async fn get_session(&self, session_id: SessionId) -> Result<Option<Session>>;
    async fn save_session(&self, session: &Session) -> Result<()>;
    async fn delete_session(&self, session_id: SessionId) -> Result<()>;
    async fn list_sessions(&self, filter: SessionFilter) -> Result<Vec<Session>>;

    // Messages
    async fn get_message(&self, message_id: &str) -> Result<Option<Message>>;
    async fn save_message(&self, message: &Message) -> Result<()>;
    async fn list_messages(
        &self,
        session_id: Option<SessionId>,
        channel_id: Option<ChannelId>,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<Message>>;

    // Users
    async fn get_user(&self, channel_id: &str, user_id: &str) -> Result<Option<User>>;
    async fn save_user(&self, user: &User) -> Result<()>;
    async fn get_or_create_user(&self, channel_id: &str, user_id: &str, display_name: &str) -> Result<User>;

    // Config
    async fn get_config_value(&self, key: &str) -> Result<Option<String>>;
    async fn set_config_value(&self, key: &str, value: &str) -> Result<()>;
    async fn delete_config_value(&self, key: &str) -> Result<()>;

    // Health & Maintenance
    async fn health_check(&self) -> Result<()>;
    async fn migrate(&self) -> Result<()>;
    async fn close(&self) -> Result<()>;
}

/// Filter for listing sessions
#[derive(Debug, Clone)]
pub struct SessionFilter {
    pub channel_id: Option<ChannelId>,
    pub user_id: Option<String>,
    pub chat_id: Option<String>,
    pub active_only: bool,
    pub limit: usize,
    pub offset: usize,
}

impl SessionFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_channel(mut self, channel_id: ChannelId) -> Self {
        self.channel_id = Some(channel_id);
        self
    }

    pub fn with_user(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    pub fn with_chat(mut self, chat_id: impl Into<String>) -> Self {
        self.chat_id = Some(chat_id.into());
        self
    }

    pub fn active_only(mut self) -> Self {
        self.active_only = true;
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }
}

impl Default for SessionFilter {
    fn default() -> Self {
        Self {
            channel_id: None,
            user_id: None,
            chat_id: None,
            active_only: false,
            // default limit should be a sensible non-zero page size so queries without explicit limit return results
            limit: 100,
            offset: 0,
        }
    }
}

/// Storage configuration
#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub backend: StorageBackend,
    pub connection_string: String,
    pub max_connections: u32,
    pub timeout_seconds: u64,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            backend: StorageBackend::Sqlite,
            connection_string: "sqlite:openclaw.db".to_string(),
            max_connections: 10,
            timeout_seconds: 30,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageBackend {
    Sqlite,
    PostgreSQL,
    Memory,
}

/// Storage factory
pub async fn create_storage(config: &StorageConfig) -> Result<Box<dyn Storage>> {
    match config.backend {
        StorageBackend::Sqlite => {
            let storage = backends::SqliteStorage::connect(&config.connection_string).await?;
            Ok(Box::new(storage))
        }
        StorageBackend::PostgreSQL => {
            let storage = backends::PostgresStorage::connect(&config.connection_string).await?;
            Ok(Box::new(storage))
        }
        StorageBackend::Memory => {
            let storage = backends::MemoryStorage::new();
            Ok(Box::new(storage))
        }
    }
}
