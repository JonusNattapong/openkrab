import AVFoundation
import Foundation
import Combine
import Speech
import UserNotifications

@MainActor
final class TCCPermissionManager: ObservableObject {
    @Published var microphoneAuthorized = false
    @Published var speechAuthorized = false
    @Published var notificationsAuthorized = false

    var hasRequiredVoicePermissions: Bool {
        microphoneAuthorized && speechAuthorized
    }

    func refresh() async {
        microphoneAuthorized = AVCaptureDevice.authorizationStatus(for: .audio) == .authorized
        speechAuthorized = SFSpeechRecognizer.authorizationStatus() == .authorized

        let center = UNUserNotificationCenter.current()
        let settings = await withCheckedContinuation { continuation in
            center.getNotificationSettings { value in
                continuation.resume(returning: value)
            }
        }
        notificationsAuthorized = settings.authorizationStatus == .authorized
    }

    func requestAll() async {
        _ = await requestMicrophone()
        _ = await requestSpeechRecognition()
        _ = await requestNotifications()
        await refresh()
    }

    private func requestMicrophone() async -> Bool {
        let current = AVCaptureDevice.authorizationStatus(for: .audio)
        if current == .authorized {
            return true
        }
        return await withCheckedContinuation { continuation in
            AVCaptureDevice.requestAccess(for: .audio) { granted in
                continuation.resume(returning: granted)
            }
        }
    }

    private func requestSpeechRecognition() async -> Bool {
        let current = SFSpeechRecognizer.authorizationStatus()
        if current == .authorized {
            return true
        }
        return await withCheckedContinuation { continuation in
            SFSpeechRecognizer.requestAuthorization { status in
                continuation.resume(returning: status == .authorized)
            }
        }
    }

    private func requestNotifications() async -> Bool {
        do {
            return try await UNUserNotificationCenter.current()
                .requestAuthorization(options: [.alert, .sound, .badge])
        } catch {
            return false
        }
    }
}
