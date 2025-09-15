use crate::domain::models::pull_request::{PullRequest, CreatePullRequest};
use crate::domain::models::repository::Repository;
use crate::presentation::state::input_state::InputMode;
use tui_textarea::TextArea;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pull_request: PullRequest,
    pub repository: Option<Repository>,
    pub input_mode: InputMode,
    pub current_field: usize,
    pub show_confirm_popup: bool,
    pub show_pat_popup: bool,
    pub show_exit_popup: bool,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub description_text_area: TextArea<'static>,
    pub pat_input: TextArea<'static>,
    pub config_pat: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        let current_branch = "main".to_string(); // Default, will be updated from git
        let text_area = TextArea::default();
        let pat_input = TextArea::default();

        Self {
            pull_request: PullRequest::new(
                String::new(),
                String::new(),
                current_branch.clone(),
                "main".to_string(),
            ),
            repository: None,
            input_mode: InputMode::Normal,
            current_field: 0,
            show_confirm_popup: false,
            show_pat_popup: false,
            show_exit_popup: false,
            error_message: None,
            success_message: None,
            description_text_area: text_area,
            pat_input,
            config_pat: String::new(),
        }
    }

    pub fn set_repository(&mut self, repo: Repository) {
        let default_branch = repo.default_branch.clone();
        self.repository = Some(repo);
        // Update pull request target branch if repository has a default branch
        if !default_branch.is_empty() {
            self.pull_request.target_branch = default_branch;
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
        let source_branch = self.pull_request.source_branch.clone();
        let target_branch = self.pull_request.target_branch.clone();

        self.pull_request = PullRequest::new(
            String::new(),
            String::new(),
            source_branch,
            target_branch,
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

    pub fn is_editing_description(&self) -> bool {
        self.current_field == 1
    }

    pub fn create_pull_request_request(&self) -> CreatePullRequest {
        CreatePullRequest::new(
            self.pull_request.title.clone(),
            self.pull_request.description.clone(),
            self.pull_request.source_branch.clone(),
            self.pull_request.target_branch.clone(),
        )
    }
}