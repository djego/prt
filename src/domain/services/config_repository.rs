use async_trait::async_trait;
use crate::domain::models::pull_request::DomainError;

#[async_trait]
pub trait ConfigRepository {
    async fn get_github_token(&self) -> Result<String, DomainError>;
    async fn save_github_token(&self, token: &str) -> Result<(), DomainError>;
    fn get_github_token_sync(&self) -> Option<String>;
}