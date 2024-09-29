use crate::core::errors::PullRequestError;
use crate::core::git::{get_current_branch, get_repo_info};
use crate::core::input_mode::InputMode;
use crate::core::pull_request::PullRequest;
use octocrab::models::pulls::PullRequest as OctocrabPullRequest;
use octocrab::Octocrab;

pub struct App {
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub pull_request: PullRequest,
    pub input_mode: InputMode,
    pub current_field: usize,
    pub show_popup: bool,
    pub repo_owner: String,
    pub repo_name: String,
    pub default_branch: String,
}

impl App {
    pub fn new() -> App {
        let (repo_owner, repo_name) = match get_repo_info() {
            Some((owner, repo)) => (owner, repo),
            None => ("-".to_string(), "-".to_string()),
        };
        let current_branch = get_current_branch().unwrap_or_else(|| "-".to_string());
        App {
            pull_request: PullRequest::new(current_branch.clone()),
            input_mode: InputMode::Normal,
            current_field: 0,
            show_popup: false,
            error_message: None,
            success_message: None,
            repo_owner,
            repo_name,
            default_branch: std::env::var("GITHUB_DEFAULT_TARGET_BRANCH")
                .unwrap_or_else(|_| "main".to_string()),
        }
    }

    pub async fn create_github_pull_request(
        &self,
    ) -> Result<OctocrabPullRequest, PullRequestError> {
        let github_token = std::env::var("GITHUB_TOKEN")
            .map_err(|_| PullRequestError::PATNotSet("Github PAT not set".to_string()))?;

        let octocrab = Octocrab::builder().personal_token(github_token).build()?;

        if self.pull_request.source_branch.is_empty() {
            return Err(PullRequestError::InvalidInput(
                "Source branch is empty".to_string(),
            ));
        }
        if self.pull_request.target_branch.is_empty() {
            return Err(PullRequestError::InvalidInput(
                "Target branch is empty".to_string(),
            ));
        }

        let pr_result = octocrab
            .pulls(&self.repo_owner, &self.repo_name)
            .create(
                &self.pull_request.title,
                &self.pull_request.source_branch,
                &self.pull_request.target_branch,
            )
            .body(&self.pull_request.description)
            .send()
            .await;

        match pr_result {
            Ok(pr) => Ok(pr),
            Err(e) => {
                if let octocrab::Error::GitHub { source, .. } = &e {
                    match source.status_code.as_u16() {
                        422 => {
                            return Err(PullRequestError::PullRequestValidationFailed(
                                e.to_string(),
                            ));
                        }
                        _ => Err(PullRequestError::ApiError(e)),
                    }
                } else {
                    Err(PullRequestError::ApiError(e))
                }
            }
        }
    }

    pub fn get_current_field_mut(&mut self) -> &mut String {
        match self.current_field {
            0 => &mut self.pull_request.title,
            1 => &mut self.pull_request.description,
            2 => &mut self.pull_request.source_branch,
            3 => &mut self.pull_request.target_branch,
            _ => unreachable!(),
        }
    }

    pub fn enter_edit_mode(&mut self, index: usize) {
        self.input_mode = InputMode::Editing;
        self.current_field = index;
    }

    pub fn confirm_pull_request(&mut self) {
        self.input_mode = InputMode::Creating;
        self.show_popup = true;
    }

    pub fn reset(&mut self) {
        self.pull_request = PullRequest::new(self.pull_request.source_branch.clone());
        self.input_mode = InputMode::Normal;
        self.current_field = 0;
        self.show_popup = false;
        self.error_message = None;
    }

    pub fn set_error(&mut self, error: String) {
        self.error_message = Some(error);
    }

    pub fn set_success(&mut self, success: String) {
        self.success_message = Some(success);
    }

    pub fn clear_success(&mut self) {
        self.success_message = None;
    }
}
