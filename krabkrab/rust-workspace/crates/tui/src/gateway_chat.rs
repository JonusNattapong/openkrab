use crate::{GatewayConnectionOptions, GatewayConfig, ResolvedConnection};
use std::sync::{Arc, Mutex};

pub struct GatewayChatClient {
    pub connection: ResolvedConnection,
    ready: Arc<Mutex<bool>>,
}

impl GatewayChatClient {
    pub fn new<F1, F2, F3>(
        opts: GatewayConnectionOptions,
        config: GatewayConfig,
        resolve_gateway_port: F1,
        pick_primary_tailnet_ipv4: F2,
        pick_primary_lan_ipv4: F3,
    ) -> Result<Self, String>
    where
        F1: Fn() -> u16,
        F2: Fn() -> Option<String>,
        F3: Fn() -> Option<String>,
    {
        let conn = super::resolve_gateway_connection(opts, config, resolve_gateway_port, pick_primary_tailnet_ipv4, pick_primary_lan_ipv4)?;
        Ok(Self { connection: conn, ready: Arc::new(Mutex::new(false)) })
    }

    pub fn start(&self) {
        let mut r = self.ready.lock().unwrap();
        *r = true;
    }

    pub fn stop(&self) {
        let mut r = self.ready.lock().unwrap();
        *r = false;
    }

    pub fn is_ready(&self) -> bool {
        *self.ready.lock().unwrap()
    }

    pub fn send_chat(&self, session_key: &str, message: &str, run_id: Option<&str>) -> Result<String, String> {
        // In a real port this would call the GatewayClient request. For now return an id.
        Ok(run_id.map(|s| s.to_string()).unwrap_or_else(|| uuid::Uuid::new_v4().to_string()))
    }
}
