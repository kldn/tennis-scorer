use serde::{Deserialize, Serialize};

use crate::types::Player;

use super::types::PointContext;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MomentumData {
    pub basic: Vec<f64>,
    pub weighted: Vec<f64>,
    pub per_set_basic: Vec<Vec<f64>>,
    pub per_set_weighted: Vec<Vec<f64>>,
}

pub fn compute_momentum(points: &[PointContext]) -> MomentumData {
    if points.is_empty() {
        return MomentumData {
            basic: vec![],
            weighted: vec![],
            per_set_basic: vec![],
            per_set_weighted: vec![],
        };
    }

    let mut basic = Vec::with_capacity(points.len());
    let mut weighted = Vec::with_capacity(points.len());

    let mut cumulative_basic = 0.0f64;
    let mut cumulative_weighted = 0.0f64;

    for p in points {
        let sign = if p.scorer == Player::Player1 {
            1.0
        } else {
            -1.0
        };

        cumulative_basic += sign;
        basic.push(cumulative_basic);

        let weight = compute_weight(p);
        cumulative_weighted += sign * weight;
        weighted.push(cumulative_weighted);
    }

    // Per-set partitioning
    let mut per_set_basic: Vec<Vec<f64>> = Vec::new();
    let mut per_set_weighted: Vec<Vec<f64>> = Vec::new();
    let mut current_set = 0u32;
    let mut set_basic = Vec::new();
    let mut set_weighted = Vec::new();
    let mut set_cum_basic = 0.0f64;
    let mut set_cum_weighted = 0.0f64;

    for p in points {
        if p.set_number != current_set {
            if current_set != 0 {
                per_set_basic.push(set_basic);
                per_set_weighted.push(set_weighted);
            }
            current_set = p.set_number;
            set_basic = Vec::new();
            set_weighted = Vec::new();
            set_cum_basic = 0.0;
            set_cum_weighted = 0.0;
        }

        let sign = if p.scorer == Player::Player1 {
            1.0
        } else {
            -1.0
        };
        set_cum_basic += sign;
        set_basic.push(set_cum_basic);

        let weight = compute_weight(p);
        set_cum_weighted += sign * weight;
        set_weighted.push(set_cum_weighted);
    }
    // Push the last set
    if !set_basic.is_empty() {
        per_set_basic.push(set_basic);
        per_set_weighted.push(set_weighted);
    }

    MomentumData {
        basic,
        weighted,
        per_set_basic,
        per_set_weighted,
    }
}

/// Compute the weight multiplier for a point based on its context.
/// Break point ×3, set point ×5, deuce game ×1.5 — multiplicative stacking.
fn compute_weight(p: &PointContext) -> f64 {
    let mut weight = 1.0f64;

    // Break point conversion: ×3
    if p.is_break_point {
        weight *= 3.0;
    }

    // Set point: ×5
    if p.is_set_point {
        weight *= 5.0;
    }

    // Deuce game: ×1.5
    if p.score_before.current_game.is_deuce
        || p.score_before.current_game.advantage.is_some()
        || p.score_before.current_game.deuce_count > 0
    {
        weight *= 1.5;
    }

    weight
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::replay::replay_with_context;
    use crate::config::MatchConfig;
    use std::time::{Duration, SystemTime};

    fn ts(secs: u64) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(1_000_000 + secs)
    }

    fn make_events(scorers: &[Player]) -> Vec<(Player, SystemTime)> {
        scorers
            .iter()
            .enumerate()
            .map(|(i, p)| (*p, ts(i as u64 * 30)))
            .collect()
    }

    #[test]
    fn test_empty_momentum() {
        let result = compute_momentum(&[]);
        assert!(result.basic.is_empty());
        assert!(result.weighted.is_empty());
        assert!(result.per_set_basic.is_empty());
        assert!(result.per_set_weighted.is_empty());
    }

    #[test]
    fn test_basic_momentum_p1_wins_3() {
        let config = MatchConfig::default();
        let events = make_events(&[Player::Player1, Player::Player1, Player::Player1]);
        let contexts = replay_with_context(&config, &events);
        let m = compute_momentum(&contexts);

        assert_eq!(m.basic, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_basic_momentum_alternating() {
        let config = MatchConfig::default();
        let events = make_events(&[Player::Player1, Player::Player2, Player::Player1]);
        let contexts = replay_with_context(&config, &events);
        let m = compute_momentum(&contexts);

        assert_eq!(m.basic, vec![1.0, 0.0, 1.0]);
    }

    #[test]
    fn test_per_set_partitioning() {
        let config = MatchConfig::default();
        let mut scorers = Vec::new();
        // Set 1: P1 wins 6-0 (24 pts)
        for _ in 0..6 {
            for _ in 0..4 {
                scorers.push(Player::Player1);
            }
        }
        // Set 2: first 3 pts
        scorers.push(Player::Player2);
        scorers.push(Player::Player2);
        scorers.push(Player::Player1);

        let events = make_events(&scorers);
        let contexts = replay_with_context(&config, &events);
        let m = compute_momentum(&contexts);

        assert_eq!(m.per_set_basic.len(), 2);
        assert_eq!(m.per_set_basic[0].len(), 24);
        assert_eq!(m.per_set_basic[1].len(), 3);
        // Set 2 starts from 0
        assert_eq!(m.per_set_basic[1][0], -1.0); // P2 wins first point of set 2
    }
}
