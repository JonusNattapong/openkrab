//! Ported from `openclaw/src/sessions/model-overrides.ts`

use super::Session;
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct ModelOverrideSelection {
    pub provider: String,
    pub model: String,
    pub is_default: bool,
}

pub struct ModelOverrideParams<'a> {
    pub entry: &'a mut Session,
    pub selection: ModelOverrideSelection,
    pub profile_override: Option<String>,
    pub profile_override_source: Option<String>,
}

pub fn apply_model_override_to_session_entry(params: ModelOverrideParams) -> bool {
    let mut updated = false;
    let fallback_source = "user".to_string();
    let source = params
        .profile_override_source
        .clone()
        .unwrap_or(fallback_source);
    let entry = params.entry;

    if params.selection.is_default {
        if entry.provider_override.is_some() {
            entry.provider_override = None;
            updated = true;
        }
        if entry.model_override.is_some() {
            entry.model_override = None;
            updated = true;
        }
    } else {
        if entry.provider_override.as_deref() != Some(&params.selection.provider) {
            entry.provider_override = Some(params.selection.provider.clone());
            updated = true;
        }
        if entry.model_override.as_deref() != Some(&params.selection.model) {
            entry.model_override = Some(params.selection.model.clone());
            updated = true;
        }
    }

    if let Some(prof) = &params.profile_override {
        if entry.auth_profile_override.as_deref() != Some(prof) {
            entry.auth_profile_override = Some(prof.clone());
            updated = true;
        }
        if entry.auth_profile_override_source.as_deref() != Some(&source) {
            entry.auth_profile_override_source = Some(source);
            updated = true;
        }
        if entry.auth_profile_override_compaction_count.is_some() {
            entry.auth_profile_override_compaction_count = None;
            updated = true;
        }
    } else {
        if entry.auth_profile_override.is_some() {
            entry.auth_profile_override = None;
            updated = true;
        }
        if entry.auth_profile_override_source.is_some() {
            entry.auth_profile_override_source = None;
            updated = true;
        }
        if entry.auth_profile_override_compaction_count.is_some() {
            entry.auth_profile_override_compaction_count = None;
            updated = true;
        }
    }

    // Clear stale fallback notice when the user explicitly switches models.
    if updated {
        entry.fallback_notice_selected_model = None;
        entry.fallback_notice_active_model = None;
        entry.fallback_notice_reason = None;
        entry.last_active = Utc::now();
    }

    updated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_model_override_to_session_entry() {
        let mut session = Session::new("test");
        session.fallback_notice_reason = Some("old reason".to_string());

        // Apply override
        let updated = apply_model_override_to_session_entry(ModelOverrideParams {
            entry: &mut session,
            selection: ModelOverrideSelection {
                provider: "openai".to_string(),
                model: "gpt-4".to_string(),
                is_default: false,
            },
            profile_override: None,
            profile_override_source: None,
        });

        assert!(updated);
        assert_eq!(session.provider_override.as_deref(), Some("openai"));
        assert_eq!(session.model_override.as_deref(), Some("gpt-4"));
        assert!(session.fallback_notice_reason.is_none());

        // Reset to default
        let updated = apply_model_override_to_session_entry(ModelOverrideParams {
            entry: &mut session,
            selection: ModelOverrideSelection {
                provider: "openai".to_string(),
                model: "gpt-4".to_string(),
                is_default: true,
            },
            profile_override: None,
            profile_override_source: None,
        });

        assert!(updated);
        assert!(session.provider_override.is_none());
        assert!(session.model_override.is_none());
    }
}
