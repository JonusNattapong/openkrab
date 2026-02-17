use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenClawError {
    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Network error: {source}")]
    Network {
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Channel error: {message}")]
    Channel { message: String },

    #[error("Authentication error: {message}")]
    Auth { message: String },

    #[error("Session error: {message}")]
    Session { message: String },

    #[error("Message error: {message}")]
    Message { message: String },

    #[error("Internal error: {message}")]
    Internal { message: String },

    #[error("Not found: {message}")]
    NotFound { message: String },

    #[error("Rate limited: {message}")]
    RateLimited { message: String },

    #[error("Permission denied: {message}")]
    PermissionDenied { message: String },

    #[error("Storage error: {message}")]
    Storage { message: String },

    #[error("Serialization error: {message}")]
    Serialization { message: String },
}

pub type Result<T> = std::result::Result<T, OpenClawError>;
