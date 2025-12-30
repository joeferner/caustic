use std::sync::Arc;

use anyhow::Result;

use crate::{
    repository::project_repository::{Project, ProjectRepository},
    routes::user_routes::AuthUser,
    services::examples_service::PROJECT_OWNER_EXAMPLE,
};

pub enum LoadProjectResult {
    Project(Project),
    NotFound,
    AccessDenied,
}

pub struct ProjectService {
    project_repository: Arc<ProjectRepository>,
}

impl ProjectService {
    pub fn new(project_repository: Arc<ProjectRepository>) -> Self {
        Self { project_repository }
    }

    pub async fn load_project(
        &self,
        project_id: &str,
        user: &Option<AuthUser>,
    ) -> Result<LoadProjectResult> {
        let project = self.project_repository.load(project_id).await?;
        match project {
            Some(project) => {
                if project.owner == PROJECT_OWNER_EXAMPLE {
                    Ok(LoadProjectResult::Project(project))
                } else if let Some(user) = &user
                    && project.owner != user.email
                {
                    Ok(LoadProjectResult::Project(project))
                } else {
                    Ok(LoadProjectResult::AccessDenied)
                }
            }
            None => Ok(LoadProjectResult::NotFound),
        }
    }
}
