import AVFoundation
import Foundation
import Speech

final class VoiceWakeEngine {
    private let audioEngine = AVAudioEngine()
    private let recognizer = SFSpeechRecognizer(locale: Locale(identifier: "en-US"))
    private var recognitionRequest: SFSpeechAudioBufferRecognitionRequest?
    private var recognitionTask: SFSpeechRecognitionTask?

    private var wakePhrase = ""
    private var onTranscript: ((String) -> Void)?
    private var onWake: ((String) -> Void)?

    private var wakeCooldownUntil = Date.distantPast

    func start(
        wakePhrase: String,
        onTranscript: @escaping (String) -> Void,
        onWake: @escaping (String) -> Void
    ) throws {
        stop()

        guard let recognizer, recognizer.isAvailable else {
            throw NSError(domain: "VoiceWakeEngine", code: 1001, userInfo: [
                NSLocalizedDescriptionKey: "Speech recognizer is unavailable"
            ])
        }

        self.wakePhrase = normalize(wakePhrase)
        self.onTranscript = onTranscript
        self.onWake = onWake

        let request = SFSpeechAudioBufferRecognitionRequest()
        request.shouldReportPartialResults = true
        recognitionRequest = request

        let inputNode = audioEngine.inputNode
        let format = inputNode.outputFormat(forBus: 0)
        inputNode.removeTap(onBus: 0)
        inputNode.installTap(onBus: 0, bufferSize: 1024, format: format) { [weak self] buffer, _ in
            self?.recognitionRequest?.append(buffer)
        }

        audioEngine.prepare()
        try audioEngine.start()

        recognitionTask = recognizer.recognitionTask(with: request) { [weak self] result, error in
            guard let self else { return }
            if let result {
                let text = result.bestTranscription.formattedString
                self.onTranscript?(text)
                self.handleWakeDetection(transcript: text)
            }

            if error != nil {
                self.stop()
            }
        }
    }

    func stop() {
        recognitionTask?.cancel()
        recognitionTask = nil

        recognitionRequest?.endAudio()
        recognitionRequest = nil

        audioEngine.inputNode.removeTap(onBus: 0)
        if audioEngine.isRunning {
            audioEngine.stop()
        }
    }

    private func handleWakeDetection(transcript: String) {
        let now = Date()
        guard now >= wakeCooldownUntil else {
            return
        }

        let normalizedTranscript = normalize(transcript)
        guard !wakePhrase.isEmpty, normalizedTranscript.contains(wakePhrase) else {
            return
        }

        wakeCooldownUntil = now.addingTimeInterval(2.5)
        onWake?(transcript)
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
