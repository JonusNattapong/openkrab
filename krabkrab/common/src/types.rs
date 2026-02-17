use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub to: String,
    pub body: String,
}
