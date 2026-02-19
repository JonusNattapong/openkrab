#[derive(Debug, Clone, Copy)]
pub struct MentionGateParams {
    pub require_mention: bool,
    pub can_detect_mention: bool,
    pub was_mentioned: bool,
    pub implicit_mention: Option<bool>,
    pub should_bypass_mention: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MentionGateResult {
    pub effective_was_mentioned: bool,
    pub should_skip: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct MentionGateWithBypassParams {
    pub is_group: bool,
    pub require_mention: bool,
    pub can_detect_mention: bool,
    pub was_mentioned: bool,
    pub implicit_mention: Option<bool>,
    pub has_any_mention: Option<bool>,
    pub allow_text_commands: bool,
    pub has_control_command: bool,
    pub command_authorized: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MentionGateWithBypassResult {
    pub effective_was_mentioned: bool,
    pub should_skip: bool,
    pub should_bypass_mention: bool,
}

pub fn resolve_mention_gating(params: MentionGateParams) -> MentionGateResult {
    let implicit = params.implicit_mention.unwrap_or(false);
    let bypass = params.should_bypass_mention.unwrap_or(false);
    let effective_was_mentioned = params.was_mentioned || implicit || bypass;
    let should_skip =
        params.require_mention && params.can_detect_mention && !effective_was_mentioned;
    MentionGateResult {
        effective_was_mentioned,
        should_skip,
    }
}

pub fn resolve_mention_gating_with_bypass(
    params: MentionGateWithBypassParams,
) -> MentionGateWithBypassResult {
    let should_bypass_mention = params.is_group
        && params.require_mention
        && !params.was_mentioned
        && !(params.has_any_mention.unwrap_or(false))
        && params.allow_text_commands
        && params.command_authorized
        && params.has_control_command;

    let base = resolve_mention_gating(MentionGateParams {
        require_mention: params.require_mention,
        can_detect_mention: params.can_detect_mention,
        was_mentioned: params.was_mentioned,
        implicit_mention: params.implicit_mention,
        should_bypass_mention: Some(should_bypass_mention),
    });

    MentionGateWithBypassResult {
        effective_was_mentioned: base.effective_was_mentioned,
        should_skip: base.should_skip,
        should_bypass_mention,
    }
}
