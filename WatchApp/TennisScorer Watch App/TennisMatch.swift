import Foundation
import Combine

extension Player {
    init(from uniffiPlayer: Player) {
        self = uniffiPlayer
    }

    var uniffiValue: Player {
        return self
    }

    var intValue: Int {
        switch self {
        case .player1: return 1
        case .player2: return 2
        }
    }
}

struct Score {
    let player1Sets: Int
    let player2Sets: Int
    let player1Games: Int
    let player2Games: Int
    let player1Points: Int
    let player2Points: Int
    let isTiebreak: Bool
    let gameState: GameState
    let winner: Player?
    let deuceCount: Int

    enum GameState {
        case playing
        case deuce
        case advantagePlayer1
        case advantagePlayer2
        case completed
    }

    var pointsDisplay: String {
        if isTiebreak {
            return "\(player1Points) - \(player2Points)"
        }

        switch gameState {
        case .deuce:
            if deuceCount > 1 {
                return "Deuce (\(deuceCount))"
            }
            return "Deuce"
        case .advantagePlayer1:
            return "Ad - 40"
        case .advantagePlayer2:
            return "40 - Ad"
        case .completed:
            return "Game"
        case .playing:
            return "\(pointToString(player1Points)) - \(pointToString(player2Points))"
        }
    }

    private func pointToString(_ points: Int) -> String {
        switch points {
        case 0: return "0"
        case 15: return "15"
        case 30: return "30"
        case 40: return "40"
        default: return "\(points)"
        }
    }

    init(from matchScore: MatchScore) {
        self.player1Sets = Int(matchScore.player1Sets)
        self.player2Sets = Int(matchScore.player2Sets)

        // Current set games (last entry in the games arrays)
        let p1GamesData = [UInt8](matchScore.player1Games)
        let p2GamesData = [UInt8](matchScore.player2Games)
        self.player1Games = Int(p1GamesData.last ?? 0)
        self.player2Games = Int(p2GamesData.last ?? 0)

        self.isTiebreak = matchScore.isTiebreak
        self.deuceCount = Int(matchScore.deuceCount)

        if let w = matchScore.winner {
            self.winner = Player(from: w)
        } else {
            self.winner = nil
        }

        // Extract points and game state from GameScore enum
        switch matchScore.currentGame {
        case .points(let p1, let p2):
            self.gameState = .playing
            self.player1Points = Int(p1) ?? 0
            self.player2Points = Int(p2) ?? 0
        case .deuce:
            self.gameState = .deuce
            self.player1Points = 40
            self.player2Points = 40
        case .advantage(let player):
            switch player {
            case .player1:
                self.gameState = .advantagePlayer1
            case .player2:
                self.gameState = .advantagePlayer2
            }
            self.player1Points = 0
            self.player2Points = 0
        case .completed(_):
            self.gameState = .completed
            self.player1Points = 0
            self.player2Points = 0
        }
    }

    // For initial empty state
    init(
        player1Sets: Int, player2Sets: Int,
        player1Games: Int, player2Games: Int,
        player1Points: Int, player2Points: Int,
        isTiebreak: Bool, gameState: GameState,
        winner: Player?, deuceCount: Int
    ) {
        self.player1Sets = player1Sets
        self.player2Sets = player2Sets
        self.player1Games = player1Games
        self.player2Games = player2Games
        self.player1Points = player1Points
        self.player2Points = player2Points
        self.isTiebreak = isTiebreak
        self.gameState = gameState
        self.winner = winner
        self.deuceCount = deuceCount
    }
}

@MainActor
class TennisMatchViewModel: ObservableObject {
    @Published private(set) var score: Score
    @Published private(set) var canUndo: Bool = false
    private(set) var matchStartedAt: Date = Date()

    private let engine: TennisMatch

    init() {
        engine = TennisMatch()
        score = Self.emptyScore()
        updateScore()
    }

    init(setsToWin: UInt8, tiebreakPoints: UInt8, finalSetTiebreak: Bool, noAdScoring: Bool) {
        let config = MatchConfig(
            setsToWin: setsToWin,
            tiebreakPoints: tiebreakPoints,
            finalSetTiebreak: finalSetTiebreak,
            noAdScoring: noAdScoring,
            isDoubles: false,
            firstServerTeam: nil
        )
        engine = TennisMatch.newWithConfig(config: config)
        score = Self.emptyScore()
        updateScore()
    }

    func scorePoint(player: Player) {
        _ = engine.scorePoint(player: player.uniffiValue)
        updateScore()
    }

    func undo() {
        _ = engine.undo()
        updateScore()
    }

    func getPointEvents() -> [(player: Int, timestamp: Date)] {
        let events = engine.getPointEvents()
        return events.map { event in
            let player = Player(from: event.player).intValue
            let timestamp = Date(timeIntervalSince1970: event.timestampEpochSecs)
            return (player: player, timestamp: timestamp)
        }
    }

    func newMatch() {
        engine.newMatch()
        matchStartedAt = Date()
        updateScore()
    }

    private func updateScore() {
        let matchScore = engine.getScore()
        canUndo = engine.canUndo()
        score = Score(from: matchScore)
    }

    private static func emptyScore() -> Score {
        Score(
            player1Sets: 0,
            player2Sets: 0,
            player1Games: 0,
            player2Games: 0,
            player1Points: 0,
            player2Points: 0,
            isTiebreak: false,
            gameState: .playing,
            winner: nil,
            deuceCount: 0
        )
    }
}
