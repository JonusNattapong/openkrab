pub type LogFn = Box<dyn Fn(&str) + Send + Sync>;

pub fn log_inbound_drop(params: &InboundDropParams) {
    let target = params.target.as_ref().map(|t| format!(" target={}", t)).unwrap_or_default();
    (params.log)(&format!("{}: drop {}{}", params.channel, params.reason, target));
}

pub fn log_typing_failure(params: &TypingFailureParams) {
    let target = params.target.as_ref().map(|t| format!(" target={}", t)).unwrap_or_default();
    let action = params.action.as_ref().map(|a| format!(" action={}", a)).unwrap_or_default();
    (params.log)(&format!("{} typing{} failed{}: {}", params.channel, action, target, format_error(&params.error)));
}

pub fn log_ack_failure(params: &AckFailureParams) {
    let target = params.target.as_ref().map(|t| format!(" target={}", t)).unwrap_or_default();
    (params.log)(&format!("{} ack cleanup failed{}: {}", params.channel, target, format_error(&params.error)));
}

fn format_error(e: &serde_json::Value) -> String {
    if e.is_string() {
        e.as_str().unwrap().to_string()
    } else {
        serde_json::to_string(e).unwrap_or_else(|_| "<error>".to_string())
    }
}

pub struct InboundDropParams {
    pub log: LogFn,
    pub channel: String,
    pub reason: String,
    pub target: Option<String>,
}

pub struct TypingFailureParams {
    pub log: LogFn,
    pub channel: String,
    pub target: Option<String>,
    pub action: Option<String>,
    pub error: serde_json::Value,
}

pub struct AckFailureParams {
    pub log: LogFn,
    pub channel: String,
    pub target: Option<String>,
    pub error: serde_json::Value,
}
