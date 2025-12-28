use std::sync::Arc;

use aws_sdk_s3::primitives::ByteStream;
use axum::{
    Json,
    body::Body,
    extract::{Path, State},
    http::{HeaderValue, header},
    response::Response,
};
use chrono::Utc;
use log::{error, info};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    PROJECT_TAG,
    repository::{
        project_repository::{CONTENT_TYPE_OPENSCAD, Project, ProjectFile},
        user_repository::{UserData, UserDataProject, UserRepository},
    },
    routes::user_routes::AuthUser,
    services::project_service::{LoadProjectResult, ProjectService},
    state::AppState,
};

#[derive(ToSchema, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectRequest {
    name: String,
}

#[derive(ToSchema, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyProjectRequest {
    project_id: String,
}

#[derive(ToSchema, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProjectsResponse {
    pub projects: Vec<UserDataProject>,
}

async fn assert_load_project(
    project_service: &ProjectService,
    project_id: &str,
    user: &AuthUser,
) -> Result<Project, StatusCode> {
    match project_service.load_project(project_id, user).await {
        Ok(project) => match project {
            LoadProjectResult::Project(project) => Ok(project),
            LoadProjectResult::NotFound => Err(StatusCode::NOT_FOUND),
            LoadProjectResult::AccessDenied => Err(StatusCode::UNAUTHORIZED),
        },
        Err(err) => {
            error!("failed to load project (project id: {project_id}): {err:?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn assert_load_user_data(
    user_repository: &UserRepository,
    user: &AuthUser,
) -> Result<UserData, StatusCode> {
    user_repository
        .load(user)
        .await
        .map_err(|err| {
            error!("failed to load user: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::UNAUTHORIZED)
}

#[utoipa::path(
    get,
    path = "/api/v1/project",
    responses(
        (status = OK, body = GetProjectsResponse),
        (status = UNAUTHORIZED),
        (status = INTERNAL_SERVER_ERROR)
    ),
    tag = PROJECT_TAG
)]
pub async fn get_projects(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
) -> Result<Json<GetProjectsResponse>, StatusCode> {
    let user_data = state.user_repository.load(&user).await.map_err(|err| {
        error!("failed to load user data: {err:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut projects = match user_data {
        Some(user_data) => user_data.projects,
        None => vec![],
    };

    for example in &state.example_service.examples {
        projects.push(example.clone());
    }

    let response = GetProjectsResponse { projects };

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/project/{project_id}",
    responses(
        (status = OK, body = Project),
        (status = NOT_FOUND),
        (status = UNAUTHORIZED),
        (status = INTERNAL_SERVER_ERROR)
    ),
    tag = PROJECT_TAG
)]
pub async fn get_project(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    Path(project_id): Path<String>,
) -> Result<Json<Project>, StatusCode> {
    match assert_load_project(&state.project_service, &project_id, &user).await {
        Ok(project) => Ok(Json(project)),
        Err(err) => Err(err),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/project/{project_id}/file/{filename}",
    responses(
        (status = OK, content_type = "application/octet-stream"),
        (status = UNAUTHORIZED),
        (status = INTERNAL_SERVER_ERROR)
    ),
    tag = PROJECT_TAG
)]
pub async fn get_project_file(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    Path((project_id, filename)): Path<(String, String)>,
) -> Result<Response, StatusCode> {
    assert_load_project(&state.project_service, &project_id, &user).await?;

    let file_data = state
        .project_repository
        .load_file(&project_id, &filename)
        .await
        .map_err(|err| {
            error!("failed to load project file: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if let Some(file_data) = file_data {
        let bytes = file_data.body.collect().await.map_err(|err| {
            error!("failed to load file data bytes: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        let body = Body::from(bytes.into_bytes());
        let mut response = Response::new(body);
        response.headers_mut().insert(
            header::CONTENT_DISPOSITION,
            HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename)).map_err(
                |err| {
                    error!("failed to parse header value: {err:?}");
                    StatusCode::INTERNAL_SERVER_ERROR
                },
            )?,
        );

        if let Some(content_type) = file_data.content_type {
            response.headers_mut().insert(
                header::CONTENT_TYPE,
                content_type
                    .parse()
                    .unwrap_or_else(|_| "application/octet-stream".parse().unwrap()),
            );
        }

        Ok(response)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
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

    let mut user_data = assert_load_user_data(&state.user_repository, &user).await?;

    let project_id = Uuid::new_v4().to_string();

    let file = ProjectFile {
        filename: "main.scad".to_string(),
    };
    let project = Project {
        id: project_id.clone(),
        owner: user.email,
        name: payload.name.clone(),
        last_modified: Utc::now(),
        files: vec![file],
    };

    user_data.projects.push(UserDataProject {
        id: project_id,
        readonly: false,
        name: payload.name,
        last_modified: Utc::now(),
    });

    state
        .project_repository
        .save_file(
            &project.id,
            "main.scad",
            CONTENT_TYPE_OPENSCAD,
            ByteStream::from("".to_string().into_bytes()),
        )
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

#[utoipa::path(
    post,
    path = "/api/v1/project/copy",
    responses(
        (status = OK, body = Project),
        (status = UNAUTHORIZED),
        (status = INTERNAL_SERVER_ERROR)
    ),
    tag = PROJECT_TAG
)]
pub async fn copy_project(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    Json(payload): Json<CopyProjectRequest>,
) -> Result<Json<Project>, StatusCode> {
    info!(
        "copying project (project id: {}, username: {})",
        payload.project_id, user.email
    );

    let mut user_data = assert_load_user_data(&state.user_repository, &user).await?;

    let existing_project =
        match assert_load_project(&state.project_service, &payload.project_id, &user).await {
            Ok(project) => project,
            Err(err) => return Err(err),
        };

    let mut new_project = Project {
        id: Uuid::new_v4().to_string(),
        name: format!("{} Copy", existing_project.name),
        owner: user.email,
        files: vec![],
        last_modified: Utc::now(),
    };

    for file in &existing_project.files {
        state
            .project_repository
            .copy_file(&existing_project.id, &new_project.id, &file.filename)
            .await
            .map_err(|err| {
                error!("failed to copy file: {err:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        new_project.files.push(ProjectFile {
            filename: file.filename.to_owned(),
        });
    }

    state
        .project_repository
        .save(&new_project)
        .await
        .map_err(|err| {
            error!("failed to load user: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    user_data.projects.push(UserDataProject {
        id: new_project.id.to_owned(),
        readonly: false,
        name: new_project.name.to_owned(),
        last_modified: Utc::now(),
    });

    state
        .user_repository
        .save(&user_data)
        .await
        .map_err(|err| {
            error!("failed to save user: {err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(new_project))
}
