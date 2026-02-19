use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct SenderLabelParams {
    pub name: Option<String>,
    pub username: Option<String>,
    pub tag: Option<String>,
    pub e164: Option<String>,
    pub id: Option<String>,
}

fn normalize(value: &Option<String>) -> Option<String> {
    value.as_deref().map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string())
}

pub fn resolve_sender_label(params: &SenderLabelParams) -> Option<String> {
    let name = normalize(&params.name);
    let username = normalize(&params.username);
    let tag = normalize(&params.tag);
    let e164 = normalize(&params.e164);
    let id = normalize(&params.id);

    let display = name.or(username).or(tag).unwrap_or_default();
    let id_part = e164.or(id).unwrap_or_default();
    if !display.is_empty() && !id_part.is_empty() && display != id_part {
        return Some(format!("{} ({})", display, id_part));
    }
    if !display.is_empty() {
        return Some(display);
    }
    if !id_part.is_empty() {
        return Some(id_part);
    }
    None
}

pub fn list_sender_label_candidates(params: &SenderLabelParams) -> Vec<String> {
    let mut candidates: HashSet<String> = HashSet::new();
    for v in [normalize(&params.name), normalize(&params.username), normalize(&params.tag), normalize(&params.e164), normalize(&params.id)] {
        if let Some(s) = v {
            candidates.insert(s);
        }
    }
    if let Some(resolved) = resolve_sender_label(params) {
        candidates.insert(resolved);
    }
    candidates.into_iter().collect()
}
