import Foundation
import Combine

@MainActor
final class MacAssistantState: ObservableObject {
    @Published var talkModeEnabled = false
    @Published var statusText = "idle"
    @Published var lastTranscript = ""
    @Published var lastReply = ""
    @Published var wakePhrase = "hey krabkrab"

    let permissions = TCCPermissionManager()
    let notifications = NotificationManager()
    private let voiceWakeEngine = VoiceWakeEngine()
    private let speechOutput = SpeechOutput()
    private let agentClient = AgentClient()

    private var waitingForCommandUntil = Date.distantPast
    private var lastHandledCommand = ""
    private var isHandlingCommand = false

    func requestRequiredPermissions() async {
        statusText = "requesting permissions"
        await permissions.requestAll()

        if permissions.hasRequiredVoicePermissions {
            statusText = "permissions ready"
        } else {
            statusText = "permissions missing"
        }
    }

    func toggleTalkMode() {
        if talkModeEnabled {
            stopTalkMode()
        } else {
            Task { await startTalkMode() }
        }
    }

    private func startTalkMode() async {
        await permissions.refresh()
        guard permissions.hasRequiredVoicePermissions else {
            statusText = "talk mode blocked: grant mic and speech"
            return
        }

        do {
            try voiceWakeEngine.start(
                wakePhrase: wakePhrase,
                onTranscript: { [weak self] transcript in
                    guard let self else { return }
                    Task { @MainActor in
                        self.lastTranscript = transcript
                        if self.talkModeEnabled && !self.isHandlingCommand {
                            self.statusText = self.waitingForCommandUntil > Date() ? "waiting command" : "listening"
                        }
                        self.handleCommandCapture(transcript)
                    }
                },
                onWake: { [weak self] transcript in
                    guard let self else { return }
                    Task { @MainActor in
                        self.statusText = "wake detected"
                        self.lastTranscript = transcript
                        self.waitingForCommandUntil = Date().addingTimeInterval(7)
                        self.speechOutput.speak("I am listening")
                        await self.notifications.send(
                            title: "Wake phrase detected",
                            body: transcript
                        )

                        let inlineCommand = self.extractCommand(from: transcript)
                        if !inlineCommand.isEmpty {
                            self.submitCommand(inlineCommand)
                        }
                    }
                }
            )
            talkModeEnabled = true
            statusText = "listening"
        } catch {
            talkModeEnabled = false
            statusText = "voice engine error: \(error.localizedDescription)"
        }
    }

    private func stopTalkMode() {
        voiceWakeEngine.stop()
        waitingForCommandUntil = Date.distantPast
        isHandlingCommand = false
        talkModeEnabled = false
        statusText = "idle"
    }

    private func handleCommandCapture(_ transcript: String) {
        if !talkModeEnabled || isHandlingCommand || Date() > waitingForCommandUntil {
            return
        }
        let command = extractCommand(from: transcript)
        if command.isEmpty {
            return
        }
        submitCommand(command)
    }

    private func submitCommand(_ command: String) {
        let normalized = normalize(command)
        if normalized.isEmpty || normalized == lastHandledCommand {
            return
        }

        lastHandledCommand = normalized
        waitingForCommandUntil = Date.distantPast
        isHandlingCommand = true
        statusText = "thinking"

        Task {
            do {
                let reply = try await agentClient.ask(command)
                await MainActor.run {
                    self.lastReply = reply
                    self.statusText = "responding"
                    self.speechOutput.speak(reply)
                }
                await notifications.send(title: "KrabKrab Reply", body: reply)
                await MainActor.run {
                    self.statusText = self.talkModeEnabled ? "listening" : "idle"
                    self.isHandlingCommand = false
                }
            } catch {
                await MainActor.run {
                    self.lastReply = ""
                    self.statusText = "agent error: \(error.localizedDescription)"
                    self.speechOutput.speak("Sorry, I could not complete that request")
                    self.isHandlingCommand = false
                }
            }
        }
    }

    private func extractCommand(from transcript: String) -> String {
        let normalizedTranscript = normalize(transcript)
        let normalizedWake = normalize(wakePhrase)
        if normalizedWake.isEmpty {
            return ""
        }

        guard let range = normalizedTranscript.range(of: normalizedWake) else {
            return ""
        }

        let remainder = normalizedTranscript[range.upperBound...].trimmingCharacters(in: .whitespacesAndNewlines)
        return remainder
    }

    private func normalize(_ value: String) -> String {
        value
            .lowercased()
            .map { c in
                if c.isLetter || c.isNumber || c.isWhitespace {
                    return c
                }
                return " "
            }
            .reduce(into: "") { partialResult, c in
                partialResult.append(c)
            }
            .split(separator: " ")
            .joined(separator: " ")
    }
}
