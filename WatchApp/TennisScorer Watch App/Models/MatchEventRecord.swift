import Foundation
import SwiftData

@Model
final class MatchEventRecord {
    var pointNumber: Int
    var player: Int
    var timestamp: Date
    var matchRecord: MatchRecord?

    init(pointNumber: Int, player: Int, timestamp: Date) {
        self.pointNumber = pointNumber
        self.player = player
        self.timestamp = timestamp
    }
}
