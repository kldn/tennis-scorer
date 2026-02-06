import SwiftUI
import SwiftData

struct MatchHistoryView: View {
    @Query(sort: \MatchRecord.startedAt, order: .reverse) private var matches: [MatchRecord]

    var body: some View {
        if matches.isEmpty {
            VStack(spacing: 8) {
                Image(systemName: "sportscourt")
                    .font(.title2)
                    .foregroundColor(.secondary)
                Text("還沒有比賽記錄")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        } else {
            List(matches) { match in
                HStack {
                    VStack(alignment: .leading, spacing: 2) {
                        HStack(spacing: 4) {
                            Text(match.winner == 1 ? "W" : "L")
                                .font(.caption)
                                .fontWeight(.bold)
                                .foregroundColor(match.winner == 1 ? .green : .red)

                            Text("\(match.player1Sets) - \(match.player2Sets)")
                                .font(.caption)
                                .fontWeight(.semibold)
                        }

                        Text(match.startedAt, style: .date)
                            .font(.caption2)
                            .foregroundColor(.secondary)
                    }

                    Spacer()

                    Image(systemName: match.isSynced ? "checkmark.icloud" : "arrow.triangle.2.circlepath.icloud")
                        .font(.caption2)
                        .foregroundColor(match.isSynced ? .green : .orange)
                }
            }
            .navigationTitle("比賽記錄")
        }
    }
}
