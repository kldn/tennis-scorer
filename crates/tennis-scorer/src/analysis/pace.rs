use serde::{Deserialize, Serialize};

use super::types::PointContext;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameDuration {
    pub set_number: u32,
    pub game_number: u32,
    pub duration_seconds: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDuration {
    pub set_number: u32,
    pub duration_seconds: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaceData {
    pub average_point_interval_seconds: f64,
    pub per_game_durations: Vec<GameDuration>,
    pub per_set_durations: Vec<SetDuration>,
    pub total_duration_seconds: f64,
    pub point_intervals: Vec<f64>,
}

pub fn compute_pace(points: &[PointContext]) -> PaceData {
    if points.is_empty() {
        return PaceData {
            average_point_interval_seconds: 0.0,
            per_game_durations: vec![],
            per_set_durations: vec![],
            total_duration_seconds: 0.0,
            point_intervals: vec![],
        };
    }

    // Point intervals
    let mut intervals = Vec::with_capacity(points.len().saturating_sub(1));
    for i in 1..points.len() {
        let dur = points[i]
            .timestamp
            .duration_since(points[i - 1].timestamp)
            .unwrap_or_default();
        intervals.push(dur.as_secs_f64());
    }

    let avg_interval = if intervals.is_empty() {
        0.0
    } else {
        intervals.iter().sum::<f64>() / intervals.len() as f64
    };

    // Total duration
    let total = if points.len() >= 2 {
        points
            .last()
            .unwrap()
            .timestamp
            .duration_since(points[0].timestamp)
            .unwrap_or_default()
            .as_secs_f64()
    } else {
        0.0
    };

    // Per-game durations
    let mut game_durations = Vec::new();
    let mut game_start_idx = 0;
    for i in 1..=points.len() {
        let game_ended = if i == points.len() {
            true
        } else {
            points[i].set_number != points[game_start_idx].set_number
                || points[i].game_number_in_set != points[game_start_idx].game_number_in_set
        };
        if game_ended {
            let game_end_idx = i - 1;
            let dur = if game_start_idx < game_end_idx {
                points[game_end_idx]
                    .timestamp
                    .duration_since(points[game_start_idx].timestamp)
                    .unwrap_or_default()
                    .as_secs_f64()
            } else {
                0.0
            };
            game_durations.push(GameDuration {
                set_number: points[game_start_idx].set_number,
                game_number: points[game_start_idx].game_number_in_set,
                duration_seconds: dur,
            });
            if i < points.len() {
                game_start_idx = i;
            }
        }
    }

    // Per-set durations
    let mut set_durations = Vec::new();
    let mut set_start_idx = 0;
    for i in 1..=points.len() {
        let set_ended = if i == points.len() {
            true
        } else {
            points[i].set_number != points[set_start_idx].set_number
        };
        if set_ended {
            let set_end_idx = i - 1;
            let dur = if set_start_idx < set_end_idx {
                points[set_end_idx]
                    .timestamp
                    .duration_since(points[set_start_idx].timestamp)
                    .unwrap_or_default()
                    .as_secs_f64()
            } else {
                0.0
            };
            set_durations.push(SetDuration {
                set_number: points[set_start_idx].set_number,
                duration_seconds: dur,
            });
            if i < points.len() {
                set_start_idx = i;
            }
        }
    }

    PaceData {
        average_point_interval_seconds: avg_interval,
        per_game_durations: game_durations,
        per_set_durations: set_durations,
        total_duration_seconds: total,
        point_intervals: intervals,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::replay::replay_with_context;
    use crate::config::MatchConfig;
    use crate::types::Player;
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
    fn test_empty_pace() {
        let result = compute_pace(&[]);
        assert_eq!(result.average_point_interval_seconds, 0.0);
        assert!(result.point_intervals.is_empty());
        assert_eq!(result.total_duration_seconds, 0.0);
    }

    #[test]
    fn test_single_point_pace() {
        let config = MatchConfig::default();
        let events = make_events(&[Player::Player1]);
        let contexts = replay_with_context(&config, &events);
        let pace = compute_pace(&contexts);

        assert!(pace.point_intervals.is_empty());
        assert_eq!(pace.average_point_interval_seconds, 0.0);
        assert_eq!(pace.total_duration_seconds, 0.0);
    }

    #[test]
    fn test_interval_calculation() {
        let config = MatchConfig::default();
        // 4 points, 30 seconds apart
        let events = make_events(&[
            Player::Player1,
            Player::Player1,
            Player::Player1,
            Player::Player1,
        ]);
        let contexts = replay_with_context(&config, &events);
        let pace = compute_pace(&contexts);

        assert_eq!(pace.point_intervals.len(), 3);
        for interval in &pace.point_intervals {
            assert!((interval - 30.0).abs() < 0.01);
        }
        assert!((pace.average_point_interval_seconds - 30.0).abs() < 0.01);
        assert!((pace.total_duration_seconds - 90.0).abs() < 0.01);
    }

    #[test]
    fn test_game_durations() {
        let config = MatchConfig::default();
        // One game: 4 points
        let events = make_events(&[
            Player::Player1,
            Player::Player1,
            Player::Player1,
            Player::Player1,
        ]);
        let contexts = replay_with_context(&config, &events);
        let pace = compute_pace(&contexts);

        assert!(!pace.per_game_durations.is_empty());
        assert_eq!(pace.per_game_durations[0].set_number, 1);
        assert_eq!(pace.per_game_durations[0].game_number, 1);
    }
}
