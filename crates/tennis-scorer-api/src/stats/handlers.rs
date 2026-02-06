use axum::Json;
use axum::extract::State;
use serde::Serialize;

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
