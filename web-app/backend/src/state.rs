use dotenvy;
use serde::Deserialize;

use crate::WebAppError;

#[derive(Deserialize, Clone)]
pub struct AppState {
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_url: String,
    pub jwt_secret: String,
    #[serde(default = "default_bind")]
    pub bind: String,
    #[serde(default = "default_jwt_expire_duration_hours")]
    pub jwt_expire_duration_hours: u32,
}

fn default_bind() -> String {
    "0.0.0.0:3000".to_string()
}

fn default_jwt_expire_duration_hours() -> u32 {
    30 * 24 // 30 days
}

impl AppState {
    pub fn new() -> Result<AppState, WebAppError> {
        dotenvy::dotenv()?;
        Ok(envy::prefixed("RAYTRACE_").from_env::<AppState>()?)
    }
}
