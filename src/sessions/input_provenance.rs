//! Ported from `openclaw/src/sessions/input-provenance.ts`

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputProvenanceKind {
    ExternalUser,
    InterSession,
    InternalSystem,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputProvenance {
    pub kind: InputProvenanceKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_session_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_tool: Option<String>,
}

impl InputProvenance {
    pub fn new(kind: InputProvenanceKind) -> Self {
        Self {
            kind,
            source_session_key: None,
            source_channel: None,
            source_tool: None,
        }
    }

    pub fn with_source_session_key(mut self, key: impl Into<String>) -> Self {
        self.source_session_key = Some(key.into());
        self
    }

    pub fn with_source_channel(mut self, channel: impl Into<String>) -> Self {
        self.source_channel = Some(channel.into());
        self
    }

    pub fn with_source_tool(mut self, tool: impl Into<String>) -> Self {
        self.source_tool = Some(tool.into());
        self
    }

    pub fn is_inter_session(&self) -> bool {
        self.kind == InputProvenanceKind::InterSession
    }
}

pub fn has_inter_session_user_provenance(provenance: Option<&InputProvenance>) -> bool {
    provenance.map(|p| p.is_inter_session()).unwrap_or(false)
}
