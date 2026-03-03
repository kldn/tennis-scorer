use axum::Json;
use axum::extract::State;
use serde::Serialize;

use super::firebase::FirebaseClaims;
use crate::AppState;
use crate::error::AppError;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

/// PUT /api/auth/me
///
/// Verifies the Firebase ID token, then finds or creates the user.
/// Uses UPSERT to handle concurrent first-login requests atomically.
pub async fn upsert_me(
    State(state): State<AppState>,
    claims: FirebaseClaims,
) -> Result<Json<UserResponse>, AppError> {
    let firebase_uid = claims.firebase_uid();
    let email = claims.email();
    let display_name = claims.display_name();
    let avatar_url = claims.avatar_url();

    let row: (uuid::Uuid, Option<String>, Option<String>, Option<String>) = sqlx::query_as(
        "INSERT INTO users (firebase_uid, email, display_name, avatar_url)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (firebase_uid) DO UPDATE
         SET email = EXCLUDED.email,
             display_name = EXCLUDED.display_name,
             avatar_url = EXCLUDED.avatar_url
         RETURNING id, email, display_name, avatar_url",
    )
    .bind(firebase_uid)
    .bind(email)
    .bind(display_name)
    .bind(avatar_url)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(UserResponse {
        id: row.0,
        email: row.1,
        display_name: row.2,
        avatar_url: row.3,
    }))
}
