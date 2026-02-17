use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

use openclaw_core::{ChannelId, Message, OpenClawError, Result, Session, SessionId, User};

use crate::{SessionFilter, Storage};

/// In-memory storage for testing
#[derive(Debug, Clone)]
pub struct MemoryStorage {
    sessions: Arc<RwLock<HashMap<SessionId, Session>>>,
    messages: Arc<RwLock<HashMap<String, Message>>>,
    users: Arc<RwLock<HashMap<(String, String), User>>>,
    config: Arc<RwLock<HashMap<String, String>>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            messages: Arc::new(RwLock::new(HashMap::new())),
            users: Arc::new(RwLock::new(HashMap::new())),
            config: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl Storage for MemoryStorage {
    async fn get_session(&self, session_id: SessionId) -> Result<Option<Session>> {
        let sessions = self.sessions.read();
        Ok(sessions.get(&session_id).cloned())
    }

    async fn save_session(&self, session: &Session) -> Result<()> {
        let mut sessions = self.sessions.write();
        sessions.insert(session.id.clone(), session.clone());
        Ok(())
    }

    async fn delete_session(&self, session_id: SessionId) -> Result<()> {
        let mut sessions = self.sessions.write();
        sessions.remove(&session_id);
        Ok(())
    }

    async fn list_sessions(&self, filter: SessionFilter) -> Result<Vec<Session>> {
        let sessions = self.sessions.read();
        let mut results: Vec<Session> = sessions
            .values()
            .filter(|s| {
                if let Some(ref ch) = filter.channel_id {
                    if &s.channel_id != ch {
                        return false;
                    }
                }
                if let Some(ref uid) = filter.user_id {
                    if &s.user_id != uid {
                        return false;
                    }
                }
                if let Some(ref cid) = filter.chat_id {
                    if &s.chat_id != cid {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        // Sort by last_activity_at descending
        results.sort_by(|a, b| b.last_activity_at.cmp(&a.last_activity_at));

        // Apply offset and limit
        let offset = filter.offset.min(results.len());
        let limit = filter.limit.min(results.len() - offset);
        results = results.into_iter().skip(offset).take(limit).collect();

        Ok(results)
    }

    async fn get_message(&self, message_id: &str) -> Result<Option<Message>> {
        let messages = self.messages.read();
        Ok(messages.get(message_id).cloned())
    }

    async fn save_message(&self, message: &Message) -> Result<()> {
        let mut messages = self.messages.write();
        messages.insert(message.id.to_string(), message.clone());
        Ok(())
    }

    async fn list_messages(
        &self,
        _session_id: Option<SessionId>,
        _channel_id: Option<ChannelId>,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<Message>> {
        let messages = self.messages.read();
        let mut results: Vec<Message> = messages.values().cloned().collect();

        // Sort by created_at descending
        results.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        let offset = offset.min(results.len());
        let limit = limit.min(results.len() - offset);
        results = results.into_iter().skip(offset).take(limit).collect();

        Ok(results)
    }

    async fn get_user(&self, channel_id: &str, user_id: &str) -> Result<Option<User>> {
        let users = self.users.read();
        Ok(users.get(&(channel_id.to_string(), user_id.to_string())).cloned())
    }

    async fn save_user(&self, user: &User) -> Result<()> {
        let mut users = self.users.write();
        users.insert(
            (user.channel_id.clone(), user.channel_user_id.clone()),
            user.clone(),
        );
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
        let config = self.config.read();
        Ok(config.get(key).cloned())
    }

    async fn set_config_value(&self, key: &str, value: &str) -> Result<()> {
        let mut config = self.config.write();
        config.insert(key.to_string(), value.to_string());
        Ok(())
    }

    async fn delete_config_value(&self, key: &str) -> Result<()> {
        let mut config = self.config.write();
        config.remove(key);
        Ok(())
    }

    async fn health_check(&self) -> Result<()> {
        Ok(())
    }

    async fn migrate(&self) -> Result<()> {
        Ok(())
    }

    async fn close(&self) -> Result<()> {
        Ok(())
    }
}

/// SQLite storage with sqlx
pub mod sqlite;
pub use sqlite::SqliteStorage;

/// PostgreSQL storage (placeholder for now)
pub struct PostgresStorage;

impl PostgresStorage {
    pub async fn connect(_connection_string: &str) -> Result<Self> {
        // TODO: Implement PostgreSQL backend
        Err(OpenClawError::Storage { message: "PostgreSQL backend not yet implemented".to_string() })
    }
}

#[async_trait]
impl Storage for PostgresStorage {
    async fn get_session(&self, _session_id: SessionId) -> Result<Option<Session>> {
        unimplemented!()
    }
    async fn save_session(&self, _session: &Session) -> Result<()> {
        unimplemented!()
    }
    async fn delete_session(&self, _session_id: SessionId) -> Result<()> {
        unimplemented!()
    }
    async fn list_sessions(&self, _filter: SessionFilter) -> Result<Vec<Session>> {
        unimplemented!()
    }
    async fn get_message(&self, _message_id: &str) -> Result<Option<Message>> {
        unimplemented!()
    }
    async fn save_message(&self, _message: &Message) -> Result<()> {
        unimplemented!()
    }
    async fn list_messages(
        &self,
        _session_id: Option<SessionId>,
        _channel_id: Option<ChannelId>,
        _limit: usize,
        _offset: usize,
    ) -> Result<Vec<Message>> {
        unimplemented!()
    }
    async fn get_user(&self, _channel_id: &str, _user_id: &str) -> Result<Option<User>> {
        unimplemented!()
    }
    async fn save_user(&self, _user: &User) -> Result<()> {
        unimplemented!()
    }
    async fn get_or_create_user(
        &self,
        _channel_id: &str,
        _user_id: &str,
        _display_name: &str,
    ) -> Result<User> {
        unimplemented!()
    }
    async fn get_config_value(&self, _key: &str) -> Result<Option<String>> {
        unimplemented!()
    }
    async fn set_config_value(&self, _key: &str, _value: &str) -> Result<()> {
        unimplemented!()
    }
    async fn delete_config_value(&self, _key: &str) -> Result<()> {
        unimplemented!()
    }
    async fn health_check(&self) -> Result<()> {
        unimplemented!()
    }
    async fn migrate(&self) -> Result<()> {
        unimplemented!()
    }
    async fn close(&self) -> Result<()> {
        unimplemented!()
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}
