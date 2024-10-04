use thiserror::Error;

#[derive(Debug, Error)]
pub enum PullRequestError {
    #[error("GitHub API error: {0}")]
    ApiError(#[from] octocrab::Error),

    #[error("Validation failed")]
    PullRequestValidationFailed(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Repo Not found: {0}")]
    RepoNotFound(String),
}
