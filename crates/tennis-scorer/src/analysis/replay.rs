use std::time::SystemTime;

use crate::config::MatchConfig;
use crate::game::GameState;
use crate::match_state::MatchState;
use crate::set::SetState;
use crate::tiebreak::TiebreakState;
use crate::types::{Player, Point};

use super::types::{GameScore, PointContext, ScoreSnapshot, SetScore};

pub fn replay_with_context(
    config: &MatchConfig,
    events: &[(Player, SystemTime)],
) -> Vec<PointContext> {
    let mut state = MatchState::new(config.clone());
    let mut contexts = Vec::with_capacity(events.len());
    let mut total_completed_games: u32 = 0;

    for (i, (scorer, timestamp)) in events.iter().enumerate() {
        let serving_player = determine_server(&state, config, total_completed_games);
        let score_before = score_snapshot_from_state(&state);
        let (game_number_in_set, set_number) = current_position(&state);
        let is_tiebreak = is_in_tiebreak(&state);

        let is_game_point = is_game_point_for_either(&state, is_tiebreak);
        let is_break_point = is_break_point_state(&state, serving_player, is_tiebreak);
        let is_set_point = is_set_point_state(&state, is_tiebreak);
        let is_match_point = is_match_point_state(&state);

        contexts.push(PointContext {
            point_number: (i + 1) as u32,
            scorer: *scorer,
            timestamp: *timestamp,
            serving_player,
            score_before,
            is_break_point,
            is_game_point,
            is_set_point,
            is_match_point,
            game_number_in_set,
            set_number,
            is_tiebreak,
            point_end_type: None,
        });

        let old_games = count_games_in_current_set(&state);
        state = state.score_point(*scorer);
        let new_games = count_games_in_current_set(&state);

        if new_games > old_games || set_just_completed(&state, set_number) {
            total_completed_games += 1;
        }
    }

    contexts
}

fn determine_server(
    state: &MatchState,
    config: &MatchConfig,
    total_completed_games: u32,
) -> Player {
    if !config.serve_order.is_empty() {
        // Doubles: use the state's current_server index into serve_order
        let idx = state.current_server() as usize;
        if idx < config.serve_order.len() {
            config.serve_order[idx].0
        } else {
            Player::Player1
        }
    } else {
        // Singles: game parity
        if is_in_tiebreak(state) {
            // During tiebreak, use tiebreak serving pattern
            let tb_points = tiebreak_points_played(state);
            let game_before_tb = total_completed_games;
            let base_server = if game_before_tb % 2 == 0 {
                Player::Player1
            } else {
                Player::Player2
            };
            let offset = if tb_points == 0 {
                0
            } else {
                (tb_points as usize).div_ceil(2)
            };
            if offset % 2 == 0 {
                base_server
            } else {
                base_server.opponent()
            }
        } else if total_completed_games % 2 == 0 {
            Player::Player1
        } else {
            Player::Player2
        }
    }
}

fn tiebreak_points_played(state: &MatchState) -> u8 {
    match state {
        MatchState::Playing {
            tiebreak_points_served,
            ..
        } => *tiebreak_points_served,
        _ => 0,
    }
}

fn is_in_tiebreak(state: &MatchState) -> bool {
    match state {
        MatchState::Playing { sets, .. } => {
            if let Some(current_set) = sets.last() {
                matches!(
                    current_set,
                    SetState::Playing {
                        tiebreak: Some(_),
                        ..
                    }
                )
            } else {
                false
            }
        }
        _ => false,
    }
}

fn count_games_in_current_set(state: &MatchState) -> u8 {
    match state {
        MatchState::Playing { sets, .. } => match sets.last() {
            Some(SetState::Playing {
                player1_games,
                player2_games,
                ..
            }) => player1_games + player2_games,
            Some(SetState::Completed {
                player1_games,
                player2_games,
                ..
            }) => player1_games + player2_games,
            None => 0,
        },
        _ => 0,
    }
}

fn set_just_completed(state: &MatchState, prev_set_number: u32) -> bool {
    match state {
        MatchState::Playing { sets, .. } => sets.len() as u32 > prev_set_number,
        MatchState::Completed { .. } => true,
    }
}

fn current_position(state: &MatchState) -> (u32, u32) {
    match state {
        MatchState::Playing { sets, .. } => {
            let set_number = sets.len() as u32;
            let game_number = match sets.last() {
                Some(SetState::Playing {
                    player1_games,
                    player2_games,
                    ..
                }) => (player1_games + player2_games) as u32 + 1,
                _ => 1,
            };
            (game_number, set_number)
        }
        MatchState::Completed { sets, .. } => {
            let set_number = sets.len() as u32;
            (0, set_number)
        }
    }
}

pub fn score_snapshot_from_state(state: &MatchState) -> ScoreSnapshot {
    match state {
        MatchState::Playing {
            sets,
            player1_sets,
            player2_sets,
            ..
        } => {
            let mut set_scores = Vec::new();
            for set in sets.iter() {
                set_scores.push(set_score_from_state(set));
            }
            let current_game = match sets.last() {
                Some(SetState::Playing {
                    current_game,
                    tiebreak,
                    ..
                }) => {
                    if let Some(tb) = tiebreak {
                        game_score_from_tiebreak(tb)
                    } else {
                        game_score_from_state(current_game)
                    }
                }
                _ => GameScore {
                    player1_points: "0".to_string(),
                    player2_points: "0".to_string(),
                    is_deuce: false,
                    advantage: None,
                    deuce_count: 0,
                },
            };
            ScoreSnapshot {
                sets: set_scores,
                current_game,
                player1_sets: *player1_sets,
                player2_sets: *player2_sets,
            }
        }
        MatchState::Completed {
            sets,
            player1_sets,
            player2_sets,
            ..
        } => {
            let set_scores = sets.iter().map(set_score_from_state).collect();
            ScoreSnapshot {
                sets: set_scores,
                current_game: GameScore {
                    player1_points: "0".to_string(),
                    player2_points: "0".to_string(),
                    is_deuce: false,
                    advantage: None,
                    deuce_count: 0,
                },
                player1_sets: *player1_sets,
                player2_sets: *player2_sets,
            }
        }
    }
}

fn set_score_from_state(set: &SetState) -> SetScore {
    match set {
        SetState::Playing {
            player1_games,
            player2_games,
            tiebreak,
            ..
        } => {
            let (tb_p1, tb_p2) = match tiebreak {
                Some(TiebreakState::Playing {
                    player1_points,
                    player2_points,
                    ..
                }) => (Some(*player1_points), Some(*player2_points)),
                Some(TiebreakState::Completed(_)) => (None, None),
                None => (None, None),
            };
            SetScore {
                player1_games: *player1_games,
                player2_games: *player2_games,
                is_tiebreak: tiebreak.is_some(),
                tiebreak_player1_points: tb_p1,
                tiebreak_player2_points: tb_p2,
            }
        }
        SetState::Completed {
            player1_games,
            player2_games,
            ..
        } => SetScore {
            player1_games: *player1_games,
            player2_games: *player2_games,
            is_tiebreak: false,
            tiebreak_player1_points: None,
            tiebreak_player2_points: None,
        },
    }
}

fn game_score_from_state(game: &GameState) -> GameScore {
    match game {
        GameState::Points { player1, player2 } => GameScore {
            player1_points: point_to_string(*player1),
            player2_points: point_to_string(*player2),
            is_deuce: false,
            advantage: None,
            deuce_count: 0,
        },
        GameState::Deuce { count } => GameScore {
            player1_points: "40".to_string(),
            player2_points: "40".to_string(),
            is_deuce: true,
            advantage: None,
            deuce_count: *count,
        },
        GameState::Advantage {
            player,
            deuce_count,
        } => GameScore {
            player1_points: "40".to_string(),
            player2_points: "40".to_string(),
            is_deuce: false,
            advantage: Some(*player),
            deuce_count: *deuce_count,
        },
        GameState::Completed(_) => GameScore {
            player1_points: "0".to_string(),
            player2_points: "0".to_string(),
            is_deuce: false,
            advantage: None,
            deuce_count: 0,
        },
    }
}

fn game_score_from_tiebreak(tb: &TiebreakState) -> GameScore {
    match tb {
        TiebreakState::Playing {
            player1_points,
            player2_points,
            ..
        } => GameScore {
            player1_points: player1_points.to_string(),
            player2_points: player2_points.to_string(),
            is_deuce: false,
            advantage: None,
            deuce_count: 0,
        },
        TiebreakState::Completed(_) => GameScore {
            player1_points: "0".to_string(),
            player2_points: "0".to_string(),
            is_deuce: false,
            advantage: None,
            deuce_count: 0,
        },
    }
}

fn point_to_string(p: Point) -> String {
    match p {
        Point::Love => "0".to_string(),
        Point::Fifteen => "15".to_string(),
        Point::Thirty => "30".to_string(),
        Point::Forty => "40".to_string(),
    }
}

/// Check if either player is at game point (one point from winning the game)
fn is_game_point_for_either(state: &MatchState, in_tiebreak: bool) -> bool {
    match state {
        MatchState::Playing { sets, config, .. } => match sets.last() {
            Some(SetState::Playing {
                current_game,
                tiebreak,
                ..
            }) => {
                if in_tiebreak {
                    if let Some(TiebreakState::Playing {
                        player1_points,
                        player2_points,
                        target_points,
                    }) = tiebreak
                    {
                        let leader = (*player1_points).max(*player2_points);
                        let trailer = (*player1_points).min(*player2_points);
                        // Game point if leader >= target-1 and leader > trailer,
                        // or both >= target-1 (one more point with 2-pt lead could win)
                        leader >= target_points - 1
                            && (leader > trailer || trailer >= target_points - 1)
                    } else {
                        false
                    }
                } else {
                    match current_game {
                        GameState::Points { player1, player2 } => {
                            *player1 == Point::Forty || *player2 == Point::Forty
                        }
                        GameState::Deuce { .. } => {
                            // At deuce, no one is at game point (need advantage first)
                            // Unless no-ad scoring
                            config.no_ad_scoring
                        }
                        GameState::Advantage { .. } => true,
                        GameState::Completed(_) => false,
                    }
                }
            }
            _ => false,
        },
        _ => false,
    }
}

/// Check if the returner could win the game (break point)
fn is_break_point_state(state: &MatchState, serving_player: Player, in_tiebreak: bool) -> bool {
    match state {
        MatchState::Playing { sets, config, .. } => match sets.last() {
            Some(SetState::Playing {
                current_game,
                tiebreak,
                ..
            }) => {
                if in_tiebreak {
                    if let Some(TiebreakState::Playing {
                        player1_points,
                        player2_points,
                        target_points,
                    }) = tiebreak
                    {
                        let (returner_pts, server_pts) = match serving_player {
                            Player::Player1 => (*player2_points, *player1_points),
                            Player::Player2 => (*player1_points, *player2_points),
                        };
                        // Returner at game point: returner >= target-1 and returner > server
                        // Or at extended tiebreak: both >= target-1 and returner > server
                        returner_pts >= target_points - 1 && returner_pts > server_pts
                    } else {
                        false
                    }
                } else {
                    let returner = serving_player.opponent();
                    match current_game {
                        GameState::Points { player1, player2 } => {
                            let returner_pts = match returner {
                                Player::Player1 => *player1,
                                Player::Player2 => *player2,
                            };
                            let server_pts = match serving_player {
                                Player::Player1 => *player1,
                                Player::Player2 => *player2,
                            };
                            returner_pts == Point::Forty && server_pts != Point::Forty
                        }
                        GameState::Deuce { .. } => {
                            // At deuce with no-ad, both could win = break point for returner
                            config.no_ad_scoring
                        }
                        GameState::Advantage { player, .. } => *player == returner,
                        GameState::Completed(_) => false,
                    }
                }
            }
            _ => false,
        },
        _ => false,
    }
}

/// Check if winning this game could win the set for either player
fn is_set_point_state(state: &MatchState, in_tiebreak: bool) -> bool {
    if !is_game_point_for_either(state, in_tiebreak) {
        return false;
    }

    match state {
        MatchState::Playing { sets, config, .. } => match sets.last() {
            Some(SetState::Playing {
                player1_games,
                player2_games,
                tiebreak,
                ..
            }) => {
                if tiebreak.is_some() {
                    // In a tiebreak, winning it always wins the set
                    return true;
                }
                // Check if winning one more game would win the set
                let p1_if_win = player1_games + 1;
                let p2_if_win = player2_games + 1;

                let player1_sets = match state {
                    MatchState::Playing { player1_sets, .. } => *player1_sets,
                    _ => 0,
                };
                let player2_sets = match state {
                    MatchState::Playing { player2_sets, .. } => *player2_sets,
                    _ => 0,
                };
                let is_final_set = player1_sets == config.sets_to_win - 1
                    && player2_sets == config.sets_to_win - 1;

                would_win_set(p1_if_win, *player2_games, is_final_set, config.final_set_tiebreak)
                    || would_win_set(
                        *player1_games,
                        p2_if_win,
                        is_final_set,
                        config.final_set_tiebreak,
                    )
            }
            _ => false,
        },
        _ => false,
    }
}

fn would_win_set(p1_games: u8, p2_games: u8, _is_final_set: bool, _final_set_tiebreak: bool) -> bool {
    let leader = p1_games.max(p2_games);
    let trailer = p1_games.min(p2_games);
    let lead = leader - trailer;
    (leader >= 6 && lead >= 2) || (leader == 7 && trailer == 6)
}

/// Check if winning this set would win the match
fn is_match_point_state(state: &MatchState) -> bool {
    match state {
        MatchState::Playing { config, .. } => {
            let in_tb = is_in_tiebreak(state);
            if !is_game_point_for_either(state, in_tb) {
                return false;
            }
            if !is_set_point_state(state, in_tb) {
                return false;
            }
            let (p1_sets, p2_sets) = match state {
                MatchState::Playing {
                    player1_sets,
                    player2_sets,
                    ..
                } => (*player1_sets, *player2_sets),
                _ => (0, 0),
            };
            p1_sets == config.sets_to_win - 1 || p2_sets == config.sets_to_win - 1
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

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
    fn test_empty_events() {
        let config = MatchConfig::default();
        let result = replay_with_context(&config, &[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_simple_game_p1_wins() {
        let config = MatchConfig::default();
        // P1 wins a game: 4 points
        let events = make_events(&[
            Player::Player1,
            Player::Player1,
            Player::Player1,
            Player::Player1,
        ]);
        let contexts = replay_with_context(&config, &events);

        assert_eq!(contexts.len(), 4);
        // All points in game 1, set 1
        for c in &contexts {
            assert_eq!(c.set_number, 1);
            assert_eq!(c.game_number_in_set, 1);
            assert_eq!(c.serving_player, Player::Player1); // Game 0 = P1 serves
            assert!(!c.is_tiebreak);
        }
        // Point 1: 0-0, not game point
        assert!(!contexts[0].is_game_point);
        // Point 3: 30-0, not game point
        assert!(!contexts[2].is_game_point);
        // Point 4: 40-0, game point for P1
        assert!(contexts[3].is_game_point);
        assert!(!contexts[3].is_break_point); // Server is winning
    }

    #[test]
    fn test_break_point_detection() {
        let config = MatchConfig::default();
        // P1 serves, P2 gets to 40-0 (break point)
        let events = make_events(&[
            Player::Player2,
            Player::Player2,
            Player::Player2, // 0-40: break point
            Player::Player2, // P2 breaks
        ]);
        let contexts = replay_with_context(&config, &events);

        assert_eq!(contexts.len(), 4);
        // Point 3: 0-30 -> P2 scores to make it 0-40
        // Actually, point 3 is scored when score is 0-30
        // Point 4: score is 0-40, break point
        assert!(contexts[3].is_break_point);
        assert!(contexts[3].is_game_point);
    }

    #[test]
    fn test_serving_alternates() {
        let config = MatchConfig::default();
        // Two quick games: P1 wins game 1, P2 wins game 2
        let mut scorers = Vec::new();
        for _ in 0..4 {
            scorers.push(Player::Player1);
        }
        for _ in 0..4 {
            scorers.push(Player::Player2);
        }
        let events = make_events(&scorers);
        let contexts = replay_with_context(&config, &events);

        // Game 1: P1 serves
        for i in 0..4 {
            assert_eq!(contexts[i].serving_player, Player::Player1);
        }
        // Game 2: P2 serves
        for i in 4..8 {
            assert_eq!(contexts[i].serving_player, Player::Player2);
        }
    }

    #[test]
    fn test_set_point_detection() {
        let config = MatchConfig::default();
        // P1 wins 5 games, P2 wins 3 games, then P1 at 40-0 in game 9
        let mut scorers = Vec::new();
        // 5 games for P1
        for _ in 0..5 {
            for _ in 0..4 {
                scorers.push(Player::Player1);
            }
        }
        // 3 games for P2
        for _ in 0..3 {
            for _ in 0..4 {
                scorers.push(Player::Player2);
            }
        }
        // Game 9: P1 gets to 40-0 (set point)
        scorers.push(Player::Player1); // 15-0
        scorers.push(Player::Player1); // 30-0
        scorers.push(Player::Player1); // 40-0
        scorers.push(Player::Player1); // wins set

        let events = make_events(&scorers);
        let contexts = replay_with_context(&config, &events);

        // The 40-0 point (last point of the set) should be set point
        let last = contexts.last().unwrap();
        assert!(last.is_set_point);
        assert!(last.is_game_point);
    }

    #[test]
    fn test_match_point_detection() {
        let config = MatchConfig::default(); // best of 3 sets (sets_to_win = 2)
        let mut scorers = Vec::new();

        // P1 wins set 1: 6-0 (24 points)
        for _ in 0..6 {
            for _ in 0..4 {
                scorers.push(Player::Player1);
            }
        }
        // P1 leads 5-0 in set 2 (20 more points)
        for _ in 0..5 {
            for _ in 0..4 {
                scorers.push(Player::Player1);
            }
        }
        // Game in set 2: P1 gets to 40-0 (match point)
        scorers.push(Player::Player1);
        scorers.push(Player::Player1);
        scorers.push(Player::Player1);
        scorers.push(Player::Player1); // wins match

        let events = make_events(&scorers);
        let contexts = replay_with_context(&config, &events);

        let last = contexts.last().unwrap();
        assert!(last.is_match_point);
        assert!(last.is_set_point);
        assert!(last.is_game_point);
    }

    #[test]
    fn test_tiebreak_detection() {
        let config = MatchConfig::default();
        let mut scorers = Vec::new();

        // Get to 6-6: alternate games
        for _ in 0..6 {
            for _ in 0..4 {
                scorers.push(Player::Player1);
            }
            for _ in 0..4 {
                scorers.push(Player::Player2);
            }
        }
        // Now in tiebreak: P1 wins 7-0
        for _ in 0..7 {
            scorers.push(Player::Player1);
        }

        let events = make_events(&scorers);
        let contexts = replay_with_context(&config, &events);

        // The tiebreak points should have is_tiebreak = true
        let tb_start = 48; // 6*4 + 6*4 = 48 points before tiebreak
        for i in tb_start..tb_start + 7 {
            assert!(contexts[i].is_tiebreak, "Point {} should be tiebreak", i);
        }
    }

    #[test]
    fn test_doubles_serve_rotation() {
        use crate::config::MatchType;
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

        // 4 games, each won by P1 in 4 points
        let mut scorers = Vec::new();
        for _ in 0..4 {
            for _ in 0..4 {
                scorers.push(Player::Player1);
            }
        }
        let events = make_events(&scorers);
        let contexts = replay_with_context(&config, &events);

        // Game 1: server index 0 -> Player1
        assert_eq!(contexts[0].serving_player, Player::Player1);
        // Game 2: server index 1 -> Player2
        assert_eq!(contexts[4].serving_player, Player::Player2);
        // Game 3: server index 2 -> Player1
        assert_eq!(contexts[8].serving_player, Player::Player1);
        // Game 4: server index 3 -> Player2
        assert_eq!(contexts[12].serving_player, Player::Player2);
    }

    #[test]
    fn test_point_numbers_are_sequential() {
        let config = MatchConfig::default();
        let events = make_events(&[
            Player::Player1,
            Player::Player2,
            Player::Player1,
        ]);
        let contexts = replay_with_context(&config, &events);

        assert_eq!(contexts[0].point_number, 1);
        assert_eq!(contexts[1].point_number, 2);
        assert_eq!(contexts[2].point_number, 3);
    }

    #[test]
    fn test_deuce_game_context() {
        let config = MatchConfig::default();
        // Get to deuce: 40-40
        let events = make_events(&[
            Player::Player1, // 15-0
            Player::Player1, // 30-0
            Player::Player1, // 40-0
            Player::Player2, // 40-15
            Player::Player2, // 40-30
            Player::Player2, // Deuce
            Player::Player1, // Ad P1
            Player::Player1, // P1 wins
        ]);
        let contexts = replay_with_context(&config, &events);

        // Point 6 (index 5): score is 40-30, next makes deuce
        // Point 7 (index 6): at deuce, P1 gets advantage
        assert!(contexts[6].score_before.current_game.is_deuce);
        // Point 8 (index 7): P1 has advantage, game point
        assert!(contexts[7].is_game_point);
        assert_eq!(contexts[7].score_before.current_game.advantage, Some(Player::Player1));
    }
}
