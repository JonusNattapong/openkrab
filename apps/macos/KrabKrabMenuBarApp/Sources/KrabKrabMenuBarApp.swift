import AppKit
import SwiftUI

@main
struct KrabKrabMenuBarApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) private var appDelegate

    var body: some Scene {
        Settings {
            SettingsView(state: appDelegate.state)
                .frame(width: 420, height: 280)
        }
    }
}

final class AppDelegate: NSObject, NSApplicationDelegate {
    let state = MacAssistantState()
    private var menuBarController: MenuBarController?

    func applicationDidFinishLaunching(_ notification: Notification) {
        NSApp.setActivationPolicy(.accessory)
        menuBarController = MenuBarController(state: state)
        menuBarController?.install()
    }
}

struct SettingsView: View {
    @ObservedObject var state: MacAssistantState

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("KrabKrab macOS")
                .font(.title2)
                .bold()

            Toggle("Talk Mode", isOn: Binding(
                get: { state.talkModeEnabled },
                set: { newValue in
                    if newValue != state.talkModeEnabled {
                        state.toggleTalkMode()
                    }
                }
            ))

            HStack {
                Text("Status:")
                    .bold()
                Text(state.statusText)
            }

            HStack {
                Text("Last Transcript:")
                    .bold()
                Text(state.lastTranscript.isEmpty ? "-" : state.lastTranscript)
                    .lineLimit(2)
            }

            HStack {
                Text("Last Reply:")
                    .bold()
                Text(state.lastReply.isEmpty ? "-" : state.lastReply)
                    .lineLimit(2)
            }

            HStack {
                Text("Wake Phrase:")
                    .bold()
                TextField("hey krabkrab", text: $state.wakePhrase)
            }

            Divider()

            HStack {
                Text("Mic: \(state.permissions.microphoneAuthorized ? "granted" : "missing")")
                Text("Speech: \(state.permissions.speechAuthorized ? "granted" : "missing")")
                Text("Notify: \(state.permissions.notificationsAuthorized ? "granted" : "missing")")
            }
            .font(.system(size: 12))

            HStack {
                Button("Request Permissions") {
                    Task { await state.requestRequiredPermissions() }
                }

                Button("Trigger Test Notification") {
                    Task {
                        await state.notifications.send(
                            title: "KrabKrab",
                            body: "Test notification from macOS menu bar app"
                        )
                    }
                }
            }

            Spacer()
        }
        .padding(16)
    }
}
