use std::time::SystemTime;

use crate::match_state::MatchState;
use crate::types::Player;

#[derive(Debug, Clone)]
pub struct MatchWithHistory {
    current: MatchState,
    history: Vec<MatchState>,
    point_events: Vec<(Player, SystemTime)>,
}

impl MatchWithHistory {
    pub fn new(state: MatchState) -> Self {
        Self {
            current: state,
            history: Vec::new(),
            point_events: Vec::new(),
        }
    }

    pub fn score_point(&self, scorer: Player) -> MatchWithHistory {
        if self.current.winner().is_some() {
            return self.clone();
        }

        let new_state = self.current.score_point(scorer);
        let mut new_history = self.history.clone();
        new_history.push(self.current.clone());

        let mut new_point_events = self.point_events.clone();
        new_point_events.push((scorer, SystemTime::now()));

        debug_assert_eq!(new_history.len(), new_point_events.len());

        MatchWithHistory {
            current: new_state,
            history: new_history,
            point_events: new_point_events,
        }
    }

    pub fn undo(&self) -> MatchWithHistory {
        if self.history.is_empty() {
            return self.clone();
        }

        let mut new_history = self.history.clone();
        let previous_state = new_history.pop().unwrap();

        let mut new_point_events = self.point_events.clone();
        new_point_events.pop();

        debug_assert_eq!(new_history.len(), new_point_events.len());

        MatchWithHistory {
            current: previous_state,
            history: new_history,
            point_events: new_point_events,
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

    pub fn point_events(&self) -> &[(Player, SystemTime)] {
        &self.point_events
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::MatchConfig;
    use std::time::Duration;

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

    #[test]
    fn test_score_point_adds_timestamp() {
        let state = MatchState::new(MatchConfig::default());
        let mwh = MatchWithHistory::new(state);

        let before = SystemTime::now();
        let mwh = mwh.score_point(Player::Player1);
        let after = SystemTime::now();

        assert_eq!(mwh.point_events().len(), 1);
        let (player, timestamp) = &mwh.point_events()[0];
        assert_eq!(*player, Player::Player1);
        assert!(*timestamp >= before);
        assert!(*timestamp <= after);
    }

    #[test]
    fn test_undo_removes_timestamp() {
        let state = MatchState::new(MatchConfig::default());
        let mwh = MatchWithHistory::new(state);

        let mwh = mwh.score_point(Player::Player1);
        assert_eq!(mwh.point_events().len(), 1);

        let mwh = mwh.undo();
        assert_eq!(mwh.point_events().len(), 0);
    }

    #[test]
    fn test_point_events_matches_history_len() {
        let state = MatchState::new(MatchConfig::default());
        let mwh = MatchWithHistory::new(state);

        let mwh = mwh.score_point(Player::Player1);
        let mwh = mwh.score_point(Player::Player2);
        let mwh = mwh.score_point(Player::Player1);
        assert_eq!(mwh.point_events().len(), mwh.history_len());

        let mwh = mwh.undo();
        assert_eq!(mwh.point_events().len(), mwh.history_len());

        let mwh = mwh.undo();
        let mwh = mwh.undo();
        assert_eq!(mwh.point_events().len(), mwh.history_len());
        assert_eq!(mwh.point_events().len(), 0);
    }

    #[test]
    fn test_rescore_after_undo_gets_new_timestamp() {
        let state = MatchState::new(MatchConfig::default());
        let mwh = MatchWithHistory::new(state);

        let mwh = mwh.score_point(Player::Player1);
        let original_timestamp = mwh.point_events()[0].1;

        let mwh = mwh.undo();
        std::thread::sleep(Duration::from_millis(1));
        let mwh = mwh.score_point(Player::Player2);

        let new_timestamp = mwh.point_events()[0].1;
        assert!(new_timestamp > original_timestamp);
        assert_eq!(mwh.point_events()[0].0, Player::Player2);
    }

    #[test]
    fn test_completed_match_no_timestamp() {
        let state = MatchState::Completed {
            winner: Player::Player1,
            player1_sets: 2,
            player2_sets: 0,
            sets: vec![],
        };
        let mwh = MatchWithHistory::new(state);

        let mwh = mwh.score_point(Player::Player2);
        assert_eq!(mwh.point_events().len(), 0);
    }
}
