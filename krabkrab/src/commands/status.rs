use serde::Serialize;

#[derive(Serialize)]
pub struct StatusSummary {
    pub healthy: bool,
    pub services: usize,
}

pub fn get_status_summary() -> StatusSummary {
    StatusSummary { healthy: true, services: 3 }
}

pub fn status_command() -> String {
    let s = get_status_summary();
    if s.healthy {
        format!("OK — {} services running", s.services)
    } else {
        "DEGRADED".into()
    }
}
