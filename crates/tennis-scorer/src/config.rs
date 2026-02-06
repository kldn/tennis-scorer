use crate::types::Player;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MatchType {
    #[default]
    Singles,
    Doubles,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchConfig {
    pub sets_to_win: u8,
    pub tiebreak_points: u8,
    pub final_set_tiebreak: bool,
    pub no_ad_scoring: bool,
    #[serde(default)]
    pub match_type: MatchType,
    #[serde(default)]
    pub serve_order: Vec<(Player, u8)>,
}

impl Default for MatchConfig {
    fn default() -> Self {
        Self {
            sets_to_win: 2,
            tiebreak_points: 7,
            final_set_tiebreak: true,
            no_ad_scoring: false,
            match_type: MatchType::Singles,
            serve_order: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_is_singles() {
        let config = MatchConfig::default();
        assert_eq!(config.match_type, MatchType::Singles);
        assert!(config.serve_order.is_empty());
    }

    #[test]
    fn test_doubles_config() {
        let config = MatchConfig {
            match_type: MatchType::Doubles,
            serve_order: vec![
                (Player::Player1, 0),
                (Player::Player2, 0),
                (Player::Player1, 1),
                (Player::Player2, 1),
            ],
            ..MatchConfig::default()
        };
        assert_eq!(config.match_type, MatchType::Doubles);
        assert_eq!(config.serve_order.len(), 4);
        assert_eq!(config.serve_order[0], (Player::Player1, 0));
        assert_eq!(config.serve_order[1], (Player::Player2, 0));
        assert_eq!(config.serve_order[2], (Player::Player1, 1));
        assert_eq!(config.serve_order[3], (Player::Player2, 1));
    }

    #[test]
    fn test_serde_roundtrip_singles() {
        let config = MatchConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: MatchConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_serde_backward_compat() {
        // Old config without match_type and serve_order should deserialize with defaults
        let json = r#"{"sets_to_win":2,"tiebreak_points":7,"final_set_tiebreak":true,"no_ad_scoring":false}"#;
        let config: MatchConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.match_type, MatchType::Singles);
        assert!(config.serve_order.is_empty());
    }
}
