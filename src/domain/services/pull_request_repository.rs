use crate::domain::models::pull_request::{CreatePullRequest, DomainError, PullRequest};
use async_trait::async_trait;

#[async_trait]
pub trait PullRequestRepository {
    async fn create(&self, request: &CreatePullRequest) -> Result<PullRequest, DomainError>;
    #[allow(dead_code)]
    async fn get(&self, id: &str) -> Result<PullRequest, DomainError>;
    #[allow(dead_code)]
    async fn list(&self) -> Result<Vec<PullRequest>, DomainError>;
}
