use crate::config::MatchConfig;
use crate::set::SetState;
use crate::types::Player;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchState {
    Playing {
        sets: Vec<SetState>,
        player1_sets: u8,
        player2_sets: u8,
        config: MatchConfig,
        serve_rotation_index: usize,
        tiebreak_serve_index: usize,
        tiebreak_points_served: u8,
    },
    Completed {
        winner: Player,
        player1_sets: u8,
        player2_sets: u8,
        sets: Vec<SetState>,
    },
}

impl MatchState {
    pub fn new(config: MatchConfig) -> Self {
        MatchState::Playing {
            sets: vec![SetState::new()],
            player1_sets: 0,
            player2_sets: 0,
            config,
            serve_rotation_index: 0,
            tiebreak_serve_index: 0,
            tiebreak_points_served: 0,
        }
    }

    pub fn score_point(&self, scorer: Player) -> MatchState {
        match self {
            MatchState::Completed { .. } => self.clone(),

            MatchState::Playing {
                sets,
                player1_sets,
                player2_sets,
                config,
                serve_rotation_index,
                tiebreak_serve_index,
                tiebreak_points_served,
            } => {
                let current_set_index = sets.len() - 1;
                let current_set = &sets[current_set_index];

                let is_final_set = *player1_sets == config.sets_to_win - 1
                    && *player2_sets == config.sets_to_win - 1;

                // Detect if we are currently in a tiebreak before scoring
                let was_in_tiebreak = Self::set_is_in_tiebreak(current_set);

                let new_set = current_set.score_point(
                    scorer,
                    config.no_ad_scoring,
                    config.tiebreak_points,
                    is_final_set,
                    config.final_set_tiebreak,
                );

                let mut new_sets = sets.clone();
                new_sets[current_set_index] = new_set.clone();

                // Determine if a game just completed (regular game, not tiebreak)
                let game_just_completed =
                    !was_in_tiebreak && Self::game_count_increased(current_set, &new_set);

                // Determine if we just entered a tiebreak
                let just_entered_tiebreak = !was_in_tiebreak && Self::set_is_in_tiebreak(&new_set);

                // Determine if we are in a tiebreak (after scoring)
                let now_in_tiebreak = Self::set_is_in_tiebreak(&new_set);

                // Compute new serve tracking
                // tiebreak_serve_index: the rotation index of the FIRST tiebreak server
                // tiebreak_points_served: total tiebreak points played so far
                let serve_len = config.serve_order.len();
                let (new_serve_idx, new_tb_serve_idx, new_tb_points) = if serve_len > 0 {
                    if just_entered_tiebreak {
                        // A game just completed that triggered the tiebreak.
                        // Advance the rotation for the game that completed.
                        let advanced = (*serve_rotation_index + 1) % serve_len;
                        // Tiebreak starts: tb_serve_index = first tiebreak server
                        // tiebreak_points_served = 0 (no points yet)
                        (advanced, advanced, 0u8)
                    } else if was_in_tiebreak && now_in_tiebreak {
                        // Still in tiebreak, a point was scored
                        let new_pts = *tiebreak_points_served + 1;
                        (*serve_rotation_index, *tiebreak_serve_index, new_pts)
                    } else if was_in_tiebreak && !now_in_tiebreak {
                        // Tiebreak just ended (set completed via tiebreak)
                        let new_pts = *tiebreak_points_served + 1;
                        // Compute who was serving the last point
                        let last_server_offset = Self::tiebreak_server_offset(new_pts - 1);
                        let last_server = (*tiebreak_serve_index + last_server_offset) % serve_len;
                        // After tiebreak, next server = next after the last tiebreak server
                        let new_rotation = (last_server + 1) % serve_len;
                        (new_rotation, 0, 0)
                    } else if game_just_completed {
                        // Regular game completed, advance rotation
                        let new_idx = (*serve_rotation_index + 1) % serve_len;
                        (new_idx, 0, 0)
                    } else {
                        // No game completed, no tiebreak change
                        (
                            *serve_rotation_index,
                            *tiebreak_serve_index,
                            *tiebreak_points_served,
                        )
                    }
                } else {
                    // Singles or no serve order configured
                    (
                        *serve_rotation_index,
                        *tiebreak_serve_index,
                        *tiebreak_points_served,
                    )
                };

                if let Some(set_winner) = new_set.winner() {
                    let (new_p1_sets, new_p2_sets) = match set_winner {
                        Player::Player1 => (player1_sets + 1, *player2_sets),
                        Player::Player2 => (*player1_sets, player2_sets + 1),
                    };

                    if new_p1_sets >= config.sets_to_win {
                        MatchState::Completed {
                            winner: Player::Player1,
                            player1_sets: new_p1_sets,
                            player2_sets: new_p2_sets,
                            sets: new_sets,
                        }
                    } else if new_p2_sets >= config.sets_to_win {
                        MatchState::Completed {
                            winner: Player::Player2,
                            player1_sets: new_p1_sets,
                            player2_sets: new_p2_sets,
                            sets: new_sets,
                        }
                    } else {
                        new_sets.push(SetState::new());
                        MatchState::Playing {
                            sets: new_sets,
                            player1_sets: new_p1_sets,
                            player2_sets: new_p2_sets,
                            config: config.clone(),
                            serve_rotation_index: new_serve_idx,
                            tiebreak_serve_index: new_tb_serve_idx,
                            tiebreak_points_served: new_tb_points,
                        }
                    }
                } else {
                    MatchState::Playing {
                        sets: new_sets,
                        player1_sets: *player1_sets,
                        player2_sets: *player2_sets,
                        config: config.clone(),
                        serve_rotation_index: new_serve_idx,
                        tiebreak_serve_index: new_tb_serve_idx,
                        tiebreak_points_served: new_tb_points,
                    }
                }
            }
        }
    }

    /// Check if a set is currently in a tiebreak
    fn set_is_in_tiebreak(set: &SetState) -> bool {
        matches!(
            set,
            SetState::Playing {
                tiebreak: Some(_),
                ..
            }
        )
    }

    /// Check if the total game count increased between old and new set states
    fn game_count_increased(old_set: &SetState, new_set: &SetState) -> bool {
        let old_total = Self::set_game_total(old_set);
        let new_total = Self::set_game_total(new_set);
        new_total > old_total
    }

    fn set_game_total(set: &SetState) -> u8 {
        match set {
            SetState::Playing {
                player1_games,
                player2_games,
                ..
            } => player1_games + player2_games,
            SetState::Completed {
                player1_games,
                player2_games,
                ..
            } => player1_games + player2_games,
        }
    }

    pub fn winner(&self) -> Option<Player> {
        match self {
            MatchState::Completed { winner, .. } => Some(*winner),
            _ => None,
        }
    }

    pub fn config(&self) -> &MatchConfig {
        match self {
            MatchState::Playing { config, .. } => config,
            MatchState::Completed { .. } => {
                panic!("Cannot get config from completed match")
            }
        }
    }

    /// Returns the current server position (0-3 index into serve_order).
    /// For singles (empty serve_order), returns 0.
    /// During a tiebreak, returns the tiebreak server position.
    pub fn current_server(&self) -> u8 {
        match self {
            MatchState::Playing {
                config,
                serve_rotation_index,
                tiebreak_serve_index,
                tiebreak_points_served,
                sets,
                ..
            } => {
                if config.serve_order.is_empty() {
                    return 0;
                }
                let serve_len = config.serve_order.len();
                let in_tiebreak = Self::set_is_in_tiebreak(sets.last().unwrap());
                if in_tiebreak {
                    let offset = Self::tiebreak_server_offset(*tiebreak_points_served);
                    ((*tiebreak_serve_index + offset) % serve_len) as u8
                } else {
                    *serve_rotation_index as u8
                }
            }
            MatchState::Completed { .. } => 0,
        }
    }

    /// Given the number of tiebreak points already played, returns the
    /// server offset from the first tiebreak server.
    /// Pattern: first server serves 1 point, then each subsequent serves 2.
    /// Points 0: offset 0
    /// Point 1: offset 1
    /// Points 2-3: offset 1 (wait, 2 served) -> actually:
    /// N=0: server 0 (offset 0)
    /// N=1: server 1 (offset 1) -- after 1 point, first server done
    /// N=2: server 1 (offset 1) -- second server, 1 of 2
    /// N=3: server 2 (offset 2) -- third server
    /// N=4: server 2 (offset 2) -- third server, 2 of 2
    /// N=5: server 3 (offset 3)
    /// Formula: if N == 0 { 0 } else { (N + 1) / 2 }
    fn tiebreak_server_offset(points_played: u8) -> usize {
        if points_played == 0 {
            0
        } else {
            (points_played as usize).div_ceil(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::GameState;

    fn score_game(state: MatchState, winner: Player) -> MatchState {
        let mut s = state;
        for _ in 0..4 {
            s = s.score_point(winner);
            if s.winner().is_some() {
                return s;
            }
        }
        s
    }

    fn score_set(state: MatchState, winner: Player) -> MatchState {
        let mut s = state;
        for _ in 0..6 {
            s = score_game(s, winner);
            if s.winner().is_some() {
                return s;
            }
        }
        s
    }

    #[test]
    fn test_match_2_0() {
        let state = MatchState::new(MatchConfig::default());
        let state = score_set(state, Player::Player1);
        let state = score_set(state, Player::Player1);

        assert!(matches!(
            state,
            MatchState::Completed {
                winner: Player::Player1,
                player1_sets: 2,
                player2_sets: 0,
                ..
            }
        ));
    }

    #[test]
    fn test_match_2_1() {
        let state = MatchState::new(MatchConfig::default());
        let state = score_set(state, Player::Player1);
        let state = score_set(state, Player::Player2);
        let state = score_set(state, Player::Player1);

        assert!(matches!(
            state,
            MatchState::Completed {
                winner: Player::Player1,
                player1_sets: 2,
                player2_sets: 1,
                ..
            }
        ));
    }

    #[test]
    fn test_best_of_5() {
        let config = MatchConfig {
            sets_to_win: 3,
            ..MatchConfig::default()
        };
        let state = MatchState::new(config);
        let state = score_set(state, Player::Player1);
        let state = score_set(state, Player::Player2);
        let state = score_set(state, Player::Player2);
        let state = score_set(state, Player::Player1);
        let state = score_set(state, Player::Player1);

        assert!(matches!(
            state,
            MatchState::Completed {
                winner: Player::Player1,
                player1_sets: 3,
                player2_sets: 2,
                ..
            }
        ));
    }

    #[test]
    fn test_completed_match_no_change() {
        let state = MatchState::Completed {
            winner: Player::Player1,
            player1_sets: 2,
            player2_sets: 0,
            sets: vec![],
        };
        let state = state.score_point(Player::Player2);

        assert!(matches!(
            state,
            MatchState::Completed {
                winner: Player::Player1,
                ..
            }
        ));
    }

    #[test]
    fn test_final_set_no_tiebreak() {
        let config = MatchConfig {
            sets_to_win: 2,
            final_set_tiebreak: false,
            ..MatchConfig::default()
        };

        let state = MatchState::Playing {
            sets: vec![
                SetState::Completed {
                    winner: Player::Player1,
                    player1_games: 6,
                    player2_games: 4,
                },
                SetState::Completed {
                    winner: Player::Player2,
                    player1_games: 4,
                    player2_games: 6,
                },
                SetState::Playing {
                    player1_games: 6,
                    player2_games: 5,
                    current_game: GameState::new(),
                    tiebreak: None,
                },
            ],
            player1_sets: 1,
            player2_sets: 1,
            config,
            serve_rotation_index: 0,
            tiebreak_serve_index: 0,
            tiebreak_points_served: 0,
        };

        let state = score_game(state, Player::Player2);

        if let MatchState::Playing { sets, .. } = &state {
            let current_set = &sets[2];
            if let SetState::Playing {
                player1_games,
                player2_games,
                tiebreak,
                ..
            } = current_set
            {
                assert_eq!(*player1_games, 6);
                assert_eq!(*player2_games, 6);
                assert!(tiebreak.is_none());
            } else {
                panic!("Expected Playing set");
            }
        } else {
            panic!("Expected Playing match");
        }
    }

    fn doubles_config() -> MatchConfig {
        use crate::config::MatchType;
        MatchConfig {
            match_type: MatchType::Doubles,
            serve_order: vec![
                (Player::Player1, 0), // index 0: Team1-A
                (Player::Player2, 0), // index 1: Team2-A
                (Player::Player1, 1), // index 2: Team1-B
                (Player::Player2, 1), // index 3: Team2-B
            ],
            ..MatchConfig::default()
        }
    }

    #[test]
    fn test_serve_rotation_across_games() {
        let state = MatchState::new(doubles_config());
        // Initial server is index 0
        assert_eq!(state.current_server(), 0);

        // Score game 1 (4 points for P1)
        let state = score_game(state, Player::Player1);
        assert_eq!(state.current_server(), 1);

        // Score game 2
        let state = score_game(state, Player::Player2);
        assert_eq!(state.current_server(), 2);

        // Score game 3
        let state = score_game(state, Player::Player1);
        assert_eq!(state.current_server(), 3);

        // Score game 4 -> wraps back to 0
        let state = score_game(state, Player::Player2);
        assert_eq!(state.current_server(), 0);

        // Score game 5 -> 1 again
        let state = score_game(state, Player::Player1);
        assert_eq!(state.current_server(), 1);
    }

    #[test]
    fn test_serve_rotation_across_set_boundaries() {
        let state = MatchState::new(doubles_config());

        // P1 wins set 6-0 = 6 games. After 6 games, rotation index = 6 % 4 = 2.
        let state = score_set(state, Player::Player1);
        assert_eq!(state.current_server(), 2);

        // Verify we're in second set
        if let MatchState::Playing { player1_sets, .. } = &state {
            assert_eq!(*player1_sets, 1);
        } else {
            panic!("Expected Playing");
        }

        // Next game advances from 2 -> 3
        let state = score_game(state, Player::Player2);
        assert_eq!(state.current_server(), 3);
    }

    #[test]
    fn test_tiebreak_serve_rotation() {
        let config = doubles_config();
        let mut state = MatchState::new(config);

        // Get to 6-6: alternate P1 and P2 winning games
        for _ in 0..6 {
            state = score_game(state, Player::Player1);
            state = score_game(state, Player::Player2);
        }

        // After 12 games: serve_rotation was advanced each game.
        // The 12th game triggers a tiebreak via just_entered_tiebreak.
        // After game 11 completed, serve_rotation_index = 11 % 4 = 3.
        // Game 12 completes triggering tiebreak: advanced = (3+1)%4 = 0.
        // So tiebreak starts with server at index 0.
        assert_eq!(state.current_server(), 0);

        // First server serves 1 point, then it rotates
        state = state.score_point(Player::Player1); // Point 1 by server 0
        assert_eq!(state.current_server(), 1);

        // Second server serves 2 points
        state = state.score_point(Player::Player2); // Point 2 by server 1 (1 of 2)
        assert_eq!(state.current_server(), 1);
        state = state.score_point(Player::Player1); // Point 3 by server 1 (2 of 2)
        assert_eq!(state.current_server(), 2);

        // Third server serves 2 points
        state = state.score_point(Player::Player2); // Point 4 by server 2 (1 of 2)
        assert_eq!(state.current_server(), 2);
        state = state.score_point(Player::Player1); // Point 5 by server 2 (2 of 2)
        assert_eq!(state.current_server(), 3);

        // Fourth server serves 2 points
        state = state.score_point(Player::Player2); // Point 6 by server 3 (1 of 2)
        assert_eq!(state.current_server(), 3);
        state = state.score_point(Player::Player1); // Point 7 by server 3 (2 of 2)
        // Score: P1=4, P2=3. Not won yet. Wraps to server 0.
        assert_eq!(state.current_server(), 0);
    }

    #[test]
    fn test_serve_after_tiebreak() {
        let config = doubles_config();
        let mut state = MatchState::new(config);

        // Get to 6-6
        for _ in 0..6 {
            state = score_game(state, Player::Player1);
            state = score_game(state, Player::Player2);
        }

        // Now in tiebreak. P1 wins it 7-0.
        for _ in 0..7 {
            state = state.score_point(Player::Player1);
        }

        // Set 1 should be complete, now in set 2.
        if let MatchState::Playing {
            player1_sets, sets, ..
        } = &state
        {
            assert_eq!(*player1_sets, 1);
            assert_eq!(sets.len(), 2);
        } else {
            panic!("Expected Playing");
        }

        // Trace tiebreak serve rotation for 7 points (all P1):
        // Point 1: server 0 (1 of 1) -> advance to 1, pts=0
        // Point 2: server 1 (1 of 2), pts=1
        // Point 3: server 1 (2 of 2) -> advance to 2, pts=0
        // Point 4: server 2 (1 of 2), pts=1
        // Point 5: server 2 (2 of 2) -> advance to 3, pts=0
        // Point 6: server 3 (1 of 2), pts=1
        // Point 7: tiebreak ends. was_in_tiebreak=true, now_in_tiebreak=false.
        //   tiebreak_serve_index before point 7 = 3.
        //   new_rotation = (3 + 1) % 4 = 0.
        assert_eq!(state.current_server(), 0);
    }

    #[test]
    fn test_singles_unaffected_by_serve_tracking() {
        let state = MatchState::new(MatchConfig::default());

        // current_server returns 0 for singles
        assert_eq!(state.current_server(), 0);

        let state = score_game(state, Player::Player1);
        assert_eq!(state.current_server(), 0);

        let state = score_set(state, Player::Player1);
        assert_eq!(state.current_server(), 0);
    }
}
