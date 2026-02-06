mod types;
mod config;
mod game;
mod tiebreak;
mod set;
mod match_state;
mod history;
pub mod ffi;

pub use types::{Player, Point};
pub use config::MatchConfig;
pub use game::GameState;
pub use tiebreak::TiebreakState;
pub use set::SetState;
pub use match_state::MatchState;
pub use history::MatchWithHistory;

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Helper to score a game (4 points for one player)
    fn score_game(mwh: MatchWithHistory, winner: Player) -> MatchWithHistory {
        let mut m = mwh;
        for _ in 0..4 {
            m = m.score_point(winner);
        }
        m
    }

    /// Helper to score a set (6 games for one player)
    fn score_set(mwh: MatchWithHistory, winner: Player) -> MatchWithHistory {
        let mut m = mwh;
        for _ in 0..6 {
            m = score_game(m, winner);
        }
        m
    }

    /// Simulate a full 2-0 match
    #[test]
    fn test_full_match_2_0() {
        let config = MatchConfig::default();
        let mwh = MatchWithHistory::new(MatchState::new(config));

        // Player 1 wins first set 6-0
        let mwh = score_set(mwh, Player::Player1);
        assert!(mwh.current().winner().is_none());

        // Player 1 wins second set 6-0
        let mwh = score_set(mwh, Player::Player1);
        assert_eq!(mwh.current().winner(), Some(Player::Player1));
    }

    /// Simulate a match with undo functionality
    #[test]
    fn test_match_with_undo() {
        let config = MatchConfig::default();
        let mwh = MatchWithHistory::new(MatchState::new(config));

        // Score some points
        let mwh = mwh.score_point(Player::Player1); // 15-0
        let mwh = mwh.score_point(Player::Player1); // 30-0
        let mwh = mwh.score_point(Player::Player1); // 40-0

        assert_eq!(mwh.history_len(), 3);

        // Undo last point
        let mwh = mwh.undo();
        assert_eq!(mwh.history_len(), 2);

        // Score for player 2 instead
        let mwh = mwh.score_point(Player::Player2); // 30-15
        assert_eq!(mwh.history_len(), 3);

        // Can undo all the way back
        let mwh = mwh.undo().undo().undo();
        assert_eq!(mwh.history_len(), 0);
        assert!(!mwh.can_undo());
    }

    /// Test tiebreak scenario
    #[test]
    fn test_tiebreak_scenario() {
        let config = MatchConfig::default();
        let state = MatchState::new(config);

        // Fast-forward to a tiebreak state (6-6 in first set)
        let mut state = state;
        for _ in 0..6 {
            // P1 wins a game
            for _ in 0..4 {
                state = state.score_point(Player::Player1);
            }
            // P2 wins a game
            for _ in 0..4 {
                state = state.score_point(Player::Player2);
            }
        }

        // Verify we're in a tiebreak
        if let MatchState::Playing { sets, .. } = &state {
            if let SetState::Playing { tiebreak, .. } = &sets[0] {
                assert!(tiebreak.is_some());
            } else {
                panic!("Expected Playing set");
            }
        }

        // P1 wins tiebreak 7-0
        for _ in 0..7 {
            state = state.score_point(Player::Player1);
        }

        // First set should be complete
        if let MatchState::Playing { sets, .. } = &state {
            assert!(sets[0].winner().is_some());
            assert_eq!(sets[0].winner(), Some(Player::Player1));
        }
    }

    #[test]
    fn test_no_ad_scoring_match() {
        let config = MatchConfig {
            no_ad_scoring: true,
            ..MatchConfig::default()
        };
        let mwh = MatchWithHistory::new(MatchState::new(config));

        // Get to deuce (40-40)
        let mwh = mwh.score_point(Player::Player1); // 15-0
        let mwh = mwh.score_point(Player::Player1); // 30-0
        let mwh = mwh.score_point(Player::Player2); // 30-15
        let mwh = mwh.score_point(Player::Player2); // 30-30
        let mwh = mwh.score_point(Player::Player1); // 40-30
        let mwh = mwh.score_point(Player::Player2); // Deuce

        // In No-Ad, one more point wins the game
        let mwh = mwh.score_point(Player::Player1);

        // Verify game was won (we're now in a new game)
        if let MatchState::Playing { sets, .. } = mwh.current() {
            if let SetState::Playing {
                player1_games,
                current_game,
                ..
            } = &sets[0]
            {
                assert_eq!(*player1_games, 1);
                assert_eq!(*current_game, GameState::new());
            }
        }
    }
}
