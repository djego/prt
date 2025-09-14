use async_trait::async_trait;
use crate::domain::models::pull_request::{PullRequest, CreatePullRequest, DomainError};

#[async_trait]
pub trait PullRequestRepository {
    async fn create(&self, request: &CreatePullRequest) -> Result<PullRequest, DomainError>;
    async fn get(&self, id: &str) -> Result<PullRequest, DomainError>;
    async fn list(&self) -> Result<Vec<PullRequest>, DomainError>;
}