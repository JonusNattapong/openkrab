//! OpenClaw Sessions and Storage
//!
//! Manages session persistence and SQLite storage.

use openclaw_core::{Result, SessionId, OpenClawError};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::path::Path;
use tracing::{info, debug};

pub struct Storage {
    pool: Pool<Sqlite>,
}

impl Storage {
    pub async fn new(database_path: &str) -> anyhow::Result<Self> {
        let path = Path::new(database_path);
        
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                tokio::fs::create_dir_all(parent).await?;
            }
        }

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&format!("sqlite:{}", database_path))
            .await?;

        let storage = Self { pool };
        storage.init().await?;
        
        info!("Storage initialized at: {}", database_path);
        Ok(storage)
    }

    async fn init(&self) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                data TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                session_id TEXT NOT NULL,
                direction TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (session_id) REFERENCES sessions(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        debug!("Database schema initialized");
        Ok(())
    }

    pub async fn create_session(&self, session_id: &SessionId) -> Result<()> {
        let id = session_id.to_string();
        let data = serde_json::json!({});
        
        sqlx::query(
            r#"
            INSERT INTO sessions (id, data) VALUES (?, ?)
            "#,
        )
        .bind(&id)
        .bind(data.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| OpenClawError::Internal {
            message: format!("Failed to create session: {}", e),
        })?;

        debug!("Created session: {}", session_id);
        Ok(())
    }

    pub async fn get_session(&self, session_id: &SessionId) -> Result<Option<serde_json::Value>> {
        let id = session_id.to_string();
        
        let row: Option<(String,)> = sqlx::query_as(
            r#"
            SELECT data FROM sessions WHERE id = ?
            "#,
        )
        .bind(&id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| OpenClawError::Internal {
            message: format!("Failed to get session: {}", e),
        })?;

        match row {
            Some((data,)) => {
                let value = serde_json::from_str(&data).map_err(|e| OpenClawError::Internal {
                    message: format!("Failed to parse session data: {}", e),
                })?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    pub async fn update_session(&self, session_id: &SessionId, data: serde_json::Value) -> Result<()> {
        let id = session_id.to_string();
        
        sqlx::query(
            r#"
            UPDATE sessions SET data = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?
            "#,
        )
        .bind(data.to_string())
        .bind(&id)
        .execute(&self.pool)
        .await
        .map_err(|e| OpenClawError::Internal {
            message: format!("Failed to update session: {}", e),
        })?;

        Ok(())
    }

    pub async fn list_sessions(&self) -> Result<Vec<SessionId>> {
        let rows: Vec<(String,)> = sqlx::query_as(
            r#"
            SELECT id FROM sessions ORDER BY updated_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| OpenClawError::Internal {
            message: format!("Failed to list sessions: {}", e),
        })?;

        let session_ids: Result<Vec<_>> = rows
            .into_iter()
            .map(|(id,)| {
                uuid::Uuid::parse_str(&id)
                    .map(|uuid| SessionId(uuid))
                    .map_err(|e| OpenClawError::Internal {
                        message: format!("Invalid session ID: {}", e),
                    })
            })
            .collect();

        session_ids
    }

    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }
}

impl Clone for Storage {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}
