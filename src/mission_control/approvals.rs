//! Approval workflows for Mission Control

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub id: String,
    pub task_id: String,
    pub requested_by: String,
    pub action: String,
    pub reason: Option<String>,
    pub status: ApprovalStatus,
    pub approver: Option<String>,
    pub created_at: i64,
    pub resolved_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
}

impl Default for ApprovalStatus {
    fn default() -> Self {
        Self::Pending
    }
}
