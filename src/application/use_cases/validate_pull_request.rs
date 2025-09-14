use crate::domain::models::pull_request::{CreatePullRequest, DomainError};

pub struct ValidatePullRequestUseCase;

impl ValidatePullRequestUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, request: &CreatePullRequest) -> Result<(), DomainError> {
        request.validate()
    }
}