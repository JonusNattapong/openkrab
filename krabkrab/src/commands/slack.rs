use crate::connectors;

pub fn slack_send_command(text: &str) -> String {
    connectors::slack::format_outbound(text)
}

