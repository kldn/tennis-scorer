use crate::types::Player;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TiebreakState {
    Playing {
        player1_points: u8,
        player2_points: u8,
        target_points: u8,
    },
    Completed(Player),
}

impl TiebreakState {
    pub fn new(target_points: u8) -> Self {
        TiebreakState::Playing {
            player1_points: 0,
            player2_points: 0,
            target_points,
        }
    }

    pub fn score_point(&self, scorer: Player) -> TiebreakState {
        match self {
            TiebreakState::Completed(_) => self.clone(),

            TiebreakState::Playing {
                player1_points,
                player2_points,
                target_points,
            } => {
                let (new_p1, new_p2) = match scorer {
                    Player::Player1 => (player1_points + 1, *player2_points),
                    Player::Player2 => (*player1_points, player2_points + 1),
                };

                let leader_points = new_p1.max(new_p2);
                let trailer_points = new_p1.min(new_p2);
                let lead = leader_points - trailer_points;

                if leader_points >= *target_points && lead >= 2 {
                    let winner = if new_p1 > new_p2 {
                        Player::Player1
                    } else {
                        Player::Player2
                    };
                    TiebreakState::Completed(winner)
                } else {
                    TiebreakState::Playing {
                        player1_points: new_p1,
                        player2_points: new_p2,
                        target_points: *target_points,
                    }
                }
            }
        }
    }

    pub fn winner(&self) -> Option<Player> {
        match self {
            TiebreakState::Completed(player) => Some(*player),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_tiebreak_win() {
        let mut tb = TiebreakState::new(7);
        for _ in 0..7 {
            tb = tb.score_point(Player::Player1);
        }
        assert_eq!(tb, TiebreakState::Completed(Player::Player1));
    }

    #[test]
    fn test_tiebreak_needs_two_point_lead() {
        let tb = TiebreakState::Playing {
            player1_points: 6,
            player2_points: 6,
            target_points: 7,
        };
        let tb = tb.score_point(Player::Player1);
        assert!(matches!(tb, TiebreakState::Playing { .. }));

        let tb = tb.score_point(Player::Player1);
        assert_eq!(tb, TiebreakState::Completed(Player::Player1));
    }

    #[test]
    fn test_super_tiebreak() {
        let mut tb = TiebreakState::new(10);
        for _ in 0..10 {
            tb = tb.score_point(Player::Player2);
        }
        assert_eq!(tb, TiebreakState::Completed(Player::Player2));
    }

    #[test]
    fn test_extended_tiebreak() {
        let tb = TiebreakState::Playing {
            player1_points: 9,
            player2_points: 9,
            target_points: 7,
        };
        let tb = tb.score_point(Player::Player1);
        assert!(matches!(tb, TiebreakState::Playing { .. }));

        let tb = tb.score_point(Player::Player1);
        assert_eq!(tb, TiebreakState::Completed(Player::Player1));
    }

    #[test]
    fn test_completed_tiebreak_no_change() {
        let tb = TiebreakState::Completed(Player::Player1);
        let tb = tb.score_point(Player::Player2);
        assert_eq!(tb, TiebreakState::Completed(Player::Player1));
    }
}
