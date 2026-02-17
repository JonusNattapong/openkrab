use openclaw_core::{ChannelId, Session, SessionId};
use std::collections::HashMap;

pub struct SessionManager {
    sessions: HashMap<SessionId, Session>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn add(&mut self, session_id: SessionId) {
        let session = Session::new(ChannelId::new(), None);
        self.sessions.insert(session_id, session);
    }

    pub fn remove(&mut self, session_id: &SessionId) {
        self.sessions.remove(session_id);
    }

    pub fn get(&self, session_id: &SessionId) -> Option<&Session> {
        self.sessions.get(session_id)
    }

    pub fn list(&self) -> Vec<SessionId> {
        self.sessions.keys().cloned().collect()
    }

    pub fn count(&self) -> usize {
        self.sessions.len()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
