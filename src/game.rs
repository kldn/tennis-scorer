use crate::types::{Player, Point};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameState {
    Points { player1: Point, player2: Point },
    Deuce { count: u8 },
    Advantage { player: Player, deuce_count: u8 },
    Completed(Player),
}

impl GameState {
    pub fn new() -> Self {
        GameState::Points {
            player1: Point::Love,
            player2: Point::Love,
        }
    }

    pub fn score_point(&self, scorer: Player, no_ad: bool) -> GameState {
        match self {
            GameState::Completed(_) => self.clone(),

            GameState::Points { player1, player2 } => {
                let (scorer_points, opponent_points) = match scorer {
                    Player::Player1 => (*player1, *player2),
                    Player::Player2 => (*player2, *player1),
                };

                if scorer_points == Point::Forty {
                    if opponent_points == Point::Forty {
                        if no_ad {
                            GameState::Completed(scorer)
                        } else {
                            // First deuce, count = 1
                            GameState::Deuce { count: 1 }
                        }
                    } else {
                        GameState::Completed(scorer)
                    }
                } else {
                    let new_points = scorer_points.increment().unwrap();
                    if new_points == Point::Forty && opponent_points == Point::Forty {
                        // First deuce, count = 1
                        GameState::Deuce { count: 1 }
                    } else {
                        match scorer {
                            Player::Player1 => GameState::Points {
                                player1: new_points,
                                player2: *player2,
                            },
                            Player::Player2 => GameState::Points {
                                player1: *player1,
                                player2: new_points,
                            },
                        }
                    }
                }
            }

            GameState::Deuce { count } => {
                if no_ad {
                    GameState::Completed(scorer)
                } else {
                    GameState::Advantage { player: scorer, deuce_count: *count }
                }
            }

            GameState::Advantage { player, deuce_count } => {
                if *player == scorer {
                    GameState::Completed(scorer)
                } else {
                    // Back to deuce, increment count
                    GameState::Deuce { count: deuce_count + 1 }
                }
            }
        }
    }

    pub fn winner(&self) -> Option<Player> {
        match self {
            GameState::Completed(player) => Some(*player),
            _ => None,
        }
    }

    pub fn deuce_count(&self) -> u8 {
        match self {
            GameState::Deuce { count } => *count,
            GameState::Advantage { deuce_count, .. } => *deuce_count,
            _ => 0,
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_progression() {
        let game = GameState::new();
        let game = game.score_point(Player::Player1, false);
        assert_eq!(
            game,
            GameState::Points {
                player1: Point::Fifteen,
                player2: Point::Love
            }
        );

        let game = game.score_point(Player::Player1, false);
        assert_eq!(
            game,
            GameState::Points {
                player1: Point::Thirty,
                player2: Point::Love
            }
        );

        let game = game.score_point(Player::Player1, false);
        assert_eq!(
            game,
            GameState::Points {
                player1: Point::Forty,
                player2: Point::Love
            }
        );

        let game = game.score_point(Player::Player1, false);
        assert_eq!(game, GameState::Completed(Player::Player1));
    }

    #[test]
    fn test_deuce() {
        let game = GameState::Points {
            player1: Point::Forty,
            player2: Point::Thirty,
        };
        let game = game.score_point(Player::Player2, false);
        assert_eq!(game, GameState::Deuce { count: 1 });
        assert_eq!(game.deuce_count(), 1);
    }

    #[test]
    fn test_advantage_and_win() {
        let game = GameState::Deuce { count: 1 };
        let game = game.score_point(Player::Player1, false);
        assert_eq!(game, GameState::Advantage { player: Player::Player1, deuce_count: 1 });

        let game = game.score_point(Player::Player1, false);
        assert_eq!(game, GameState::Completed(Player::Player1));
    }

    #[test]
    fn test_advantage_back_to_deuce() {
        let game = GameState::Advantage { player: Player::Player1, deuce_count: 1 };
        let game = game.score_point(Player::Player2, false);
        assert_eq!(game, GameState::Deuce { count: 2 });
        assert_eq!(game.deuce_count(), 2);
    }

    #[test]
    fn test_no_ad_scoring() {
        let game = GameState::Deuce { count: 1 };
        let game = game.score_point(Player::Player1, true);
        assert_eq!(game, GameState::Completed(Player::Player1));
    }

    #[test]
    fn test_deuce_count_increments() {
        // Start at 40-30, P2 scores -> deuce (count = 1)
        let game = GameState::Points {
            player1: Point::Forty,
            player2: Point::Thirty,
        };
        let game = game.score_point(Player::Player2, false);
        assert_eq!(game.deuce_count(), 1);

        // P1 gets advantage
        let game = game.score_point(Player::Player1, false);
        assert_eq!(game.deuce_count(), 1); // Still 1 during advantage

        // P2 breaks advantage -> deuce (count = 2)
        let game = game.score_point(Player::Player2, false);
        assert_eq!(game.deuce_count(), 2);

        // P2 gets advantage
        let game = game.score_point(Player::Player2, false);
        assert_eq!(game.deuce_count(), 2);

        // P1 breaks advantage -> deuce (count = 3)
        let game = game.score_point(Player::Player1, false);
        assert_eq!(game.deuce_count(), 3);
    }

    #[test]
    fn test_deuce_count_zero_in_normal_play() {
        let game = GameState::new();
        assert_eq!(game.deuce_count(), 0);

        let game = game.score_point(Player::Player1, false);
        assert_eq!(game.deuce_count(), 0);
    }

    #[test]
    fn test_completed_game_no_change() {
        let game = GameState::Completed(Player::Player1);
        let game = game.score_point(Player::Player2, false);
        assert_eq!(game, GameState::Completed(Player::Player1));
    }

    #[test]
    fn test_direct_win_from_forty() {
        let game = GameState::Points {
            player1: Point::Forty,
            player2: Point::Thirty,
        };
        let game = game.score_point(Player::Player1, false);
        assert_eq!(game, GameState::Completed(Player::Player1));
    }
}
