use async_trait::async_trait;
use openclaw_core::{ChannelId, Message, OpenClawError, Result, Session, SessionId, User};
use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::{SessionFilter, Storage};

/// In-memory storage backend for testing
pub struct MemoryStorage {
    sessions: RwLock<HashMap<SessionId, Session>>,
    messages: RwLock<HashMap<String, Message>>,
    users: RwLock<HashMap<(String, String), User>>, // (channel_id, user_id) -> User
    config: RwLock<HashMap<String, String>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            messages: RwLock::new(HashMap::new()),
            users: RwLock::new(HashMap::new()),
            config: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Storage for MemoryStorage {
    // Sessions
    async fn get_session(&self, session_id: SessionId) -> Result<Option<Session>> {
        let sessions = self.sessions.read().await;
        Ok(sessions.get(&session_id).cloned())
    }

    async fn save_session(&self, session: &Session) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        sessions.insert(session.id.clone(), session.clone());
        Ok(())
    }

    async fn delete_session(&self, session_id: SessionId) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        sessions.remove(&session_id);
        Ok(())
    }

    async fn list_sessions(&self,
        filter: SessionFilter,
    ) -> Result<Vec<Session>> {
        let sessions = self.sessions.read().await;
        let mut result: Vec<Session> = sessions
            .values()
            .filter(|s| {
                if let Some(ref channel_id) = filter.channel_id {
                    if s.channel_id != *channel_id {
                        return false;
                    }
                }
                if let Some(ref user_id) = filter.user_id {
                    if s.user_id != *user_id {
                        return false;
                    }
                }
                if let Some(ref chat_id) = filter.chat_id {
                    if s.chat_id != *chat_id {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        // Apply pagination
        let start = filter.offset.min(result.len());
        let end = (filter.offset + filter.limit).min(result.len());
        result = result[start..end].to_vec();

        Ok(result)
    }

    // Messages
    async fn get_message(&self, message_id: &str) -> Result<Option<Message>> {
        let messages = self.messages.read().await;
        Ok(messages.get(message_id).cloned())
    }

    async fn save_message(&self, message: &Message) -> Result<()> {
        let mut messages = self.messages.write().await;
        messages.insert(message.id.clone(), message.clone());
        Ok(())
    }

    async fn list_messages(
        &self,
        _session_id: Option<SessionId>,
        _channel_id: Option<ChannelId>,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<Message>> {
        let messages = self.messages.read().await;
        let mut result: Vec<Message> = messages.values().cloned().collect();

        // Sort by created_at descending
        result.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        // Apply pagination
        let start = offset.min(result.len());
        let end = (offset + limit).min(result.len());
        result = result[start..end].to_vec();

        Ok(result)
    }

    // Users
    async fn get_user(&self, channel_id: &str, user_id: &str) -> Result<Option<User>> {
        let users = self.users.read().await;
        Ok(users.get(&(channel_id.to_string(), user_id.to_string())).cloned())
    }

    async fn save_user(&self, user: &User) -> Result<()> {
        let mut users = self.users.write().await;
        users.insert(
            (user.channel_id.clone(), user.user_id.clone()),
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
            user_id: user_id.to_string(),
            display_name: display_name.to_string(),
            metadata: serde_json::Value::Null,
            created_at: chrono::Utc::now(),
        };

        self.save_user(&user).await?;
        Ok(user)
    }

    // Config
    async fn get_config_value(&self, key: &str) -> Result<Option<String>> {
        let config = self.config.read().await;
        Ok(config.get(key).cloned())
    }

    async fn set_config_value(&self, key: &str, value: &str) -> Result<()> {
        let mut config = self.config.write().await;
        config.insert(key.to_string(), value.to_string());
        Ok(())
    }

    async fn delete_config_value(&self, key: &str) -> Result<()> {
        let mut config = self.config.write().await;
        config.remove(key);
        Ok(())
    }

    // Health & Maintenance
    async fn health_check(&self) -> Result<()> {
        // Memory storage is always healthy
        Ok(())
    }

    async fn migrate(&self) -> Result<()> {
        // No migrations needed for memory storage
        Ok(())
    }

    async fn close(&self) -> Result<()> {
        // Nothing to close for memory storage
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openclaw_core::{SessionConfig, SessionState};

    #[tokio::test]
    async fn test_memory_storage_session() {
        let storage = MemoryStorage::new();

        let session = Session {
            id: SessionId::new(),
            channel_id: ChannelId::new(),
            user_id: "user123".to_string(),
            chat_id: "chat456".to_string(),
            config: SessionConfig::default(),
            state: SessionState::Active,
            context: openclaw_core::SessionContext::default(),
        };

        // Save session
        storage.save_session(&session).await.unwrap();

        // Get session
        let retrieved = storage.get_session(session.id.clone()).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().user_id, "user123");

        // List sessions
        let filter = SessionFilter::new().with_user("user123");
        let sessions = storage.list_sessions(filter).await.unwrap();
        assert_eq!(sessions.len(), 1);

        // Delete session
        storage.delete_session(session.id.clone()).await.unwrap();
        let retrieved = storage.get_session(session.id.clone()).await.unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_memory_storage_user() {
        let storage = MemoryStorage::new();

        // Create user
        let user = storage
            .get_or_create_user("telegram", "12345", "Test User")
            .await
            .unwrap();
        assert_eq!(user.display_name, "Test User");

        // Get existing user
        let user2 = storage
            .get_or_create_user("telegram", "12345", "Different Name")
            .await
            .unwrap();
        assert_eq!(user2.display_name, "Test User"); // Should return original name

        // Get user directly
        let user3 = storage.get_user("telegram", "12345").await.unwrap();
        assert!(user3.is_some());
    }
}
