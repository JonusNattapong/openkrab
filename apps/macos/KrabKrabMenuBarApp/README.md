# KrabKrab macOS Menu Bar App (Swift/SwiftUI)

This app provides a native macOS host for:

- Menu bar controls (AppKit `NSStatusItem` + `NSMenu`)
- Voice Wake and Talk Mode (`AVFoundation` + `Speech`)
- TCC permission flow (microphone, speech recognition, notifications)
- Local notifications (`UserNotifications`)
- Live assistant round-trip (`krabkrab ask`) with spoken replies

## Frameworks used

- `AppKit`
- `SwiftUI`
- `AVFoundation`
- `Speech`
- `UserNotifications`

## Build

Open `apps/macos/KrabKrabMenuBarApp/Package.swift` with Xcode 15+ and run target `KrabKrabMenuBarApp`.

If `krabkrab` binary is not on PATH, set:

- `KRABKRAB_CLI_PATH=/absolute/path/to/krabkrab`

## Runtime permissions (TCC)

When building as a proper app bundle, include these usage descriptions in your app Info.plist:

- `NSMicrophoneUsageDescription`
- `NSSpeechRecognitionUsageDescription`

For notifications, app requests authorization at runtime.

## Voice Wake/Talk behavior

- Talk Mode starts live speech recognition.
- Wake phrase defaults to `hey krabkrab`.
- On wake phrase detection, app enters command-capture window.
- Spoken command is sent to `krabkrab ask`.
- Returned answer is spoken via `AVSpeechSynthesizer` and shown in notifications.
