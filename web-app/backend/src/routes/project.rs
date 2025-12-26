use std::sync::Arc;

use axum::{Json, extract::State};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{PROJECT_TAG, routes::user::AuthUser, state::AppState};

#[derive(ToSchema, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectRequest {
    name: String,
}

#[derive(ToSchema, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    id: String,
    name: String,
    files: Vec<ProjectFile>,
}

#[derive(ToSchema, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectFile {
    filename: String,
    url: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/project",
    responses(
        (status = OK, body = Project),
        (status = UNAUTHORIZED),
        (status = INTERNAL_SERVER_ERROR)
    ),
    tag = PROJECT_TAG
)]
pub async fn create_project(
    State(_state): State<Arc<AppState>>,
    _user: AuthUser,
    Json(payload): Json<CreateProjectRequest>,
) -> Result<Json<Project>, StatusCode> {
    let id = "123".to_string();

    let file = ProjectFile {
        filename: "main.scad".to_string(),
        url: format!("/api/v1/project/{id}/file/main.scad"),
    };

    Ok(Json(Project {
        id,
        name: payload.name,
        files: vec![file],
    }))
}
