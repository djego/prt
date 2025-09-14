use crate::domain::services::pull_request_repository::PullRequestRepository;
use crate::domain::models::pull_request::{CreatePullRequest, PullRequest, DomainError};

pub struct CreatePullRequestUseCase<R: PullRequestRepository> {
    repository: R,
}

impl<R: PullRequestRepository> CreatePullRequestUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, request: CreatePullRequest) -> Result<PullRequest, DomainError> {
        // Validate the request
        request.validate()?;

        // Create the pull request
        self.repository.create(&request).await
    }
}