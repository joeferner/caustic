use axum::Json;
use serde::Serialize;
use tower_http::cors::CorsLayer;
use utoipa::{OpenApi, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

const USER_TAG: &str = "user";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = USER_TAG, description = "User management endpoints")
    )
)]
struct ApiDoc;

#[derive(ToSchema, Debug, Serialize)]
struct UserMe {
    pub name: String,
}

#[utoipa::path(get, path = "/api/v1/user/me", responses((status = OK, body = UserMe)), tag = USER_TAG)]
async fn get_user_me() -> Json<UserMe> {
    Json(UserMe {
        name: "test".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(get_user_me))
        .layer(CorsLayer::permissive())
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening http://0.0.0.0:3000");
    axum::serve(listener, router).await.unwrap();
}
