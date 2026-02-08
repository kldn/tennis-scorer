import Foundation
#if canImport(Speech)
import Speech
import AVFoundation
#endif

// MARK: - State & Action enums

enum ScoringAction {
    case player1Point   // 辨識到「我」
    case player2Point   // 辨識到「對手」
    case undo           // 辨識到「取消」

    var displayText: String {
        switch self {
        case .player1Point: return "我"
        case .player2Point: return "對手"
        case .undo: return "取消"
        }
    }
}

enum SpeechState: Equatable {
    case idle
    case listening
    case processing
    case result(ScoringAction)
    case error

    static func == (lhs: SpeechState, rhs: SpeechState) -> Bool {
        switch (lhs, rhs) {
        case (.idle, .idle): return true
        case (.listening, .listening): return true
        case (.processing, .processing): return true
        case (.result(let a), .result(let b)):
            switch (a, b) {
            case (.player1Point, .player1Point): return true
            case (.player2Point, .player2Point): return true
            case (.undo, .undo): return true
            default: return false
            }
        case (.error, .error): return true
        default: return false
        }
    }
}

// MARK: - Keyword matching (extracted for testability)

enum KeywordMatcher {
    /// Match keywords from recognized text.
    /// Checks longer keywords ("對手", "取消") before "我" to avoid false positives.
    static func match(_ text: String) -> ScoringAction? {
        if text.contains("對手") {
            return .player2Point
        }
        if text.contains("取消") {
            return .undo
        }
        if text.contains("我") {
            return .player1Point
        }
        return nil
    }
}

#if canImport(Speech)
// MARK: - SpeechRecognizer

@MainActor
class SpeechRecognizer: ObservableObject {
    @Published private(set) var state: SpeechState = .idle
    @Published private(set) var permissionDenied: Bool = false
    @Published private(set) var isAvailable: Bool = false

    private var speechRecognizer: SFSpeechRecognizer?
    private var audioEngine: AVAudioEngine?
    private var recognitionRequest: SFSpeechAudioBufferRecognitionRequest?
    private var recognitionTask: SFSpeechRecognitionTask?
    private var timeoutTask: Task<Void, Never>?

    private static let timeoutDuration: UInt64 = 2_000_000_000 // 2 seconds in nanoseconds

    init() {
        speechRecognizer = SFSpeechRecognizer(locale: Locale(identifier: "zh-Hant"))
        updateAvailability()
    }

    // MARK: - Availability

    private func updateAvailability() {
        guard let recognizer = speechRecognizer else {
            isAvailable = false
            return
        }
        isAvailable = recognizer.isAvailable && recognizer.supportsOnDeviceRecognition
    }

    // MARK: - Permissions

    func checkPermissions() async -> Bool {
        let speechStatus = await withCheckedContinuation { continuation in
            SFSpeechRecognizer.requestAuthorization { status in
                continuation.resume(returning: status)
            }
        }

        guard speechStatus == .authorized else {
            permissionDenied = true
            return false
        }

        let audioSession = AVAudioSession.sharedInstance()
        let micStatus: Bool
        if #available(watchOS 10.0, *) {
            micStatus = await AVAudioApplication.requestRecordPermission()
        } else {
            micStatus = await withCheckedContinuation { continuation in
                audioSession.requestRecordPermission { granted in
                    continuation.resume(returning: granted)
                }
            }
        }

        guard micStatus else {
            permissionDenied = true
            return false
        }

        permissionDenied = false
        return true
    }

    // MARK: - Toggle

    func toggleListening() async {
        switch state {
        case .listening, .processing:
            stopListening()
        default:
            await startListening()
        }
    }

    // MARK: - Start Listening

    private func startListening() async {
        // Check permissions first
        let permitted = await checkPermissions()
        guard permitted else { return }

        guard let speechRecognizer = speechRecognizer,
              speechRecognizer.isAvailable else {
            state = .error
            scheduleReturnToIdle()
            return
        }

        let engine = AVAudioEngine()
        let request = SFSpeechAudioBufferRecognitionRequest()
        request.shouldReportPartialResults = true
        request.requiresOnDeviceRecognition = true

        // Configure audio session
        let audioSession = AVAudioSession.sharedInstance()
        do {
            try audioSession.setCategory(.record, mode: .measurement, policy: .default, options: .duckOthers)
            try audioSession.setActive(true, options: .notifyOthersOnDeactivation)
        } catch {
            state = .error
            scheduleReturnToIdle()
            return
        }

        // Install audio tap
        let inputNode = engine.inputNode
        let recordingFormat = inputNode.outputFormat(forBus: 0)
        inputNode.installTap(onBus: 0, bufferSize: 1024, format: recordingFormat) { buffer, _ in
            request.append(buffer)
        }

        // Start audio engine
        do {
            engine.prepare()
            try engine.start()
        } catch {
            inputNode.removeTap(onBus: 0)
            state = .error
            scheduleReturnToIdle()
            return
        }

        self.audioEngine = engine
        self.recognitionRequest = request
        self.state = .listening

        // Start recognition task
        recognitionTask = speechRecognizer.recognitionTask(with: request) { [weak self] result, error in
            Task { @MainActor [weak self] in
                guard let self = self else { return }

                if let error = error {
                    // Only handle error if we haven't already moved to result state
                    if case .result = self.state { return }
                    self.handleRecognitionError(error)
                    return
                }

                guard let result = result else { return }

                let text = result.bestTranscription.formattedString

                // Try to match a keyword
                if let action = KeywordMatcher.match(text) {
                    self.state = .result(action)
                    self.stopListening()
                    return
                }

                // We have partial results but no keyword match yet - reset timeout
                if !result.isFinal {
                    self.state = .processing
                    self.resetTimeout()
                }

                // Final result with no keyword match
                if result.isFinal {
                    self.state = .error
                    self.stopListening()
                }
            }
        }

        // Start initial timeout
        resetTimeout()
    }

    // MARK: - Stop Listening

    func stopListening() {
        timeoutTask?.cancel()
        timeoutTask = nil

        audioEngine?.stop()
        audioEngine?.inputNode.removeTap(onBus: 0)
        audioEngine = nil

        recognitionRequest?.endAudio()
        recognitionRequest = nil

        recognitionTask?.cancel()
        recognitionTask = nil

        // If we stopped without a result, go to error briefly then idle
        if state == .listening || state == .processing {
            state = .error
            scheduleReturnToIdle()
        } else if case .result = state {
            // Result state: will be handled by ContentView, then return to idle
            scheduleReturnToIdle()
        } else if state == .error {
            scheduleReturnToIdle()
        }
    }

    // MARK: - Timeout

    private func resetTimeout() {
        timeoutTask?.cancel()
        timeoutTask = Task { [weak self] in
            do {
                try await Task.sleep(nanoseconds: SpeechRecognizer.timeoutDuration)
                guard let self = self else { return }
                // Timeout reached - no keyword recognized
                if self.state == .listening || self.state == .processing {
                    self.stopListening()
                }
            } catch {
                // Task was cancelled (timeout reset or stopped) - do nothing
            }
        }
    }

    private func scheduleReturnToIdle() {
        Task {
            try? await Task.sleep(nanoseconds: 800_000_000) // 0.8 second display time
            if case .result = self.state {
                self.state = .idle
            } else if self.state == .error {
                self.state = .idle
            }
        }
    }

    // MARK: - Error handling

    private func handleRecognitionError(_ error: Error) {
        // Cancelled errors are expected when we stop listening
        let nsError = error as NSError
        if nsError.domain == "kAFAssistantErrorDomain" && nsError.code == 216 {
            // Recognition cancelled - not a real error
            if state == .listening || state == .processing {
                state = .error
                scheduleReturnToIdle()
            }
            return
        }

        state = .error
        stopListening()
    }
}
#else
// Stub for platforms where Speech is unavailable (e.g. watchOS Simulator)
import Combine

@MainActor
class SpeechRecognizer: ObservableObject {
    @Published private(set) var state: SpeechState = .idle
    @Published private(set) var permissionDenied: Bool = false
    @Published private(set) var isAvailable: Bool = false

    func toggleListening() async {}
    func stopListening() {}
    func checkPermissions() async -> Bool { return false }
}
#endif
