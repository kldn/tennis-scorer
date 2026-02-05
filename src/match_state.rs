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
            } => {
                let current_set_index = sets.len() - 1;
                let current_set = &sets[current_set_index];

                let is_final_set = *player1_sets == config.sets_to_win - 1
                    && *player2_sets == config.sets_to_win - 1;

                let new_set = current_set.score_point(
                    scorer,
                    config.no_ad_scoring,
                    config.tiebreak_points,
                    is_final_set,
                    config.final_set_tiebreak,
                );

                let mut new_sets = sets.clone();
                new_sets[current_set_index] = new_set.clone();

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
                        }
                    }
                } else {
                    MatchState::Playing {
                        sets: new_sets,
                        player1_sets: *player1_sets,
                        player2_sets: *player2_sets,
                        config: config.clone(),
                    }
                }
            }
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
}
