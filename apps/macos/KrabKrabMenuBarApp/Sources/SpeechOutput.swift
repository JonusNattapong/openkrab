import AVFoundation
import Foundation

@MainActor
final class SpeechOutput: NSObject {
    private let synthesizer = AVSpeechSynthesizer()

    func speak(_ text: String) {
        let clean = text.trimmingCharacters(in: .whitespacesAndNewlines)
        if clean.isEmpty {
            return
        }
        if synthesizer.isSpeaking {
            synthesizer.stopSpeaking(at: .immediate)
        }

        let utterance = AVSpeechUtterance(string: clean)
        utterance.rate = 0.5
        utterance.pitchMultiplier = 1.0
        utterance.volume = 1.0
        synthesizer.speak(utterance)
    }
}
