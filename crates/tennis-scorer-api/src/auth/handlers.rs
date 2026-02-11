use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

use super::jwt;
use crate::AppState;
use crate::error::AppError;

static EMAIL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap());

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub id: uuid::Uuid,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<RegisterResponse>), AppError> {
    // Validate input
    if !EMAIL_REGEX.is_match(&req.email) {
        return Err(AppError::Unprocessable("Invalid email format".to_string()));
    }
    if req.password.len() < 8 {
        return Err(AppError::Unprocessable(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("Password hashing failed: {e}")))?
        .to_string();

    // Insert user
    let row = sqlx::query_scalar::<_, uuid::Uuid>(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id",
    )
    .bind(&req.email)
    .bind(&password_hash)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.constraint() == Some("users_email_key") => {
            AppError::Conflict("Email already registered".to_string())
        }
        other => AppError::from(other),
    })?;

    Ok((StatusCode::CREATED, Json(RegisterResponse { id: row })))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    // Find user
    let user: Option<(uuid::Uuid, String)> =
        sqlx::query_as("SELECT id, password_hash FROM users WHERE email = $1")
            .bind(&req.email)
            .fetch_optional(&state.pool)
            .await?;

    let (user_id, stored_hash) =
        user.ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

    // Verify password
    let parsed_hash = PasswordHash::new(&stored_hash)
        .map_err(|_| AppError::Internal("Invalid stored hash".to_string()))?;
    Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized("Invalid credentials".to_string()))?;

    // Generate tokens
    let access_token = jwt::create_access_token(user_id, &state.jwt_secret)
        .map_err(|e| AppError::Internal(format!("Token creation failed: {e}")))?;
    let refresh_token = jwt::create_refresh_token(user_id, &state.jwt_secret)
        .map_err(|e| AppError::Internal(format!("Token creation failed: {e}")))?;

    Ok(Json(TokenResponse {
        access_token,
        refresh_token,
    }))
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(req): Json<RefreshRequest>,
) -> Result<Json<AccessTokenResponse>, AppError> {
    let claims = jwt::validate_token(&req.refresh_token, &state.jwt_secret)
        .map_err(|_| AppError::Unauthorized("Invalid or expired refresh token".to_string()))?;

    if claims.token_type != "refresh" {
        return Err(AppError::Unauthorized("Invalid token type".to_string()));
    }

    let access_token = jwt::create_access_token(claims.sub, &state.jwt_secret)
        .map_err(|e| AppError::Internal(format!("Token creation failed: {e}")))?;

    Ok(Json(AccessTokenResponse { access_token }))
}
