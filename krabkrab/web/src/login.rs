use crate::session::{create_wa_socket, wait_for_wa_connection};

pub fn login_web(verbose: bool, account_id: Option<&str>) {
    let auth_dir = account_id
        .map(|id| format!(".krabkrab/credentials/web/{id}"))
        .unwrap_or_else(|| ".krabkrab/credentials/web/default".to_string());

    let sock = create_wa_socket(true, verbose, Some(&auth_dir));
    wait_for_wa_connection(&sock);
    println!("[web] linked and saved credentials (stub)");
}
