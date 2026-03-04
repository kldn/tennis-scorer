//! Integration tests -- require a running PostgreSQL database.
//!
//! Run with:
//! ```sh
//! DATABASE_URL=postgres://user:pass@localhost/tennis_scorer_test \
//! FIREBASE_PROJECT_ID=test-project \
//! cargo test --test integration -- --ignored
//! ```
//!
//! All tests are marked `#[ignore]` so that a plain `cargo test` does not
//! fail when no database is available.
//!
//! ## Mock Firebase Token
//!
//! Tests use a mock RSA key pair to sign Firebase-style JWTs locally.
//! The `FirebaseTokenVerifier` fetches Google certs on first use, but in
//! tests we inject pre-cached certs via the verifier so no network call
//! is needed.

use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use chrono::{Duration, Utc};
use http_body_util::BodyExt;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Serialize;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use tower::ServiceExt;

use tennis_scorer_api::AppState;
use tennis_scorer_api::auth::firebase::FirebaseTokenVerifier;
use tennis_scorer_api::config::AppConfig;

// ---------------------------------------------------------------------------
// RSA key pair for mock Firebase tokens (generated once, reused across tests)
// ---------------------------------------------------------------------------

const TEST_PROJECT_ID: &str = "test-project";
const TEST_KID: &str = "test-key-1";

// 2048-bit RSA private key in PKCS#8 PEM format (for signing test JWTs)
const TEST_RSA_PRIVATE_KEY: &str = "-----BEGIN PRIVATE KEY-----
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQDP+T2UAhapy2X1
zJqykgWdd/+kiqUFLZnj7uqhAkmw+PlDLPxC41uf8tZhllRCbHKWT4+QRjrAawtm
Ud1DQK6oPAQf+eP9baQk5IiOfbz5oDPW6rT7QG4H1Rgd9cBaKcO3kW/WPxAM5u8M
tYQUdMPV4iWPvuA71D5xZxVt3lQjhkCetY1GRHEXaGFpLO4HK8HjxHE8kTrW9mhG
ZEeDI5HnlK8NDec1cAeqXKeC22ogNQcgApPjRmvfN0EE45ctDMbGO8ZNRoNIajYL
Le63VXt6uivZs4BwbLc9icLGNlXMdk7o3vs3ih/7ITmi/TR7nl37TMHghqprjfu5
F3XAjtXxAgMBAAECggEAHLJkh86999HkEMWZtvglJDRRpw+mc492Q5hM8ciSCIRi
SJ2ldUlP9EMax75pg/zY1trFkX/PTYu3t/el00jSkM4vN4ZQqkB9vMV3/kllUQCF
Bqu+K0kZpaUGveOSFh7bLbI4v1RWT6Fx7MwDHJt8BkA9NJd+82J290jlERLzgy8c
V2Qtank6nFvcexo2Kq/UCEPdwsh2i2ZDBPfKDr8LA2d75NDb5GuvcKnIHqhfX4D0
d9Wnzs5BWSEJ0e6pAMt/9AXxbN8Dtal7DmGNZWm/hhIGhxiQDDDgkKFjUpKKdcxk
d2o1Y7XvBgG3Kk+UQeJ0QeSzUwKjxEn4ETdFxN6q0QKBgQD3oM6ssE7SuDNRgj2z
kfUGDdiSD7QRKKOATXGK6Jorlbba64hqULCV8QqQAjCGBPm0NBcP0RqBDdvdoW6N
QFv4AVblXKYXAjqcFxGqMuSgbiG3wT8iyazBVqh7gAvqd1euO2MYzJ3bmdhzPLO6
jgtxKoQcXkOuTxbT0OW3De5O/QKBgQDXATpIGBQa8apogskIGUQEySgkLLdN3S5Y
3O1xM30JDoE/yEUB/tUiDU7JKJyt0BcBB27aM+oQ039e419w0Mq3vSsoXont04AB
6KgCE5/oRlSqE+TrSgoUt1mmgd+nXd8Q5pe/CJFRNK0njlLNXtGgOrfL4D/KzHFe
6BJLOHXnBQKBgQDfHBwaq5/Za+2Q+u/s4w0JL2B5+XwcGal26E/tADYoHvRput1m
LN1tu4fwyIg/uCvjmStOLPDcZkg7IEAjNGGoykwoy5k6EeAM0xwvZTto8NGgZpUk
GuF0MUgMPgp+bpipewiGR5XTToIfEgo9g837YHs3tBb27nt6zTSsAfk9YQKBgQCp
BQ4MHuGvTMvp3OastzABkyE7TuvLClWlBgijNRbWR9DTk1ysdOiYHF4TRRnmie+L
n4xFfQpEr/8xWQ1uYrT6PHvxAGDt1ZaL6ZoqB8Ntldx416reTRYfswOHIhHwQJtb
betdAh86924n6nqteBzTGVXjsCZ2BsIZGddHytrlAQKBgBiA1E8OpNDE+Ufx3WOx
WOCdoqu5k95ld5OOv7gQ7Vg5vDWFdoKg+62kEFw6S6kZKQicXxGsaqlFQnfXSAnP
/S74nRQKPyXln6Jt6A/roXC2Gq8FQOE2V0CDF2vA5IYCXx7oQs3/ETBViJsdxzoZ
CmYCBfcZ9EWMuXdZt/eYmZlm
-----END PRIVATE KEY-----";

// Corresponding public certificate in X.509 PEM format (for verification)
const TEST_RSA_CERT: &str = "-----BEGIN CERTIFICATE-----
MIIDCTCCAfGgAwIBAgIURT7jO5M8G0kLYteYQt2FjqJv584wDQYJKoZIhvcNAQEL
BQAwFDESMBAGA1UEAwwJbG9jYWxob3N0MB4XDTI2MDMwMzEzNTcxMFoXDTM2MDIy
OTEzNTcxMFowFDESMBAGA1UEAwwJbG9jYWxob3N0MIIBIjANBgkqhkiG9w0BAQEF
AAOCAQ8AMIIBCgKCAQEAz/k9lAIWqctl9cyaspIFnXf/pIqlBS2Z4+7qoQJJsPj5
Qyz8QuNbn/LWYZZUQmxylk+PkEY6wGsLZlHdQ0CuqDwEH/nj/W2kJOSIjn28+aAz
1uq0+0BuB9UYHfXAWinDt5Fv1j8QDObvDLWEFHTD1eIlj77gO9Q+cWcVbd5UI4ZA
nrWNRkRxF2hhaSzuByvB48RxPJE61vZoRmRHgyOR55SvDQ3nNXAHqlyngttqIDUH
IAKT40Zr3zdBBOOXLQzGxjvGTUaDSGo2Cy3ut1V7eror2bOAcGy3PYnCxjZVzHZO
6N77N4of+yE5ov00e55d+0zB4Iaqa437uRd1wI7V8QIDAQABo1MwUTAdBgNVHQ4E
FgQU2gJgeG1I6z6wwTMI6B4GLcii88EwHwYDVR0jBBgwFoAU2gJgeG1I6z6wwTMI
6B4GLcii88EwDwYDVR0TAQH/BAUwAwEB/zANBgkqhkiG9w0BAQsFAAOCAQEAKQ1e
hVS2p79YmAQUHNDmDlHi/lXqYCZ2tPC/qhBq3FSzi4418am0Ax32vmGVm/Naj0sn
bLgUCIcG1DmPQAOnl0dRZZYyzds+LQU9YYOC2WVzqY2XTKTJXa+5ecPhoB3Sfyqu
ryeLwfrxKECWJous5caZtm3spWuDcHOSJfjRLVD8ByKxAB8bBSKKNJyAMaDdR0hU
9CRrZi8EQmFF6wbCd7OiIxtg+gKFck6J3OAeVbqOVnMKdiM7aAbRiplJATRUxbLp
o3KaRfEwDubFFqva8FoJ+iujY827C6SrcxPwbDh5RuSOpGrmSynq8fWsuPE3tcdp
5kPtM8TsdoJ7WpB64g==
-----END CERTIFICATE-----";

// ---------------------------------------------------------------------------
// Mock Firebase token creation
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct MockFirebaseClaims {
    sub: String,
    email: Option<String>,
    name: Option<String>,
    picture: Option<String>,
    iss: String,
    aud: String,
    exp: i64,
    iat: i64,
}

fn create_mock_firebase_token(
    firebase_uid: &str,
    email: Option<&str>,
    display_name: Option<&str>,
    avatar_url: Option<&str>,
) -> String {
    let now = Utc::now();
    let claims = MockFirebaseClaims {
        sub: firebase_uid.to_string(),
        email: email.map(String::from),
        name: display_name.map(String::from),
        picture: avatar_url.map(String::from),
        iss: format!("https://securetoken.google.com/{TEST_PROJECT_ID}"),
        aud: TEST_PROJECT_ID.to_string(),
        exp: (now + Duration::hours(1)).timestamp(),
        iat: now.timestamp(),
    };

    let mut header = Header::new(jsonwebtoken::Algorithm::RS256);
    header.kid = Some(TEST_KID.to_string());

    let key = EncodingKey::from_rsa_pem(TEST_RSA_PRIVATE_KEY.as_bytes())
        .expect("Failed to create encoding key from test RSA private key");

    encode(&header, &claims, &key).expect("Failed to encode mock Firebase token")
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Ensures schema is created exactly once across all parallel tests.
static SCHEMA_READY: AtomicBool = AtomicBool::new(false);
static SCHEMA_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Final database schema (combines all migrations into idempotent statements).
const SCHEMA_STATEMENTS: &[&str] = &[
    "CREATE TABLE IF NOT EXISTS users (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        email TEXT UNIQUE,
        firebase_uid TEXT NOT NULL UNIQUE,
        display_name TEXT,
        avatar_url TEXT,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now()
    )",
    "CREATE TABLE IF NOT EXISTS matches (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        user_id UUID NOT NULL REFERENCES users(id),
        client_id UUID UNIQUE,
        match_type TEXT NOT NULL DEFAULT 'singles',
        config JSONB NOT NULL,
        winner SMALLINT NOT NULL,
        player1_sets SMALLINT NOT NULL,
        player2_sets SMALLINT NOT NULL,
        started_at TIMESTAMPTZ NOT NULL,
        ended_at TIMESTAMPTZ NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now()
    )",
    "CREATE INDEX IF NOT EXISTS idx_matches_user_id ON matches(user_id)",
    "CREATE INDEX IF NOT EXISTS idx_matches_started_at ON matches(started_at)",
    "CREATE TABLE IF NOT EXISTS match_events (
        id BIGSERIAL PRIMARY KEY,
        match_id UUID NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
        point_number INT NOT NULL,
        player SMALLINT NOT NULL,
        timestamp TIMESTAMPTZ NOT NULL,
        UNIQUE(match_id, point_number)
    )",
    "CREATE INDEX IF NOT EXISTS idx_match_events_match_id ON match_events(match_id)",
];

async fn ensure_schema(pool: &sqlx::PgPool) {
    if SCHEMA_READY.load(Ordering::Acquire) {
        return;
    }
    let _guard = SCHEMA_LOCK.lock().unwrap();
    if SCHEMA_READY.load(Ordering::Relaxed) {
        return;
    }
    // Clean slate
    sqlx::query("DROP TABLE IF EXISTS match_events, matches, users CASCADE")
        .execute(pool)
        .await
        .expect("Failed to clean database");
    for sql in SCHEMA_STATEMENTS {
        sqlx::query(sql)
            .execute(pool)
            .await
            .unwrap_or_else(|e| panic!("Schema setup failed: {e}\nSQL: {sql}"));
    }
    SCHEMA_READY.store(true, Ordering::Release);
}

async fn setup() -> axum::Router {
    dotenvy::dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for integration tests");

    let pool = tennis_scorer_api::db::create_pool(&database_url)
        .await
        .expect("Failed to connect to database");

    ensure_schema(&pool).await;

    let mut certs = HashMap::new();
    certs.insert(TEST_KID.to_string(), TEST_RSA_CERT.to_string());
    let verifier = FirebaseTokenVerifier::new_with_certs(TEST_PROJECT_ID.to_string(), certs);

    let state = AppState {
        pool,
        firebase_verifier: verifier,
    };

    let config = AppConfig {
        firebase_project_id: TEST_PROJECT_ID.to_string(),
        allowed_origins: Vec::new(),
    };

    tennis_scorer_api::create_router_with_state(state, &config)
}

async fn body_json(response: axum::response::Response) -> Value {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

fn auth_json_request(method: &str, uri: &str, body: Value, token: &str) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap()
}

fn auth_request(method: &str, uri: &str, token: &str) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap()
}

/// Create a user via PUT /api/auth/me and return the Firebase token.
async fn create_user_and_get_token(app: &axum::Router, email: &str) -> String {
    let firebase_uid = format!("firebase_{}", uuid::Uuid::new_v4());
    let token = create_mock_firebase_token(&firebase_uid, Some(email), Some("Test User"), None);

    let resp = app
        .clone()
        .oneshot(auth_request("PUT", "/api/auth/me", &token))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    token
}

// ---------------------------------------------------------------------------
// Auth: PUT /api/auth/me
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn test_auth_me_creates_new_user() {
    let app = setup().await;
    let firebase_uid = format!("firebase_{}", uuid::Uuid::new_v4());
    let email = format!("new_{}@example.com", uuid::Uuid::new_v4());
    let token = create_mock_firebase_token(&firebase_uid, Some(&email), Some("New User"), None);

    let resp = app
        .clone()
        .oneshot(auth_request("PUT", "/api/auth/me", &token))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert!(body["id"].is_string());
    assert_eq!(body["email"], email);
    assert_eq!(body["display_name"], "New User");
    assert!(body["avatar_url"].is_null());
}

#[tokio::test]
#[ignore]
async fn test_auth_me_returns_existing_user() {
    let app = setup().await;
    let firebase_uid = format!("firebase_{}", uuid::Uuid::new_v4());
    let email = format!("existing_{}@example.com", uuid::Uuid::new_v4());
    let token = create_mock_firebase_token(&firebase_uid, Some(&email), Some("User"), None);

    // First call creates
    let resp = app
        .clone()
        .oneshot(auth_request("PUT", "/api/auth/me", &token))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let first_id = body_json(resp).await["id"].as_str().unwrap().to_string();

    // Second call returns same user
    let resp = app
        .clone()
        .oneshot(auth_request("PUT", "/api/auth/me", &token))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let second_id = body_json(resp).await["id"].as_str().unwrap().to_string();
    assert_eq!(first_id, second_id);
}

#[tokio::test]
#[ignore]
async fn test_auth_me_updates_profile() {
    let app = setup().await;
    let firebase_uid = format!("firebase_{}", uuid::Uuid::new_v4());
    let email = format!("profile_{}@example.com", uuid::Uuid::new_v4());

    // Create with initial name
    let token1 = create_mock_firebase_token(&firebase_uid, Some(&email), Some("Old Name"), None);
    let resp = app
        .clone()
        .oneshot(auth_request("PUT", "/api/auth/me", &token1))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(body_json(resp).await["display_name"], "Old Name");

    // Login again with updated name and avatar
    let token2 = create_mock_firebase_token(
        &firebase_uid,
        Some(&email),
        Some("New Name"),
        Some("https://example.com/avatar.jpg"),
    );
    let resp = app
        .clone()
        .oneshot(auth_request("PUT", "/api/auth/me", &token2))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert_eq!(body["display_name"], "New Name");
    assert_eq!(body["avatar_url"], "https://example.com/avatar.jpg");
}

// ---------------------------------------------------------------------------
// Auth error cases
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn test_unauthorized_access_without_token() {
    let app = setup().await;

    // GET /api/matches without a token
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/matches")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    // GET /api/stats/summary without a token
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/stats/summary")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
#[ignore]
async fn test_invalid_firebase_token() {
    let app = setup().await;

    let resp = app
        .clone()
        .oneshot(auth_request("PUT", "/api/auth/me", "not-a-valid-jwt"))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
#[ignore]
async fn test_middleware_rejects_unknown_user() {
    let app = setup().await;

    // Create a valid Firebase token for a user that hasn't called /auth/me
    let firebase_uid = format!("firebase_{}", uuid::Uuid::new_v4());
    let token = create_mock_firebase_token(&firebase_uid, Some("unknown@example.com"), None, None);

    // Try accessing a protected endpoint (not /auth/me) without creating user first
    let resp = app
        .clone()
        .oneshot(auth_request("GET", "/api/matches", &token))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

// ---------------------------------------------------------------------------
// Match CRUD
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn test_match_create_list_get_delete() {
    let app = setup().await;
    let email = format!("match_{}@example.com", uuid::Uuid::new_v4());
    let token = create_user_and_get_token(&app, &email).await;

    // Create a match
    let resp = app
        .clone()
        .oneshot(auth_json_request(
            "POST",
            "/api/matches",
            json!({
                "match_type": "singles",
                "config": {"sets_to_win": 2, "tiebreak_points": 7, "final_set_tiebreak": true, "no_ad_scoring": false},
                "winner": 1,
                "player1_sets": 2,
                "player2_sets": 0,
                "started_at": "2026-02-06T10:00:00Z",
                "ended_at": "2026-02-06T11:00:00Z",
                "events": [
                    {"point_number": 1, "player": 1, "timestamp": "2026-02-06T10:01:00Z"},
                    {"point_number": 2, "player": 1, "timestamp": "2026-02-06T10:02:00Z"}
                ]
            }),
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body = body_json(resp).await;
    let match_id = body["id"].as_str().unwrap().to_string();

    // List matches -- should contain exactly 1
    let resp = app
        .clone()
        .oneshot(auth_request("GET", "/api/matches", &token))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert_eq!(body["total"], 1);
    assert_eq!(body["matches"].as_array().unwrap().len(), 1);

    // Get match detail -- should include 2 events
    let resp = app
        .clone()
        .oneshot(auth_request(
            "GET",
            &format!("/api/matches/{match_id}"),
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert_eq!(body["id"].as_str().unwrap(), match_id);
    assert_eq!(body["events"].as_array().unwrap().len(), 2);

    // Delete match
    let resp = app
        .clone()
        .oneshot(auth_request(
            "DELETE",
            &format!("/api/matches/{match_id}"),
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // List should now be empty
    let resp = app
        .clone()
        .oneshot(auth_request("GET", "/api/matches", &token))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert_eq!(body["total"], 0);
}

// ---------------------------------------------------------------------------
// Match idempotency via client_id
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn test_match_idempotency_with_client_id() {
    let app = setup().await;
    let email = format!("idempotent_{}@example.com", uuid::Uuid::new_v4());
    let token = create_user_and_get_token(&app, &email).await;

    let client_id = uuid::Uuid::new_v4().to_string();
    let payload = json!({
        "client_id": client_id,
        "match_type": "singles",
        "config": {"sets_to_win": 2, "tiebreak_points": 7, "final_set_tiebreak": true, "no_ad_scoring": false},
        "winner": 1,
        "player1_sets": 2,
        "player2_sets": 1,
        "started_at": "2026-02-06T10:00:00Z",
        "ended_at": "2026-02-06T11:00:00Z",
        "events": []
    });

    // First create -- 201
    let resp = app
        .clone()
        .oneshot(auth_json_request(
            "POST",
            "/api/matches",
            payload.clone(),
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    let first_id = body_json(resp).await["id"].as_str().unwrap().to_string();

    // Second create with same client_id -- 200 with same id
    let resp = app
        .clone()
        .oneshot(auth_json_request(
            "POST",
            "/api/matches",
            payload.clone(),
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let second_id = body_json(resp).await["id"].as_str().unwrap().to_string();
    assert_eq!(
        first_id, second_id,
        "idempotent request should return same match id"
    );
}

// ---------------------------------------------------------------------------
// Stats summary
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn test_stats_summary() {
    let app = setup().await;
    let email = format!("stats_{}@example.com", uuid::Uuid::new_v4());
    let token = create_user_and_get_token(&app, &email).await;

    // Empty stats
    let resp = app
        .clone()
        .oneshot(auth_request("GET", "/api/stats/summary", &token))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert_eq!(body["total_matches"], 0);
    assert_eq!(body["wins"], 0);
    assert_eq!(body["losses"], 0);
    assert_eq!(body["win_rate"], 0.0);

    // Create a won match (winner = 1 means user won)
    let resp = app
        .clone()
        .oneshot(auth_json_request(
            "POST",
            "/api/matches",
            json!({
                "match_type": "singles",
                "config": {"sets_to_win": 2, "tiebreak_points": 7, "final_set_tiebreak": true, "no_ad_scoring": false},
                "winner": 1,
                "player1_sets": 2,
                "player2_sets": 0,
                "started_at": "2026-02-06T10:00:00Z",
                "ended_at": "2026-02-06T11:00:00Z",
                "events": []
            }),
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Create a lost match (winner = 2 means opponent won)
    let resp = app
        .clone()
        .oneshot(auth_json_request(
            "POST",
            "/api/matches",
            json!({
                "match_type": "singles",
                "config": {"sets_to_win": 2, "tiebreak_points": 7, "final_set_tiebreak": true, "no_ad_scoring": false},
                "winner": 2,
                "player1_sets": 0,
                "player2_sets": 2,
                "started_at": "2026-02-06T12:00:00Z",
                "ended_at": "2026-02-06T13:00:00Z",
                "events": []
            }),
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Stats should reflect 2 matches, 1 win, 1 loss
    let resp = app
        .clone()
        .oneshot(auth_request("GET", "/api/stats/summary", &token))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert_eq!(body["total_matches"], 2);
    assert_eq!(body["wins"], 1);
    assert_eq!(body["losses"], 1);
    assert_eq!(body["win_rate"], 0.5);
    assert_eq!(
        body["current_streak"]["streak_type"].as_str().unwrap(),
        "loss"
    );
    assert_eq!(body["current_streak"]["count"], 1);

    let form = body["recent_form"].as_array().unwrap();
    // Most recent first: loss, then win
    assert_eq!(form[0].as_str().unwrap(), "L");
    assert_eq!(form[1].as_str().unwrap(), "W");
}

// ---------------------------------------------------------------------------
// Health check
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn test_health_check() {
    let app = setup().await;

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert_eq!(body["status"], "ok");
}

// ---------------------------------------------------------------------------
// Match analysis endpoints
// ---------------------------------------------------------------------------

async fn create_match_with_events(app: &axum::Router, token: &str) -> String {
    let resp = app
        .clone()
        .oneshot(auth_json_request(
            "POST",
            "/api/matches",
            json!({
                "match_type": "singles",
                "config": {"sets_to_win": 2, "tiebreak_points": 7, "final_set_tiebreak": true, "no_ad_scoring": false},
                "winner": 1,
                "player1_sets": 2,
                "player2_sets": 0,
                "started_at": "2026-02-06T10:00:00Z",
                "ended_at": "2026-02-06T11:00:00Z",
                "events": [
                    {"point_number": 1, "player": 1, "timestamp": "2026-02-06T10:01:00Z"},
                    {"point_number": 2, "player": 1, "timestamp": "2026-02-06T10:02:00Z"},
                    {"point_number": 3, "player": 2, "timestamp": "2026-02-06T10:03:00Z"},
                    {"point_number": 4, "player": 1, "timestamp": "2026-02-06T10:04:00Z"}
                ]
            }),
            token,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    body_json(resp).await["id"].as_str().unwrap().to_string()
}

#[tokio::test]
#[ignore]
async fn test_match_analysis_valid() {
    let app = setup().await;
    let email = format!("analysis_{}@example.com", uuid::Uuid::new_v4());
    let token = create_user_and_get_token(&app, &email).await;
    let match_id = create_match_with_events(&app, &token).await;

    let resp = app
        .clone()
        .oneshot(auth_request(
            "GET",
            &format!("/api/stats/match/{match_id}/analysis"),
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert!(body["player1"].is_object(), "should have player1 stats");
    assert!(body["player2"].is_object(), "should have player2 stats");
    assert!(body["player1"]["total_points"].is_object());
}

#[tokio::test]
#[ignore]
async fn test_match_momentum_valid() {
    let app = setup().await;
    let email = format!("momentum_{}@example.com", uuid::Uuid::new_v4());
    let token = create_user_and_get_token(&app, &email).await;
    let match_id = create_match_with_events(&app, &token).await;

    let resp = app
        .clone()
        .oneshot(auth_request(
            "GET",
            &format!("/api/stats/match/{match_id}/momentum"),
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert!(body["basic"].is_array(), "should have basic momentum array");
    assert!(
        body["weighted"].is_array(),
        "should have weighted momentum array"
    );
}

#[tokio::test]
#[ignore]
async fn test_match_pace_valid() {
    let app = setup().await;
    let email = format!("pace_{}@example.com", uuid::Uuid::new_v4());
    let token = create_user_and_get_token(&app, &email).await;
    let match_id = create_match_with_events(&app, &token).await;

    let resp = app
        .clone()
        .oneshot(auth_request(
            "GET",
            &format!("/api/stats/match/{match_id}/pace"),
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert!(
        body["point_intervals"].is_array(),
        "should have point intervals"
    );
    assert!(
        body["total_duration_seconds"].is_number(),
        "should have total duration"
    );
}

#[tokio::test]
#[ignore]
async fn test_match_analysis_not_found() {
    let app = setup().await;
    let email = format!("anf_{}@example.com", uuid::Uuid::new_v4());
    let token = create_user_and_get_token(&app, &email).await;

    let fake_id = uuid::Uuid::new_v4();
    let resp = app
        .clone()
        .oneshot(auth_request(
            "GET",
            &format!("/api/stats/match/{fake_id}/analysis"),
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
#[ignore]
async fn test_match_analysis_not_owned() {
    let app = setup().await;
    let email_a = format!("owner_{}@example.com", uuid::Uuid::new_v4());
    let email_b = format!("other_{}@example.com", uuid::Uuid::new_v4());
    let token_a = create_user_and_get_token(&app, &email_a).await;
    let token_b = create_user_and_get_token(&app, &email_b).await;

    let match_id = create_match_with_events(&app, &token_a).await;

    // User B cannot access user A's match analysis
    for endpoint in &["analysis", "momentum", "pace"] {
        let resp = app
            .clone()
            .oneshot(auth_request(
                "GET",
                &format!("/api/stats/match/{match_id}/{endpoint}"),
                &token_b,
            ))
            .await
            .unwrap();
        assert_eq!(
            resp.status(),
            StatusCode::NOT_FOUND,
            "{endpoint} should return 404 for non-owner"
        );
    }
}

#[tokio::test]
#[ignore]
async fn test_match_analysis_no_events() {
    let app = setup().await;
    let email = format!("noevents_{}@example.com", uuid::Uuid::new_v4());
    let token = create_user_and_get_token(&app, &email).await;

    // Create match with no events
    let resp = app
        .clone()
        .oneshot(auth_json_request(
            "POST",
            "/api/matches",
            json!({
                "match_type": "singles",
                "config": {"sets_to_win": 2, "tiebreak_points": 7, "final_set_tiebreak": true, "no_ad_scoring": false},
                "winner": 1,
                "player1_sets": 2,
                "player2_sets": 0,
                "started_at": "2026-02-06T10:00:00Z",
                "ended_at": "2026-02-06T11:00:00Z",
                "events": []
            }),
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    let match_id = body_json(resp).await["id"].as_str().unwrap().to_string();

    // Analysis should return 200 with zeroed stats
    let resp = app
        .clone()
        .oneshot(auth_request(
            "GET",
            &format!("/api/stats/match/{match_id}/analysis"),
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert_eq!(body["player1"]["total_points"]["total_points"], 0);

    // Momentum should return 200 with empty arrays
    let resp = app
        .clone()
        .oneshot(auth_request(
            "GET",
            &format!("/api/stats/match/{match_id}/momentum"),
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert_eq!(body["basic"].as_array().unwrap().len(), 0);

    // Pace should return 200 with zero duration
    let resp = app
        .clone()
        .oneshot(auth_request(
            "GET",
            &format!("/api/stats/match/{match_id}/pace"),
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert_eq!(body["total_duration_seconds"], 0.0);
}

// ---------------------------------------------------------------------------
// Cross-user isolation
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn test_cross_user_isolation() {
    let app = setup().await;

    let email_a = format!("user_a_{}@example.com", uuid::Uuid::new_v4());
    let email_b = format!("user_b_{}@example.com", uuid::Uuid::new_v4());
    let token_a = create_user_and_get_token(&app, &email_a).await;
    let token_b = create_user_and_get_token(&app, &email_b).await;

    // User A creates a match
    let resp = app
        .clone()
        .oneshot(auth_json_request(
            "POST",
            "/api/matches",
            json!({
                "match_type": "singles",
                "config": {"sets_to_win": 2, "tiebreak_points": 7, "final_set_tiebreak": true, "no_ad_scoring": false},
                "winner": 1,
                "player1_sets": 2,
                "player2_sets": 0,
                "started_at": "2026-02-06T10:00:00Z",
                "ended_at": "2026-02-06T11:00:00Z",
                "events": []
            }),
            &token_a,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    let match_id = body_json(resp).await["id"].as_str().unwrap().to_string();

    // User B should see 0 matches in their list
    let resp = app
        .clone()
        .oneshot(auth_request("GET", "/api/matches", &token_b))
        .await
        .unwrap();
    let body = body_json(resp).await;
    assert_eq!(body["total"], 0);

    // User B cannot access User A's match by id
    let resp = app
        .clone()
        .oneshot(auth_request(
            "GET",
            &format!("/api/matches/{match_id}"),
            &token_b,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    // User B cannot delete User A's match
    let resp = app
        .clone()
        .oneshot(auth_request(
            "DELETE",
            &format!("/api/matches/{match_id}"),
            &token_b,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
