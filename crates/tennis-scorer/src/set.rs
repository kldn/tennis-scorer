use crate::game::GameState;
use crate::tiebreak::TiebreakState;
use crate::types::Player;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetState {
    Playing {
        player1_games: u8,
        player2_games: u8,
        current_game: GameState,
        tiebreak: Option<TiebreakState>,
    },
    Completed {
        winner: Player,
        player1_games: u8,
        player2_games: u8,
    },
}

impl SetState {
    pub fn new() -> Self {
        SetState::Playing {
            player1_games: 0,
            player2_games: 0,
            current_game: GameState::new(),
            tiebreak: None,
        }
    }

    pub fn score_point(
        &self,
        scorer: Player,
        no_ad: bool,
        tiebreak_points: u8,
        is_final_set: bool,
        final_set_tiebreak: bool,
    ) -> SetState {
        match self {
            SetState::Completed { .. } => self.clone(),

            SetState::Playing {
                player1_games,
                player2_games,
                current_game,
                tiebreak,
            } => {
                if let Some(tb) = tiebreak {
                    let new_tb = tb.score_point(scorer);
                    if let Some(winner) = new_tb.winner() {
                        let (final_p1, final_p2) = match winner {
                            Player::Player1 => (player1_games + 1, *player2_games),
                            Player::Player2 => (*player1_games, player2_games + 1),
                        };
                        SetState::Completed {
                            winner,
                            player1_games: final_p1,
                            player2_games: final_p2,
                        }
                    } else {
                        SetState::Playing {
                            player1_games: *player1_games,
                            player2_games: *player2_games,
                            current_game: current_game.clone(),
                            tiebreak: Some(new_tb),
                        }
                    }
                } else {
                    let new_game = current_game.score_point(scorer, no_ad);

                    if let Some(game_winner) = new_game.winner() {
                        let (new_p1, new_p2) = match game_winner {
                            Player::Player1 => (player1_games + 1, *player2_games),
                            Player::Player2 => (*player1_games, player2_games + 1),
                        };

                        if let Some(set_winner) =
                            Self::check_set_winner(new_p1, new_p2, is_final_set, final_set_tiebreak)
                        {
                            SetState::Completed {
                                winner: set_winner,
                                player1_games: new_p1,
                                player2_games: new_p2,
                            }
                        } else if new_p1 == 6
                            && new_p2 == 6
                            && (!is_final_set || final_set_tiebreak)
                        {
                            SetState::Playing {
                                player1_games: new_p1,
                                player2_games: new_p2,
                                current_game: GameState::new(),
                                tiebreak: Some(TiebreakState::new(tiebreak_points)),
                            }
                        } else {
                            SetState::Playing {
                                player1_games: new_p1,
                                player2_games: new_p2,
                                current_game: GameState::new(),
                                tiebreak: None,
                            }
                        }
                    } else {
                        SetState::Playing {
                            player1_games: *player1_games,
                            player2_games: *player2_games,
                            current_game: new_game,
                            tiebreak: None,
                        }
                    }
                }
            }
        }
    }

    fn check_set_winner(
        p1_games: u8,
        p2_games: u8,
        _is_final_set: bool,
        _final_set_tiebreak: bool,
    ) -> Option<Player> {
        let leader = p1_games.max(p2_games);
        let trailer = p1_games.min(p2_games);
        let lead = leader - trailer;

        if (leader >= 6 && lead >= 2) || (leader == 7 && trailer == 6) {
            if p1_games > p2_games {
                Some(Player::Player1)
            } else {
                Some(Player::Player2)
            }
        } else {
            None
        }
    }

    pub fn winner(&self) -> Option<Player> {
        match self {
            SetState::Completed { winner, .. } => Some(*winner),
            _ => None,
        }
    }
}

impl Default for SetState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn score_game(set: SetState, winner: Player, no_ad: bool) -> SetState {
        let mut s = set;
        for _ in 0..4 {
            s = s.score_point(winner, no_ad, 7, false, true);
            if matches!(s, SetState::Completed { .. }) {
                return s;
            }
            if let SetState::Playing { current_game, .. } = &s {
                if current_game.winner().is_some() {
                    return s;
                }
            }
        }
        s
    }

    #[test]
    fn test_set_6_4() {
        let mut set = SetState::new();
        for _ in 0..6 {
            set = score_game(set, Player::Player1, false);
        }
        for _ in 0..4 {
            set = score_game(set, Player::Player2, false);
        }

        let set = SetState::Playing {
            player1_games: 5,
            player2_games: 4,
            current_game: GameState::new(),
            tiebreak: None,
        };
        let set = score_game(set, Player::Player1, false);

        assert!(matches!(
            set,
            SetState::Completed {
                winner: Player::Player1,
                player1_games: 6,
                player2_games: 4
            }
        ));
    }

    #[test]
    fn test_set_7_5() {
        let set = SetState::Playing {
            player1_games: 6,
            player2_games: 5,
            current_game: GameState::new(),
            tiebreak: None,
        };
        let set = score_game(set, Player::Player1, false);

        assert!(matches!(
            set,
            SetState::Completed {
                winner: Player::Player1,
                player1_games: 7,
                player2_games: 5
            }
        ));
    }

    #[test]
    fn test_tiebreak_trigger_at_6_6() {
        let set = SetState::Playing {
            player1_games: 6,
            player2_games: 5,
            current_game: GameState::new(),
            tiebreak: None,
        };
        let set = score_game(set, Player::Player2, false);

        assert!(matches!(
            set,
            SetState::Playing {
                player1_games: 6,
                player2_games: 6,
                tiebreak: Some(_),
                ..
            }
        ));
    }

    #[test]
    fn test_tiebreak_win() {
        let mut set = SetState::Playing {
            player1_games: 6,
            player2_games: 6,
            current_game: GameState::new(),
            tiebreak: Some(TiebreakState::new(7)),
        };

        for _ in 0..7 {
            set = set.score_point(Player::Player1, false, 7, false, true);
        }

        assert!(matches!(
            set,
            SetState::Completed {
                winner: Player::Player1,
                player1_games: 7,
                player2_games: 6
            }
        ));
    }

    #[test]
    fn test_completed_set_no_change() {
        let set = SetState::Completed {
            winner: Player::Player1,
            player1_games: 6,
            player2_games: 4,
        };
        let set = set.score_point(Player::Player2, false, 7, false, true);

        assert!(matches!(
            set,
            SetState::Completed {
                winner: Player::Player1,
                ..
            }
        ));
    }
}
