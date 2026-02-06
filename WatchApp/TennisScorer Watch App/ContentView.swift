import SwiftUI
import WatchKit

struct ContentView: View {
    @StateObject private var match = TennisMatch()
    @StateObject private var speechRecognizer = SpeechRecognizer()

    var body: some View {
        if let winner = match.score.winner {
            // Match complete view - no mic button
            VStack(spacing: 10) {
                Text(winner == .player1 ? "我贏了！" : "對手贏了")
                    .font(.title2)
                    .fontWeight(.bold)

                Text("\(match.score.player1Sets) - \(match.score.player2Sets)")
                    .font(.title3)

                Button("New Match") {
                    WKInterfaceDevice.current().play(.click)
                    match.newMatch()
                }
                .buttonStyle(.borderedProminent)
            }
        } else {
            // Active match view
            VStack(spacing: 4) {
                // Sets
                HStack {
                    Text("Sets:")
                        .font(.caption2)
                        .foregroundColor(.secondary)
                    Text("\(match.score.player1Sets) - \(match.score.player2Sets)")
                        .font(.caption)
                        .fontWeight(.semibold)
                }

                // Games
                HStack {
                    Text("Games:")
                        .font(.caption2)
                        .foregroundColor(.secondary)
                    Text("\(match.score.player1Games) - \(match.score.player2Games)")
                        .font(.caption)
                        .fontWeight(.semibold)
                }

                // Tiebreak indicator
                if match.score.isTiebreak {
                    Text("Tiebreak")
                        .font(.caption2)
                        .foregroundColor(.orange)
                }

                // Current game points - large display
                Text(match.score.pointsDisplay)
                    .font(.system(size: 36, weight: .bold, design: .rounded))
                    .minimumScaleFactor(0.5)
                    .padding(.vertical, 4)

                // Score buttons
                HStack(spacing: 20) {
                    Button {
                        WKInterfaceDevice.current().play(.click)
                        match.scorePoint(player: .player1)
                    } label: {
                        VStack {
                            Image(systemName: "person.fill")
                            Text("我")
                                .font(.caption2)
                        }
                    }
                    .buttonStyle(.bordered)
                    .tint(.blue)

                    Button {
                        WKInterfaceDevice.current().play(.click)
                        match.scorePoint(player: .player2)
                    } label: {
                        VStack {
                            Image(systemName: "person.fill")
                            Text("對手")
                                .font(.caption2)
                        }
                    }
                    .buttonStyle(.bordered)
                    .tint(.green)
                }

                // Undo and Mic buttons row
                HStack(spacing: 16) {
                    // Undo button
                    if match.canUndo {
                        Button {
                            WKInterfaceDevice.current().play(.click)
                            match.undo()
                        } label: {
                            Image(systemName: "arrow.uturn.backward")
                        }
                        .buttonStyle(.plain)
                        .foregroundColor(.orange)
                        .font(.caption)
                    }

                    // Microphone button
                    if speechRecognizer.isAvailable {
                        micButton
                    }
                }
            }
            .alert("需要權限", isPresented: .constant(speechRecognizer.permissionDenied)) {
                Button("好") {
                    // Dismiss
                }
            } message: {
                Text("請在設定中允許麥克風和語音辨識權限以使用語音記分功能")
            }
            .onChange(of: speechRecognizer.state) { newState in
                handleSpeechStateChange(newState)
            }
        }
    }

    // MARK: - Microphone Button

    @ViewBuilder
    private var micButton: some View {
        Button {
            Task {
                await speechRecognizer.toggleListening()
            }
        } label: {
            micButtonLabel
        }
        .buttonStyle(.plain)
        .foregroundColor(micButtonColor)
        .font(.caption)
    }

    @ViewBuilder
    private var micButtonLabel: some View {
        switch speechRecognizer.state {
        case .listening, .processing:
            Image(systemName: "mic.fill")
                .symbolEffect(.pulse)
        case .result(let action):
            Text(action.displayText)
                .font(.caption2)
                .fontWeight(.semibold)
        case .error:
            Image(systemName: "mic.slash")
        default:
            Image(systemName: "mic.fill")
        }
    }

    private var micButtonColor: Color {
        switch speechRecognizer.state {
        case .listening, .processing:
            return .red
        case .result:
            return .green
        case .error:
            return .gray
        default:
            return .white
        }
    }

    // MARK: - Speech State Handling

    private func handleSpeechStateChange(_ newState: SpeechState) {
        switch newState {
        case .result(let action):
            // Execute the scoring action
            switch action {
            case .player1Point:
                match.scorePoint(player: .player1)
                WKInterfaceDevice.current().play(.success)
            case .player2Point:
                match.scorePoint(player: .player2)
                WKInterfaceDevice.current().play(.success)
            case .undo:
                if match.canUndo {
                    match.undo()
                    WKInterfaceDevice.current().play(.success)
                } else {
                    WKInterfaceDevice.current().play(.failure)
                }
            }
        case .error:
            WKInterfaceDevice.current().play(.failure)
        default:
            break
        }
    }
}

#Preview {
    ContentView()
}
