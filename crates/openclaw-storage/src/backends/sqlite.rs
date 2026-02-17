use async_trait::async_trait;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

use openclaw_core::{ChannelId, Message, MessageContent, OpenClawError, Result, Session, SessionId, User, UserId};

use crate::{SessionFilter, Storage};

/// SQLite storage backend
#[derive(Debug, Clone)]
pub struct SqliteStorage {
    pool: Pool<Sqlite>,
}

impl SqliteStorage {
    pub async fn connect(connection_string: &str) -> Result<Self> {
        // Try several common SQLite connection string forms to be robust across platforms
        // (tests often provide `sqlite://<path>` which on Windows contains backslashes).
        let mut last_err: Option<String> = None;

        // Build a candidate path by stripping the scheme if present and normalizing slashes
        let raw_path = if connection_string.starts_with("sqlite://") {
            connection_string["sqlite://".len()..].replace('\\', "/")
        } else if connection_string.starts_with("sqlite:") {
            connection_string["sqlite:".len()..].replace('\\', "/")
        } else {
            // Not a sqlite scheme – try as provided
            connection_string.to_string()
        };

        // Candidate connection string forms to try, ordered by preference.
        // We include several variants to accommodate platform differences (Windows drive letters,
        // leading slashes, and the simple `sqlite:<path>` form sqlx accepts).
        let candidates = if raw_path == connection_string {
            // `connection_string` was not recognized as a sqlite:<...> form – try as-is only
            vec![connection_string.to_string()]
        } else {
            let mut v = vec![];
            // simple: sqlite:<path>
            v.push(format!("sqlite:{}", raw_path));
            // URL-like: sqlite://<path>
            v.push(format!("sqlite://{}", raw_path));
            // URL-like with three slashes (absolute paths)
            v.push(format!("sqlite:///{}", raw_path.trim_start_matches('/')));
            v
        };

        let mut pool_opt: Option<Pool<Sqlite>> = None;
        // First, if we have a raw path, try connecting using SqliteConnectOptions with a filename
        // This avoids connection string parsing pitfalls on Windows.
        if raw_path != connection_string {
            let path = std::path::PathBuf::from(raw_path.clone());
            tracing::debug!("SQLite raw path: {:?}", path);

            // Ensure parent directory exists (tempdir should), try to create the file so sqlite can open it
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    if let Err(e) = std::fs::create_dir_all(parent) {
                        tracing::debug!("Failed to create parent dirs for sqlite db {:?}: {}", parent, e);
                    }
                }
            }

            if let Err(e) = std::fs::OpenOptions::new().create(true).write(true).open(&path) {
                tracing::debug!("Failed to ensure sqlite db file exists {:?}: {}", path, e);
            } else {
                tracing::debug!("Ensured sqlite db file exists: {:?}", path);
            }

            // Build connect options from filename (sqlx accepts Path)
            let opts = sqlx::sqlite::SqliteConnectOptions::new().filename(path);
            match SqlitePoolOptions::new().max_connections(10).connect_with(opts).await {
                Ok(p) => pool_opt = Some(p),
                Err(e) => tracing::debug!("connect_with failed: {}", e),
            }
        }
        for cand in candidates {
            // Log attempt and check if the path portion exists (when applicable)
            tracing::debug!("Attempting SQLite connect with: {}", cand);

            match SqlitePoolOptions::new().max_connections(10).connect(&cand).await {
                Ok(p) => {
                    pool_opt = Some(p);
                    break;
                }
                Err(e) => {
                    tracing::debug!("Failed to connect with `{}`: {}", cand, e);
                    last_err = Some(e.to_string());
                }
            }
        }

        let pool = match pool_opt {
            Some(p) => p,
            None => {
                return Err(OpenClawError::Storage { message: format!(
                    "Failed to connect to SQLite with candidates for '{}': {}",
                    connection_string,
                    last_err.unwrap_or_else(|| "unknown".to_string())
                ) });
            }
        };

        let storage = Self { pool };
        storage.migrate().await?;

        tracing::info!("Connected to SQLite database: {}", connection_string);
        Ok(storage)
    }

    fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }
}

#[async_trait]
impl Storage for SqliteStorage {
    async fn get_session(&self, session_id: SessionId) -> Result<Option<Session>> {
        let row = sqlx::query_as::<_, SessionRow>(
            r#"
            SELECT id, channel_id, user_id, chat_id, config, state, context,
                   created_at, updated_at, last_activity_at
            FROM sessions
            WHERE id = ?
            "#,
        )
        .bind(session_id.to_string())
        .fetch_optional(self.pool())
        .await
        .map_err(|e| OpenClawError::Storage { message: format!("Failed to get session: {}", e) })?;

        Ok(row.map(|r| r.into_session()))
    }

    async fn save_session(&self, session: &Session) -> Result<()> {
        let config_json = serde_json::to_string(&session.config)
            .map_err(|e| OpenClawError::Serialization { message: e.to_string() })?;
        let context_json = serde_json::to_string(&session.context)
            .map_err(|e| OpenClawError::Serialization { message: e.to_string() })?;

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO sessions
            (id, channel_id, user_id, chat_id, config, state, context,
             created_at, updated_at, last_activity_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(session.id.to_string())
        .bind(session.channel_id.to_string())
        .bind(&session.user_id)
        .bind(&session.chat_id)
        .bind(config_json)
        .bind(format!("{:?}", session.state))
        .bind(context_json)
        .bind(session.created_at)
        .bind(session.updated_at)
        .bind(session.last_activity_at)
        .execute(self.pool())
            .await
            .map_err(|e| OpenClawError::Storage { message: format!("Failed to save session: {}", e) })?;

        Ok(())
    }

    async fn delete_session(&self, session_id: SessionId) -> Result<()> {
        sqlx::query("DELETE FROM sessions WHERE id = ?")
            .bind(session_id.to_string())
            .execute(self.pool())
            .await
            .map_err(|e| OpenClawError::Storage { message: format!("Failed to delete session: {}", e) })?;

        Ok(())
    }

    async fn list_sessions(&self, filter: SessionFilter) -> Result<Vec<Session>> {
        let mut query = String::from(
            "SELECT id, channel_id, user_id, chat_id, config, state, context, created_at, updated_at, last_activity_at FROM sessions WHERE 1=1"
        );

        if filter.channel_id.is_some() {
            query.push_str(" AND channel_id = ?");
        }
        if filter.user_id.is_some() {
            query.push_str(" AND user_id = ?");
        }
        if filter.chat_id.is_some() {
            query.push_str(" AND chat_id = ?");
        }

        query.push_str(" ORDER BY last_activity_at DESC LIMIT ? OFFSET ?");

        let mut q = sqlx::query_as::<_, SessionRow>(&query);

        if let Some(ch) = filter.channel_id {
            q = q.bind(ch.to_string());
        }
        if let Some(uid) = filter.user_id {
            q = q.bind(uid);
        }
        if let Some(cid) = filter.chat_id {
            q = q.bind(cid);
        }

        let rows = q
            .bind(filter.limit as i64)
            .bind(filter.offset as i64)
            .fetch_all(self.pool())
            .await
            .map_err(|e| OpenClawError::Storage { message: format!("Failed to list sessions: {}", e) })?;

        Ok(rows.into_iter().map(|r| r.into_session()).collect())
    }

    async fn get_message(&self, message_id: &str) -> Result<Option<Message>> {
        let row = sqlx::query_as::<_, MessageRow>(
            r#"
            SELECT id, session_id, channel_id, chat_id, user_id, direction, content, metadata, created_at
            FROM messages WHERE id = ?
            "#,
        )
        .bind(message_id)
        .fetch_optional(self.pool())
            .await
            .map_err(|e| OpenClawError::Storage { message: format!("Failed to get message: {}", e) })?;

        Ok(row.map(|r| r.into_message()))
    }

    async fn save_message(&self, message: &Message) -> Result<()> {
        let content_json = serde_json::to_string(&message.content)
            .map_err(|e| OpenClawError::Serialization { message: e.to_string() })?;
        let metadata_json = serde_json::to_string(&message.metadata)
            .map_err(|e| OpenClawError::Serialization { message: e.to_string() })?;

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO messages
            (id, session_id, channel_id, chat_id, user_id, direction, content, metadata, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(message.id.to_string())
        .bind(message.session_id.as_ref().map(|s| s.to_string()))
        .bind(message.channel_id.to_string())
        .bind(&message.chat_id)
        .bind(message.user_id.as_ref())
        .bind(format!("{:?}", message.direction))
        .bind(content_json)
        .bind(metadata_json)
        .bind(message.created_at)
        .execute(self.pool())
            .await
            .map_err(|e| OpenClawError::Storage { message: format!("Failed to save message: {}", e) })?;

        Ok(())
    }

    async fn list_messages(
        &self,
        session_id: Option<SessionId>,
        channel_id: Option<ChannelId>,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<Message>> {
        let mut query = String::from(
            "SELECT id, session_id, channel_id, chat_id, user_id, direction, content, metadata, created_at FROM messages WHERE 1=1"
        );

        if session_id.is_some() {
            query.push_str(" AND session_id = ?");
        }
        if channel_id.is_some() {
            query.push_str(" AND channel_id = ?");
        }

        query.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");

        let mut q = sqlx::query_as::<_, MessageRow>(&query);

        if let Some(sid) = session_id {
            q = q.bind(sid.to_string());
        }
        if let Some(cid) = channel_id {
            q = q.bind(cid.to_string());
        }

        let rows = q
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(self.pool())
            .await
            .map_err(|e| OpenClawError::Storage { message: format!("Failed to list messages: {}", e) })?;

        Ok(rows.into_iter().map(|r| r.into_message()).collect())
    }

    async fn get_user(&self, channel_id: &str, user_id: &str) -> Result<Option<User>> {
        let row = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT channel_id, channel_user_id, global_user_id, display_name, metadata, created_at
            FROM users WHERE channel_id = ? AND channel_user_id = ?
            "#,
        )
        .bind(channel_id)
        .bind(user_id)
        .fetch_optional(self.pool())
            .await
            .map_err(|e| OpenClawError::Storage { message: format!("Failed to get user: {}", e) })?;

        Ok(row.map(|r| r.into_user()))
    }

    async fn save_user(&self, user: &User) -> Result<()> {
        let metadata_json = serde_json::to_string(&user.metadata)
            .map_err(|e| OpenClawError::Serialization { message: e.to_string() })?;

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO users
            (channel_id, channel_user_id, global_user_id, display_name, metadata, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&user.channel_id)
        .bind(&user.channel_user_id)
        .bind(user.global_user_id.as_ref())
        .bind(&user.display_name)
        .bind(metadata_json)
        .bind(user.created_at)
        .execute(self.pool())
            .await
            .map_err(|e| OpenClawError::Storage { message: format!("Failed to save user: {}", e) })?;

        Ok(())
    }

    async fn get_or_create_user(
        &self,
        channel_id: &str,
        user_id: &str,
        display_name: &str,
    ) -> Result<User> {
        if let Some(user) = self.get_user(channel_id, user_id).await? {
            return Ok(user);
        }

        let user = User {
            channel_id: channel_id.to_string(),
            channel_user_id: user_id.to_string(),
            global_user_id: None,
            display_name: display_name.to_string(),
            metadata: serde_json::Value::Null,
            created_at: chrono::Utc::now(),
        };

        self.save_user(&user).await?;
        Ok(user)
    }

    async fn get_config_value(&self, key: &str) -> Result<Option<String>> {
        let row: Option<(String,)> = sqlx::query_as::<_, (String,)>("SELECT value FROM config WHERE key = ?")
            .bind(key)
            .fetch_optional(self.pool())
            .await
            .map_err(|e| OpenClawError::Storage { message: format!("Failed to get config: {}", e) })?;

        Ok(row.map(|r| r.0))
    }

    async fn set_config_value(&self, key: &str, value: &str) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO config (key, value, updated_at)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(key)
        .bind(value)
        .bind(chrono::Utc::now())
        .execute(self.pool())
            .await
            .map_err(|e| OpenClawError::Storage { message: format!("Failed to set config: {}", e) })?;

        Ok(())
    }

    async fn delete_config_value(&self, key: &str) -> Result<()> {
        sqlx::query("DELETE FROM config WHERE key = ?")
            .bind(key)
            .execute(self.pool())
            .await
            .map_err(|e| OpenClawError::Storage { message: format!("Failed to delete config: {}", e) })?;

        Ok(())
    }

    async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .fetch_one(self.pool())
            .await
            .map_err(|e| OpenClawError::Storage { message: format!("Health check failed: {}", e) })?;
        Ok(())
    }

    async fn migrate(&self) -> Result<()> {
        let migrations = vec![
            r#"
            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                channel_id TEXT NOT NULL,
                user_id TEXT NOT NULL,
                chat_id TEXT NOT NULL,
                config TEXT NOT NULL,
                state TEXT NOT NULL,
                context TEXT NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                last_activity_at TIMESTAMP NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_sessions_channel ON sessions(channel_id);
            CREATE INDEX IF NOT EXISTS idx_sessions_user ON sessions(user_id);
            CREATE INDEX IF NOT EXISTS idx_sessions_chat ON sessions(chat_id);
            "#,
            r#"
            CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                session_id TEXT REFERENCES sessions(id),
                channel_id TEXT NOT NULL,
                chat_id TEXT NOT NULL,
                user_id TEXT,
                direction TEXT NOT NULL,
                content TEXT NOT NULL,
                metadata TEXT,
                created_at TIMESTAMP NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_messages_session ON messages(session_id);
            CREATE INDEX IF NOT EXISTS idx_messages_channel ON messages(channel_id);
            CREATE INDEX IF NOT EXISTS idx_messages_created ON messages(created_at);
            "#,
            r#"
            CREATE TABLE IF NOT EXISTS users (
                channel_id TEXT NOT NULL,
                channel_user_id TEXT NOT NULL,
                global_user_id TEXT,
                display_name TEXT NOT NULL,
                metadata TEXT,
                created_at TIMESTAMP NOT NULL,
                PRIMARY KEY (channel_id, channel_user_id)
            );
            CREATE INDEX IF NOT EXISTS idx_users_global ON users(global_user_id);
            "#,
            r#"
            CREATE TABLE IF NOT EXISTS config (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TIMESTAMP NOT NULL
            );
            "#,
        ];

        for migration in migrations {
            sqlx::query(migration)
                .execute(self.pool())
                .await
                .map_err(|e| OpenClawError::Storage { message: format!("Migration failed: {}", e) })?;
        }

        tracing::info!("SQLite migrations completed");
        Ok(())
    }

    async fn close(&self) -> Result<()> {
        self.pool.close().await;
        Ok(())
    }
}

// Database row types
#[derive(sqlx::FromRow)]
struct SessionRow {
    id: String,
    channel_id: String,
    user_id: String,
    chat_id: String,
    config: String,
    state: String,
    context: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    last_activity_at: chrono::DateTime<chrono::Utc>,
}

impl SessionRow {
    fn into_session(self) -> Session {
        Session {
            id: SessionId::from(self.id),
            name: "default".to_string(),
            channel_id: ChannelId::from(self.channel_id),
            user_id: self.user_id,
            chat_id: self.chat_id,
            config: serde_json::from_str(&self.config).unwrap_or_default(),
            state: parse_session_state(&self.state),
            context: serde_json::from_str(&self.context).unwrap_or_default(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            last_activity_at: self.last_activity_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct MessageRow {
    id: String,
    session_id: Option<String>,
    channel_id: String,
    chat_id: String,
    user_id: Option<String>,
    direction: String,
    content: String,
    metadata: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl MessageRow {
    fn into_message(self) -> Message {
        Message {
            id: self.id.into(),
            session_id: self.session_id.map(|s| SessionId::from(s)),
            channel_id: ChannelId::from(self.channel_id),
            chat_id: self.chat_id,
            user_id: self.user_id,
            sender: UserId::new(),
            recipient: UserId::new(),
            direction: parse_direction(&self.direction),
            content: serde_json::from_str(&self.content).unwrap_or_else(|_| MessageContent::Text { text: "".to_string() }),
            metadata: self
                .metadata
                .and_then(|m| serde_json::from_str(&m).ok())
                .unwrap_or(serde_json::Value::Null),
            created_at: self.created_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct UserRow {
    channel_id: String,
    channel_user_id: String,
    global_user_id: Option<String>,
    display_name: String,
    metadata: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl UserRow {
    fn into_user(self) -> User {
        User {
            channel_id: self.channel_id,
            channel_user_id: self.channel_user_id,
            global_user_id: self.global_user_id,
            display_name: self.display_name,
            metadata: serde_json::from_str(&self.metadata).unwrap_or_default(),
            created_at: self.created_at,
        }
    }
}

fn parse_session_state(s: &str) -> openclaw_core::SessionState {
    match s {
        "Active" => openclaw_core::SessionState::Active,
        "Inactive" => openclaw_core::SessionState::Inactive,
        "Closed" => openclaw_core::SessionState::Closed,
        _ => openclaw_core::SessionState::Active,
    }
}

fn parse_direction(s: &str) -> openclaw_core::Direction {
    match s {
        "Incoming" => openclaw_core::Direction::Incoming,
        "Outgoing" => openclaw_core::Direction::Outgoing,
        _ => openclaw_core::Direction::Incoming,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openclaw_core::{SessionState, Direction, MessageContent};
    use tempfile::NamedTempFile;

    async fn create_test_storage() -> SqliteStorage {
        // Create a temporary SQLite database file
        let temp_file = NamedTempFile::new().unwrap();
        let connection_string = format!("sqlite:{}", temp_file.path().display());
        SqliteStorage::connect(&connection_string).await.unwrap()
    }

    #[tokio::test]
    async fn test_sqlite_connect() {
        let storage = create_test_storage().await;
        // Health check should pass
        assert!(storage.health_check().await.is_ok());
    }

    #[tokio::test]
    async fn test_session_crud() {
        let storage = create_test_storage().await;
        
        let mut session = Session::new(ChannelId::new(), Some("test-session".to_string()));
        session.user_id = "user123".to_string();
        session.chat_id = "chat456".to_string();

        // Save session
        storage.save_session(&session).await.unwrap();
        
        // Get session
        let retrieved = storage.get_session(session.id.clone()).await.unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.id, session.id);
        assert_eq!(retrieved.user_id, session.user_id);
        assert_eq!(retrieved.chat_id, session.chat_id);

        // List sessions
        let filter = SessionFilter::new().with_channel(session.channel_id.clone());
        let sessions = storage.list_sessions(filter).await.unwrap();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].id, session.id);

        // Delete session
        storage.delete_session(session.id.clone()).await.unwrap();
        let deleted = storage.get_session(session.id).await.unwrap();
        assert!(deleted.is_none());
    }

    #[tokio::test]
    async fn test_message_crud() {
        let storage = create_test_storage().await;
        
        let sender = UserId::new();
        let recipient = UserId::new();
        let mut message = Message::new_text(ChannelId::new(), sender, recipient, "Hello, world!".to_string());
        // Do not set session_id because the session does not exist in the DB (would violate FK)
        message.session_id = None;
        message.chat_id = "chat456".to_string();
        message.user_id = Some("user123".to_string());
        message.direction = Direction::Incoming;

        // Save message
        storage.save_message(&message).await.unwrap();
        
        // Get message
        let retrieved = storage.get_message(&message.id.to_string()).await.unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.id, message.id);
        match retrieved.content {
            MessageContent::Text { text } => assert_eq!(text, "Hello, world!"),
            _ => panic!("Expected text content"),
        }

        // List messages
        let messages = storage.list_messages(None, None, 10, 0).await.unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].id, message.id);
    }

    #[tokio::test]
    async fn test_user_crud() {
        let storage = create_test_storage().await;
        
        let user = User {
            channel_id: "telegram".to_string(),
            channel_user_id: "user123".to_string(),
            global_user_id: Some("global_123".to_string()),
            display_name: "Test User".to_string(),
            metadata: serde_json::Value::Null,
            created_at: chrono::Utc::now(),
        };

        // Save user
        storage.save_user(&user).await.unwrap();
        
        // Get user
        let retrieved = storage.get_user(&user.channel_id, &user.channel_user_id)
            .await
            .unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.channel_user_id, user.channel_user_id);
        assert_eq!(retrieved.display_name, user.display_name);

        // Get or create user (existing)
        let existing = storage.get_or_create_user(
            &user.channel_id,
            &user.channel_user_id,
            "New Name"
        ).await.unwrap();
        assert_eq!(existing.channel_user_id, user.channel_user_id);
        assert_eq!(existing.display_name, "Test User"); // Should not update

        // Get or create user (new)
        let new_user = storage.get_or_create_user(
            "discord",
            "user456",
            "Discord User"
        ).await.unwrap();
        assert_eq!(new_user.channel_user_id, "user456");
        assert_eq!(new_user.display_name, "Discord User");
    }

    #[tokio::test]
    async fn test_config_crud() {
        let storage = create_test_storage().await;
        
        // Set config
        storage.set_config_value("test_key", "test_value").await.unwrap();
        
        // Get config
        let value = storage.get_config_value("test_key").await.unwrap();
        assert_eq!(value, Some("test_value".to_string()));

        // Delete config
        storage.delete_config_value("test_key").await.unwrap();
        let deleted = storage.get_config_value("test_key").await.unwrap();
        assert!(deleted.is_none());
    }
}
