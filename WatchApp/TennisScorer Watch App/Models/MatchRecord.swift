import Foundation
import SwiftData

@Model
final class MatchRecord {
    var id: UUID
    var matchType: String
    var config: Data
    var winner: Int
    var player1Sets: Int
    var player2Sets: Int
    var startedAt: Date
    var endedAt: Date
    var isSynced: Bool
    var createdAt: Date

    @Relationship(deleteRule: .cascade, inverse: \MatchEventRecord.matchRecord)
    var events: [MatchEventRecord]

    init(
        id: UUID = UUID(),
        matchType: String = "singles",
        config: Data = Data(),
        winner: Int,
        player1Sets: Int,
        player2Sets: Int,
        startedAt: Date,
        endedAt: Date
    ) {
        self.id = id
        self.matchType = matchType
        self.config = config
        self.winner = winner
        self.player1Sets = player1Sets
        self.player2Sets = player2Sets
        self.startedAt = startedAt
        self.endedAt = endedAt
        self.isSynced = false
        self.createdAt = Date()
        self.events = []
    }
}
