//! notifications — Desktop notification support via notify-rust.
//! Used for sending system-level notifications to the active user (macOS/Windows/Linux).

use anyhow::Result;
use notify_rust::Notification;

/// Send a native system notification
pub fn send_notification(title: &str, body: &str) -> Result<()> {
    Notification::new()
        .summary(title)
        .body(body)
        .appname("OpenKrab")
        // .icon("krabkrab")
        .show()?;
    Ok(())
}

/// Send a native system notification with a sound
pub fn send_notification_with_sound(title: &str, body: &str, sound_name: &str) -> Result<()> {
    Notification::new()
        .summary(title)
        .body(body)
        .appname("OpenKrab")
        .sound_name(sound_name) // e.g. "Ping" or "Submarine" or "default"
        .show()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Don't run this randomly in CI/automated testing as it triggers a real UI notification
    fn test_notification() {
        assert!(send_notification("OpenKrab Test", "System notifications are working!").is_ok());
    }
}
