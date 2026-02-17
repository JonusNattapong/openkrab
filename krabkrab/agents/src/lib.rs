use krabkrab_common::{Message, Result};

pub fn init_agents() {
    println!("[agents] initialized (stub)");
}

pub fn send_message(to: &str, body: &str) -> Result<()> {
    let msg = Message {
        to: to.to_string(),
        body: body.to_string(),
    };
    println!("[agents] send_message -> to='{}' body='{}'", msg.to, msg.body);
    Ok(())
}
