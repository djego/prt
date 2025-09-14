use async_trait::async_trait;
use crate::domain::services::config_repository::ConfigRepository;
use crate::domain::models::pull_request::DomainError;
use crate::infrastructure::config::file_config::Config;

pub struct FileConfigRepository;

impl FileConfigRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ConfigRepository for FileConfigRepository {
    async fn get_github_token(&self) -> Result<String, DomainError> {
        let config = Config::load()
            .ok_or_else(|| DomainError::ConfigError("Configuration not found".to_string()))?;

        if config.github.pat.is_empty() {
            return Err(DomainError::ConfigError("GitHub token not configured".to_string()));
        }

        Ok(config.github.pat)
    }

    async fn save_github_token(&self, token: &str) -> Result<(), DomainError> {
        let config = Config {
            github: crate::infrastructure::config::file_config::GitHubConfig {
                pat: token.to_string(),
            },
        };

        config.save()
            .map_err(|e| DomainError::ConfigError(format!("Failed to save config: {}", e)))
    }

    fn get_github_token_sync(&self) -> Option<String> {
        Config::load().map(|config| config.github.pat)
    }
}