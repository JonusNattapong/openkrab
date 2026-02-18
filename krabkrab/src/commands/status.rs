use serde::Serialize;

#[derive(Serialize)]
pub struct StatusSummary {
    pub healthy: bool,
    pub services: usize,
    pub providers: Vec<String>,
}

pub fn get_status_summary() -> StatusSummary {
    let registry = crate::providers::default_registry_from_env();
    let providers = registry.list().into_iter().map(|s| s.to_string()).collect::<Vec<_>>();
    let services = providers.len();
    StatusSummary {
        healthy: true,
        services,
        providers,
    }
}

pub fn status_command() -> String {
    let s = get_status_summary();
    let provider_text = if s.providers.is_empty() {
        "none".to_string()
    } else {
        s.providers.join(", ")
    };
    if s.healthy {
        format!("OK — {} providers registered [{}]", s.services, provider_text)
    } else {
        "DEGRADED".into()
    }
}
