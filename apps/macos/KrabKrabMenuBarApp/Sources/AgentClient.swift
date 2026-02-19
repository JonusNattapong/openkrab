import Foundation

struct AgentClient {
    private let executablePath: String

    init(executablePath: String = ProcessInfo.processInfo.environment["KRABKRAB_CLI_PATH"] ?? "krabkrab") {
        self.executablePath = executablePath
    }

    func ask(_ query: String) async throws -> String {
        let trimmed = query.trimmingCharacters(in: .whitespacesAndNewlines)
        if trimmed.isEmpty {
            return ""
        }

        return try await withCheckedThrowingContinuation { continuation in
            let process = Process()
            process.executableURL = URL(fileURLWithPath: "/usr/bin/env")
            process.arguments = [executablePath, "ask", trimmed]

            let stdout = Pipe()
            let stderr = Pipe()
            process.standardOutput = stdout
            process.standardError = stderr

            process.terminationHandler = { proc in
                let outData = stdout.fileHandleForReading.readDataToEndOfFile()
                let errData = stderr.fileHandleForReading.readDataToEndOfFile()
                let out = String(data: outData, encoding: .utf8)?.trimmingCharacters(in: .whitespacesAndNewlines) ?? ""
                let err = String(data: errData, encoding: .utf8)?.trimmingCharacters(in: .whitespacesAndNewlines) ?? ""

                if proc.terminationStatus == 0 {
                    continuation.resume(returning: out)
                } else {
                    continuation.resume(throwing: NSError(
                        domain: "AgentClient",
                        code: Int(proc.terminationStatus),
                        userInfo: [NSLocalizedDescriptionKey: err.isEmpty ? "krabkrab ask failed" : err]
                    ))
                }
            }

            do {
                try process.run()
            } catch {
                continuation.resume(throwing: error)
            }
        }
    }
}
