pub mod active_listener;
pub mod auth_store;
pub mod inbound;
pub mod login;
pub mod logout;
pub mod outbound;
pub mod reconnect;
pub mod session;

pub fn init_web() {
    println!("[web] initialized (ported scaffold)");
}
