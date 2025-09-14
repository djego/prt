use async_trait::async_trait;
use crate::domain::services::repository_repository::RepositoryRepository;
use crate::domain::models::repository::Repository;
use crate::domain::models::pull_request::DomainError;
use crate::infrastructure::git::client::GitClient;

pub struct GitRepositoryRepository {
    git_client: GitClient,
}

impl GitRepositoryRepository {
    pub fn new() -> Self {
        Self {
            git_client: GitClient::new(),
        }
    }
}

#[async_trait]
impl RepositoryRepository for GitRepositoryRepository {
    async fn get_info(&self) -> Result<Repository, DomainError> {
        let (owner, repo) = self.git_client.get_repo_info()
            .ok_or_else(|| DomainError::RepositoryError("Could not get repository info from git".to_string()))?;

        let default_branch = self.git_client.get_current_branch()
            .unwrap_or_else(|| "main".to_string());

        Ok(Repository::new(
            owner,
            repo,
            String::new(), // URL will be filled by GitHub repository
            default_branch,
        ))
    }

    async fn sync_info(&self) -> Result<Repository, DomainError> {
        self.get_info().await
    }
}