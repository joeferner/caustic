use std::sync::Arc;

use aws_sdk_s3::Client as S3Client;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    repository::{RepositoryError, Result},
    utils::s3::{write_json_to_s3, write_to_s3},
};

#[derive(ToSchema, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub files: Vec<ProjectFile>,
}

#[derive(ToSchema, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectFile {
    pub filename: String,
    pub url: String,
}

#[derive(Clone)]
pub struct ProjectRepository {
    client: Arc<S3Client>,
    bucket: String,
}

impl ProjectRepository {
    pub fn new(client: Arc<S3Client>, bucket: &str) -> Self {
        Self {
            client,
            bucket: bucket.to_owned(),
        }
    }

    pub async fn save(&self, project: &Project) -> Result<()> {
        let bucket = &self.bucket;
        let key = self.get_project_json_key(project);
        write_json_to_s3(&self.client, bucket, &key, project)
            .await
            .map_err(|err| RepositoryError::FailedToWrite(format!("s3://{bucket}/{key} {err}")))
    }

    pub async fn save_file(&self, project_id: &str, filename: &str, data: &[u8]) -> Result<()> {
        let bucket = &self.bucket;
        let key = self.get_project_file_key(project_id, filename)?;
        write_to_s3(&self.client, bucket, &key, data)
            .await
            .map_err(|err| RepositoryError::FailedToWrite(format!("s3://{bucket}/{key} {err}")))
    }

    fn get_project_json_key(&self, project: &Project) -> String {
        format!("store/project/{}/project.json", &project.id)
    }

    fn get_project_file_key(&self, project_id: &str, filename: &str) -> Result<String> {
        if filename == "project.json" {
            Err(RepositoryError::InvalidFilename(filename.to_owned()))
        } else {
            Ok(format!("store/project/{project_id}/{filename}"))
        }
    }
}
