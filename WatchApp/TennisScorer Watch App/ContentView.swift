import SwiftUI
import WatchKit

struct ContentView: View {
    @StateObject private var match = TennisMatch()

    var body: some View {
        if let winner = match.score.winner {
            // Match complete view
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
            }
        }
    }
}

#Preview {
    ContentView()
}
