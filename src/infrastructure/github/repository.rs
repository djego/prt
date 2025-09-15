use async_trait::async_trait;
use crate::domain::services::pull_request_repository::PullRequestRepository;
use crate::domain::services::repository_repository::RepositoryRepository;
use crate::domain::models::pull_request::{PullRequest, CreatePullRequest, PullRequestStatus, DomainError};
use crate::domain::models::repository::Repository;
use crate::infrastructure::github::client::GitHubClient;

pub struct GitHubPullRequestRepository {
    client: GitHubClient,
    owner: String,
    repo: String,
}

impl GitHubPullRequestRepository {
    pub fn new(client: GitHubClient, owner: String, repo: String) -> Self {
        Self { client, owner, repo }
    }
}

#[async_trait]
impl PullRequestRepository for GitHubPullRequestRepository {
    async fn create(&self, request: &CreatePullRequest) -> Result<PullRequest, DomainError> {
        let pr_result = self.client.get_client()
            .pulls(&self.owner, &self.repo)
            .create(
                &request.title,
                &request.source_branch,
                &request.target_branch,
            )
            .body(&request.description)
            .send()
            .await;

        match pr_result {
            Ok(pr) => {
                let pull_request = PullRequest {
                    id: Some(pr.number.to_string()),
                    title: pr.title.unwrap_or_default(),
                    description: pr.body.unwrap_or_default(),
                    source_branch: request.source_branch.clone(),
                    target_branch: request.target_branch.clone(),
                    status: PullRequestStatus::Open,
                    html_url: pr.html_url.map(|url| url.to_string()),
                };
                Ok(pull_request)
            }
            Err(e) => {
                if let octocrab::Error::GitHub { source, .. } = &e {
                    match source.status_code.as_u16() {
                        422 => {
                            return Err(DomainError::Validation(e.to_string()));
                        }
                        _ => Err(DomainError::Repository(e.to_string())),
                    }
                } else {
                    Err(DomainError::Repository(e.to_string()))
                }
            }
        }
    }

    async fn get(&self, id: &str) -> Result<PullRequest, DomainError> {
        let pr_number: u64 = id.parse().map_err(|_| DomainError::Validation("Invalid PR ID".to_string()))?;

        let pr = self.client.get_client()
            .pulls(&self.owner, &self.repo)
            .get(pr_number)
            .await
            .map_err(|e| DomainError::Repository(e.to_string()))?;

        let status = match pr.state {
            Some(octocrab::models::IssueState::Open) => PullRequestStatus::Open,
            Some(octocrab::models::IssueState::Closed) => {
                if pr.merged_at.is_some() {
                    PullRequestStatus::Merged
                } else {
                    PullRequestStatus::Closed
                }
            }
            _ => PullRequestStatus::Closed,
        };

        Ok(PullRequest {
            id: Some(pr.number.to_string()),
            title: pr.title.unwrap_or_default(),
            description: pr.body.unwrap_or_default(),
            source_branch: pr.head.ref_field,
            target_branch: pr.base.ref_field,
            status,
            html_url: pr.html_url.map(|url| url.to_string()),
        })
    }

    async fn list(&self) -> Result<Vec<PullRequest>, DomainError> {
        let prs = self.client.get_client()
            .pulls(&self.owner, &self.repo)
            .list()
            .send()
            .await
            .map_err(|e| DomainError::Repository(e.to_string()))?;

        let mut pull_requests = Vec::new();
        for pr in prs.items {
            let status = match pr.state {
                Some(octocrab::models::IssueState::Open) => PullRequestStatus::Open,
                Some(octocrab::models::IssueState::Closed) => {
                    if pr.merged_at.is_some() {
                        PullRequestStatus::Merged
                    } else {
                        PullRequestStatus::Closed
                    }
                }
                _ => PullRequestStatus::Closed,
            };

            pull_requests.push(PullRequest {
                id: Some(pr.number.to_string()),
                title: pr.title.unwrap_or_default(),
                description: pr.body.unwrap_or_default(),
                source_branch: pr.head.ref_field,
                target_branch: pr.base.ref_field,
                status,
                html_url: pr.html_url.map(|url| url.to_string()),
            });
        }

        Ok(pull_requests)
    }
}

pub struct GitHubRepositoryRepository {
    client: GitHubClient,
    owner: String,
    repo: String,
}

impl GitHubRepositoryRepository {
    pub fn new(client: GitHubClient, owner: String, repo: String) -> Self {
        Self { client, owner, repo }
    }
}

#[async_trait]
impl RepositoryRepository for GitHubRepositoryRepository {
    async fn get_info(&self) -> Result<Repository, DomainError> {
        let repo_info = self.client.get_client()
            .repos(&self.owner, &self.repo)
            .get()
            .await
            .map_err(|e| DomainError::Repository(e.to_string()))?;

        Ok(Repository::new(
            self.owner.clone(),
            self.repo.clone(),
            repo_info.html_url.map(|u| u.to_string()).unwrap_or_default(),
            repo_info.default_branch.unwrap_or_else(|| "main".to_string()),
        ))
    }

    async fn sync_info(&self) -> Result<Repository, DomainError> {
        self.get_info().await
    }
}