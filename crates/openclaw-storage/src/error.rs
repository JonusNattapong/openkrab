use thiserror::Error;

pub type Result<T> = std::result::Result<T, StorageError>;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Session not found: {0}")]
    SessionNotFound(String),

    #[error("Message not found: {0}")]
    MessageNotFound(String),

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Migration error: {0}")]
    Migration(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("SQLx error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

impl StorageError {
    pub fn database<S: Into<String>>(msg: S) -> Self {
        Self::Database(msg.into())
    }

    pub fn config<S: Into<String>>(msg: S) -> Self {
        Self::Config(msg.into())
    }
}
