use std::sync::Arc;

use crate::{
    WebAppError,
    repository::{project_repository::ProjectRepository, user_repository::UserRepository},
};
use aws_config::{BehaviorVersion, meta::region::RegionProviderChain};
use aws_sdk_s3::Client as S3Client;
use dotenvy;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AppStateSettings {
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_url: String,
    pub jwt_secret: String,
    #[serde(default = "default_bind")]
    pub bind: String,
    #[serde(default = "default_jwt_expire_duration_hours")]
    pub jwt_expire_duration_hours: u32,
    pub data_bucket: String,
}

#[derive(Clone)]
pub struct AppState {
    pub settings: AppStateSettings,
    pub project_repository: ProjectRepository,
    pub user_repository: UserRepository,
}

fn default_bind() -> String {
    "0.0.0.0:3000".to_string()
}

fn default_jwt_expire_duration_hours() -> u32 {
    30 * 24 // 30 days
}

impl AppState {
    pub async fn new() -> Result<AppState, WebAppError> {
        dotenvy::dotenv()?;

        let settings = envy::prefixed("RAYTRACE_").from_env::<AppStateSettings>()?;

        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let config = aws_config::defaults(BehaviorVersion::v2025_08_07())
            .region(region_provider)
            .load()
            .await;
        let s3_client = Arc::new(S3Client::new(&config));

        let project_repository = ProjectRepository::new(s3_client.clone(), &settings.data_bucket);
        let user_repository = UserRepository::new(s3_client, &settings.data_bucket);

        Ok(AppState {
            settings,
            project_repository,
            user_repository,
        })
    }
}
