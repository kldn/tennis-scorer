use std::time::SystemTime;

use crate::config::MatchConfig;
use crate::game::GameState;
use crate::history::MatchWithHistory;
use crate::match_state::MatchState;
use crate::set::SetState;
use crate::types::{Player, Point};

// ============================================================================
// Constants
// ============================================================================

pub const PLAYER_1: u8 = 1;
pub const PLAYER_2: u8 = 2;

// GameState codes
pub const GAME_STATE_PLAYING: u8 = 0;
pub const GAME_STATE_DEUCE: u8 = 1;
pub const GAME_STATE_ADVANTAGE_P1: u8 = 2;
pub const GAME_STATE_ADVANTAGE_P2: u8 = 3;
pub const GAME_STATE_COMPLETED: u8 = 4;

// ============================================================================
// Types
// ============================================================================

/// Opaque handle to a tennis match with history
pub struct TennisMatch {
    inner: MatchWithHistory,
}

/// C-compatible point event with timestamp
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PointEvent {
    pub player: u8,
    pub timestamp: f64,
}

/// C-compatible score representation
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct MatchScore {
    pub player1_sets: u8,
    pub player2_sets: u8,
    pub player1_games: u8,
    pub player2_games: u8,
    pub player1_points: u8,
    pub player2_points: u8,
    pub game_state: u8,
    pub is_tiebreak: bool,
    pub winner: u8,
    pub deuce_count: u8,
}

// ============================================================================
// Lifecycle Functions
// ============================================================================

/// Create a new match with default settings (Best-of-3, Ad scoring, 7-point tiebreak)
#[unsafe(no_mangle)]
pub extern "C" fn tennis_match_new_default() -> *mut TennisMatch {
    let config = MatchConfig::default();
    let inner = MatchWithHistory::new(MatchState::new(config));
    Box::into_raw(Box::new(TennisMatch { inner }))
}

/// Create a new Best-of-5 match
#[unsafe(no_mangle)]
pub extern "C" fn tennis_match_new_best_of_5() -> *mut TennisMatch {
    let config = MatchConfig {
        sets_to_win: 3,
        ..MatchConfig::default()
    };
    let inner = MatchWithHistory::new(MatchState::new(config));
    Box::into_raw(Box::new(TennisMatch { inner }))
}

/// Create a new No-Ad scoring match (Best-of-3)
#[unsafe(no_mangle)]
pub extern "C" fn tennis_match_new_no_ad() -> *mut TennisMatch {
    let config = MatchConfig {
        no_ad_scoring: true,
        ..MatchConfig::default()
    };
    let inner = MatchWithHistory::new(MatchState::new(config));
    Box::into_raw(Box::new(TennisMatch { inner }))
}

/// Create a new match with custom settings
#[unsafe(no_mangle)]
pub extern "C" fn tennis_match_new_custom(
    sets_to_win: u8,
    tiebreak_points: u8,
    final_set_tiebreak: bool,
    no_ad_scoring: bool,
) -> *mut TennisMatch {
    let config = MatchConfig {
        sets_to_win,
        tiebreak_points,
        final_set_tiebreak,
        no_ad_scoring,
    };
    let inner = MatchWithHistory::new(MatchState::new(config));
    Box::into_raw(Box::new(TennisMatch { inner }))
}

/// Free a match. Safe to call with null.
#[unsafe(no_mangle)]
pub extern "C" fn tennis_match_free(match_ptr: *mut TennisMatch) {
    if !match_ptr.is_null() {
        unsafe {
            drop(Box::from_raw(match_ptr));
        }
    }
}

// ============================================================================
// Scoring Functions
// ============================================================================

/// Score a point for the given player (PLAYER_1 or PLAYER_2).
/// Returns false if match_ptr is null or player is invalid.
#[unsafe(no_mangle)]
pub extern "C" fn tennis_match_score_point(match_ptr: *mut TennisMatch, player: u8) -> bool {
    if match_ptr.is_null() {
        return false;
    }

    let player = match player {
        PLAYER_1 => Player::Player1,
        PLAYER_2 => Player::Player2,
        _ => return false,
    };

    unsafe {
        let match_ref = &mut *match_ptr;
        match_ref.inner = match_ref.inner.score_point(player);
    }

    true
}

/// Undo the last point. Returns false if match_ptr is null.
#[unsafe(no_mangle)]
pub extern "C" fn tennis_match_undo(match_ptr: *mut TennisMatch) -> bool {
    if match_ptr.is_null() {
        return false;
    }

    unsafe {
        let match_ref = &mut *match_ptr;
        match_ref.inner = match_ref.inner.undo();
    }

    true
}

// ============================================================================
// Query Functions
// ============================================================================

/// Get the current score. Returns zeroed MatchScore if match_ptr is null.
#[unsafe(no_mangle)]
pub extern "C" fn tennis_match_get_score(match_ptr: *const TennisMatch) -> MatchScore {
    if match_ptr.is_null() {
        return MatchScore::default();
    }

    unsafe {
        let match_ref = &*match_ptr;
        build_match_score(match_ref.inner.current())
    }
}

/// Check if undo is available. Returns false if match_ptr is null.
#[unsafe(no_mangle)]
pub extern "C" fn tennis_match_can_undo(match_ptr: *const TennisMatch) -> bool {
    if match_ptr.is_null() {
        return false;
    }

    unsafe {
        let match_ref = &*match_ptr;
        match_ref.inner.can_undo()
    }
}

/// Check if the match is complete. Returns false if match_ptr is null.
#[unsafe(no_mangle)]
pub extern "C" fn tennis_match_is_complete(match_ptr: *const TennisMatch) -> bool {
    if match_ptr.is_null() {
        return false;
    }

    unsafe {
        let match_ref = &*match_ptr;
        match_ref.inner.current().winner().is_some()
    }
}

/// Get the winner (PLAYER_1, PLAYER_2, or 0 if no winner yet).
/// Returns 0 if match_ptr is null.
#[unsafe(no_mangle)]
pub extern "C" fn tennis_match_get_winner(match_ptr: *const TennisMatch) -> u8 {
    if match_ptr.is_null() {
        return 0;
    }

    unsafe {
        let match_ref = &*match_ptr;
        match match_ref.inner.current().winner() {
            Some(Player::Player1) => PLAYER_1,
            Some(Player::Player2) => PLAYER_2,
            None => 0,
        }
    }
}

// ============================================================================
// Point Event Functions
// ============================================================================

/// Get the number of point events recorded in the match.
/// Returns 0 if match_ptr is null.
#[unsafe(no_mangle)]
pub extern "C" fn tennis_match_get_point_count(match_ptr: *const TennisMatch) -> u32 {
    if match_ptr.is_null() {
        return 0;
    }

    unsafe {
        let match_ref = &*match_ptr;
        match_ref.inner.point_events().len() as u32
    }
}

/// Fill a caller-provided buffer with point events.
/// Returns false if match_ptr or buffer is null, or buffer_size is too small.
#[unsafe(no_mangle)]
pub extern "C" fn tennis_match_get_points(
    match_ptr: *const TennisMatch,
    buffer: *mut PointEvent,
    buffer_size: u32,
) -> bool {
    if match_ptr.is_null() || buffer.is_null() {
        return false;
    }

    unsafe {
        let match_ref = &*match_ptr;
        let events = match_ref.inner.point_events();

        if (buffer_size as usize) < events.len() {
            return false;
        }

        for (i, (player, timestamp)) in events.iter().enumerate() {
            let player_code = match player {
                Player::Player1 => PLAYER_1,
                Player::Player2 => PLAYER_2,
            };
            *buffer.add(i) = PointEvent {
                player: player_code,
                timestamp: system_time_to_epoch_secs(timestamp),
            };
        }
    }

    true
}

// ============================================================================
// Helper Functions
// ============================================================================

fn system_time_to_epoch_secs(time: &SystemTime) -> f64 {
    match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration.as_secs_f64(),
        Err(_) => 0.0,
    }
}

fn build_match_score(state: &MatchState) -> MatchScore {
    match state {
        MatchState::Completed {
            winner,
            player1_sets,
            player2_sets,
            ..
        } => {
            let winner_code = match winner {
                Player::Player1 => PLAYER_1,
                Player::Player2 => PLAYER_2,
            };
            MatchScore {
                player1_sets: *player1_sets,
                player2_sets: *player2_sets,
                player1_games: 0,
                player2_games: 0,
                player1_points: 0,
                player2_points: 0,
                game_state: GAME_STATE_COMPLETED,
                is_tiebreak: false,
                winner: winner_code,
                deuce_count: 0,
            }
        }
        MatchState::Playing {
            sets,
            player1_sets,
            player2_sets,
            ..
        } => {
            let current_set = sets.last().unwrap();
            let (p1_games, p2_games, game_state, p1_points, p2_points, is_tiebreak, deuce_count) =
                extract_set_info(current_set);

            MatchScore {
                player1_sets: *player1_sets,
                player2_sets: *player2_sets,
                player1_games: p1_games,
                player2_games: p2_games,
                player1_points: p1_points,
                player2_points: p2_points,
                game_state,
                is_tiebreak,
                winner: 0,
                deuce_count,
            }
        }
    }
}

fn extract_set_info(set: &SetState) -> (u8, u8, u8, u8, u8, bool, u8) {
    match set {
        SetState::Completed {
            player1_games,
            player2_games,
            ..
        } => (*player1_games, *player2_games, GAME_STATE_COMPLETED, 0, 0, false, 0),
        SetState::Playing {
            player1_games,
            player2_games,
            current_game,
            tiebreak,
        } => {
            if let Some(tb) = tiebreak {
                let (p1_pts, p2_pts, tb_complete) = extract_tiebreak_info(tb);
                let game_state = if tb_complete {
                    GAME_STATE_COMPLETED
                } else {
                    GAME_STATE_PLAYING
                };
                (*player1_games, *player2_games, game_state, p1_pts, p2_pts, true, 0)
            } else {
                let (game_state, p1_pts, p2_pts, deuce_count) = extract_game_info(current_game);
                (*player1_games, *player2_games, game_state, p1_pts, p2_pts, false, deuce_count)
            }
        }
    }
}

fn extract_game_info(game: &GameState) -> (u8, u8, u8, u8) {
    match game {
        GameState::Points { player1, player2 } => {
            (GAME_STATE_PLAYING, point_to_number(*player1), point_to_number(*player2), 0)
        }
        GameState::Deuce { count } => (GAME_STATE_DEUCE, 40, 40, *count),
        GameState::Advantage { player: Player::Player1, deuce_count } => (GAME_STATE_ADVANTAGE_P1, 0, 0, *deuce_count),
        GameState::Advantage { player: Player::Player2, deuce_count } => (GAME_STATE_ADVANTAGE_P2, 0, 0, *deuce_count),
        GameState::Completed(_) => (GAME_STATE_COMPLETED, 0, 0, 0),
    }
}

fn extract_tiebreak_info(tb: &crate::tiebreak::TiebreakState) -> (u8, u8, bool) {
    match tb {
        crate::tiebreak::TiebreakState::Playing {
            player1_points,
            player2_points,
            ..
        } => (*player1_points, *player2_points, false),
        crate::tiebreak::TiebreakState::Completed(_) => (0, 0, true),
    }
}

fn point_to_number(point: Point) -> u8 {
    match point {
        Point::Love => 0,
        Point::Fifteen => 15,
        Point::Thirty => 30,
        Point::Forty => 40,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_lifecycle() {
        let m = tennis_match_new_default();
        assert!(!m.is_null());
        tennis_match_free(m);
    }

    #[test]
    fn test_free_null_is_safe() {
        tennis_match_free(ptr::null_mut());
    }

    #[test]
    fn test_score_point() {
        let m = tennis_match_new_default();
        assert!(tennis_match_score_point(m, PLAYER_1));

        let score = tennis_match_get_score(m);
        assert_eq!(score.player1_points, 15);
        assert_eq!(score.player2_points, 0);

        tennis_match_free(m);
    }

    #[test]
    fn test_score_point_null() {
        assert!(!tennis_match_score_point(ptr::null_mut(), PLAYER_1));
    }

    #[test]
    fn test_score_point_invalid_player() {
        let m = tennis_match_new_default();
        assert!(!tennis_match_score_point(m, 0));
        assert!(!tennis_match_score_point(m, 3));
        tennis_match_free(m);
    }

    #[test]
    fn test_undo() {
        let m = tennis_match_new_default();
        tennis_match_score_point(m, PLAYER_1);
        assert!(tennis_match_can_undo(m));

        assert!(tennis_match_undo(m));
        assert!(!tennis_match_can_undo(m));

        let score = tennis_match_get_score(m);
        assert_eq!(score.player1_points, 0);

        tennis_match_free(m);
    }

    #[test]
    fn test_undo_null() {
        assert!(!tennis_match_undo(ptr::null_mut()));
    }

    #[test]
    fn test_get_score_null() {
        let score = tennis_match_get_score(ptr::null());
        assert_eq!(score.player1_sets, 0);
        assert_eq!(score.winner, 0);
    }

    #[test]
    fn test_complete_game() {
        let m = tennis_match_new_default();

        // Score 4 points for player 1 to win a game
        for _ in 0..4 {
            tennis_match_score_point(m, PLAYER_1);
        }

        let score = tennis_match_get_score(m);
        assert_eq!(score.player1_games, 1);
        assert_eq!(score.player2_games, 0);

        tennis_match_free(m);
    }

    #[test]
    fn test_custom_match() {
        let m = tennis_match_new_custom(3, 10, false, true);
        assert!(!m.is_null());
        tennis_match_free(m);
    }

    #[test]
    fn test_is_complete_and_winner() {
        let m = tennis_match_new_default();
        assert!(!tennis_match_is_complete(m));
        assert_eq!(tennis_match_get_winner(m), 0);
        tennis_match_free(m);
    }

    #[test]
    fn test_get_point_count_empty() {
        let m = tennis_match_new_default();
        assert_eq!(tennis_match_get_point_count(m), 0);
        tennis_match_free(m);
    }

    #[test]
    fn test_get_point_count_after_scoring() {
        let m = tennis_match_new_default();
        tennis_match_score_point(m, PLAYER_1);
        tennis_match_score_point(m, PLAYER_2);
        tennis_match_score_point(m, PLAYER_1);
        assert_eq!(tennis_match_get_point_count(m), 3);
        tennis_match_free(m);
    }

    #[test]
    fn test_get_points_fills_buffer() {
        let m = tennis_match_new_default();
        tennis_match_score_point(m, PLAYER_1);
        tennis_match_score_point(m, PLAYER_2);

        let mut buffer = [PointEvent::default(); 2];
        assert!(tennis_match_get_points(m, buffer.as_mut_ptr(), 2));

        assert_eq!(buffer[0].player, PLAYER_1);
        assert_eq!(buffer[1].player, PLAYER_2);
        assert!(buffer[0].timestamp > 0.0);
        assert!(buffer[1].timestamp >= buffer[0].timestamp);

        tennis_match_free(m);
    }

    #[test]
    fn test_get_points_reflects_undo() {
        let m = tennis_match_new_default();
        tennis_match_score_point(m, PLAYER_1);
        tennis_match_score_point(m, PLAYER_2);
        tennis_match_score_point(m, PLAYER_1);
        assert_eq!(tennis_match_get_point_count(m), 3);

        tennis_match_undo(m);
        assert_eq!(tennis_match_get_point_count(m), 2);

        let mut buffer = [PointEvent::default(); 2];
        assert!(tennis_match_get_points(m, buffer.as_mut_ptr(), 2));
        assert_eq!(buffer[0].player, PLAYER_1);
        assert_eq!(buffer[1].player, PLAYER_2);

        tennis_match_free(m);
    }

    #[test]
    fn test_point_functions_null_safety() {
        assert_eq!(tennis_match_get_point_count(ptr::null()), 0);
        assert!(!tennis_match_get_points(ptr::null(), ptr::null_mut(), 0));

        let m = tennis_match_new_default();
        assert!(!tennis_match_get_points(m, ptr::null_mut(), 0));
        tennis_match_free(m);
    }

    #[test]
    fn test_deuce_count() {
        let m = tennis_match_new_default();

        // Get to 40-40 (deuce)
        // P1: 15, 30, 40
        tennis_match_score_point(m, PLAYER_1);
        tennis_match_score_point(m, PLAYER_1);
        tennis_match_score_point(m, PLAYER_1);
        // P2: 15, 30, 40
        tennis_match_score_point(m, PLAYER_2);
        tennis_match_score_point(m, PLAYER_2);
        tennis_match_score_point(m, PLAYER_2);

        let score = tennis_match_get_score(m);
        assert_eq!(score.game_state, GAME_STATE_DEUCE);
        assert_eq!(score.deuce_count, 1);

        // P1 advantage
        tennis_match_score_point(m, PLAYER_1);
        let score = tennis_match_get_score(m);
        assert_eq!(score.game_state, GAME_STATE_ADVANTAGE_P1);
        assert_eq!(score.deuce_count, 1);

        // Back to deuce (count = 2)
        tennis_match_score_point(m, PLAYER_2);
        let score = tennis_match_get_score(m);
        assert_eq!(score.game_state, GAME_STATE_DEUCE);
        assert_eq!(score.deuce_count, 2);

        tennis_match_free(m);
    }
}
