use std::sync::RwLock;
use std::time::{Duration, SystemTime};

use tennis_scorer::{
    analysis::{
        compute_analysis as core_compute_analysis, compute_momentum as core_compute_momentum,
        compute_pace as core_compute_pace, replay_with_context as core_replay_with_context,
        PointContext as CorePointContext,
    },
    GameState as CoreGameState, MatchConfig as CoreMatchConfig, MatchState, MatchType,
    MatchWithHistory, Player as CorePlayer, Point, SetState, TiebreakState,
};

uniffi::setup_scaffolding!();

// --- UniFFI enum wrappers ---

#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Player1,
    Player2,
}

impl From<CorePlayer> for Player {
    fn from(p: CorePlayer) -> Self {
        match p {
            CorePlayer::Player1 => Player::Player1,
            CorePlayer::Player2 => Player::Player2,
        }
    }
}

impl From<Player> for CorePlayer {
    fn from(p: Player) -> Self {
        match p {
            Player::Player1 => CorePlayer::Player1,
            Player::Player2 => CorePlayer::Player2,
        }
    }
}

#[derive(uniffi::Enum, Debug, Clone, PartialEq, Eq)]
pub enum GameScore {
    Points { player1: String, player2: String },
    Deuce,
    Advantage { player: Player },
    Completed { winner: Player },
}

fn point_to_string(p: Point) -> String {
    match p {
        Point::Love => "0".to_string(),
        Point::Fifteen => "15".to_string(),
        Point::Thirty => "30".to_string(),
        Point::Forty => "40".to_string(),
    }
}

impl From<&CoreGameState> for GameScore {
    fn from(gs: &CoreGameState) -> Self {
        match gs {
            CoreGameState::Points { player1, player2 } => GameScore::Points {
                player1: point_to_string(*player1),
                player2: point_to_string(*player2),
            },
            CoreGameState::Deuce { .. } => GameScore::Deuce,
            CoreGameState::Advantage { player, .. } => GameScore::Advantage {
                player: (*player).into(),
            },
            CoreGameState::Completed(winner) => GameScore::Completed {
                winner: (*winner).into(),
            },
        }
    }
}

// --- UniFFI records ---

#[derive(uniffi::Record, Debug, Clone)]
pub struct MatchConfig {
    pub sets_to_win: u8,
    pub tiebreak_points: u8,
    pub final_set_tiebreak: bool,
    pub no_ad_scoring: bool,
    pub is_doubles: bool,
    pub first_server_team: Option<Player>,
}

impl From<&MatchConfig> for CoreMatchConfig {
    fn from(c: &MatchConfig) -> Self {
        let (match_type, serve_order) = if c.is_doubles {
            let first = match c.first_server_team {
                Some(Player::Player2) => CorePlayer::Player2,
                _ => CorePlayer::Player1,
            };
            let second = first.opponent();
            (
                MatchType::Doubles,
                vec![(first, 0), (second, 0), (first, 1), (second, 1)],
            )
        } else {
            (MatchType::Singles, Vec::new())
        };

        CoreMatchConfig {
            sets_to_win: c.sets_to_win,
            tiebreak_points: c.tiebreak_points,
            final_set_tiebreak: c.final_set_tiebreak,
            no_ad_scoring: c.no_ad_scoring,
            match_type,
            serve_order,
        }
    }
}

#[derive(uniffi::Record, Debug, Clone)]
pub struct MatchScore {
    pub player1_sets: u8,
    pub player2_sets: u8,
    pub player1_games: Vec<u8>,
    pub player2_games: Vec<u8>,
    pub current_game: GameScore,
    pub winner: Option<Player>,
    pub is_tiebreak: bool,
    pub deuce_count: u8,
    pub current_server: u8,
}

#[derive(uniffi::Record, Debug, Clone)]
pub struct PointEvent {
    pub player: Player,
    pub timestamp_epoch_secs: f64,
}

fn system_time_to_epoch_secs(time: &SystemTime) -> f64 {
    match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration.as_secs_f64(),
        Err(_) => 0.0,
    }
}

fn extract_score(state: &MatchState) -> MatchScore {
    let current_server = state.current_server();
    match state {
        MatchState::Playing {
            sets,
            player1_sets,
            player2_sets,
            ..
        } => {
            let mut p1_games = Vec::new();
            let mut p2_games = Vec::new();
            let mut current_game = GameScore::Points {
                player1: "0".to_string(),
                player2: "0".to_string(),
            };
            let mut is_tiebreak = false;
            let mut deuce_count: u8 = 0;

            for set in sets {
                match set {
                    SetState::Playing {
                        player1_games: p1g,
                        player2_games: p2g,
                        current_game: cg,
                        tiebreak,
                    } => {
                        p1_games.push(*p1g);
                        p2_games.push(*p2g);
                        if let Some(tb) = tiebreak {
                            is_tiebreak = true;
                            match tb {
                                TiebreakState::Playing {
                                    player1_points,
                                    player2_points,
                                    ..
                                } => {
                                    current_game = GameScore::Points {
                                        player1: player1_points.to_string(),
                                        player2: player2_points.to_string(),
                                    };
                                }
                                TiebreakState::Completed(winner) => {
                                    current_game = GameScore::Completed {
                                        winner: (*winner).into(),
                                    };
                                }
                            }
                        } else {
                            deuce_count = cg.deuce_count();
                            current_game = GameScore::from(cg);
                        }
                    }
                    SetState::Completed {
                        player1_games: p1g,
                        player2_games: p2g,
                        ..
                    } => {
                        p1_games.push(*p1g);
                        p2_games.push(*p2g);
                    }
                }
            }

            MatchScore {
                player1_sets: *player1_sets,
                player2_sets: *player2_sets,
                player1_games: p1_games,
                player2_games: p2_games,
                current_game,
                winner: None,
                is_tiebreak,
                deuce_count,
                current_server,
            }
        }
        MatchState::Completed {
            winner,
            player1_sets,
            player2_sets,
            sets,
        } => {
            let mut p1_games = Vec::new();
            let mut p2_games = Vec::new();
            for set in sets {
                match set {
                    SetState::Completed {
                        player1_games: p1g,
                        player2_games: p2g,
                        ..
                    }
                    | SetState::Playing {
                        player1_games: p1g,
                        player2_games: p2g,
                        ..
                    } => {
                        p1_games.push(*p1g);
                        p2_games.push(*p2g);
                    }
                }
            }

            MatchScore {
                player1_sets: *player1_sets,
                player2_sets: *player2_sets,
                player1_games: p1_games,
                player2_games: p2_games,
                current_game: GameScore::Completed {
                    winner: (*winner).into(),
                },
                winner: Some((*winner).into()),
                is_tiebreak: false,
                deuce_count: 0,
                current_server,
            }
        }
    }
}

// --- UniFFI Object wrapping MatchWithHistory ---

#[derive(uniffi::Object)]
pub struct TennisMatch {
    inner: RwLock<MatchWithHistory>,
    config: RwLock<CoreMatchConfig>,
}

#[uniffi::export]
impl TennisMatch {
    #[uniffi::constructor]
    pub fn new() -> Self {
        let config = CoreMatchConfig::default();
        let state = MatchState::new(config.clone());
        TennisMatch {
            inner: RwLock::new(MatchWithHistory::new(state)),
            config: RwLock::new(config),
        }
    }

    #[uniffi::constructor]
    pub fn new_with_config(config: MatchConfig) -> Self {
        let core_config = CoreMatchConfig::from(&config);
        let state = MatchState::new(core_config.clone());
        TennisMatch {
            inner: RwLock::new(MatchWithHistory::new(state)),
            config: RwLock::new(core_config),
        }
    }

    pub fn score_point(&self, player: Player) -> MatchScore {
        let mut inner = self.inner.write().unwrap();
        let new_state = inner.score_point(CorePlayer::from(player));
        *inner = new_state;
        extract_score(inner.current())
    }

    pub fn undo(&self) -> MatchScore {
        let mut inner = self.inner.write().unwrap();
        let new_state = inner.undo();
        *inner = new_state;
        extract_score(inner.current())
    }

    pub fn can_undo(&self) -> bool {
        let inner = self.inner.read().unwrap();
        inner.can_undo()
    }

    pub fn get_score(&self) -> MatchScore {
        let inner = self.inner.read().unwrap();
        extract_score(inner.current())
    }

    pub fn get_point_events(&self) -> Vec<PointEvent> {
        let inner = self.inner.read().unwrap();
        inner
            .point_events()
            .iter()
            .map(|(player, timestamp)| PointEvent {
                player: (*player).into(),
                timestamp_epoch_secs: system_time_to_epoch_secs(timestamp),
            })
            .collect()
    }

    pub fn new_match(&self) {
        let config = self.config.read().unwrap().clone();
        let state = MatchState::new(config);
        let mut inner = self.inner.write().unwrap();
        *inner = MatchWithHistory::new(state);
    }
}

// --- Analysis UniFFI types ---

#[derive(uniffi::Record, Debug, Clone)]
pub struct AnalysisPointContext {
    pub point_number: u32,
    pub scorer: Player,
    pub timestamp_epoch_secs: f64,
    pub serving_player: Player,
    pub is_break_point: bool,
    pub is_game_point: bool,
    pub is_set_point: bool,
    pub is_match_point: bool,
    pub game_number_in_set: u32,
    pub set_number: u32,
    pub is_tiebreak: bool,
}

impl From<&CorePointContext> for AnalysisPointContext {
    fn from(p: &CorePointContext) -> Self {
        AnalysisPointContext {
            point_number: p.point_number,
            scorer: p.scorer.into(),
            timestamp_epoch_secs: system_time_to_epoch_secs(&p.timestamp),
            serving_player: p.serving_player.into(),
            is_break_point: p.is_break_point,
            is_game_point: p.is_game_point,
            is_set_point: p.is_set_point,
            is_match_point: p.is_match_point,
            game_number_in_set: p.game_number_in_set,
            set_number: p.set_number,
            is_tiebreak: p.is_tiebreak,
        }
    }
}

#[derive(uniffi::Record, Debug, Clone)]
pub struct BreakPointStatsFFI {
    pub break_points_created: u32,
    pub break_points_converted: u32,
    pub break_points_faced: u32,
    pub break_points_saved: u32,
    pub break_point_conversion_rate: f64,
}

#[derive(uniffi::Record, Debug, Clone)]
pub struct ServiceStatsFFI {
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

#[derive(uniffi::Record, Debug, Clone)]
pub struct DeuceStatsFFI {
    pub deuce_games_count: u32,
    pub deuce_games_won: u32,
    pub deuce_game_win_rate: f64,
    pub total_deuce_count: u32,
    pub average_deuces_per_deuce_game: f64,
}

#[derive(uniffi::Record, Debug, Clone)]
pub struct ConversionRateStatsFFI {
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

#[derive(uniffi::Record, Debug, Clone)]
pub struct StreakStatsFFI {
    pub longest_point_streak: u32,
    pub longest_point_drought: u32,
    pub longest_service_hold_streak: u32,
    pub max_games_in_a_row: u32,
}

#[derive(uniffi::Record, Debug, Clone)]
pub struct ClutchStatsFFI {
    pub break_point_win_rate: f64,
    pub set_point_win_rate: f64,
    pub match_point_win_rate: f64,
    pub normal_point_win_rate: f64,
    pub clutch_score: f64,
}

#[derive(uniffi::Record, Debug, Clone)]
pub struct TiebreakStatsFFI {
    pub tiebreaks_played: u32,
    pub tiebreaks_won: u32,
    pub tiebreak_win_rate: f64,
    pub average_tiebreak_margin: f64,
}

#[derive(uniffi::Record, Debug, Clone)]
pub struct TotalPointsStatsFFI {
    pub points_won: u32,
    pub total_points: u32,
    pub points_won_percentage: f64,
}

#[derive(uniffi::Record, Debug, Clone)]
pub struct PlayerStatsFFI {
    pub break_points: BreakPointStatsFFI,
    pub service: ServiceStatsFFI,
    pub deuce: DeuceStatsFFI,
    pub conversion: ConversionRateStatsFFI,
    pub streaks: StreakStatsFFI,
    pub clutch: ClutchStatsFFI,
    pub tiebreak: TiebreakStatsFFI,
    pub total_points: TotalPointsStatsFFI,
}

#[derive(uniffi::Record, Debug, Clone)]
pub struct MatchAnalysisFFI {
    pub player1: PlayerStatsFFI,
    pub player2: PlayerStatsFFI,
}

#[derive(uniffi::Record, Debug, Clone)]
pub struct MomentumDataFFI {
    pub basic: Vec<f64>,
    pub weighted: Vec<f64>,
    pub per_set_basic: Vec<Vec<f64>>,
    pub per_set_weighted: Vec<Vec<f64>>,
}

#[derive(uniffi::Record, Debug, Clone)]
pub struct PaceDataFFI {
    pub average_point_interval_seconds: f64,
    pub per_set_durations_seconds: Vec<f64>,
    pub total_duration_seconds: f64,
}

fn core_analysis_to_ffi(a: &tennis_scorer::analysis::MatchAnalysis) -> MatchAnalysisFFI {
    MatchAnalysisFFI {
        player1: player_stats_to_ffi(&a.player1),
        player2: player_stats_to_ffi(&a.player2),
    }
}

fn player_stats_to_ffi(
    s: &tennis_scorer::analysis::PlayerStats,
) -> PlayerStatsFFI {
    PlayerStatsFFI {
        break_points: BreakPointStatsFFI {
            break_points_created: s.break_points.break_points_created,
            break_points_converted: s.break_points.break_points_converted,
            break_points_faced: s.break_points.break_points_faced,
            break_points_saved: s.break_points.break_points_saved,
            break_point_conversion_rate: s.break_points.break_point_conversion_rate,
        },
        service: ServiceStatsFFI {
            service_games_played: s.service.service_games_played,
            service_games_held: s.service.service_games_held,
            hold_percentage: s.service.hold_percentage,
            return_games_played: s.service.return_games_played,
            return_games_won: s.service.return_games_won,
            break_percentage: s.service.break_percentage,
            service_points_won: s.service.service_points_won,
            service_points_total: s.service.service_points_total,
            return_points_won: s.service.return_points_won,
            return_points_total: s.service.return_points_total,
            dominance_ratio: s.service.dominance_ratio,
        },
        deuce: DeuceStatsFFI {
            deuce_games_count: s.deuce.deuce_games_count,
            deuce_games_won: s.deuce.deuce_games_won,
            deuce_game_win_rate: s.deuce.deuce_game_win_rate,
            total_deuce_count: s.deuce.total_deuce_count,
            average_deuces_per_deuce_game: s.deuce.average_deuces_per_deuce_game,
        },
        conversion: ConversionRateStatsFFI {
            game_points_total: s.conversion.game_points_total,
            game_points_converted: s.conversion.game_points_converted,
            game_point_conversion_rate: s.conversion.game_point_conversion_rate,
            set_points_total: s.conversion.set_points_total,
            set_points_converted: s.conversion.set_points_converted,
            set_point_conversion_rate: s.conversion.set_point_conversion_rate,
            match_points_total: s.conversion.match_points_total,
            match_points_converted: s.conversion.match_points_converted,
            match_point_conversion_rate: s.conversion.match_point_conversion_rate,
        },
        streaks: StreakStatsFFI {
            longest_point_streak: s.streaks.longest_point_streak,
            longest_point_drought: s.streaks.longest_point_drought,
            longest_service_hold_streak: s.streaks.longest_service_hold_streak,
            max_games_in_a_row: s.streaks.max_games_in_a_row,
        },
        clutch: ClutchStatsFFI {
            break_point_win_rate: s.clutch.break_point_win_rate,
            set_point_win_rate: s.clutch.set_point_win_rate,
            match_point_win_rate: s.clutch.match_point_win_rate,
            normal_point_win_rate: s.clutch.normal_point_win_rate,
            clutch_score: s.clutch.clutch_score,
        },
        tiebreak: TiebreakStatsFFI {
            tiebreaks_played: s.tiebreak.tiebreaks_played,
            tiebreaks_won: s.tiebreak.tiebreaks_won,
            tiebreak_win_rate: s.tiebreak.tiebreak_win_rate,
            average_tiebreak_margin: s.tiebreak.average_tiebreak_margin,
        },
        total_points: TotalPointsStatsFFI {
            points_won: s.total_points.points_won,
            total_points: s.total_points.total_points,
            points_won_percentage: s.total_points.points_won_percentage,
        },
    }
}

fn epoch_secs_to_system_time(secs: f64) -> SystemTime {
    SystemTime::UNIX_EPOCH + Duration::from_secs_f64(secs)
}

// --- Analysis exported functions ---

#[uniffi::export]
pub fn analyze_match(config: MatchConfig, events: Vec<PointEvent>) -> MatchAnalysisFFI {
    let core_config = CoreMatchConfig::from(&config);
    let core_events: Vec<(CorePlayer, SystemTime)> = events
        .iter()
        .map(|e| {
            (
                CorePlayer::from(e.player),
                epoch_secs_to_system_time(e.timestamp_epoch_secs),
            )
        })
        .collect();
    let contexts = core_replay_with_context(&core_config, &core_events);
    let analysis = core_compute_analysis(&contexts);
    core_analysis_to_ffi(&analysis)
}

#[uniffi::export]
pub fn compute_match_momentum(config: MatchConfig, events: Vec<PointEvent>) -> MomentumDataFFI {
    let core_config = CoreMatchConfig::from(&config);
    let core_events: Vec<(CorePlayer, SystemTime)> = events
        .iter()
        .map(|e| {
            (
                CorePlayer::from(e.player),
                epoch_secs_to_system_time(e.timestamp_epoch_secs),
            )
        })
        .collect();
    let contexts = core_replay_with_context(&core_config, &core_events);
    let momentum = core_compute_momentum(&contexts);
    MomentumDataFFI {
        basic: momentum.basic,
        weighted: momentum.weighted,
        per_set_basic: momentum.per_set_basic,
        per_set_weighted: momentum.per_set_weighted,
    }
}

#[uniffi::export]
pub fn compute_match_pace(config: MatchConfig, events: Vec<PointEvent>) -> PaceDataFFI {
    let core_config = CoreMatchConfig::from(&config);
    let core_events: Vec<(CorePlayer, SystemTime)> = events
        .iter()
        .map(|e| {
            (
                CorePlayer::from(e.player),
                epoch_secs_to_system_time(e.timestamp_epoch_secs),
            )
        })
        .collect();
    let contexts = core_replay_with_context(&core_config, &core_events);
    let pace = core_compute_pace(&contexts);
    PaceDataFFI {
        average_point_interval_seconds: pace.average_point_interval_seconds,
        per_set_durations_seconds: pace.per_set_durations.iter().map(|s| s.duration_seconds).collect(),
        total_duration_seconds: pace.total_duration_seconds,
    }
}

#[uniffi::export]
pub fn replay_match_with_context(
    config: MatchConfig,
    events: Vec<PointEvent>,
) -> Vec<AnalysisPointContext> {
    let core_config = CoreMatchConfig::from(&config);
    let core_events: Vec<(CorePlayer, SystemTime)> = events
        .iter()
        .map(|e| {
            (
                CorePlayer::from(e.player),
                epoch_secs_to_system_time(e.timestamp_epoch_secs),
            )
        })
        .collect();
    let contexts = core_replay_with_context(&core_config, &core_events);
    contexts.iter().map(AnalysisPointContext::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_match_score() {
        let m = TennisMatch::new();
        let score = m.get_score();
        assert_eq!(score.player1_sets, 0);
        assert_eq!(score.player2_sets, 0);
        assert_eq!(score.winner, None);
        assert_eq!(score.deuce_count, 0);
        assert_eq!(score.current_server, 0);
    }

    #[test]
    fn test_score_point_returns_updated_score() {
        let m = TennisMatch::new();
        let score = m.score_point(Player::Player1);
        assert_eq!(
            score.current_game,
            GameScore::Points {
                player1: "15".to_string(),
                player2: "0".to_string(),
            }
        );
    }

    #[test]
    fn test_full_game() {
        let m = TennisMatch::new();
        m.score_point(Player::Player1);
        m.score_point(Player::Player1);
        m.score_point(Player::Player1);
        let score = m.score_point(Player::Player1);
        assert_eq!(score.player1_games, vec![1]);
        assert_eq!(
            score.current_game,
            GameScore::Points {
                player1: "0".to_string(),
                player2: "0".to_string(),
            }
        );
    }

    #[test]
    fn test_custom_config() {
        let config = MatchConfig {
            sets_to_win: 3,
            tiebreak_points: 10,
            final_set_tiebreak: false,
            no_ad_scoring: true,
            is_doubles: false,
            first_server_team: None,
        };
        let m = TennisMatch::new_with_config(config);
        let score = m.get_score();
        assert_eq!(score.player1_sets, 0);
        assert_eq!(score.winner, None);
    }

    #[test]
    fn test_doubles_config() {
        let config = MatchConfig {
            sets_to_win: 2,
            tiebreak_points: 7,
            final_set_tiebreak: true,
            no_ad_scoring: false,
            is_doubles: true,
            first_server_team: Some(Player::Player1),
        };
        let m = TennisMatch::new_with_config(config);
        let score = m.get_score();
        assert_eq!(score.current_server, 0);

        // Win a game, server should rotate
        m.score_point(Player::Player1);
        m.score_point(Player::Player1);
        m.score_point(Player::Player1);
        let score = m.score_point(Player::Player1);
        assert_eq!(score.current_server, 1);
    }

    #[test]
    fn test_undo() {
        let m = TennisMatch::new();
        m.score_point(Player::Player1);
        assert!(m.can_undo());

        let score = m.undo();
        assert!(!m.can_undo());
        assert_eq!(
            score.current_game,
            GameScore::Points {
                player1: "0".to_string(),
                player2: "0".to_string(),
            }
        );
    }

    #[test]
    fn test_deuce_count() {
        let m = TennisMatch::new();
        // Get to deuce: 40-40
        m.score_point(Player::Player1); // 15-0
        m.score_point(Player::Player1); // 30-0
        m.score_point(Player::Player1); // 40-0
        m.score_point(Player::Player2); // 40-15
        m.score_point(Player::Player2); // 40-30
        let score = m.score_point(Player::Player2); // Deuce
        assert_eq!(score.current_game, GameScore::Deuce);
        assert_eq!(score.deuce_count, 1);

        // Advantage P1
        let score = m.score_point(Player::Player1);
        assert_eq!(
            score.current_game,
            GameScore::Advantage {
                player: Player::Player1
            }
        );
        assert_eq!(score.deuce_count, 1);

        // Back to deuce
        let score = m.score_point(Player::Player2);
        assert_eq!(score.current_game, GameScore::Deuce);
        assert_eq!(score.deuce_count, 2);
    }

    #[test]
    fn test_point_events() {
        let m = TennisMatch::new();
        m.score_point(Player::Player1);
        m.score_point(Player::Player2);

        let events = m.get_point_events();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].player, Player::Player1);
        assert_eq!(events[1].player, Player::Player2);
        assert!(events[0].timestamp_epoch_secs > 0.0);
        assert!(events[1].timestamp_epoch_secs >= events[0].timestamp_epoch_secs);
    }

    #[test]
    fn test_point_events_after_undo() {
        let m = TennisMatch::new();
        m.score_point(Player::Player1);
        m.score_point(Player::Player2);
        m.undo();

        let events = m.get_point_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].player, Player::Player1);
    }

    #[test]
    fn test_new_match_resets() {
        let m = TennisMatch::new();
        m.score_point(Player::Player1);
        m.score_point(Player::Player1);
        assert!(m.can_undo());

        m.new_match();
        let score = m.get_score();
        assert_eq!(score.player1_sets, 0);
        assert_eq!(score.player2_sets, 0);
        assert!(!m.can_undo());
        assert_eq!(m.get_point_events().len(), 0);
    }

    #[test]
    fn test_full_match_completion() {
        let m = TennisMatch::new();
        // Win two sets 6-0 each (48 points total)
        for _ in 0..2 {
            for _ in 0..6 {
                for _ in 0..4 {
                    m.score_point(Player::Player1);
                }
            }
        }
        let score = m.get_score();
        assert_eq!(score.winner, Some(Player::Player1));
        assert_eq!(score.player1_sets, 2);
    }

    #[test]
    fn test_no_ad_scoring() {
        let config = MatchConfig {
            sets_to_win: 2,
            tiebreak_points: 7,
            final_set_tiebreak: true,
            no_ad_scoring: true,
            is_doubles: false,
            first_server_team: None,
        };
        let m = TennisMatch::new_with_config(config);
        // Get to deuce
        m.score_point(Player::Player1); // 15-0
        m.score_point(Player::Player1); // 30-0
        m.score_point(Player::Player2); // 30-15
        m.score_point(Player::Player2); // 30-30
        m.score_point(Player::Player1); // 40-30
        m.score_point(Player::Player2); // Deuce (no-ad: next point wins)
        // One more point wins the game
        let score = m.score_point(Player::Player1);
        assert_eq!(score.player1_games, vec![1]);
    }
}
