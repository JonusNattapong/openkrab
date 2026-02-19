#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandAuthorizer {
    pub configured: bool,
    pub allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandGatingModeWhenAccessGroupsOff {
    Allow,
    Deny,
    Configured,
}

pub fn resolve_command_authorized_from_authorizers(
    use_access_groups: bool,
    authorizers: &[CommandAuthorizer],
    mode_when_off: Option<CommandGatingModeWhenAccessGroupsOff>,
) -> bool {
    let mode = mode_when_off.unwrap_or(CommandGatingModeWhenAccessGroupsOff::Allow);
    if !use_access_groups {
        match mode {
            CommandGatingModeWhenAccessGroupsOff::Allow => return true,
            CommandGatingModeWhenAccessGroupsOff::Deny => return false,
            CommandGatingModeWhenAccessGroupsOff::Configured => {
                let any_configured = authorizers.iter().any(|a| a.configured);
                if !any_configured {
                    return true;
                }
                return authorizers.iter().any(|a| a.configured && a.allowed);
            }
        }
    }
    authorizers.iter().any(|a| a.configured && a.allowed)
}

pub struct ControlCommandGateResult {
    pub command_authorized: bool,
    pub should_block: bool,
}

pub fn resolve_control_command_gate(
    use_access_groups: bool,
    authorizers: &[CommandAuthorizer],
    allow_text_commands: bool,
    has_control_command: bool,
    mode_when_off: Option<CommandGatingModeWhenAccessGroupsOff>,
) -> ControlCommandGateResult {
    let command_authorized =
        resolve_command_authorized_from_authorizers(use_access_groups, authorizers, mode_when_off);
    let should_block = allow_text_commands && has_control_command && !command_authorized;
    ControlCommandGateResult {
        command_authorized,
        should_block,
    }
}
