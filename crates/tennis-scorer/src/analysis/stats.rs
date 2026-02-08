use serde::{Deserialize, Serialize};

use crate::types::Player;

use super::types::PointContext;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreakPointStats {
    pub break_points_created: u32,
    pub break_points_converted: u32,
    pub break_points_faced: u32,
    pub break_points_saved: u32,
    pub break_point_conversion_rate: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceStats {
    pub service_games_played: u32,
    pub service_games_held: u32,
    pub hold_percentage: f64,
    pub return_games_played: u32,
    pub return_games_won: u32,
    pub break_percentage: f64,
    pub service_points_won: u32,
    pub service_points_total: u32,
    pub return_points_won: u32,
    pub return_points_total: u32,
    pub dominance_ratio: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeuceStats {
    pub deuce_games_count: u32,
    pub deuce_games_won: u32,
    pub deuce_game_win_rate: f64,
    pub total_deuce_count: u32,
    pub average_deuces_per_deuce_game: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConversionRateStats {
    pub game_points_total: u32,
    pub game_points_converted: u32,
    pub game_point_conversion_rate: f64,
    pub set_points_total: u32,
    pub set_points_converted: u32,
    pub set_point_conversion_rate: f64,
    pub match_points_total: u32,
    pub match_points_converted: u32,
    pub match_point_conversion_rate: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreakStats {
    pub longest_point_streak: u32,
    pub longest_point_drought: u32,
    pub longest_service_hold_streak: u32,
    pub max_games_in_a_row: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClutchStats {
    pub break_point_win_rate: f64,
    pub set_point_win_rate: f64,
    pub match_point_win_rate: f64,
    pub normal_point_win_rate: f64,
    pub clutch_score: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TiebreakStats {
    pub tiebreaks_played: u32,
    pub tiebreaks_won: u32,
    pub tiebreak_win_rate: f64,
    pub average_tiebreak_margin: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TotalPointsStats {
    pub points_won: u32,
    pub total_points: u32,
    pub points_won_percentage: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerStats {
    pub break_points: BreakPointStats,
    pub service: ServiceStats,
    pub deuce: DeuceStats,
    pub conversion: ConversionRateStats,
    pub streaks: StreakStats,
    pub clutch: ClutchStats,
    pub tiebreak: TiebreakStats,
    pub total_points: TotalPointsStats,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchAnalysis {
    pub player1: PlayerStats,
    pub player2: PlayerStats,
}

pub fn compute_analysis(points: &[PointContext]) -> MatchAnalysis {
    MatchAnalysis {
        player1: compute_player_stats(points, Player::Player1),
        player2: compute_player_stats(points, Player::Player2),
    }
}

fn compute_player_stats(points: &[PointContext], player: Player) -> PlayerStats {
    PlayerStats {
        break_points: compute_break_points(points, player),
        service: compute_service_stats(points, player),
        deuce: compute_deuce_stats(points, player),
        conversion: compute_conversion_rates(points, player),
        streaks: compute_streaks(points, player),
        clutch: compute_clutch(points, player),
        tiebreak: compute_tiebreak_stats(points, player),
        total_points: compute_total_points(points, player),
    }
}

pub fn compute_break_points(points: &[PointContext], player: Player) -> BreakPointStats {
    let mut created = 0u32;
    let mut converted = 0u32;
    let mut faced = 0u32;
    let mut saved = 0u32;

    for p in points {
        if p.is_break_point {
            if p.serving_player == player {
                // Player is serving, facing a break point
                faced += 1;
                if p.scorer == player {
                    saved += 1;
                }
            } else {
                // Player is returning, has a break point opportunity
                created += 1;
                if p.scorer == player {
                    converted += 1;
                }
            }
        }
    }

    BreakPointStats {
        break_points_created: created,
        break_points_converted: converted,
        break_points_faced: faced,
        break_points_saved: saved,
        break_point_conversion_rate: if created > 0 {
            converted as f64 / created as f64
        } else {
            0.0
        },
    }
}

pub fn compute_service_stats(points: &[PointContext], player: Player) -> ServiceStats {
    let mut service_points_won = 0u32;
    let mut service_points_total = 0u32;
    let mut return_points_won = 0u32;
    let mut return_points_total = 0u32;

    // Track game outcomes for hold/break stats
    // Group points by (set_number, game_number_in_set) to detect game completions
    let mut games: std::collections::BTreeMap<(u32, u32), Vec<&PointContext>> =
        std::collections::BTreeMap::new();

    for p in points {
        if p.serving_player == player {
            service_points_total += 1;
            if p.scorer == player {
                service_points_won += 1;
            }
        } else {
            return_points_total += 1;
            if p.scorer == player {
                return_points_won += 1;
            }
        }
        games
            .entry((p.set_number, p.game_number_in_set))
            .or_default()
            .push(p);
    }

    let mut service_games_played = 0u32;
    let mut service_games_held = 0u32;
    let mut return_games_played = 0u32;
    let mut return_games_won = 0u32;

    for game_points in games.values() {
        if game_points.is_empty() {
            continue;
        }
        let server = game_points[0].serving_player;
        let last_point = game_points.last().unwrap();

        // Check if this game completed by seeing if the next point is in a different game
        // We detect game completion by checking if the last point's scorer won the game
        // A simpler heuristic: the last point of a game is the one that completed it
        // We'll look at whether the game_number changed after this group
        if server == player {
            service_games_played += 1;
            // The last point scorer won the game (if the game completed)
            // We check: did points in this game lead to a game completion?
            // Heuristic: if the game is followed by a different game key, it completed
            if last_point.scorer == player {
                service_games_held += 1;
            }
        } else {
            return_games_played += 1;
            if last_point.scorer == player {
                return_games_won += 1;
            }
        }
    }

    let hold_percentage = if service_games_played > 0 {
        service_games_held as f64 / service_games_played as f64
    } else {
        0.0
    };
    let break_percentage = if return_games_played > 0 {
        return_games_won as f64 / return_games_played as f64
    } else {
        0.0
    };

    let service_pct = if service_points_total > 0 {
        service_points_won as f64 / service_points_total as f64
    } else {
        0.0
    };
    let return_pct = if return_points_total > 0 {
        return_points_won as f64 / return_points_total as f64
    } else {
        0.0
    };
    let serve_loss_pct = 1.0 - service_pct;
    let dominance_ratio = if serve_loss_pct > 0.0 {
        return_pct / serve_loss_pct
    } else {
        0.0
    };

    ServiceStats {
        service_games_played,
        service_games_held,
        hold_percentage,
        return_games_played,
        return_games_won,
        break_percentage,
        service_points_won,
        service_points_total,
        return_points_won,
        return_points_total,
        dominance_ratio,
    }
}

pub fn compute_deuce_stats(points: &[PointContext], player: Player) -> DeuceStats {
    // Track games that reach deuce and their outcomes
    let mut deuce_games: std::collections::BTreeMap<(u32, u32), (u8, Option<Player>)> =
        std::collections::BTreeMap::new();

    for p in points {
        let key = (p.set_number, p.game_number_in_set);
        if p.score_before.current_game.is_deuce
            || p.score_before.current_game.advantage.is_some()
            || (p.score_before.current_game.player1_points == "40"
                && p.score_before.current_game.player2_points == "40"
                && !p.is_tiebreak)
        {
            let entry = deuce_games.entry(key).or_insert((0, None));
            let dc = p.score_before.current_game.deuce_count;
            if dc > entry.0 {
                entry.0 = dc;
            }
            // Track who eventually wins (last point scorer in deuce state)
            entry.1 = Some(p.scorer);
        }
    }

    let deuce_games_count = deuce_games.len() as u32;
    let mut deuce_games_won = 0u32;
    let mut total_deuce_count = 0u32;

    for (max_dc, winner) in deuce_games.values() {
        total_deuce_count += *max_dc as u32;
        if *winner == Some(player) {
            deuce_games_won += 1;
        }
    }

    DeuceStats {
        deuce_games_count,
        deuce_games_won,
        deuce_game_win_rate: if deuce_games_count > 0 {
            deuce_games_won as f64 / deuce_games_count as f64
        } else {
            0.0
        },
        total_deuce_count,
        average_deuces_per_deuce_game: if deuce_games_count > 0 {
            total_deuce_count as f64 / deuce_games_count as f64
        } else {
            0.0
        },
    }
}

pub fn compute_conversion_rates(points: &[PointContext], player: Player) -> ConversionRateStats {
    let mut game_pts = 0u32;
    let mut game_converted = 0u32;
    let mut set_pts = 0u32;
    let mut set_converted = 0u32;
    let mut match_pts = 0u32;
    let mut match_converted = 0u32;

    for p in points {
        // Game point for this player: they could win the game
        if p.is_game_point && is_point_opportunity_for(p, player) {
            game_pts += 1;
            if p.scorer == player {
                game_converted += 1;
            }
        }
        if p.is_set_point && is_point_opportunity_for(p, player) {
            set_pts += 1;
            if p.scorer == player {
                set_converted += 1;
            }
        }
        if p.is_match_point && is_point_opportunity_for(p, player) {
            match_pts += 1;
            if p.scorer == player {
                match_converted += 1;
            }
        }
    }

    ConversionRateStats {
        game_points_total: game_pts,
        game_points_converted: game_converted,
        game_point_conversion_rate: rate(game_converted, game_pts),
        set_points_total: set_pts,
        set_points_converted: set_converted,
        set_point_conversion_rate: rate(set_converted, set_pts),
        match_points_total: match_pts,
        match_points_converted: match_converted,
        match_point_conversion_rate: rate(match_converted, match_pts),
    }
}

/// Determine if a game/set/match point is an opportunity for this player
/// (i.e., this player could win by winning this point)
fn is_point_opportunity_for(p: &PointContext, player: Player) -> bool {
    let gs = &p.score_before.current_game;

    if p.is_tiebreak {
        // In tiebreak: check if player's points are ahead or at game point
        let p1_pts: u8 = gs.player1_points.parse().unwrap_or(0);
        let p2_pts: u8 = gs.player2_points.parse().unwrap_or(0);
        let (my_pts, opp_pts) = match player {
            Player::Player1 => (p1_pts, p2_pts),
            Player::Player2 => (p2_pts, p1_pts),
        };
        my_pts > opp_pts || (my_pts >= 6 && my_pts == opp_pts)
    } else {
        // Regular game
        let my_pts = match player {
            Player::Player1 => &gs.player1_points,
            Player::Player2 => &gs.player2_points,
        };
        let opp_pts = match player {
            Player::Player1 => &gs.player2_points,
            Player::Player2 => &gs.player1_points,
        };
        if my_pts == "40" && opp_pts != "40" {
            return true;
        }
        if let Some(adv_player) = &gs.advantage {
            return *adv_player == player;
        }
        // At deuce with no-ad, both have opportunity
        if gs.is_deuce {
            return true;
        }
        false
    }
}

pub fn compute_streaks(points: &[PointContext], player: Player) -> StreakStats {
    let mut longest_point_streak = 0u32;
    let mut longest_point_drought = 0u32;
    let mut current_streak = 0u32;
    let mut current_drought = 0u32;

    for p in points {
        if p.scorer == player {
            current_streak += 1;
            current_drought = 0;
            longest_point_streak = longest_point_streak.max(current_streak);
        } else {
            current_drought += 1;
            current_streak = 0;
            longest_point_drought = longest_point_drought.max(current_drought);
        }
    }

    // Track game-level streaks
    let mut games: Vec<(u32, u32, Player, Player)> = Vec::new(); // (set, game, server, last_scorer)
    let mut game_map: std::collections::BTreeMap<(u32, u32), (Player, Player)> =
        std::collections::BTreeMap::new();
    for p in points {
        let key = (p.set_number, p.game_number_in_set);
        game_map
            .entry(key)
            .and_modify(|e| e.1 = p.scorer)
            .or_insert((p.serving_player, p.scorer));
    }
    for (server, last_scorer) in game_map.values() {
        games.push((0, 0, *server, *last_scorer));
    }

    let mut longest_hold = 0u32;
    let mut current_hold = 0u32;
    let mut max_games_in_row = 0u32;
    let mut current_games = 0u32;

    for (_, _, server, game_winner) in &games {
        if *game_winner == player {
            current_games += 1;
            max_games_in_row = max_games_in_row.max(current_games);
            if *server == player {
                current_hold += 1;
                longest_hold = longest_hold.max(current_hold);
            }
        } else {
            current_games = 0;
            if *server == player {
                current_hold = 0;
            }
        }
    }

    StreakStats {
        longest_point_streak,
        longest_point_drought,
        longest_service_hold_streak: longest_hold,
        max_games_in_a_row: max_games_in_row,
    }
}

pub fn compute_clutch(points: &[PointContext], player: Player) -> ClutchStats {
    let mut bp_total = 0u32;
    let mut bp_won = 0u32;
    let mut sp_total = 0u32;
    let mut sp_won = 0u32;
    let mut mp_total = 0u32;
    let mut mp_won = 0u32;
    let mut normal_total = 0u32;
    let mut normal_won = 0u32;

    for p in points {
        let is_critical = p.is_break_point || p.is_set_point || p.is_match_point;

        if p.is_break_point && p.serving_player != player {
            // Player is returner on break point
            bp_total += 1;
            if p.scorer == player {
                bp_won += 1;
            }
        }
        if p.is_set_point && is_point_opportunity_for(p, player) {
            sp_total += 1;
            if p.scorer == player {
                sp_won += 1;
            }
        }
        if p.is_match_point && is_point_opportunity_for(p, player) {
            mp_total += 1;
            if p.scorer == player {
                mp_won += 1;
            }
        }
        if !is_critical {
            normal_total += 1;
            if p.scorer == player {
                normal_won += 1;
            }
        }
    }

    let bp_rate = rate(bp_won, bp_total);
    let sp_rate = rate(sp_won, sp_total);
    let mp_rate = rate(mp_won, mp_total);
    let normal_rate = rate(normal_won, normal_total);
    let clutch_score = 0.4 * bp_rate + 0.35 * sp_rate + 0.25 * mp_rate;

    ClutchStats {
        break_point_win_rate: bp_rate,
        set_point_win_rate: sp_rate,
        match_point_win_rate: mp_rate,
        normal_point_win_rate: normal_rate,
        clutch_score,
    }
}

pub fn compute_tiebreak_stats(points: &[PointContext], player: Player) -> TiebreakStats {
    // Group tiebreak points by set
    let mut tiebreak_sets: std::collections::BTreeMap<u32, Vec<&PointContext>> =
        std::collections::BTreeMap::new();

    for p in points {
        if p.is_tiebreak {
            tiebreak_sets.entry(p.set_number).or_default().push(p);
        }
    }

    let mut played = 0u32;
    let mut won = 0u32;
    let mut total_margin = 0f64;

    for tb_points in tiebreak_sets.values() {
        if tb_points.is_empty() {
            continue;
        }
        played += 1;
        let last = tb_points.last().unwrap();
        let gs = &last.score_before.current_game;
        let p1: u8 = gs.player1_points.parse().unwrap_or(0);
        let p2: u8 = gs.player2_points.parse().unwrap_or(0);

        // After the last tiebreak point, the scorer won
        let (winner_pts, loser_pts) = if last.scorer == Player::Player1 {
            (p1 + 1, p2)
        } else {
            (p2 + 1, p1)
        };
        total_margin += (winner_pts as i32 - loser_pts as i32).unsigned_abs() as f64;

        if last.scorer == player {
            won += 1;
        }
    }

    TiebreakStats {
        tiebreaks_played: played,
        tiebreaks_won: won,
        tiebreak_win_rate: rate(won, played),
        average_tiebreak_margin: if played > 0 {
            total_margin / played as f64
        } else {
            0.0
        },
    }
}

pub fn compute_total_points(points: &[PointContext], player: Player) -> TotalPointsStats {
    let total = points.len() as u32;
    let won = points.iter().filter(|p| p.scorer == player).count() as u32;

    TotalPointsStats {
        points_won: won,
        total_points: total,
        points_won_percentage: rate(won, total),
    }
}

fn rate(numerator: u32, denominator: u32) -> f64 {
    if denominator > 0 {
        numerator as f64 / denominator as f64
    } else {
        0.0
    }
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

    /// Helper: P1 wins 6-0 6-0 (48 points)
    fn dominant_match_events() -> Vec<(Player, SystemTime)> {
        let mut scorers = Vec::new();
        for _ in 0..12 {
            for _ in 0..4 {
                scorers.push(Player::Player1);
            }
        }
        make_events(&scorers)
    }

    #[test]
    fn test_total_points_dominant_match() {
        let config = MatchConfig::default();
        let events = dominant_match_events();
        let contexts = replay_with_context(&config, &events);
        let analysis = compute_analysis(&contexts);

        assert_eq!(analysis.player1.total_points.points_won, 48);
        assert_eq!(analysis.player2.total_points.points_won, 0);
        assert_eq!(analysis.player1.total_points.total_points, 48);
        assert_eq!(analysis.player1.total_points.points_won_percentage, 1.0);
        assert_eq!(analysis.player2.total_points.points_won_percentage, 0.0);
    }

    #[test]
    fn test_break_points_no_breaks() {
        // P1 wins all service games, P2 never gets to break point
        let config = MatchConfig::default();
        let events = dominant_match_events();
        let contexts = replay_with_context(&config, &events);
        let analysis = compute_analysis(&contexts);

        assert_eq!(analysis.player2.break_points.break_points_created, 0);
    }

    #[test]
    fn test_service_stats_dominant_match() {
        let config = MatchConfig::default();
        let events = dominant_match_events();
        let contexts = replay_with_context(&config, &events);
        let analysis = compute_analysis(&contexts);

        assert_eq!(analysis.player1.service.hold_percentage, 1.0);
        assert_eq!(analysis.player1.service.service_points_won, 24); // 6 service games * 4 pts
    }

    #[test]
    fn test_streaks_dominant_match() {
        let config = MatchConfig::default();
        let events = dominant_match_events();
        let contexts = replay_with_context(&config, &events);
        let analysis = compute_analysis(&contexts);

        assert_eq!(analysis.player1.streaks.longest_point_streak, 48);
        assert_eq!(analysis.player1.streaks.longest_point_drought, 0);
        assert_eq!(analysis.player2.streaks.longest_point_streak, 0);
        assert_eq!(analysis.player2.streaks.longest_point_drought, 48);
    }

    #[test]
    fn test_break_point_in_deuce_game() {
        let config = MatchConfig::default();
        // P1 serves, game goes to deuce, P2 gets advantage (break point), P2 converts
        let events = make_events(&[
            Player::Player1, // 15-0
            Player::Player1, // 30-0
            Player::Player1, // 40-0
            Player::Player2, // 40-15
            Player::Player2, // 40-30
            Player::Player2, // Deuce
            Player::Player2, // Ad P2 (break point)
            Player::Player2, // P2 breaks
        ]);
        let contexts = replay_with_context(&config, &events);
        let bp = compute_break_points(&contexts, Player::Player2);

        assert!(bp.break_points_created >= 1);
        assert!(bp.break_points_converted >= 1);
    }

    #[test]
    fn test_deuce_stats() {
        let config = MatchConfig::default();
        // One deuce game
        let events = make_events(&[
            Player::Player1,
            Player::Player1,
            Player::Player1,
            Player::Player2,
            Player::Player2,
            Player::Player2, // Deuce
            Player::Player1, // Ad P1
            Player::Player1, // P1 wins
        ]);
        let contexts = replay_with_context(&config, &events);
        let deuce = compute_deuce_stats(&contexts, Player::Player1);

        assert_eq!(deuce.deuce_games_count, 1);
        assert_eq!(deuce.deuce_games_won, 1);
    }

    #[test]
    fn test_empty_match_analysis() {
        let contexts: Vec<PointContext> = vec![];
        let analysis = compute_analysis(&contexts);

        assert_eq!(analysis.player1.total_points.total_points, 0);
        assert_eq!(analysis.player1.break_points.break_points_created, 0);
        assert_eq!(analysis.player1.streaks.longest_point_streak, 0);
    }

    #[test]
    fn test_tiebreak_stats() {
        let config = MatchConfig::default();
        let mut scorers = Vec::new();
        // Get to 6-6
        for _ in 0..6 {
            for _ in 0..4 {
                scorers.push(Player::Player1);
            }
            for _ in 0..4 {
                scorers.push(Player::Player2);
            }
        }
        // Tiebreak: P1 wins 7-0
        for _ in 0..7 {
            scorers.push(Player::Player1);
        }

        let events = make_events(&scorers);
        let contexts = replay_with_context(&config, &events);
        let tb = compute_tiebreak_stats(&contexts, Player::Player1);

        assert_eq!(tb.tiebreaks_played, 1);
        assert_eq!(tb.tiebreaks_won, 1);
        assert_eq!(tb.tiebreak_win_rate, 1.0);
    }
}
