//! thread_ownership — Thread ownership tracking.
//! Ported from `openkrab/extensions/thread-ownership/` (Phase 12).
//!
//! Tracks which bot / agent instance "owns" a conversation thread,
//! preventing duplicate responses when multiple agents are active.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Types ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClaimResult {
    /// This agent now owns the thread.
    Claimed,
    /// Thread is already owned by this agent.
    AlreadyOwned,
    /// Thread is owned by a different agent.
    Contested { owner_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadOwner {
    pub thread_id: String,
    pub owner_id: String,
    pub claimed_at: i64,
    /// Optional TTL — unclaim after this many seconds if not refreshed.
    pub ttl_secs: Option<u64>,
    pub last_refreshed: i64,
}

impl ThreadOwner {
    pub fn new(
        thread_id: impl Into<String>,
        owner_id: impl Into<String>,
        ttl_secs: Option<u64>,
    ) -> Self {
        let now = now_secs();
        Self {
            thread_id: thread_id.into(),
            owner_id: owner_id.into(),
            claimed_at: now,
            ttl_secs,
            last_refreshed: now,
        }
    }

    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl_secs {
            let now = now_secs();
            (now - self.last_refreshed) as u64 > ttl
        } else {
            false
        }
    }

    pub fn refresh(&mut self) {
        self.last_refreshed = now_secs();
    }
}

fn now_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

// ─── Ownership registry ───────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct ThreadOwnershipRegistry {
    /// thread_id → ThreadOwner
    owners: HashMap<String, ThreadOwner>,
}

impl ThreadOwnershipRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Try to claim a thread for an agent.
    pub fn claim(&mut self, thread_id: &str, agent_id: &str, ttl_secs: Option<u64>) -> ClaimResult {
        // Evict expired entries first
        self.gc();

        match self.owners.get(thread_id) {
            None => {
                self.owners.insert(
                    thread_id.to_string(),
                    ThreadOwner::new(thread_id, agent_id, ttl_secs),
                );
                ClaimResult::Claimed
            }
            Some(owner) if owner.owner_id == agent_id => ClaimResult::AlreadyOwned,
            Some(owner) => ClaimResult::Contested {
                owner_id: owner.owner_id.clone(),
            },
        }
    }

    /// Release a thread claim (only if the caller owns it).
    pub fn release(&mut self, thread_id: &str, agent_id: &str) -> bool {
        if let Some(owner) = self.owners.get(thread_id) {
            if owner.owner_id == agent_id {
                self.owners.remove(thread_id);
                return true;
            }
        }
        false
    }

    /// Refresh the TTL of an owned thread.
    pub fn refresh(&mut self, thread_id: &str, agent_id: &str) -> bool {
        if let Some(owner) = self.owners.get_mut(thread_id) {
            if owner.owner_id == agent_id {
                owner.refresh();
                return true;
            }
        }
        false
    }

    /// Check ownership without claiming.
    pub fn get_owner(&self, thread_id: &str) -> Option<&ThreadOwner> {
        self.owners.get(thread_id).filter(|o| !o.is_expired())
    }

    pub fn is_owner(&self, thread_id: &str, agent_id: &str) -> bool {
        self.get_owner(thread_id)
            .map(|o| o.owner_id == agent_id)
            .unwrap_or(false)
    }

    /// Remove all expired entries.
    pub fn gc(&mut self) {
        self.owners.retain(|_, v| !v.is_expired());
    }

    pub fn len(&self) -> usize {
        self.owners.len()
    }
    pub fn is_empty(&self) -> bool {
        self.owners.is_empty()
    }

    /// Force-release a thread (admin override).
    pub fn force_release(&mut self, thread_id: &str) -> bool {
        self.owners.remove(thread_id).is_some()
    }
}

// ─── Helper: should_handle ────────────────────────────────────────────────────

/// Returns true if the agent should handle this message (either owns the thread or claims it).
pub fn should_handle(
    registry: &mut ThreadOwnershipRegistry,
    thread_id: &str,
    agent_id: &str,
    ttl_secs: Option<u64>,
) -> bool {
    match registry.claim(thread_id, agent_id, ttl_secs) {
        ClaimResult::Claimed | ClaimResult::AlreadyOwned => true,
        ClaimResult::Contested { .. } => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claim_and_own() {
        let mut reg = ThreadOwnershipRegistry::new();
        assert_eq!(reg.claim("thread1", "bot-a", None), ClaimResult::Claimed);
        assert_eq!(
            reg.claim("thread1", "bot-a", None),
            ClaimResult::AlreadyOwned
        );
    }

    #[test]
    fn contested_thread() {
        let mut reg = ThreadOwnershipRegistry::new();
        reg.claim("thread1", "bot-a", None);
        let result = reg.claim("thread1", "bot-b", None);
        assert!(matches!(result, ClaimResult::Contested { owner_id } if owner_id == "bot-a"));
    }

    #[test]
    fn release_and_reclaim() {
        let mut reg = ThreadOwnershipRegistry::new();
        reg.claim("t1", "bot-a", None);
        assert!(reg.release("t1", "bot-a"));
        assert_eq!(reg.claim("t1", "bot-b", None), ClaimResult::Claimed);
    }

    #[test]
    fn release_by_non_owner_fails() {
        let mut reg = ThreadOwnershipRegistry::new();
        reg.claim("t1", "bot-a", None);
        assert!(!reg.release("t1", "bot-b"));
    }

    #[test]
    fn is_owner_check() {
        let mut reg = ThreadOwnershipRegistry::new();
        reg.claim("t1", "bot-a", None);
        assert!(reg.is_owner("t1", "bot-a"));
        assert!(!reg.is_owner("t1", "bot-b"));
    }

    #[test]
    fn should_handle_logic() {
        let mut reg = ThreadOwnershipRegistry::new();
        assert!(should_handle(&mut reg, "t1", "bot-a", None));
        assert!(!should_handle(&mut reg, "t1", "bot-b", None));
        assert!(should_handle(&mut reg, "t1", "bot-a", None));
    }
}
