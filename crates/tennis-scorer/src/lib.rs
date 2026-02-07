mod config;
mod game;
mod history;
mod match_state;
mod set;
mod tiebreak;
mod types;

pub use config::{MatchConfig, MatchType};
pub use game::GameState;
pub use history::MatchWithHistory;
pub use match_state::MatchState;
pub use set::SetState;
pub use tiebreak::TiebreakState;
pub use types::{Player, Point};

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

    fn doubles_config() -> MatchConfig {
        MatchConfig {
            match_type: MatchType::Doubles,
            serve_order: vec![
                (Player::Player1, 0),
                (Player::Player2, 0),
                (Player::Player1, 1),
                (Player::Player2, 1),
            ],
            ..MatchConfig::default()
        }
    }

    /// Full doubles match with serve rotation verification
    #[test]
    fn test_full_doubles_match_with_serve_rotation() {
        let config = doubles_config();
        let state = MatchState::new(config);
        let mwh = MatchWithHistory::new(state);

        // Verify initial server
        assert_eq!(mwh.current().current_server(), 0);

        // P1 wins first set 6-0 (6 games)
        let mwh = score_set(mwh, Player::Player1);
        // After 6 games, server index = 6 % 4 = 2
        assert_eq!(mwh.current().current_server(), 2);
        assert!(mwh.current().winner().is_none());

        // P1 wins second set 6-0 (6 more games)
        let mwh = score_set(mwh, Player::Player1);
        // Match should be complete
        assert_eq!(mwh.current().winner(), Some(Player::Player1));
    }

    /// Doubles tiebreak serve rotation integration test
    #[test]
    fn test_doubles_tiebreak_serve_rotation_integration() {
        let config = doubles_config();
        let state = MatchState::new(config);
        let mut mwh = MatchWithHistory::new(state);

        // Get to 6-6 in first set
        for _ in 0..6 {
            mwh = score_game(mwh, Player::Player1);
            mwh = score_game(mwh, Player::Player2);
        }

        // Should be in tiebreak now
        if let MatchState::Playing { sets, .. } = mwh.current() {
            if let SetState::Playing { tiebreak, .. } = &sets[0] {
                assert!(tiebreak.is_some());
            } else {
                panic!("Expected Playing set");
            }
        }

        // After 12 games, tiebreak server should be at position 0
        assert_eq!(mwh.current().current_server(), 0);

        // Score a tiebreak point
        mwh = mwh.score_point(Player::Player1);
        // After 1 point, server advances
        assert_eq!(mwh.current().current_server(), 1);

        // Win tiebreak for P1 (need 6 more points for 7-0)
        for _ in 0..6 {
            mwh = mwh.score_point(Player::Player1);
        }

        // Set should be complete, now in set 2
        if let MatchState::Playing { player1_sets, .. } = mwh.current() {
            assert_eq!(*player1_sets, 1);
        } else {
            panic!("Expected Playing");
        }
    }

    /// Undo preserves/restores serve rotation state
    #[test]
    fn test_doubles_undo_preserves_serve_rotation() {
        let config = doubles_config();
        let state = MatchState::new(config);
        let mwh = MatchWithHistory::new(state);

        assert_eq!(mwh.current().current_server(), 0);

        // Win a game
        let mwh = score_game(mwh, Player::Player1);
        assert_eq!(mwh.current().current_server(), 1);

        // Undo last point of the game
        let mwh = mwh.undo();
        // Server should be back to 0 (before game completed)
        assert_eq!(mwh.current().current_server(), 0);

        // Score that point again to complete the game
        let mwh = mwh.score_point(Player::Player1);
        assert_eq!(mwh.current().current_server(), 1);

        // Win another game
        let mwh = score_game(mwh, Player::Player2);
        assert_eq!(mwh.current().current_server(), 2);

        // Undo all the way back through both games (4 + 4 = 8 undo operations,
        // but we already undid and re-scored, so history is 4 + 1 + 4 = 9)
        let mut m = mwh;
        while m.can_undo() {
            m = m.undo();
        }
        assert_eq!(m.current().current_server(), 0);
    }
}
