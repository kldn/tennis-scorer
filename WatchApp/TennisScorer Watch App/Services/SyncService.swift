import Foundation
import SwiftData

@MainActor
class SyncService: ObservableObject {
    @Published var isSyncing = false

    private let modelContext: ModelContext

    init(modelContext: ModelContext) {
        self.modelContext = modelContext
    }

    func syncAll() async {
        guard KeychainHelper.accessToken != nil else { return }
        guard !isSyncing else { return }

        isSyncing = true
        defer { isSyncing = false }

        let descriptor = FetchDescriptor<MatchRecord>(
            predicate: #Predicate { !$0.isSynced }
        )

        guard let unsynced = try? modelContext.fetch(descriptor) else { return }

        for match in unsynced {
            await syncMatch(match)
        }
    }

    func syncMatch(_ match: MatchRecord) async {
        guard KeychainHelper.accessToken != nil else { return }

        let events: [[String: Any]] = match.events
            .sorted { $0.pointNumber < $1.pointNumber }
            .map { event in
                [
                    "point_number": event.pointNumber,
                    "player": event.player,
                    "timestamp": ISO8601DateFormatter().string(from: event.timestamp),
                ]
            }

        let payload: [String: Any] = [
            "client_id": match.id.uuidString,
            "match_type": match.matchType,
            "config": (try? JSONSerialization.jsonObject(with: match.config)) ?? [:],
            "winner": match.winner,
            "player1_sets": match.player1Sets,
            "player2_sets": match.player2Sets,
            "started_at": ISO8601DateFormatter().string(from: match.startedAt),
            "ended_at": ISO8601DateFormatter().string(from: match.endedAt),
            "events": events,
        ]

        do {
            try await APIClient.shared.uploadMatch(payload)
            match.isSynced = true
            try? modelContext.save()
        } catch {
            // Leave isSynced = false for retry
            print("Sync failed for match \(match.id): \(error)")
        }
    }
}
