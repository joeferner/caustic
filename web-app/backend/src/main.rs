pub mod routes;
pub mod state;
pub mod utils;

use env_logger::Env;
use thiserror::Error;

use std::sync::Arc;

use routes::user::{
    __path_get_user_me, __path_google_token_verify, get_user_me, google_token_verify,
};
use tower_http::{cors, cors::CorsLayer};
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use crate::state::AppState;

pub const USER_TAG: &str = "user";

#[derive(Error, Debug)]
pub enum WebAppError {
    #[error("DotEnv: {0}")]
    DotEnv(#[from] dotenvy::Error),
    #[error("Environment variable error: {0}")]
    EnvVar(#[from] envy::Error),
    #[error("std::io: {0}")]
    StdIo(#[from] std::io::Error),
}

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = USER_TAG, description = "User management endpoints")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), WebAppError> {
    let env = Env::default().default_filter_or("info");
    env_logger::init_from_env(env);
    let state = Arc::new(AppState::new()?);
    let bind = state.bind.clone();

    let cors = CorsLayer::new()
        .allow_origin(cors::Any)
        .allow_methods(cors::Any)
        .allow_headers(cors::Any);

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(get_user_me, google_token_verify))
        .with_state(state)
        .layer(cors)
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api));

    let listener = tokio::net::TcpListener::bind(&bind).await?;
    println!("listening http://{bind}");
    axum::serve(listener, router).await?;
    Ok(())
}
