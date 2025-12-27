use std::error::Error;

use thiserror::Error;

pub mod project_repository;
pub mod user_repository;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("failed to read s3://{bucket}/{key}: {cause:?}")]
    FailedToRead {
        bucket: String,
        key: String,
        cause: Box<dyn Error>,
    },
    #[error("failed to write s3://{bucket}/{key}: {cause:?}")]
    FailedToWrite {
        bucket: String,
        key: String,
        cause: Box<dyn Error>,
    },
    #[error("invalid filename {0}")]
    InvalidFilename(String),
}

pub type Result<T> = std::result::Result<T, RepositoryError>;
