use crate::domain::services::repository_repository::RepositoryRepository;
use crate::domain::models::repository::Repository;
use crate::domain::models::pull_request::DomainError;

pub struct SyncRepositoryUseCase<R: RepositoryRepository> {
    repository: R,
}

impl<R: RepositoryRepository> SyncRepositoryUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Result<Repository, DomainError> {
        self.repository.sync_info().await
    }
}