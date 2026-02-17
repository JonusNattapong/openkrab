use crate::connectors;

pub fn telegram_send_command(text: &str) -> String {
    connectors::telegram::format_outbound(text)
}

