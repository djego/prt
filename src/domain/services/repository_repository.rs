use async_trait::async_trait;
use crate::domain::models::repository::Repository;
use crate::domain::models::pull_request::DomainError;

#[async_trait]
pub trait RepositoryRepository {
    async fn get_info(&self) -> Result<Repository, DomainError>;
    async fn sync_info(&self) -> Result<Repository, DomainError>;
}