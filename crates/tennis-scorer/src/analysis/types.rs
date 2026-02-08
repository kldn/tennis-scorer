use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::types::Player;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PointEndType {
    Ace,
    DoubleFault,
    Winner,
    UnforcedError,
    ForcedError,
    Normal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameScore {
    pub player1_points: String,
    pub player2_points: String,
    pub is_deuce: bool,
    pub advantage: Option<Player>,
    pub deuce_count: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetScore {
    pub player1_games: u8,
    pub player2_games: u8,
    pub is_tiebreak: bool,
    pub tiebreak_player1_points: Option<u8>,
    pub tiebreak_player2_points: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScoreSnapshot {
    pub sets: Vec<SetScore>,
    pub current_game: GameScore,
    pub player1_sets: u8,
    pub player2_sets: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointContext {
    pub point_number: u32,
    pub scorer: Player,
    pub timestamp: SystemTime,
    pub serving_player: Player,
    pub score_before: ScoreSnapshot,
    pub is_break_point: bool,
    pub is_game_point: bool,
    pub is_set_point: bool,
    pub is_match_point: bool,
    pub game_number_in_set: u32,
    pub set_number: u32,
    pub is_tiebreak: bool,
    #[serde(default)]
    pub point_end_type: Option<PointEndType>,
}
