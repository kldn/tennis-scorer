use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use tracing::error;
use uuid::Uuid;

use super::firebase::FirebaseClaims;
use crate::AppState;
use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Verify Firebase ID token
        let claims = FirebaseClaims::from_request_parts(parts, state).await?;

        // Look up user by firebase_uid
        let user_id: Option<Uuid> =
            sqlx::query_scalar("SELECT id FROM users WHERE firebase_uid = $1")
                .bind(claims.firebase_uid())
                .fetch_optional(&state.pool)
                .await
                .map_err(|e| {
                    error!(error = %e, firebase_uid = claims.firebase_uid(), "Failed to look up user");
                    AppError::Internal("Database error".to_string())
                })?;

        let user_id = user_id.ok_or_else(|| {
            AppError::Unauthorized("User not found. Call PUT /api/auth/me first.".to_string())
        })?;

        Ok(AuthUser { user_id })
    }
}
