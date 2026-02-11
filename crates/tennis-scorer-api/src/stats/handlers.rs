use std::time::SystemTime;

use axum::Json;
use axum::extract::{Path, State};
use serde::Serialize;
use uuid::Uuid;

use tennis_scorer::analysis::{
    self, MatchAnalysis, MomentumData, PaceData,
};
use tennis_scorer::{MatchConfig, Player};

use crate::AppState;
use crate::auth::middleware::AuthUser;
use crate::error::AppError;

#[derive(Serialize)]
pub struct StatsSummary {
    pub total_matches: i64,
    pub wins: i64,
    pub losses: i64,
    pub win_rate: f64,
    pub current_streak: Streak,
    pub recent_form: Vec<String>,
}

#[derive(Serialize)]
pub struct Streak {
    pub streak_type: String,
    pub count: i64,
}

pub async fn summary(
    auth: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<StatsSummary>, AppError> {
    // Get total, wins, losses
    let (total, wins): (i64, i64) = sqlx::query_as::<_, (i64, i64)>(
        "SELECT COUNT(*), COALESCE(SUM(CASE WHEN winner = 1 THEN 1 ELSE 0 END), 0)
         FROM matches WHERE user_id = $1",
    )
    .bind(auth.user_id)
    .fetch_one(&state.pool)
    .await?;

    let losses = total - wins;
    let win_rate = if total > 0 {
        wins as f64 / total as f64
    } else {
        0.0
    };

    // Recent form (last 10)
    let recent_results = sqlx::query_scalar::<_, i16>(
        "SELECT winner FROM matches WHERE user_id = $1
         ORDER BY started_at DESC LIMIT 10",
    )
    .bind(auth.user_id)
    .fetch_all(&state.pool)
    .await?;

    let recent_form: Vec<String> = recent_results
        .iter()
        .map(|w| {
            if *w == 1 {
                "W".to_string()
            } else {
                "L".to_string()
            }
        })
        .collect();

    // Current streak
    let (streak_type, count) = if recent_form.is_empty() {
        ("none".to_string(), 0i64)
    } else {
        let first = &recent_form[0];
        let count = recent_form.iter().take_while(|r| r == &first).count() as i64;
        (if first == "W" { "win" } else { "loss" }.to_string(), count)
    };

    Ok(Json(StatsSummary {
        total_matches: total,
        wins,
        losses,
        win_rate,
        current_streak: Streak { streak_type, count },
        recent_form,
    }))
}

// --- Per-match analysis helpers ---

async fn load_match_analysis_data(
    pool: &sqlx::PgPool,
    user_id: Uuid,
    match_id: Uuid,
) -> Result<(MatchConfig, Vec<(Player, SystemTime)>), AppError> {
    // Load match config (JSON) and verify ownership
    let row = sqlx::query_as::<_, (serde_json::Value,)>(
        "SELECT config FROM matches WHERE id = $1 AND user_id = $2",
    )
    .bind(match_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Match not found".to_string()))?;

    let config: MatchConfig =
        serde_json::from_value(row.0).map_err(|e| AppError::Internal(format!("Invalid config: {e}")))?;

    // Load point events
    let events = sqlx::query_as::<_, (i16, chrono::DateTime<chrono::Utc>)>(
        "SELECT player, timestamp FROM match_events
         WHERE match_id = $1 ORDER BY point_number",
    )
    .bind(match_id)
    .fetch_all(pool)
    .await?;

    let point_events: Vec<(Player, SystemTime)> = events
        .into_iter()
        .map(|(player, ts)| {
            let p = if player == 1 {
                Player::Player1
            } else {
                Player::Player2
            };
            let system_time: SystemTime = ts.into();
            (p, system_time)
        })
        .collect();

    Ok((config, point_events))
}

pub async fn match_analysis(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(match_id): Path<Uuid>,
) -> Result<Json<MatchAnalysis>, AppError> {
    let (config, events) = load_match_analysis_data(&state.pool, auth.user_id, match_id).await?;
    let contexts = analysis::replay_with_context(&config, &events);
    let result = analysis::compute_analysis(&contexts);
    Ok(Json(result))
}

pub async fn match_momentum(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(match_id): Path<Uuid>,
) -> Result<Json<MomentumData>, AppError> {
    let (config, events) = load_match_analysis_data(&state.pool, auth.user_id, match_id).await?;
    let contexts = analysis::replay_with_context(&config, &events);
    let result = analysis::compute_momentum(&contexts);
    Ok(Json(result))
}

pub async fn match_pace(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(match_id): Path<Uuid>,
) -> Result<Json<PaceData>, AppError> {
    let (config, events) = load_match_analysis_data(&state.pool, auth.user_id, match_id).await?;
    let contexts = analysis::replay_with_context(&config, &events);
    let result = analysis::compute_pace(&contexts);
    Ok(Json(result))
}
