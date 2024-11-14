use thiserror::Error;

#[derive(Error, Debug)]
pub enum BranchError {
    #[error("Git repository error: {0}")]
    GitError(#[from] git2::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Branch operation failed: {0}")]
    OperationError(String),
}