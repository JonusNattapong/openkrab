#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayStatus {
    pub healthy: bool,
    pub endpoint: String,
}

pub fn gateway_status() -> GatewayStatus {
    GatewayStatus {
        healthy: true,
        endpoint: "http://127.0.0.1:3000".to_string(),
    }
}

