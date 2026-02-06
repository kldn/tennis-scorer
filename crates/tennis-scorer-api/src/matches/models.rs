use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateMatchRequest {
    pub client_id: Option<Uuid>,
    pub match_type: String,
    pub config: serde_json::Value,
    pub winner: i16,
    pub player1_sets: i16,
    pub player2_sets: i16,
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
    pub events: Vec<CreateMatchEvent>,
}

#[derive(Deserialize)]
pub struct CreateMatchEvent {
    pub point_number: i32,
    pub player: i16,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct MatchResponse {
    pub id: Uuid,
    pub client_id: Option<Uuid>,
    pub match_type: String,
    pub config: serde_json::Value,
    pub winner: i16,
    pub player1_sets: i16,
    pub player2_sets: i16,
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<MatchEventResponse>>,
}

#[derive(Serialize)]
pub struct MatchEventResponse {
    pub point_number: i32,
    pub player: i16,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct MatchListResponse {
    pub matches: Vec<MatchResponse>,
    pub total: i64,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
