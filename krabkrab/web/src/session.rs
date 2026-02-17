use crate::reconnect::new_connection_id;

#[derive(Debug, Clone)]
pub struct WaSocket {
    pub connection_id: String,
    pub auth_dir: String,
    pub verbose: bool,
}

pub fn create_wa_socket(print_qr: bool, verbose: bool, auth_dir: Option<&str>) -> WaSocket {
    if print_qr {
        println!("[web] QR login requested (stub)");
    }

    WaSocket {
        connection_id: new_connection_id(),
        auth_dir: auth_dir.unwrap_or(".krabkrab/credentials/web").to_string(),
        verbose,
    }
}

pub fn wait_for_wa_connection(_sock: &WaSocket) {
    // stub: in real implementation this waits for websocket events.
}

pub fn format_error(err: &str) -> String {
    err.trim().to_string()
}
