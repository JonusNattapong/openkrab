#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DaemonStatus {
    pub running: bool,
    pub pid: Option<u32>,
}

pub fn daemon_status() -> DaemonStatus {
    DaemonStatus {
        running: false,
        pid: None,
    }
}
