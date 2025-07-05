use thiserror::Error;

#[derive(Debug, Error)]
pub enum PullRequestError {
    #[error("GitHub API error: {message} (Source: {source:?})")]
    ApiError {
        message: String,
        source: Option<octocrab::GitHubError>,
    },

    #[error("Validation failed: {0}")]
    PullRequestValidationFailed(String),

    #[error("Invalid input: {field} - {reason}")]
    InvalidInput { field: String, reason: String },

    #[error("Repository not found: {path}")]
    RepoNotFound { path: String },
}

impl From<octocrab::Error> for PullRequestError {
    fn from(error: octocrab::Error) -> Self {
        match error {
            octocrab::Error::GitHub { source, .. } => PullRequestError::ApiError {
                message: source.message.clone(),
                source: Some(*source),
            },
            _ => PullRequestError::ApiError {
                message: error.to_string(),
                source: None,
            },
        }
    }
}
