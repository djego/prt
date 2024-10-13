use crate::core::config::load_config;
use crate::core::errors::PullRequestError;
use crate::core::git::{get_current_branch, get_repo_info};
use crate::core::github::GithubRepository;
use crate::core::input_mode::InputMode;
use crate::core::pull_request::PullRequest;
use octocrab::models::pulls::PullRequest as OctocrabPullRequest;
use octocrab::{models::Repository, Octocrab};
use tui_textarea::TextArea;

pub struct App {
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub pull_request: PullRequest,
    pub input_mode: InputMode,
    pub current_field: usize,
    pub show_confirm_popup: bool,
    pub show_pat_popup: bool,
    pub github_repository: GithubRepository,
    pub repo_owner: String,
    pub repo_name: String,
    pub config_pat: String,
    pub description_text_area: TextArea<'static>,
    pub pat_input: TextArea<'static>,
}

impl App {
    pub fn new() -> App {
        let (repo_owner, repo_name) = match get_repo_info() {
            Some((owner, repo)) => (owner, repo),
            None => ("-".to_string(), "-".to_string()),
        };
        let current_branch = get_current_branch().unwrap_or_else(|| "-".to_string());

        let config_pat = load_config()
            .map(|config| config.github.pat)
            .unwrap_or_else(|| String::from(""));

        let text_area = TextArea::default();
        let pat_input = TextArea::default();

        App {
            pull_request: PullRequest::new(current_branch.clone(), "main".to_string()),
            input_mode: InputMode::Normal,
            current_field: 0,
            show_confirm_popup: false,
            show_pat_popup: false,
            error_message: None,
            success_message: None,
            config_pat,
            github_repository: GithubRepository::new(),
            repo_owner,
            repo_name,
            description_text_area: text_area,
            pat_input,
        }
    }

    pub async fn create_github_pull_request(
        &self,
    ) -> Result<OctocrabPullRequest, PullRequestError> {
        let pat = self.config_pat.clone();
        let octocrab = Octocrab::builder().personal_token(pat).build()?;
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
        self.show_confirm_popup = true;
    }

    pub fn reset(&mut self) {
        self.pull_request = PullRequest::new(
            self.pull_request.source_branch.clone(),
            self.pull_request.target_branch.clone(),
        );
        self.input_mode = InputMode::Normal;
        self.current_field = 0;
        self.show_confirm_popup = false;
        self.description_text_area = TextArea::default();
        self.clear_message();
    }

    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
    }

    pub fn set_success(&mut self, success: String) {
        self.success_message = Some(success);
    }

    pub fn clear_message(&mut self) {
        self.success_message = None;
        self.error_message = None;
    }

    pub async fn fetch_github_repo_info(&self) -> Result<Repository, PullRequestError> {
        let pat = self.config_pat.clone();
        let octocrab = Octocrab::builder().personal_token(pat).build()?;
        if self.repo_name.is_empty() {
            return Err(PullRequestError::InvalidInput(
                "Repository name is empty".to_string(),
            ));
        }
        if self.repo_owner.is_empty() {
            return Err(PullRequestError::InvalidInput(
                "Repository owner is empty".to_string(),
            ));
        }

        let repo_result = octocrab
            .repos(&self.repo_owner, &self.repo_name)
            .get()
            .await;
        match repo_result {
            Ok(repo) => Ok(repo),
            Err(e) => {
                if let octocrab::Error::GitHub { source, .. } = &e {
                    match source.status_code.as_u16() {
                        404 => {
                            return Err(PullRequestError::RepoNotFound(e.to_string()));
                        }
                        _ => Err(PullRequestError::ApiError(e)),
                    }
                } else {
                    Err(PullRequestError::ApiError(e))
                }
            }
        }
    }
    pub fn is_editing_description(&self) -> bool {
        self.current_field == 1
    }
}
