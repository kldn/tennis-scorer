#[cfg(test)]
mod tests {
    use tennis_scorer::{MatchConfig, MatchState, MatchWithHistory, Player};

    #[test]
    fn test_core_dependency() {
        let config = MatchConfig::default();
        let mwh = MatchWithHistory::new(MatchState::new(config));
        let mwh = mwh.score_point(Player::Player1);
        assert_eq!(mwh.point_events().len(), 1);
    }
}
