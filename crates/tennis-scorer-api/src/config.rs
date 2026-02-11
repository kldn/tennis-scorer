#[derive(Debug, Clone)]
pub struct AppConfig {
    pub jwt_secret: String,
    pub allowed_origins: Vec<String>,
}
