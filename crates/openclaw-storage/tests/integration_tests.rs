//! Integration tests for storage backends

use openclaw_core::{ChannelId, Message, MessageContent, MessageId, Session, SessionId, User, UserId};
use openclaw_storage::{create_storage, backends::MemoryStorage, SessionFilter, Storage, StorageBackend, StorageConfig};

/// Helper function to create a test session
fn create_test_session(channel_id: ChannelId) -> Session {
    let mut s = Session::new(channel_id, Some("test_user".to_string()));
    s.chat_id = "test_chat".to_string();
    s
}

/// Helper function to create a test message
fn create_test_message(channel_id: ChannelId, text: &str) -> Message {
    Message {
        id: MessageId::new(),
        session_id: None,
        channel_id,
        chat_id: "test_chat".to_string(),
        user_id: Some("test_user".to_string()),
        sender: UserId::new(),
        recipient: UserId::new(),
        content: MessageContent::Text { text: text.to_string() },
        direction: openclaw_core::Direction::Inbound,
        created_at: chrono::Utc::now(),
        metadata: serde_json::json!({}),
    }
}

/// Helper function to create a test user
fn create_test_user(channel_id: &str, user_id: &str) -> User {
    User::new(channel_id, user_id, format!("Test User {}", user_id))
}

#[tokio::test]
async fn test_memory_storage_session_crud() {
    let storage = MemoryStorage::new();
    let channel_id = ChannelId::new();

    // Create
    let session = create_test_session(channel_id);
    storage.save_session(&session).await.unwrap();

    // Read
    let retrieved = storage.get_session(session.id).await.unwrap();
    assert!(retrieved.is_some());
    let retrieved = retrieved.unwrap();
    assert_eq!(retrieved.id, session.id);
    assert_eq!(retrieved.channel_id, channel_id);

    // Update
    let mut updated_session = session.clone();
    updated_session.update_activity();
    storage.save_session(&updated_session).await.unwrap();

    let retrieved = storage.get_session(session.id).await.unwrap().unwrap();
    assert!(retrieved.last_activity_at > session.last_activity_at);

    // Delete
    storage.delete_session(session.id).await.unwrap();
    let deleted = storage.get_session(session.id).await.unwrap();
    assert!(deleted.is_none());
}

#[tokio::test]
async fn test_memory_storage_session_filter() {
    let storage = MemoryStorage::new();
    let channel_id = ChannelId::new();
    let channel_id_2 = ChannelId::new();

    // Create sessions in different channels
    let session1 = create_test_session(channel_id);
    let session2 = create_test_session(channel_id);
    let session3 = create_test_session(channel_id_2);

    storage.save_session(&session1).await.unwrap();
    storage.save_session(&session2).await.unwrap();
    storage.save_session(&session3).await.unwrap();

    // Filter by channel
    let filter = SessionFilter::new().with_channel(channel_id).limit(10);
    let results = storage.list_sessions(filter).await.unwrap();
    assert_eq!(results.len(), 2);

    // Filter by channel 2
    let filter = SessionFilter::new().with_channel(channel_id_2).limit(10);
    let results = storage.list_sessions(filter).await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, session3.id);
}

#[tokio::test]
async fn test_memory_storage_message_crud() {
    let storage = MemoryStorage::new();
    let channel_id = ChannelId::new();

    let message = create_test_message(channel_id, "Hello, world!");
    storage.save_message(&message).await.unwrap();

    let retrieved = storage.get_message(&message.id.to_string()).await.unwrap();
    assert!(retrieved.is_some());

    let retrieved = retrieved.unwrap();
    assert_eq!(retrieved.id, message.id);
    match retrieved.content {
        MessageContent::Text { text } => assert_eq!(text, "Hello, world!"),
        _ => panic!("Expected text message"),
    }
}

#[tokio::test]
async fn test_memory_storage_user_crud() {
    let storage = MemoryStorage::new();

    let user = create_test_user("telegram", "user123");
    storage.save_user(&user).await.unwrap();

    let retrieved = storage.get_user("telegram", "user123").await.unwrap();
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().display_name, "Test User user123");
}

#[tokio::test]
async fn test_memory_storage_config() {
    let storage = MemoryStorage::new();

    // Set config
    storage.set_config_value("gateway.port", "18789").await.unwrap();
    storage.set_config_value("agents.model", "gpt-4").await.unwrap();

    // Get config
    let port = storage.get_config_value("gateway.port").await.unwrap();
    assert_eq!(port, Some("18789".to_string()));

    // Delete config
    storage.delete_config_value("gateway.port").await.unwrap();
    let port = storage.get_config_value("gateway.port").await.unwrap();
    assert_eq!(port, None);
}

#[tokio::test]
async fn test_memory_storage_health_check() {
    let storage = MemoryStorage::new();
    storage.health_check().await.unwrap();
}

#[tokio::test]
async fn test_sqlite_storage_session_crud() {
    // Create temp database
    let temp_dir = tempfile::tempdir().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let connection_string = format!("sqlite://{}", db_path.to_str().unwrap());

    let config = StorageConfig {
        backend: StorageBackend::Sqlite,
        connection_string,
        max_connections: 5,
        timeout_seconds: 30,
    };

    let storage = create_storage(&config).await.unwrap();
    let channel_id = ChannelId::new();

    // Create
    let session = create_test_session(channel_id);
    storage.save_session(&session).await.unwrap();

    // Read
    let retrieved = storage.get_session(session.id).await.unwrap();
    assert!(retrieved.is_some());
    let retrieved = retrieved.unwrap();
    assert_eq!(retrieved.id, session.id);

    // Cleanup
    storage.close().await.unwrap();
}

#[tokio::test]
async fn test_sqlite_storage_message_persistence() {
    let temp_dir = tempfile::tempdir().unwrap();
    let db_path = temp_dir.path().join("test_messages.db");
    let connection_string = format!("sqlite://{}", db_path.to_str().unwrap());

    let config = StorageConfig {
        backend: StorageBackend::Sqlite,
        connection_string: connection_string.clone(),
        max_connections: 5,
        timeout_seconds: 30,
    };

    let channel_id = ChannelId::new();
    let mut message_id: MessageId = MessageId::new();

    // Save messages
    {
        let storage = create_storage(&config).await.unwrap();

        for i in 0..5 {
            let msg = create_test_message(channel_id, &format!("Message {}", i));
            if i == 0 {
                message_id = msg.id;
            }
            storage.save_message(&msg).await.unwrap();
        }

        storage.close().await.unwrap();
    }

    // Reconnect and verify persistence
    {
        let storage = create_storage(&config).await.unwrap();

        let retrieved = storage.get_message(&message_id.to_string()).await.unwrap();
        assert!(retrieved.is_some());

        let messages = storage.list_messages(None, Some(channel_id), 10, 0).await.unwrap();
        assert_eq!(messages.len(), 5);

        storage.close().await.unwrap();
    }
}

#[tokio::test]
async fn test_storage_factory_memory() {
    let config = StorageConfig {
        backend: StorageBackend::Memory,
        connection_string: "".to_string(),
        max_connections: 1,
        timeout_seconds: 1,
    };

    let storage = create_storage(&config).await.unwrap();

    let channel_id = ChannelId::new();
    let session = create_test_session(channel_id);
    storage.save_session(&session).await.unwrap();

    let retrieved = storage.get_session(session.id).await.unwrap();
    assert!(retrieved.is_some());

    storage.close().await.unwrap();
}

#[tokio::test]
async fn test_concurrent_session_access() {
    use std::sync::Arc;

    let storage = Arc::new(MemoryStorage::new());
    let channel_id = ChannelId::new();

    let mut handles = vec![];

    // Spawn multiple tasks that save sessions
    for i in 0..10 {
        let storage_clone = Arc::clone(&storage);
        let channel_id_clone = channel_id;

        let handle = tokio::spawn(async move {
            let session = create_test_session(channel_id_clone);
            storage_clone.save_session(&session).await.unwrap();
            session.id
        });

        handles.push(handle);
    }

    // Wait for all tasks
    let ids = futures::future::join_all(handles).await;

    // Verify all sessions were saved
    for id in ids {
        let id = id.unwrap();
        let session = storage.get_session(id).await.unwrap();
        assert!(session.is_some());
    }

    // Verify total count
    let all_sessions = storage.list_sessions(SessionFilter::new().limit(100)).await.unwrap();
    assert_eq!(all_sessions.len(), 10);
}
