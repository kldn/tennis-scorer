use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::Deserialize;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::error::AppError;

const APPLE_JWKS_URL: &str = "https://appleid.apple.com/auth/keys";
const APPLE_ISSUER: &str = "https://appleid.apple.com";
const JWKS_TIMEOUT: Duration = Duration::from_secs(5);
const JWKS_CACHE_TTL: Duration = Duration::from_secs(3600);

#[derive(Debug, Deserialize)]
struct JwkSet {
    keys: Vec<Jwk>,
}

#[derive(Debug, Clone, Deserialize)]
struct Jwk {
    kid: String,
    kty: String,
    n: String,
    e: String,
}

#[derive(Debug, Deserialize)]
pub struct AppleIdTokenClaims {
    pub sub: String,
    pub email: Option<String>,
    pub iss: String,
    pub aud: String,
    pub nonce: Option<String>,
}

struct CachedKeys {
    keys: Vec<Jwk>,
    fetched_at: Instant,
}

#[derive(Clone)]
pub struct AppleTokenVerifier {
    bundle_id: String,
    cached_keys: Arc<RwLock<Option<CachedKeys>>>,
    http_client: reqwest::Client,
}

impl AppleTokenVerifier {
    pub fn new(bundle_id: String) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(JWKS_TIMEOUT)
            .build()
            .expect("Failed to build HTTP client for Apple JWKS");

        Self {
            bundle_id,
            cached_keys: Arc::new(RwLock::new(None)),
            http_client,
        }
    }

    async fn fetch_keys(&self) -> Result<Vec<Jwk>, AppError> {
        let response = self
            .http_client
            .get(APPLE_JWKS_URL)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to fetch Apple JWKS: {e}")))?;

        if !response.status().is_success() {
            return Err(AppError::Internal(format!(
                "Apple JWKS returned status {}",
                response.status()
            )));
        }

        let jwk_set: JwkSet = response
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to parse Apple JWKS: {e}")))?;
        Ok(jwk_set.keys)
    }

    async fn get_keys(&self, force_refresh: bool) -> Result<Vec<Jwk>, AppError> {
        if !force_refresh {
            let cached = self.cached_keys.read().await;
            if let Some(entry) = cached.as_ref() {
                if entry.fetched_at.elapsed() < JWKS_CACHE_TTL {
                    return Ok(entry.keys.clone());
                }
            }
        }

        // Double-checked locking: re-check under write lock to avoid parallel fetches
        let mut cache = self.cached_keys.write().await;
        if !force_refresh {
            if let Some(entry) = cache.as_ref() {
                if entry.fetched_at.elapsed() < JWKS_CACHE_TTL {
                    return Ok(entry.keys.clone());
                }
            }
        }

        let keys = self.fetch_keys().await?;
        *cache = Some(CachedKeys {
            keys: keys.clone(),
            fetched_at: Instant::now(),
        });
        Ok(keys)
    }

    fn decode_with_key(
        &self,
        token: &str,
        jwk: &Jwk,
    ) -> Result<AppleIdTokenClaims, jsonwebtoken::errors::Error> {
        let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)?;
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[APPLE_ISSUER]);
        validation.set_audience(&[&self.bundle_id]);

        let token_data = decode::<AppleIdTokenClaims>(token, &decoding_key, &validation)?;
        Ok(token_data.claims)
    }

    fn validate_nonce(
        claims: &AppleIdTokenClaims,
        expected_nonce: Option<&str>,
    ) -> Result<(), AppError> {
        if let Some(expected) = expected_nonce {
            match claims.nonce.as_deref() {
                Some(actual) if actual == expected => Ok(()),
                _ => Err(AppError::Unauthorized(
                    "Nonce mismatch in Apple identity token".to_string(),
                )),
            }
        } else {
            Ok(())
        }
    }

    pub async fn verify(
        &self,
        identity_token: &str,
        expected_nonce: Option<&str>,
    ) -> Result<AppleIdTokenClaims, AppError> {
        let header = decode_header(identity_token)
            .map_err(|_| AppError::Unauthorized("Invalid Apple identity token".to_string()))?;

        let kid = header
            .kid
            .ok_or_else(|| AppError::Unauthorized("Missing kid in token header".to_string()))?;

        // Try with cached keys first
        let keys = self.get_keys(false).await?;
        if let Some(jwk) = keys.iter().find(|k| k.kid == kid && k.kty == "RSA") {
            match self.decode_with_key(identity_token, jwk) {
                Ok(claims) => {
                    Self::validate_nonce(&claims, expected_nonce)?;
                    return Ok(claims);
                }
                Err(e) => {
                    // Only refresh JWKS on signature errors; other errors
                    // (expired, wrong audience/issuer) won't be fixed by new keys
                    if !matches!(
                        e.kind(),
                        jsonwebtoken::errors::ErrorKind::InvalidSignature
                            | jsonwebtoken::errors::ErrorKind::InvalidRsaKey(_)
                    ) {
                        return Err(AppError::Unauthorized(
                            "Invalid Apple identity token".to_string(),
                        ));
                    }
                }
            }
        }

        // Refresh keys and retry (key rotation or missing kid)
        let keys = self.get_keys(true).await?;
        let jwk = keys
            .iter()
            .find(|k| k.kid == kid && k.kty == "RSA")
            .ok_or_else(|| AppError::Unauthorized("No matching Apple key found".to_string()))?;

        let claims = self
            .decode_with_key(identity_token, jwk)
            .map_err(|_| AppError::Unauthorized("Invalid Apple identity token".to_string()))?;
        Self::validate_nonce(&claims, expected_nonce)?;
        Ok(claims)
    }
}
