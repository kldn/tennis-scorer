//! Integration tests -- require a running PostgreSQL database.
//!
//! Run with:
//! ```sh
//! DATABASE_URL=postgres://user:pass@localhost/tennis_scorer_test \
//! JWT_SECRET=test-secret \
//! cargo test --test integration -- --ignored
//! ```
//!
//! All tests are marked `#[ignore]` so that a plain `cargo test` does not
//! fail when no database is available.

use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;

use tennis_scorer_api::config::AppConfig;
use tennis_scorer_api::create_router;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

async fn setup() -> axum::Router {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for integration tests");
    let jwt_secret =
        std::env::var("JWT_SECRET").expect("JWT_SECRET must be set for integration tests");

    let config = AppConfig { jwt_secret };

    let pool = tennis_scorer_api::db::create_pool(&database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations manually (files are plain SQL, not sqlx-managed).
    for sql in [
        include_str!("../migrations/001_create_users.sql"),
        include_str!("../migrations/002_create_matches.sql"),
        include_str!("../migrations/003_create_match_events.sql"),
    ] {
        sqlx::query(sql)
            .execute(&pool)
            .await
            .expect("Migration failed");
    }

    create_router(pool, &config)
}

async fn body_json(response: axum::response::Response) -> Value {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

fn json_request(method: &str, uri: &str, body: Value) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap()
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

/// Register a new user and log in, returning the access token.
async fn register_and_login(app: &axum::Router, email: &str, password: &str) -> String {
    app.clone()
        .oneshot(json_request(
            "POST",
            "/api/auth/register",
            json!({"email": email, "password": password}),
        ))
        .await
        .unwrap();

    let resp = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/api/auth/login",
            json!({"email": email, "password": password}),
        ))
        .await
        .unwrap();

    let body = body_json(resp).await;
    body["access_token"].as_str().unwrap().to_string()
}

// ---------------------------------------------------------------------------
// Auth flow
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn test_auth_register_login_refresh() {
    let app = setup().await;
    let email = format!("auth_{}@example.com", uuid::Uuid::new_v4());

    // Register
    let resp = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/api/auth/register",
            json!({"email": email, "password": "testpassword123"}),
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body = body_json(resp).await;
    assert!(body["id"].is_string(), "register should return a user id");

    // Login
    let resp = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/api/auth/login",
            json!({"email": email, "password": "testpassword123"}),
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert!(body["access_token"].is_string());
    assert!(body["refresh_token"].is_string());

    // Refresh
    let refresh_token = body["refresh_token"].as_str().unwrap();
    let resp = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/api/auth/refresh",
            json!({"refresh_token": refresh_token}),
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert!(body["access_token"].is_string());
}

// ---------------------------------------------------------------------------
// Error cases -- duplicate email
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn test_duplicate_email_returns_conflict() {
    let app = setup().await;
    let email = format!("dup_{}@example.com", uuid::Uuid::new_v4());

    // First registration succeeds
    let resp = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/api/auth/register",
            json!({"email": email, "password": "testpassword123"}),
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Second registration with same email returns 409 Conflict
    let resp = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/api/auth/register",
            json!({"email": email, "password": "testpassword123"}),
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CONFLICT);
}

// ---------------------------------------------------------------------------
// Error cases -- wrong password
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn test_wrong_password_returns_unauthorized() {
    let app = setup().await;
    let email = format!("wrongpw_{}@example.com", uuid::Uuid::new_v4());

    // Register
    app.clone()
        .oneshot(json_request(
            "POST",
            "/api/auth/register",
            json!({"email": email, "password": "testpassword123"}),
        ))
        .await
        .unwrap();

    // Login with wrong password
    let resp = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/api/auth/login",
            json!({"email": email, "password": "wrongpassword"}),
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

// ---------------------------------------------------------------------------
// Error cases -- unauthorized access to protected routes
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

// ---------------------------------------------------------------------------
// Match CRUD
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn test_match_create_list_get_delete() {
    let app = setup().await;
    let email = format!("match_{}@example.com", uuid::Uuid::new_v4());
    let token = register_and_login(&app, &email, "testpassword123").await;

    // Create a match
    let resp = app
        .clone()
        .oneshot(auth_json_request(
            "POST",
            "/api/matches",
            json!({
                "match_type": "singles",
                "config": {"sets_to_win": 2},
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
    let token = register_and_login(&app, &email, "testpassword123").await;

    let client_id = uuid::Uuid::new_v4().to_string();
    let payload = json!({
        "client_id": client_id,
        "match_type": "singles",
        "config": {},
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
    let token = register_and_login(&app, &email, "testpassword123").await;

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
                "config": {},
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
                "config": {},
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
// Validation
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn test_register_validation_short_password() {
    let app = setup().await;
    let email = format!("short_{}@example.com", uuid::Uuid::new_v4());

    let resp = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/api/auth/register",
            json!({"email": email, "password": "short"}),
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
#[ignore]
async fn test_register_validation_invalid_email() {
    let app = setup().await;

    let resp = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/api/auth/register",
            json!({"email": "not-an-email", "password": "testpassword123"}),
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

// ---------------------------------------------------------------------------
// Match analysis endpoints
// ---------------------------------------------------------------------------

/// Helper: create a match with point events and return its id.
async fn create_match_with_events(app: &axum::Router, token: &str) -> String {
    let resp = app
        .clone()
        .oneshot(auth_json_request(
            "POST",
            "/api/matches",
            json!({
                "match_type": "singles",
                "config": {"sets_to_win": 2},
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
    let token = register_and_login(&app, &email, "testpassword123").await;
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
    // Should have player1 and player2 stats
    assert!(body["player1"].is_object(), "should have player1 stats");
    assert!(body["player2"].is_object(), "should have player2 stats");
    assert!(body["player1"]["total_points"].is_object());
}

#[tokio::test]
#[ignore]
async fn test_match_momentum_valid() {
    let app = setup().await;
    let email = format!("momentum_{}@example.com", uuid::Uuid::new_v4());
    let token = register_and_login(&app, &email, "testpassword123").await;
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
    assert!(body["weighted"].is_array(), "should have weighted momentum array");
}

#[tokio::test]
#[ignore]
async fn test_match_pace_valid() {
    let app = setup().await;
    let email = format!("pace_{}@example.com", uuid::Uuid::new_v4());
    let token = register_and_login(&app, &email, "testpassword123").await;
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
    assert!(body["point_intervals_secs"].is_array(), "should have point intervals");
    assert!(body["total_duration_secs"].is_number(), "should have total duration");
}

#[tokio::test]
#[ignore]
async fn test_match_analysis_not_found() {
    let app = setup().await;
    let email = format!("anf_{}@example.com", uuid::Uuid::new_v4());
    let token = register_and_login(&app, &email, "testpassword123").await;

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
    let token_a = register_and_login(&app, &email_a, "testpassword123").await;
    let token_b = register_and_login(&app, &email_b, "testpassword123").await;

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
    let token = register_and_login(&app, &email, "testpassword123").await;

    // Create match with no events
    let resp = app
        .clone()
        .oneshot(auth_json_request(
            "POST",
            "/api/matches",
            json!({
                "match_type": "singles",
                "config": {},
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
    assert_eq!(body["player1"]["total_points"]["total"], 0);

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
    assert_eq!(body["total_duration_secs"], 0.0);
}

// ---------------------------------------------------------------------------
// Cross-user isolation -- one user cannot see another user's matches
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn test_cross_user_isolation() {
    let app = setup().await;

    let email_a = format!("user_a_{}@example.com", uuid::Uuid::new_v4());
    let email_b = format!("user_b_{}@example.com", uuid::Uuid::new_v4());
    let token_a = register_and_login(&app, &email_a, "testpassword123").await;
    let token_b = register_and_login(&app, &email_b, "testpassword123").await;

    // User A creates a match
    let resp = app
        .clone()
        .oneshot(auth_json_request(
            "POST",
            "/api/matches",
            json!({
                "match_type": "singles",
                "config": {},
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
