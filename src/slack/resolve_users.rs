use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlackUserLookup {
    pub id: String,
    pub name: String,
    pub display_name: Option<String>,
    pub real_name: Option<String>,
    pub email: Option<String>,
    pub deleted: bool,
    pub is_bot: bool,
    pub is_app_user: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlackUserResolution {
    pub input: String,
    pub resolved: bool,
    pub id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub deleted: Option<bool>,
    pub is_bot: Option<bool>,
    pub note: Option<String>,
}

fn score_slack_user(user: &SlackUserLookup, name_match: &Option<String>, email_match: &Option<String>) -> i32 {
    let mut score = 0;
    if !user.deleted { score += 3; }
    if !user.is_bot && !user.is_app_user { score += 2; }
    if let Some(email) = email_match {
        if let Some(user_email) = &user.email {
            if user_email == email { score += 5; }
        }
    }
    if let Some(name) = name_match {
        let target = name.to_lowercase();
        let candidates = vec![&user.name, user.display_name.as_ref().unwrap_or(&"".to_string()), user.real_name.as_ref().unwrap_or(&"".to_string())];
        if candidates.iter().any(|v| v.to_lowercase() == target) { score += 2; }
    }
    score
}

pub fn resolve_slack_user_allowlist(entries: &[String], users: &[SlackUserLookup]) -> Vec<SlackUserResolution> {
    let mut out = Vec::new();
    for input in entries {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            out.push(SlackUserResolution { input: input.clone(), resolved: false, id: None, name: None, email: None, deleted: None, is_bot: None, note: None });
            continue;
        }
        // id mention
        if trimmed.starts_with("<@") && trimmed.ends_with('>') {
            let id = trimmed[2..trimmed.len()-1].to_uppercase();
            let match_u = users.iter().find(|u| u.id == id);
            out.push(SlackUserResolution { input: input.clone(), resolved: true, id: Some(id), name: match_u.map(|m| m.display_name.clone().or(m.real_name.clone()).unwrap_or(m.name.clone())), email: match_u.and_then(|m| m.email.clone()), deleted: match_u.map(|m| m.deleted), is_bot: match_u.map(|m| m.is_bot), note: None });
            continue;
        }
        if trimmed.contains('@') {
            let email = trimmed.to_lowercase();
            let matches: Vec<_> = users.iter().filter(|u| u.email.as_ref().map(|e| e == &email).unwrap_or(false)).cloned().collect();
            if !matches.is_empty() {
                let mut scored: Vec<_> = matches.iter().map(|u| (u, score_slack_user(u, &None, &Some(email.clone())))).collect();
                scored.sort_by(|a,b| b.1.cmp(&a.1));
                let best = scored.first().unwrap().0;
                out.push(SlackUserResolution { input: input.clone(), resolved: true, id: Some(best.id.clone()), name: Some(best.display_name.clone().unwrap_or(best.real_name.clone().unwrap_or(best.name.clone()))), email: best.email.clone(), deleted: Some(best.deleted), is_bot: Some(best.is_bot), note: if scored.len() > 1 { Some("multiple matches; chose best".to_string()) } else { None } });
                continue;
            }
        }
        // name match
        let name = trimmed.trim_start_matches('@').to_string();
        if !name.is_empty() {
            let target = name.to_lowercase();
            let matches: Vec<_> = users.iter().filter(|u| {
                let candidates = vec![u.name.to_lowercase(), u.display_name.clone().unwrap_or_default().to_lowercase(), u.real_name.clone().unwrap_or_default().to_lowercase()];
                candidates.contains(&target)
            }).cloned().collect();
            if !matches.is_empty() {
                let mut scored: Vec<_> = matches.iter().map(|u| (u, score_slack_user(u, &Some(name.clone()), &None))).collect();
                scored.sort_by(|a,b| b.1.cmp(&a.1));
                let best = scored.first().unwrap().0;
                out.push(SlackUserResolution { input: input.clone(), resolved: true, id: Some(best.id.clone()), name: Some(best.display_name.clone().unwrap_or(best.real_name.clone().unwrap_or(best.name.clone()))), email: best.email.clone(), deleted: Some(best.deleted), is_bot: Some(best.is_bot), note: if scored.len() > 1 { Some("multiple matches; chose best".to_string()) } else { None } });
                continue;
            }
        }
        out.push(SlackUserResolution { input: input.clone(), resolved: false, id: None, name: None, email: None, deleted: None, is_bot: None, note: None });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_users() -> Vec<SlackUserLookup> {
        vec![
            SlackUserLookup { id: "U1".to_string(), name: "alice".to_string(), display_name: Some("Alice".to_string()), real_name: Some("Alice A".to_string()), email: Some("alice@example.com".to_string()), deleted: false, is_bot: false, is_app_user: false },
            SlackUserLookup { id: "U2".to_string(), name: "bob".to_string(), display_name: None, real_name: Some("Bob".to_string()), email: Some("bob@example.com".to_string()), deleted: false, is_bot: false, is_app_user: false },
        ]
    }

    #[test]
    fn test_resolve_by_mention() {
        let users = sample_users();
        let entries = vec!["<@U1>".to_string()];
        let r = resolve_slack_user_allowlist(&entries, &users);
        assert!(r[0].resolved);
        assert_eq!(r[0].id.as_deref(), Some("U1"));
    }

    #[test]
    fn test_resolve_by_email() {
        let users = sample_users();
        let entries = vec!["alice@example.com".to_string()];
        let r = resolve_slack_user_allowlist(&entries, &users);
        assert!(r[0].resolved);
        assert_eq!(r[0].id.as_deref(), Some("U1"));
    }

    #[test]
    fn test_resolve_by_name() {
        let users = sample_users();
        let entries = vec!["@bob".to_string()];
        let r = resolve_slack_user_allowlist(&entries, &users);
        assert!(r[0].resolved);
        assert_eq!(r[0].id.as_deref(), Some("U2"));
    }
}
