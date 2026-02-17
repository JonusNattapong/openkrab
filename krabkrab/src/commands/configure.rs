use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConfigureInput {
    pub profile: String,
    pub verbose: bool,
}

pub fn configure_command(input: ConfigureInput) -> String {
    format!(
        "configured profile={} verbose={}",
        input.profile,
        if input.verbose { "on" } else { "off" }
    )
}

