use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlackConnector;

impl SlackConnector {
    pub fn new() -> Self {
        SlackConnector
    }

    /// Parse a Slack user mention like `<@U12345>` and return the user id `U12345`.
    pub fn parse_user_mention(s: &str) -> Option<String> {
        // Slack mentions are often like <@U12345> or <@U12345|name>
        let re = Regex::new(r"^<@([A-Z0-9]+)(?:\|[^>]+)?>$").unwrap();
        re.captures(s)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
    }

    /// Extract first user mention in a message text, e.g. "hello <@U1> and <@U2>" -> "U1"
    pub fn extract_first_mention(text: &str) -> Option<String> {
        let re = Regex::new(r"<@([A-Z0-9]+)(?:\|[^>]+)?>").unwrap();
        re.captures(text)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_user_mention_simple() {
        assert_eq!(
            SlackConnector::parse_user_mention("<@U12345>"),
            Some("U12345".to_string())
        );
        assert_eq!(
            SlackConnector::parse_user_mention("<@U12345|alice>"),
            Some("U12345".to_string())
        );
        assert_eq!(SlackConnector::parse_user_mention("not a mention"), None);
    }

    #[test]
    fn test_extract_first_mention() {
        let txt = "hello <@U1> and <@U2|bob>";
        assert_eq!(
            SlackConnector::extract_first_mention(txt),
            Some("U1".to_string())
        );
    }
}
use crate::common::{Message, UserId};

pub fn normalize_inbound(text: &str, channel: &str, user: &str) -> Message {
    Message {
        id: format!("slack:{channel}:{user}"),
        text: text.to_string(),
        from: Some(UserId(format!("slack:{user}"))),
    }
}

pub fn format_outbound(text: &str) -> String {
    format!("[slack] {text}")
}
