---
summary: "Enable and configure Native Desktop Notifications across macOS, Windows, and Linux"
read_when:
  - You want to receive system notifications from OpenKrab
  - You need to debug desktop notifications
title: "Desktop Notifications"
---

# Desktop Notifications

OpenKrab supports native desktop notifications across macOS, Windows, and Linux via the `notify-rust` crate. When enabled, actions or inbound events (like messages from a channel) can trigger popup notifications with sound directly to your desktop.

## Supported Platforms

- **macOS**: Prompts standard macOS Notification Center using `NSUserNotificationCenter` semantics.
- **Windows**: Supports Action Center notifications (requires `tauri-winrt-notification` behind the scenes if built on Windows).
- **Linux**: Supports standard D-Bus notification daemons (`notify-osd`, `dunst`, `gnome-shell`, `plasma-workspace`).

## Features

OpenKrab's notification system supports:

- Custom Titles and Bodies
- Sound playback mapped to native identifiers
- Event-triggered notifications (e.g. `system.notify`)

## Configuration

Notifications are enabled automatically when interacting with the dashboard or when your Agent wants to explicitly send a notification based on tools.

To verify notifications are working:

```bash
OpenKrab system notify --title "Test" --body "Hello from OpenKrab!"
```

## Sound Identifiers

When triggering sounds, different platforms support different sound IDs:

- **macOS**: `Ping`, `Basso`, `Blow`, `Bottle`, `Frog`, `Funk`, `Glass`, `Hero`, `Morse`, `Purr`, `Sosumi`, `Submarine`, `Tink`.
- **Windows**: `default`, `IM`, `Mail`, `Reminder`, `SMS`, `Alarm`.
- **Linux**: Usually depends on your freedesktop sound theme (e.g., `message-new-instant`, `bell`).

*(By default, OpenKrab attempts to fall back cleanly if a requested sound is missing.)*

