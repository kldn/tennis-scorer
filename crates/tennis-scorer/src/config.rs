use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchConfig {
    pub sets_to_win: u8,
    pub tiebreak_points: u8,
    pub final_set_tiebreak: bool,
    pub no_ad_scoring: bool,
}

impl Default for MatchConfig {
    fn default() -> Self {
        Self {
            sets_to_win: 2,
            tiebreak_points: 7,
            final_set_tiebreak: true,
            no_ad_scoring: false,
        }
    }
}
