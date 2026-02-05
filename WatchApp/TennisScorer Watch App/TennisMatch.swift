import Foundation
import Combine
import TennisScorerFFI

enum Player {
    case player1
    case player2

    var ffiValue: UInt8 {
        switch self {
        case .player1: return UInt8(PLAYER_1)
        case .player2: return UInt8(PLAYER_2)
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
}

@MainActor
class TennisMatch: ObservableObject {
    @Published private(set) var score: Score
    @Published private(set) var canUndo: Bool = false

    private var handle: OpaquePointer?

    init() {
        handle = tennis_match_new_default()
        score = Self.emptyScore()
        updateScore()
    }

    init(setsToWin: UInt8, tiebreakPoints: UInt8, finalSetTiebreak: Bool, noAdScoring: Bool) {
        handle = tennis_match_new_custom(setsToWin, tiebreakPoints, finalSetTiebreak, noAdScoring)
        score = Self.emptyScore()
        updateScore()
    }

    deinit {
        if let handle = handle {
            tennis_match_free(handle)
        }
    }

    func scorePoint(player: Player) {
        guard let handle = handle else { return }
        tennis_match_score_point(handle, player.ffiValue)
        updateScore()
    }

    func undo() {
        guard let handle = handle else { return }
        tennis_match_undo(handle)
        updateScore()
    }

    func newMatch() {
        if let handle = handle {
            tennis_match_free(handle)
        }
        handle = tennis_match_new_default()
        updateScore()
    }

    private func updateScore() {
        guard let handle = handle else { return }

        let ffiScore = tennis_match_get_score(handle)
        canUndo = tennis_match_can_undo(handle)

        let gameState: Score.GameState
        switch Int32(ffiScore.game_state) {
        case GAME_STATE_DEUCE:
            gameState = .deuce
        case GAME_STATE_ADVANTAGE_P1:
            gameState = .advantagePlayer1
        case GAME_STATE_ADVANTAGE_P2:
            gameState = .advantagePlayer2
        case GAME_STATE_COMPLETED:
            gameState = .completed
        default:
            gameState = .playing
        }

        let winner: Player?
        switch Int32(ffiScore.winner) {
        case PLAYER_1:
            winner = .player1
        case PLAYER_2:
            winner = .player2
        default:
            winner = nil
        }

        score = Score(
            player1Sets: Int(ffiScore.player1_sets),
            player2Sets: Int(ffiScore.player2_sets),
            player1Games: Int(ffiScore.player1_games),
            player2Games: Int(ffiScore.player2_games),
            player1Points: Int(ffiScore.player1_points),
            player2Points: Int(ffiScore.player2_points),
            isTiebreak: ffiScore.is_tiebreak,
            gameState: gameState,
            winner: winner
        )
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
            winner: nil
        )
    }
}
