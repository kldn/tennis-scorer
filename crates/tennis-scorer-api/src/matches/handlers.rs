use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use uuid::Uuid;

use super::models::*;
use crate::AppState;
use crate::auth::middleware::AuthUser;
use crate::error::AppError;

pub async fn create_match(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(req): Json<CreateMatchRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    create_match_inner(auth.user_id, &state, req).await
}

/// Debug endpoint: create match without auth (uses first user in DB)
#[cfg(debug_assertions)]
pub async fn create_match_debug(
    State(state): State<AppState>,
    Json(req): Json<CreateMatchRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    let user_id = sqlx::query_scalar::<_, Uuid>("SELECT id FROM users LIMIT 1")
        .fetch_optional(&state.pool)
        .await?;
    let user_id = match user_id {
        Some(id) => id,
        None => {
            sqlx::query_scalar::<_, Uuid>(
                "INSERT INTO users (email, password_hash) VALUES ('debug@localhost', 'debug') RETURNING id",
            )
            .fetch_one(&state.pool)
            .await?
        }
    };
    create_match_inner(user_id, &state, req).await
}

async fn create_match_inner(
    user_id: Uuid,
    state: &AppState,
    req: CreateMatchRequest,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    // Idempotency check
    if let Some(client_id) = req.client_id
        && let Some(existing) = sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM matches WHERE client_id = $1 AND user_id = $2",
        )
        .bind(client_id)
        .bind(user_id)
        .fetch_optional(&state.pool)
        .await?
    {
        return Ok((StatusCode::OK, Json(serde_json::json!({"id": existing}))));
    }

    let mut tx = state.pool.begin().await?;

    let match_id = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO matches (user_id, client_id, match_type, config, winner, player1_sets, player2_sets, started_at, ended_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         RETURNING id"
    )
    .bind(user_id)
    .bind(req.client_id)
    .bind(&req.match_type)
    .bind(&req.config)
    .bind(req.winner)
    .bind(req.player1_sets)
    .bind(req.player2_sets)
    .bind(req.started_at)
    .bind(req.ended_at)
    .fetch_one(&mut *tx)
    .await?;

    for event in &req.events {
        sqlx::query(
            "INSERT INTO match_events (match_id, point_number, player, timestamp)
             VALUES ($1, $2, $3, $4)",
        )
        .bind(match_id)
        .bind(event.point_number)
        .bind(event.player)
        .bind(event.timestamp)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({"id": match_id})),
    ))
}

pub async fn list_matches(
    auth: AuthUser,
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<MatchListResponse>, AppError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);

    let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM matches WHERE user_id = $1")
        .bind(auth.user_id)
        .fetch_one(&state.pool)
        .await?;

    let rows = sqlx::query_as::<_, (Uuid, Option<Uuid>, String, serde_json::Value, i16, i16, i16, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>(
        "SELECT id, client_id, match_type, config, winner, player1_sets, player2_sets, started_at, ended_at, created_at
         FROM matches WHERE user_id = $1
         ORDER BY started_at DESC
         LIMIT $2 OFFSET $3"
    )
    .bind(auth.user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.pool)
    .await?;

    let matches = rows
        .into_iter()
        .map(|r| MatchResponse {
            id: r.0,
            client_id: r.1,
            match_type: r.2,
            config: r.3,
            winner: r.4,
            player1_sets: r.5,
            player2_sets: r.6,
            started_at: r.7,
            ended_at: r.8,
            created_at: r.9,
            events: None,
        })
        .collect();

    Ok(Json(MatchListResponse { matches, total }))
}

pub async fn get_match(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(match_id): Path<Uuid>,
) -> Result<Json<MatchResponse>, AppError> {
    let row = sqlx::query_as::<_, (Uuid, Option<Uuid>, String, serde_json::Value, i16, i16, i16, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>(
        "SELECT id, client_id, match_type, config, winner, player1_sets, player2_sets, started_at, ended_at, created_at
         FROM matches WHERE id = $1 AND user_id = $2"
    )
    .bind(match_id)
    .bind(auth.user_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Match not found".to_string()))?;

    let events = sqlx::query_as::<_, (i32, i16, chrono::DateTime<chrono::Utc>)>(
        "SELECT point_number, player, timestamp FROM match_events
         WHERE match_id = $1 ORDER BY point_number",
    )
    .bind(match_id)
    .fetch_all(&state.pool)
    .await?;

    let event_responses: Vec<MatchEventResponse> = events
        .into_iter()
        .map(|e| MatchEventResponse {
            point_number: e.0,
            player: e.1,
            timestamp: e.2,
        })
        .collect();

    Ok(Json(MatchResponse {
        id: row.0,
        client_id: row.1,
        match_type: row.2,
        config: row.3,
        winner: row.4,
        player1_sets: row.5,
        player2_sets: row.6,
        started_at: row.7,
        ended_at: row.8,
        created_at: row.9,
        events: Some(event_responses),
    }))
}

pub async fn delete_match(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(match_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM matches WHERE id = $1 AND user_id = $2")
        .bind(match_id)
        .bind(auth.user_id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Match not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
