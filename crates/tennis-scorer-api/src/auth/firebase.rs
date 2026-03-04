use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

use crate::AppState;
use crate::error::AppError;

const GOOGLE_CERTS_URL: &str = "https://www.googleapis.com/service_account/v1/metadata/x509/securetoken@system.gserviceaccount.com";
const JWKS_TIMEOUT: Duration = Duration::from_secs(5);
const BEARER_PREFIX: &str = "Bearer ";

#[derive(Debug, Clone, Deserialize)]
pub struct FirebaseClaims {
    sub: String,
    email: Option<String>,
    name: Option<String>,
    picture: Option<String>,
}

impl FirebaseClaims {
    pub fn firebase_uid(&self) -> &str {
        &self.sub
    }

    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    pub fn display_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn avatar_url(&self) -> Option<&str> {
        self.picture.as_deref()
    }
}

/// Axum extractor: verifies Firebase ID token from Authorization header.
///
/// This extractor only validates the Firebase token. It does NOT verify the
/// user exists in the database. Most handlers should use `AuthUser` instead.
/// Only use `FirebaseClaims` directly for endpoints that provision users
/// (e.g., `PUT /auth/me`).
impl FromRequestParts<AppState> for FirebaseClaims {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

        let token = auth_header
            .strip_prefix(BEARER_PREFIX)
            .ok_or_else(|| AppError::Unauthorized("Invalid authorization format".to_string()))?;

        state.firebase_verifier.verify(token).await
    }
}

type KeyCache = Arc<RwLock<Option<Arc<HashMap<String, DecodingKey>>>>>;

#[derive(Clone)]
pub struct FirebaseTokenVerifier {
    validation: Validation,
    /// kid -> pre-parsed DecodingKey
    cached_keys: KeyCache,
    http_client: reqwest::Client,
}

fn build_http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(JWKS_TIMEOUT)
        .build()
        .expect("Failed to build HTTP client for Firebase JWKS")
}

fn build_validation(project_id: &str) -> Validation {
    let issuer = format!("https://securetoken.google.com/{project_id}");
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[&issuer]);
    validation.set_audience(&[project_id]);
    validation
}

fn parse_certs_to_keys(
    certs: HashMap<String, String>,
) -> Result<HashMap<String, DecodingKey>, AppError> {
    certs
        .into_iter()
        .map(|(kid, pem)| {
            let key = DecodingKey::from_rsa_pem(pem.as_bytes())
                .map_err(|e| AppError::Internal(format!("Invalid cert for kid {kid}: {e}")))?;
            Ok((kid, key))
        })
        .collect()
}

impl FirebaseTokenVerifier {
    pub fn new(project_id: String) -> Self {
        let validation = build_validation(&project_id);
        Self {
            validation,
            cached_keys: Arc::new(RwLock::new(None)),
            http_client: build_http_client(),
        }
    }

    /// Create a verifier with pre-cached certificates (for testing).
    pub fn new_with_certs(project_id: String, certs: HashMap<String, String>) -> Self {
        let validation = build_validation(&project_id);
        let keys = parse_certs_to_keys(certs).expect("Invalid test certificate");
        Self {
            validation,
            cached_keys: Arc::new(RwLock::new(Some(Arc::new(keys)))),
            http_client: reqwest::Client::new(),
        }
    }

    async fn fetch_keys(&self) -> Result<HashMap<String, DecodingKey>, AppError> {
        let response = self
            .http_client
            .get(GOOGLE_CERTS_URL)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to fetch Google certs: {e}")))?;

        if !response.status().is_success() {
            return Err(AppError::Internal(format!(
                "Google certs returned status {}",
                response.status()
            )));
        }

        let certs: HashMap<String, String> = response
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to parse Google certs: {e}")))?;
        parse_certs_to_keys(certs)
    }

    async fn get_keys(
        &self,
        force_refresh: bool,
    ) -> Result<Arc<HashMap<String, DecodingKey>>, AppError> {
        if !force_refresh {
            let cached = self.cached_keys.read().await;
            if let Some(keys) = cached.as_ref() {
                return Ok(Arc::clone(keys));
            }
        }

        // Double-checked locking
        let mut cache = self.cached_keys.write().await;
        if !force_refresh && let Some(keys) = cache.as_ref() {
            return Ok(Arc::clone(keys));
        }

        let keys = Arc::new(self.fetch_keys().await?);
        *cache = Some(Arc::clone(&keys));
        Ok(keys)
    }

    fn decode_with_key(
        &self,
        token: &str,
        key: &DecodingKey,
    ) -> Result<FirebaseClaims, jsonwebtoken::errors::Error> {
        let token_data = decode::<FirebaseClaims>(token, key, &self.validation)?;
        Ok(token_data.claims)
    }

    pub async fn verify(&self, token: &str) -> Result<FirebaseClaims, AppError> {
        let header = decode_header(token)
            .map_err(|_| AppError::Unauthorized("Invalid Firebase ID token".to_string()))?;

        let kid = header
            .kid
            .ok_or_else(|| AppError::Unauthorized("Missing kid in token header".to_string()))?;

        // Try with cached keys first
        let keys = self.get_keys(false).await?;
        if let Some(key) = keys.get(&kid) {
            match self.decode_with_key(token, key) {
                Ok(claims) => return Ok(claims),
                Err(e) => {
                    // Only refresh keys on signature errors
                    if !matches!(
                        e.kind(),
                        jsonwebtoken::errors::ErrorKind::InvalidSignature
                            | jsonwebtoken::errors::ErrorKind::InvalidRsaKey(_)
                    ) {
                        return Err(AppError::Unauthorized(
                            "Invalid Firebase ID token".to_string(),
                        ));
                    }
                }
            }
        }

        // Refresh keys and retry (key rotation or missing kid)
        let keys = self.get_keys(true).await?;
        let key = keys
            .get(&kid)
            .ok_or_else(|| AppError::Unauthorized("No matching Google cert found".to_string()))?;

        self.decode_with_key(token, key)
            .map_err(|_| AppError::Unauthorized("Invalid Firebase ID token".to_string()))
    }
}
