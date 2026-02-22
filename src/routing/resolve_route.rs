//! Ported from `openclaw/src/routing/resolve-route.ts`

use crate::channels::chat_type::{normalize_chat_type, ChatType};
use crate::openkrab_config::{AgentBinding, BindingMatch, OpenKrabConfig};
use crate::routing::session_key::{
    build_agent_main_session_key, build_agent_peer_session_key, normalize_account_id,
    normalize_agent_id, DmScope, PeerSessionKeyParams, DEFAULT_ACCOUNT_ID, DEFAULT_MAIN_KEY,
};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct RoutePeer {
    pub kind: String,
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct ResolveAgentRouteInput<'a> {
    pub cfg: &'a OpenKrabConfig,
    pub channel: &'a str,
    pub account_id: Option<&'a str>,
    pub peer: Option<RoutePeer>,
    pub parent_peer: Option<RoutePeer>,
    pub guild_id: Option<&'a str>,
    pub team_id: Option<&'a str>,
    pub member_role_ids: Option<&'a [String]>,
}

#[derive(Debug, Clone)]
pub struct ResolvedAgentRoute {
    pub agent_id: String,
    pub channel: String,
    pub account_id: String,
    pub session_key: String,
    pub main_session_key: String,
    pub matched_by: String,
}

fn normalize_token(value: Option<&str>) -> String {
    value.unwrap_or("").trim().to_lowercase()
}

fn normalize_id(value: Option<&str>) -> String {
    value.unwrap_or("").trim().to_string()
}

fn matches_account_id(match_val: Option<&str>, actual: &str) -> bool {
    let trimmed = match_val.unwrap_or("").trim();
    if trimmed.is_empty() {
        return actual == DEFAULT_ACCOUNT_ID;
    }
    if trimmed == "*" {
        return true;
    }
    trimmed == actual
}

fn pick_first_existing_agent_id(cfg: &OpenKrabConfig, agent_id: &str) -> String {
    let trimmed = agent_id.trim();
    if trimmed.is_empty() {
        return resolve_default_agent_id(cfg);
    }

    let _normalized = normalize_agent_id(Some(trimmed));

    if let Some(agents_cfg) = &cfg.agents {
        if let Some(ref _list) = agents_cfg.defaults {
            // Note: In openclaw, it walks the `cfg.agents.list`. However OpenKrabConfig
            // doesn't have a list of agents inside `agents` right now. It just has defaults.
            // So we'll skip the actual checking against list, and just return what was requested.
            // When openkrab's config supports agent lists, we can refine this.
        }
    }

    normalize_agent_id(Some(trimmed))
}

fn resolve_default_agent_id(_cfg: &OpenKrabConfig) -> String {
    "main".to_string()
}

fn matches_channel(match_val: &str, channel: &str) -> bool {
    let key = normalize_token(Some(match_val));
    if key.is_empty() {
        return false;
    }
    key == channel
}

#[derive(Debug, Clone, PartialEq)]
enum NormalizedPeerState {
    None,
    Invalid,
    Valid { kind: String, id: String },
}

#[derive(Debug, Clone)]
struct NormalizedBindingMatch {
    account_pattern: String,
    peer: NormalizedPeerState,
    guild_id: Option<String>,
    team_id: Option<String>,
    roles: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
struct EvaluatedBinding<'a> {
    binding: &'a AgentBinding,
    match_data: NormalizedBindingMatch,
}

struct BindingScope {
    peer: Option<RoutePeer>,
    guild_id: String,
    team_id: String,
    member_role_ids: HashSet<String>,
}

fn normalize_peer_constraint(
    peer_kind: Option<&str>,
    peer_id: Option<&str>,
) -> NormalizedPeerState {
    if peer_kind.is_none() && peer_id.is_none() {
        return NormalizedPeerState::None;
    }

    let kind = match normalize_chat_type(peer_kind) {
        Some(ChatType::Direct) => "direct",
        Some(ChatType::Group) => "group",
        Some(ChatType::Channel) => "channel",
        None => "",
    }
    .to_string();

    let id = normalize_id(peer_id);

    if kind.is_empty() || id.is_empty() {
        return NormalizedPeerState::Invalid;
    }

    NormalizedPeerState::Valid { kind, id }
}

fn normalize_binding_match(m: &BindingMatch) -> NormalizedBindingMatch {
    NormalizedBindingMatch {
        account_pattern: m.account_id.as_deref().unwrap_or("").trim().to_string(),
        peer: normalize_peer_constraint(
            m.peer.as_ref().map(|p| p.kind.as_str()),
            m.peer.as_ref().map(|p| p.id.as_str()),
        ),
        guild_id: m.guild_id.clone(),
        team_id: m.team_id.clone(),
        roles: m.roles.clone(),
    }
}

fn has_guild_constraint(m: &NormalizedBindingMatch) -> bool {
    m.guild_id.is_some()
}

fn has_team_constraint(m: &NormalizedBindingMatch) -> bool {
    m.team_id.is_some()
}

fn has_roles_constraint(m: &NormalizedBindingMatch) -> bool {
    if let Some(ref roles) = m.roles {
        !roles.is_empty()
    } else {
        false
    }
}

fn matches_binding_scope(m: &NormalizedBindingMatch, scope: &BindingScope) -> bool {
    match m.peer {
        NormalizedPeerState::Invalid => return false,
        NormalizedPeerState::Valid { ref kind, ref id } => {
            if let Some(ref p) = scope.peer {
                if &p.kind != kind || &p.id != id {
                    return false;
                }
            } else {
                return false;
            }
        }
        NormalizedPeerState::None => {}
    }

    if let Some(ref gid) = m.guild_id {
        if gid != &scope.guild_id {
            return false;
        }
    }

    if let Some(ref tid) = m.team_id {
        if tid != &scope.team_id {
            return false;
        }
    }

    if let Some(ref roles) = m.roles {
        for role in roles {
            if scope.member_role_ids.contains(role) {
                return true;
            }
        }
        return false;
    }

    true
}

pub fn resolve_agent_route(input: ResolveAgentRouteInput) -> ResolvedAgentRoute {
    let channel = normalize_token(Some(input.channel));
    let account_id = normalize_account_id(input.account_id);
    let peer = input.peer;
    let guild_id = normalize_id(input.guild_id);
    let team_id = normalize_id(input.team_id);

    let member_role_id_set: HashSet<String> = input
        .member_role_ids
        .unwrap_or(&[])
        .iter()
        .cloned()
        .collect();

    let mut evaluated: Vec<EvaluatedBinding> = Vec::new();
    for binding in &input.cfg.bindings {
        if !matches_channel(&binding.match_.channel, &channel) {
            continue;
        }
        if !matches_account_id(binding.match_.account_id.as_deref(), &account_id) {
            continue;
        }
        evaluated.push(EvaluatedBinding {
            binding,
            match_data: normalize_binding_match(&binding.match_),
        });
    }

    let identity_links = None;
    if let Some(_session_cfg) = &input.cfg.session {
        // Build identity links dictionary if we had the struct fields...
    }

    let choose = |agent_id: &str, matched_by: &str| -> ResolvedAgentRoute {
        let resolved_agent_id = pick_first_existing_agent_id(input.cfg, agent_id);

        // This is a simplified DmScope to keep compilation simple initially
        let session_key = build_agent_peer_session_key(PeerSessionKeyParams {
            agent_id: resolved_agent_id.clone(),
            main_key: Some(DEFAULT_MAIN_KEY.to_string()),
            channel: Some(channel.clone()),
            account_id: Some(account_id.clone()),
            peer_kind: peer.as_ref().map(|p| p.kind.clone()),
            peer_id: peer.as_ref().map(|p| p.id.clone()),
            identity_links: identity_links.clone(),
            dm_scope: Some(DmScope::Main), // Fallback
        })
        .to_lowercase();

        let main_session_key =
            build_agent_main_session_key(&resolved_agent_id, Some(DEFAULT_MAIN_KEY)).to_lowercase();

        ResolvedAgentRoute {
            agent_id: resolved_agent_id,
            channel: channel.clone(),
            account_id: account_id.clone(),
            session_key,
            main_session_key,
            matched_by: matched_by.to_string(),
        }
    };

    let parent_peer = input.parent_peer;

    macro_rules! check_tier {
        ($matched_by:expr, $enabled:expr, $scope_peer:expr, $predicate:expr) => {
            if $enabled {
                let scope = BindingScope {
                    peer: $scope_peer,
                    guild_id: guild_id.clone(),
                    team_id: team_id.clone(),
                    member_role_ids: member_role_id_set.clone(),
                };
                for candidate in &evaluated {
                    if $predicate(candidate) && matches_binding_scope(&candidate.match_data, &scope)
                    {
                        return choose(&candidate.binding.agent_id, $matched_by);
                    }
                }
            }
        };
    }

    check_tier!(
        "binding.peer",
        peer.is_some(),
        peer.clone(),
        |c: &EvaluatedBinding| matches!(c.match_data.peer, NormalizedPeerState::Valid { .. })
    );

    check_tier!(
        "binding.peer.parent",
        parent_peer.as_ref().map_or(false, |p| !p.id.is_empty()),
        parent_peer.clone(),
        |c: &EvaluatedBinding| matches!(c.match_data.peer, NormalizedPeerState::Valid { .. })
    );

    check_tier!(
        "binding.guild+roles",
        !guild_id.is_empty() && !member_role_id_set.is_empty(),
        peer.clone(),
        |c: &EvaluatedBinding| has_guild_constraint(&c.match_data)
            && has_roles_constraint(&c.match_data)
    );

    check_tier!(
        "binding.guild",
        !guild_id.is_empty(),
        peer.clone(),
        |c: &EvaluatedBinding| has_guild_constraint(&c.match_data)
            && !has_roles_constraint(&c.match_data)
    );

    check_tier!(
        "binding.team",
        !team_id.is_empty(),
        peer.clone(),
        |c: &EvaluatedBinding| has_team_constraint(&c.match_data)
    );

    check_tier!(
        "binding.account",
        true,
        peer.clone(),
        |c: &EvaluatedBinding| c.match_data.account_pattern != "*"
    );

    check_tier!(
        "binding.channel",
        true,
        peer.clone(),
        |c: &EvaluatedBinding| c.match_data.account_pattern == "*"
    );

    choose(&resolve_default_agent_id(input.cfg), "default")
}
