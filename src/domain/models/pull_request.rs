use serde::{Deserialize, Serialize};
use octocrab;

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Repository error: {0}")]
    Repository(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("GitHub error: {0}")]
    GitHub(#[from] octocrab::Error),
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

    #[allow(dead_code)]
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.title.trim().is_empty() {
            return Err(DomainError::Validation("Title cannot be empty".to_string()));
        }
        if self.source_branch.trim().is_empty() {
            return Err(DomainError::Validation("Source branch cannot be empty".to_string()));
        }
        if self.target_branch.trim().is_empty() {
            return Err(DomainError::Validation("Target branch cannot be empty".to_string()));
        }
        if self.source_branch == self.target_branch {
            return Err(DomainError::Validation("Source and target branches cannot be the same".to_string()));
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
            return Err(DomainError::Validation("Title cannot be empty".to_string()));
        }
        if self.source_branch.trim().is_empty() {
            return Err(DomainError::Validation("Source branch cannot be empty".to_string()));
        }
        if self.target_branch.trim().is_empty() {
            return Err(DomainError::Validation("Target branch cannot be empty".to_string()));
        }
        Ok(())
    }
}