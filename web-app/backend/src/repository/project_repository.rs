use std::sync::Arc;

use aws_sdk_s3::{Client as S3Client, primitives::ByteStream};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    repository::{RepositoryError, Result},
    utils::s3::{ReadFromS3Data, read_from_s3, read_json_from_s3, write_json_to_s3, write_to_s3},
};

#[derive(ToSchema, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub owner: String,
    pub name: String,
    pub files: Vec<ProjectFile>,
}

#[derive(ToSchema, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectFile {
    pub filename: String,
}

#[derive(Clone)]
pub struct ProjectRepository {
    client: Arc<S3Client>,
    bucket: String,
}

pub const PROJECT_OWNER_EXAMPLE: &str = "example";
pub const CONTENT_TYPE_OPENSCAD: &str = "application/x-openscad";

impl ProjectRepository {
    pub fn new(client: Arc<S3Client>, bucket: &str) -> Self {
        Self {
            client,
            bucket: bucket.to_owned(),
        }
    }

    pub async fn load(&self, project_id: &str) -> Result<Option<Project>> {
        let bucket = &self.bucket;
        let key = self.get_project_json_key(project_id);
        read_json_from_s3::<Project>(&self.client, bucket, &key)
            .await
            .map_err(|err| RepositoryError::FailedToRead {
                bucket: self.bucket.to_owned(),
                key,
                cause: err,
            })
    }

    pub async fn load_file(
        &self,
        project_id: &str,
        filename: &str,
    ) -> Result<Option<ReadFromS3Data>> {
        let bucket = &self.bucket;
        let key = self.get_project_file_key(project_id, filename)?;
        read_from_s3(&self.client, bucket, &key)
            .await
            .map_err(|err| RepositoryError::FailedToRead {
                bucket: self.bucket.to_owned(),
                key,
                cause: err,
            })
    }

    pub async fn save(&self, project: &Project) -> Result<()> {
        let bucket = &self.bucket;
        let key = self.get_project_json_key(&project.id);
        write_json_to_s3(&self.client, bucket, &key, project)
            .await
            .map_err(|err| RepositoryError::FailedToWrite {
                bucket: self.bucket.to_owned(),
                key,
                cause: err,
            })
    }

    pub async fn save_file(
        &self,
        project_id: &str,
        filename: &str,
        content_type: &str,
        data: ByteStream,
    ) -> Result<()> {
        let bucket = &self.bucket;
        let key = self.get_project_file_key(project_id, filename)?;
        write_to_s3(&self.client, bucket, &key, content_type, data)
            .await
            .map_err(|err| RepositoryError::FailedToWrite {
                bucket: self.bucket.to_owned(),
                key,
                cause: err,
            })
    }

    fn get_project_json_key(&self, project_id: &str) -> String {
        format!("store/project/{}/project.json", project_id)
    }

    fn get_project_file_key(&self, project_id: &str, filename: &str) -> Result<String> {
        if filename == "project.json" {
            Err(RepositoryError::InvalidFilename(filename.to_owned()))
        } else {
            Ok(format!("store/project/{project_id}/{filename}"))
        }
    }
}
