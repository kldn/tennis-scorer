use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Player {
    Player1,
    Player2,
}

impl Player {
    pub fn opponent(self) -> Player {
        match self {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point {
    Love,
    Fifteen,
    Thirty,
    Forty,
}

impl Point {
    pub fn increment(self) -> Option<Point> {
        match self {
            Point::Love => Some(Point::Fifteen),
            Point::Fifteen => Some(Point::Thirty),
            Point::Thirty => Some(Point::Forty),
            Point::Forty => None,
        }
    }
}
