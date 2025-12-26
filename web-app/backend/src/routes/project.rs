use std::sync::Arc;

use axum::{Json, extract::State};
use log::{error, info};
use reqwest::StatusCode;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    PROJECT_TAG,
    repository::{
        project_repository::{Project, ProjectFile},
        user_repository::UserData,
    },
    routes::user::AuthUser,
    state::AppState,
};

#[derive(ToSchema, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectRequest {
    name: String,
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
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    Json(payload): Json<CreateProjectRequest>,
) -> Result<Json<Project>, StatusCode> {
    info!(
        "creating project (name: {}, username: {})",
        payload.name, user.email
    );

    let user_data = state
        .user_repository
        .load(&user)
        .await
        .map_err(|err| {
            error!("failed to load user: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .unwrap_or_else(|| UserData::new(&user));

    let project_id = Uuid::new_v4().to_string();

    let file = ProjectFile {
        filename: "main.scad".to_string(),
        url: format!("/api/v1/project/{project_id}/file/main.scad"),
    };
    let project = Project {
        id: project_id,
        name: payload.name,
        files: vec![file],
    };

    state
        .project_repository
        .save_file(&project.id, "main.scad", "".as_bytes())
        .await
        .map_err(|err| {
            error!("failed to save project file: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    state
        .project_repository
        .save(&project)
        .await
        .map_err(|err| {
            error!("failed to save project: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    state
        .user_repository
        .save(&user_data)
        .await
        .map_err(|err| {
            error!("failed to save user: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(project))
}
