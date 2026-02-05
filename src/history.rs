use crate::match_state::MatchState;
use crate::types::Player;

#[derive(Debug, Clone)]
pub struct MatchWithHistory {
    current: MatchState,
    history: Vec<MatchState>,
}

impl MatchWithHistory {
    pub fn new(state: MatchState) -> Self {
        Self {
            current: state,
            history: Vec::new(),
        }
    }

    pub fn score_point(&self, scorer: Player) -> MatchWithHistory {
        if self.current.winner().is_some() {
            return self.clone();
        }

        let new_state = self.current.score_point(scorer);
        let mut new_history = self.history.clone();
        new_history.push(self.current.clone());

        MatchWithHistory {
            current: new_state,
            history: new_history,
        }
    }

    pub fn undo(&self) -> MatchWithHistory {
        if self.history.is_empty() {
            return self.clone();
        }

        let mut new_history = self.history.clone();
        let previous_state = new_history.pop().unwrap();

        MatchWithHistory {
            current: previous_state,
            history: new_history,
        }
    }

    pub fn current(&self) -> &MatchState {
        &self.current
    }

    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    pub fn can_undo(&self) -> bool {
        !self.history.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::MatchConfig;

    #[test]
    fn test_score_and_undo() {
        let state = MatchState::new(MatchConfig::default());
        let mwh = MatchWithHistory::new(state);

        let mwh = mwh.score_point(Player::Player1);
        assert_eq!(mwh.history_len(), 1);

        let mwh = mwh.undo();
        assert_eq!(mwh.history_len(), 0);
    }

    #[test]
    fn test_multiple_undo() {
        let state = MatchState::new(MatchConfig::default());
        let mwh = MatchWithHistory::new(state);

        let mwh = mwh.score_point(Player::Player1);
        let mwh = mwh.score_point(Player::Player1);
        let mwh = mwh.score_point(Player::Player2);

        assert_eq!(mwh.history_len(), 3);

        let mwh = mwh.undo();
        let mwh = mwh.undo();
        let mwh = mwh.undo();

        assert_eq!(mwh.history_len(), 0);
        assert!(!mwh.can_undo());
    }

    #[test]
    fn test_undo_empty_history() {
        let state = MatchState::new(MatchConfig::default());
        let mwh = MatchWithHistory::new(state);

        let mwh = mwh.undo();
        assert_eq!(mwh.history_len(), 0);
    }

    #[test]
    fn test_undo_and_rescore() {
        let state = MatchState::new(MatchConfig::default());
        let mwh = MatchWithHistory::new(state);

        let mwh = mwh.score_point(Player::Player1);
        let mwh = mwh.score_point(Player::Player1);

        let mwh = mwh.undo();
        assert_eq!(mwh.history_len(), 1);

        let mwh = mwh.score_point(Player::Player2);
        assert_eq!(mwh.history_len(), 2);
    }

    #[test]
    fn test_no_history_on_completed_match() {
        let state = MatchState::Completed {
            winner: Player::Player1,
            player1_sets: 2,
            player2_sets: 0,
            sets: vec![],
        };
        let mwh = MatchWithHistory::new(state);
        let initial_len = mwh.history_len();

        let mwh = mwh.score_point(Player::Player2);
        assert_eq!(mwh.history_len(), initial_len);
    }
}
