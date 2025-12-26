use thiserror::Error;

pub mod project_repository;
pub mod user_repository;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("failed to read {0}")]
    FailedToRead(String),
    #[error("failed to write {0}")]
    FailedToWrite(String),
    #[error("invalid filename {0}")]
    InvalidFilename(String),
}

pub type Result<T> = std::result::Result<T, RepositoryError>;
