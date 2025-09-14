use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PullRequestStatus {
    Draft,
    Open,
    Closed,
    Merged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub source_branch: String,
    pub target_branch: String,
    pub status: PullRequestStatus,
    pub html_url: Option<String>,
}

impl PullRequest {
    pub fn new(title: String, description: String, source_branch: String, target_branch: String) -> Self {
        Self {
            id: None,
            title,
            description,
            source_branch,
            target_branch,
            status: PullRequestStatus::Draft,
            html_url: None,
        }
    }

    pub fn validate(&self) -> Result<(), DomainError> {
        if self.title.trim().is_empty() {
            return Err(DomainError::ValidationError("Title cannot be empty".to_string()));
        }
        if self.source_branch.trim().is_empty() {
            return Err(DomainError::ValidationError("Source branch cannot be empty".to_string()));
        }
        if self.target_branch.trim().is_empty() {
            return Err(DomainError::ValidationError("Target branch cannot be empty".to_string()));
        }
        if self.source_branch == self.target_branch {
            return Err(DomainError::ValidationError("Source and target branches cannot be the same".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CreatePullRequest {
    pub title: String,
    pub description: String,
    pub source_branch: String,
    pub target_branch: String,
}

impl CreatePullRequest {
    pub fn new(title: String, description: String, source_branch: String, target_branch: String) -> Self {
        Self {
            title,
            description,
            source_branch,
            target_branch,
        }
    }

    pub fn validate(&self) -> Result<(), DomainError> {
        if self.title.trim().is_empty() {
            return Err(DomainError::ValidationError("Title cannot be empty".to_string()));
        }
        if self.source_branch.trim().is_empty() {
            return Err(DomainError::ValidationError("Source branch cannot be empty".to_string()));
        }
        if self.target_branch.trim().is_empty() {
            return Err(DomainError::ValidationError("Target branch cannot be empty".to_string()));
        }
        Ok(())
    }
}