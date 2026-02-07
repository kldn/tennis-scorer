use std::sync::RwLock;

use tennis_scorer::{
    GameState as CoreGameState, MatchConfig, MatchState, MatchWithHistory, Player as CorePlayer,
    Point, SetState, TiebreakState,
};

uniffi::setup_scaffolding!();

// --- UniFFI enum wrappers ---

#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Player1,
    Player2,
}

impl From<CorePlayer> for Player {
    fn from(p: CorePlayer) -> Self {
        match p {
            CorePlayer::Player1 => Player::Player1,
            CorePlayer::Player2 => Player::Player2,
        }
    }
}

impl From<Player> for CorePlayer {
    fn from(p: Player) -> Self {
        match p {
            Player::Player1 => CorePlayer::Player1,
            Player::Player2 => CorePlayer::Player2,
        }
    }
}

#[derive(uniffi::Enum, Debug, Clone, PartialEq, Eq)]
pub enum GameScore {
    Points {
        player1: String,
        player2: String,
    },
    Deuce,
    Advantage {
        player: Player,
    },
    Completed {
        winner: Player,
    },
}

fn point_to_string(p: Point) -> String {
    match p {
        Point::Love => "0".to_string(),
        Point::Fifteen => "15".to_string(),
        Point::Thirty => "30".to_string(),
        Point::Forty => "40".to_string(),
    }
}

impl From<&CoreGameState> for GameScore {
    fn from(gs: &CoreGameState) -> Self {
        match gs {
            CoreGameState::Points { player1, player2 } => GameScore::Points {
                player1: point_to_string(*player1),
                player2: point_to_string(*player2),
            },
            CoreGameState::Deuce { .. } => GameScore::Deuce,
            CoreGameState::Advantage { player, .. } => GameScore::Advantage {
                player: (*player).into(),
            },
            CoreGameState::Completed(winner) => GameScore::Completed {
                winner: (*winner).into(),
            },
        }
    }
}

// --- UniFFI record for match score ---

#[derive(uniffi::Record, Debug, Clone)]
pub struct MatchScore {
    pub player1_sets: u8,
    pub player2_sets: u8,
    pub player1_games: Vec<u8>,
    pub player2_games: Vec<u8>,
    pub current_game: GameScore,
    pub winner: Option<Player>,
    pub is_tiebreak: bool,
}

fn extract_score(state: &MatchState) -> MatchScore {
    match state {
        MatchState::Playing {
            sets,
            player1_sets,
            player2_sets,
            ..
        } => {
            let mut p1_games = Vec::new();
            let mut p2_games = Vec::new();
            let mut current_game = GameScore::Points {
                player1: "0".to_string(),
                player2: "0".to_string(),
            };
            let mut is_tiebreak = false;

            for set in sets {
                match set {
                    SetState::Playing {
                        player1_games: p1g,
                        player2_games: p2g,
                        current_game: cg,
                        tiebreak,
                    } => {
                        p1_games.push(*p1g);
                        p2_games.push(*p2g);
                        if let Some(tb) = tiebreak {
                            is_tiebreak = true;
                            match tb {
                                TiebreakState::Playing {
                                    player1_points,
                                    player2_points,
                                    ..
                                } => {
                                    current_game = GameScore::Points {
                                        player1: player1_points.to_string(),
                                        player2: player2_points.to_string(),
                                    };
                                }
                                TiebreakState::Completed(winner) => {
                                    current_game = GameScore::Completed {
                                        winner: (*winner).into(),
                                    };
                                }
                            }
                        } else {
                            current_game = GameScore::from(cg);
                        }
                    }
                    SetState::Completed {
                        player1_games: p1g,
                        player2_games: p2g,
                        ..
                    } => {
                        p1_games.push(*p1g);
                        p2_games.push(*p2g);
                    }
                }
            }

            MatchScore {
                player1_sets: *player1_sets,
                player2_sets: *player2_sets,
                player1_games: p1_games,
                player2_games: p2_games,
                current_game,
                winner: None,
                is_tiebreak,
            }
        }
        MatchState::Completed {
            winner,
            player1_sets,
            player2_sets,
            sets,
        } => {
            let mut p1_games = Vec::new();
            let mut p2_games = Vec::new();
            for set in sets {
                match set {
                    SetState::Completed {
                        player1_games: p1g,
                        player2_games: p2g,
                        ..
                    } => {
                        p1_games.push(*p1g);
                        p2_games.push(*p2g);
                    }
                    SetState::Playing {
                        player1_games: p1g,
                        player2_games: p2g,
                        ..
                    } => {
                        p1_games.push(*p1g);
                        p2_games.push(*p2g);
                    }
                }
            }

            MatchScore {
                player1_sets: *player1_sets,
                player2_sets: *player2_sets,
                player1_games: p1_games,
                player2_games: p2_games,
                current_game: GameScore::Completed {
                    winner: (*winner).into(),
                },
                winner: Some((*winner).into()),
                is_tiebreak: false,
            }
        }
    }
}

// --- UniFFI Object wrapping MatchWithHistory ---

#[derive(uniffi::Object)]
pub struct TennisMatch {
    inner: RwLock<MatchWithHistory>,
}

#[uniffi::export]
impl TennisMatch {
    #[uniffi::constructor]
    pub fn new() -> Self {
        let config = MatchConfig::default();
        let state = MatchState::new(config);
        TennisMatch {
            inner: RwLock::new(MatchWithHistory::new(state)),
        }
    }

    pub fn score_point(&self, player: Player) -> MatchScore {
        let mut inner = self.inner.write().unwrap();
        let new_state = inner.score_point(CorePlayer::from(player));
        *inner = new_state;
        extract_score(inner.current())
    }

    pub fn get_score(&self) -> MatchScore {
        let inner = self.inner.read().unwrap();
        extract_score(inner.current())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_match_score() {
        let m = TennisMatch::new();
        let score = m.get_score();
        assert_eq!(score.player1_sets, 0);
        assert_eq!(score.player2_sets, 0);
        assert_eq!(score.winner, None);
    }

    #[test]
    fn test_score_point_returns_updated_score() {
        let m = TennisMatch::new();
        let score = m.score_point(Player::Player1);
        assert_eq!(
            score.current_game,
            GameScore::Points {
                player1: "15".to_string(),
                player2: "0".to_string(),
            }
        );
    }

    #[test]
    fn test_full_game() {
        let m = TennisMatch::new();
        m.score_point(Player::Player1); // 15-0
        m.score_point(Player::Player1); // 30-0
        m.score_point(Player::Player1); // 40-0
        let score = m.score_point(Player::Player1); // Game
        assert_eq!(score.player1_games, vec![1]);
        assert_eq!(
            score.current_game,
            GameScore::Points {
                player1: "0".to_string(),
                player2: "0".to_string(),
            }
        );
    }
}
