import SwiftUI
import SwiftData
import WatchKit

struct ContentView: View {
    @Environment(\.modelContext) private var modelContext
    @Query private var allMatches: [MatchRecord]
    @StateObject private var match = TennisMatch()
    @StateObject private var speechRecognizer = SpeechRecognizer()
    @State private var syncService: SyncService?
    @State private var isLoggedIn = KeychainHelper.accessToken != nil
    @State private var matchSaved = false

    private var wins: Int { allMatches.filter { $0.winner == 1 }.count }
    private var losses: Int { allMatches.filter { $0.winner != 1 }.count }
    private var winRate: Int {
        let total = allMatches.count
        guard total > 0 else { return 0 }
        return Int(round(Double(wins) / Double(total) * 100))
    }

    var body: some View {
        NavigationStack {
            if let winner = match.score.winner {
                // Match complete view
                VStack(spacing: 10) {
                    Text(winner == .player1 ? "我贏了！" : "對手贏了")
                        .font(.title2)
                        .fontWeight(.bold)

                    Text("\(match.score.player1Sets) - \(match.score.player2Sets)")
                        .font(.title3)

                    if !allMatches.isEmpty {
                        Text("戰績: \(wins)勝 \(losses)敗 (\(winRate)%)")
                            .font(.caption2)
                            .foregroundColor(.secondary)
                    }

                    Button("New Match") {
                        WKInterfaceDevice.current().play(.click)
                        matchSaved = false
                        match.newMatch()
                    }
                    .buttonStyle(.borderedProminent)
                }
                .task {
                    if !matchSaved {
                        await saveAndSyncMatch()
                        matchSaved = true
                    }
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
        .toolbar {
            ToolbarItem(placement: .topBarLeading) {
                NavigationLink(destination: MatchHistoryView()) {
                    Image(systemName: "list.bullet")
                }
            }
            ToolbarItem(placement: .topBarTrailing) {
                if isLoggedIn {
                    Button {
                        KeychainHelper.accessToken = nil
                        KeychainHelper.refreshToken = nil
                        isLoggedIn = false
                    } label: {
                        Image(systemName: "person.crop.circle.badge.checkmark")
                            .foregroundColor(.green)
                    }
                } else {
                    NavigationLink(destination: AuthView(isLoggedIn: $isLoggedIn)) {
                        Image(systemName: "person.crop.circle.badge.xmark")
                            .foregroundColor(.orange)
                    }
                }
            }
        }
        .onAppear {
            if syncService == nil {
                syncService = SyncService(modelContext: modelContext)
            }
            isLoggedIn = KeychainHelper.accessToken != nil
            if isLoggedIn {
                Task { await syncService?.syncAll() }
            }
        }
    }

    // MARK: - Match Saving

    private func saveAndSyncMatch() async {
        guard let winner = match.score.winner else { return }

        let record = MatchRecord(
            winner: winner == .player1 ? 1 : 2,
            player1Sets: match.score.player1Sets,
            player2Sets: match.score.player2Sets,
            startedAt: match.matchStartedAt,
            endedAt: Date()
        )

        // Extract point events from FFI
        let pointEvents = match.getPointEvents()
        for (index, event) in pointEvents.enumerated() {
            let eventRecord = MatchEventRecord(
                pointNumber: index + 1,
                player: event.player,
                timestamp: event.timestamp
            )
            record.events.append(eventRecord)
        }

        modelContext.insert(record)
        try? modelContext.save()

        // Sync to backend
        if isLoggedIn {
            if syncService == nil {
                syncService = SyncService(modelContext: modelContext)
            }
            await syncService?.syncMatch(record)
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
