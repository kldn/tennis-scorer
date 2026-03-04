#[derive(Debug, Clone)]
pub struct AppConfig {
    pub firebase_project_id: String,
    pub allowed_origins: Vec<String>,
}
