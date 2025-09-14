use crate::application::use_cases::{create_pull_request::CreatePullRequestUseCase, sync_repository::SyncRepositoryUseCase};
use crate::domain::services::{pull_request_repository::PullRequestRepository, repository_repository::RepositoryRepository, config_repository::ConfigRepository};
use crate::domain::models::pull_request::{CreatePullRequest, DomainError};
use crate::domain::models::repository::Repository;
use crate::infrastructure::github::client::GitHubClient;
use crate::infrastructure::github::repository::{GitHubPullRequestRepository, GitHubRepositoryRepository};
use crate::infrastructure::git::repository::GitRepositoryRepository;
use crate::infrastructure::config::repository::FileConfigRepository;
use crate::presentation::state::app_state::AppState;
use crate::presentation::state::input_state::InputMode;
use crossterm::event::{Event, KeyCode};
use tui_textarea::TextArea;
use std::sync::Arc;

pub struct AppController<R: PullRequestRepository, RepoR: RepositoryRepository, C: ConfigRepository> {
    app_state: AppState,
    create_pr_use_case: CreatePullRequestUseCase<R>,
    sync_repo_use_case: SyncRepositoryUseCase<RepoR>,
    config_repo: Arc<C>,
}

impl<R: PullRequestRepository, RepoR: RepositoryRepository, C: ConfigRepository> AppController<R, RepoR, C> {
    pub fn new(
        app_state: AppState,
        create_pr_use_case: CreatePullRequestUseCase<R>,
        sync_repo_use_case: SyncRepositoryUseCase<RepoR>,
        config_repo: Arc<C>,
    ) -> Self {
        Self {
            app_state,
            create_pr_use_case,
            sync_repo_use_case,
            config_repo,
        }
    }

    pub fn get_app_state(&self) -> &AppState {
        &self.app_state
    }

    pub fn get_app_state_mut(&mut self) -> &mut AppState {
        &mut self.app_state
    }

    pub async fn handle_event(&mut self, event: Event) -> Result<bool, DomainError> {
        if let Event::Key(key) = event {
            if self.app_state.show_pat_popup {
                return self.handle_pat_popup_event(key.code).await;
            }
            if self.app_state.show_exit_popup {
                return self.handle_exit_popup_event(key.code);
            }
            if self.app_state.show_confirm_popup {
                return self.handle_confirm_popup_event(key.code).await;
            }

            match self.app_state.input_mode {
                InputMode::Normal => self.handle_normal_mode_event(key.code).await,
                InputMode::Editing => self.handle_editing_mode_event(key.code),
                InputMode::Creating => self.handle_creating_mode_event(key.code),
            }
        } else {
            Ok(false)
        }
    }

    async fn handle_pat_popup_event(&mut self, key_code: KeyCode) -> Result<bool, DomainError> {
        match key_code {
            KeyCode::Backspace => {
                // For now, handle backspace manually since we can't easily create the right Event
                let lines = self.app_state.pat_input.lines();
                if let Some(last_line) = lines.last() {
                    if !last_line.is_empty() {
                        let mut new_lines = lines[..lines.len() - 1].to_vec();
                        let new_last_line = &last_line[..last_line.len().saturating_sub(1)];
                        new_lines.push(new_last_line.to_string());
                        self.app_state.pat_input = TextArea::new(new_lines);
                    }
                }
            }
            KeyCode::Enter => {
                if !self.app_state.pat_input.is_empty() {
                    let pat = self.app_state.pat_input.lines().join("\n");
                    self.app_state.config_pat = pat.clone();
                    self.app_state.clear_message();

                    // Save the PAT
                    self.config_repo.save_github_token(&pat).await?;

                    // Try to sync repository info
                    match self.sync_repo_use_case.execute().await {
                        Ok(repo) => {
                            self.app_state.set_repository(repo);
                            self.app_state.show_pat_popup = false;
                            self.app_state.set_success("PAT saved successfully ✅".to_string());
                        }
                        Err(e) => {
                            self.app_state.set_error(format!("Error syncing repository: {:?}", e));
                        }
                    }
                } else {
                    self.app_state.set_error("PAT cannot be empty!".to_string());
                }
            }
            KeyCode::Esc => return Ok(true), // Exit
            KeyCode::Char(c) => {
                // Handle character input manually
                let mut lines = self.app_state.pat_input.lines();
                if lines.is_empty() {
                    lines.push(c.to_string());
                } else {
                    let last_idx = lines.len() - 1;
                    lines[last_idx].push(c);
                }
                self.app_state.pat_input = TextArea::new(lines);
            }
            _ => {}
        }
        Ok(false)
    }

    fn handle_exit_popup_event(&mut self, key_code: KeyCode) -> Result<bool, DomainError> {
        match key_code {
            KeyCode::Char('y') => return Ok(true), // Exit
            KeyCode::Char('n') => self.app_state.show_exit_popup = false,
            _ => {}
        }
        Ok(false)
    }

    async fn handle_confirm_popup_event(&mut self, key_code: KeyCode) -> Result<bool, DomainError> {
        match key_code {
            KeyCode::Enter | KeyCode::Char('y') => {
                self.app_state.input_mode = InputMode::Normal;
                self.app_state.show_confirm_popup = false;
                self.app_state.pull_request.description = self.app_state.description_text_area.lines().join("\n");

                let request = self.app_state.create_pull_request_request();
                match self.create_pr_use_case.execute(request).await {
                    Ok(pr) => {
                        let url_str = pr.html_url.as_ref()
                            .map(|url| url.to_string())
                            .unwrap_or_else(|| "No URL available".to_string());
                        self.app_state.reset();
                        self.app_state.set_success(format!(
                            "Pull request created successfully ✅\n Url: {}",
                            url_str
                        ));
                    }
                    Err(e) => {
                        self.app_state.set_error(format!("Failed to create pull request: {}", e));
                    }
                }
            }
            KeyCode::Char('e') | KeyCode::Char('n') => {
                self.app_state.input_mode = InputMode::Editing;
                self.app_state.show_confirm_popup = false;
            }
            KeyCode::Esc => {
                self.app_state.input_mode = InputMode::Normal;
                self.app_state.show_confirm_popup = false;
            }
            _ => {}
        }
        Ok(false)
    }

    async fn handle_normal_mode_event(&mut self, key_code: KeyCode) -> Result<bool, DomainError> {
        match key_code {
            KeyCode::Esc => {
                self.app_state.show_exit_popup = true;
            }
            KeyCode::Char('e') => {
                self.app_state.clear_message();
                self.app_state.enter_edit_mode(self.app_state.current_field);
            }
            KeyCode::Char('n') => {
                self.app_state.reset();
                self.app_state.clear_message();
                self.app_state.enter_edit_mode(0);

                // Try to sync repository if we don't have info
                if self.app_state.repository.is_none() {
                    match self.sync_repo_use_case.execute().await {
                        Ok(repo) => {
                            self.app_state.set_repository(repo);
                        }
                        Err(e) => {
                            self.app_state.set_error(format!("Error syncing repository: {:?}", e));
                        }
                    }
                }
            }
            KeyCode::Down => {
                self.app_state.current_field = (self.app_state.current_field + 1) % 4;
            }
            KeyCode::Up => {
                self.app_state.current_field = (self.app_state.current_field + 3) % 4;
            }
            KeyCode::Char('s') => {
                match self.sync_repo_use_case.execute().await {
                    Ok(repo) => {
                        self.app_state.set_repository(repo);
                        self.app_state.set_success("Repository has been synced successfully ✅".to_string());
                    }
                    Err(e) => {
                        self.app_state.set_error(format!("Error syncing repository: {:?}", e));
                    }
                }
            }
            _ => {}
        }
        Ok(false)
    }

    fn handle_editing_mode_event(&mut self, key_code: KeyCode) -> Result<bool, DomainError> {
        match key_code {
            KeyCode::Esc => {
                self.app_state.input_mode = InputMode::Normal;
            }
            KeyCode::Char(c) => {
                if self.app_state.is_editing_description() {
                    // For description editing, we need to handle this differently
                    // For now, just push the character to the description
                    self.app_state.pull_request.description.push(c);
                } else {
                    let current_field = self.app_state.get_current_field_mut();
                    current_field.push(c);
                }
            }
            KeyCode::Backspace => {
                if self.app_state.is_editing_description() {
                    self.app_state.pull_request.description.pop();
                } else {
                    let current_field = self.app_state.get_current_field_mut();
                    current_field.pop();
                }
            }
            KeyCode::Enter => {
                if self.app_state.is_editing_description() {
                    self.app_state.pull_request.description.push('\n');
                } else {
                    self.app_state.confirm_pull_request();
                }
            }
            KeyCode::Tab => {
                self.app_state.current_field = (self.app_state.current_field + 1) % 4;
            }
            KeyCode::BackTab => {
                self.app_state.current_field = (self.app_state.current_field + 3) % 4;
            }
            _ => {}
        }
        Ok(false)
    }

    fn handle_creating_mode_event(&mut self, key_code: KeyCode) -> Result<bool, DomainError> {
        match key_code {
            KeyCode::Enter | KeyCode::Char('y') => {
                self.app_state.input_mode = InputMode::Normal;
                self.app_state.show_confirm_popup = false;
                self.app_state.pull_request.description = self.app_state.description_text_area.lines().join("\n");

                let request = self.app_state.create_pull_request_request();
                // Note: This would need to be async, but we're in a sync context
                // In a real implementation, this would be handled differently
            }
            KeyCode::Char('e') | KeyCode::Char('n') => {
                self.app_state.input_mode = InputMode::Editing;
                self.app_state.show_confirm_popup = false;
            }
            KeyCode::Esc => {
                self.app_state.input_mode = InputMode::Normal;
                self.app_state.show_confirm_popup = false;
            }
            _ => {}
        }
        Ok(false)
    }
}

// Factory function to create the controller with all dependencies
pub async fn create_app_controller() -> Result<AppController<
    GitHubPullRequestRepository,
    GitHubRepositoryRepository,
    FileConfigRepository
>, DomainError> {
    // Initialize config
    let config_repo = Arc::new(FileConfigRepository::new());

    // Try to get existing token
    let token = config_repo.get_github_token_sync();

    let (app_state, create_pr_use_case, sync_repo_use_case) = if let Some(token) = token.clone() {
        // We have a token, try to initialize GitHub client
        let github_client = GitHubClient::new(token)?;

        // Get repository info from git
        let git_repo = GitRepositoryRepository::new();
        let repo_info = git_repo.get_info().await?;

        // Create GitHub repositories
        let github_pr_repo = GitHubPullRequestRepository::new(
            github_client.clone(),
            repo_info.owner.clone(),
            repo_info.name.clone(),
        );
        let github_repo_repo = GitHubRepositoryRepository::new(
            github_client,
            repo_info.owner.clone(),
            repo_info.name.clone(),
        );

        // Create use cases
        let create_pr_use_case = CreatePullRequestUseCase::new(github_pr_repo);
        let sync_repo_use_case = SyncRepositoryUseCase::new(github_repo_repo);

        let mut app_state = AppState::new();
        app_state.config_pat = token;

        // Try to sync repository info
        match sync_repo_use_case.execute().await {
            Ok(repo) => {
                app_state.set_repository(repo);
            }
            Err(_) => {
                // If sync fails, we'll show the PAT popup later if needed
            }
        }

        (app_state, create_pr_use_case, sync_repo_use_case)
    } else {
        // No token, show PAT popup and create dummy repositories
        let mut app_state = AppState::new();
        app_state.show_pat_popup = true;

        // Create dummy repositories for when we don't have a token yet
        let dummy_github_client = GitHubClient::new("dummy".to_string())?;
        let dummy_pr_repo = GitHubPullRequestRepository::new(
            dummy_github_client.clone(),
            "dummy".to_string(),
            "dummy".to_string(),
        );
        let dummy_repo_repo = GitHubRepositoryRepository::new(
            dummy_github_client,
            "dummy".to_string(),
            "dummy".to_string(),
        );

        let create_pr_use_case = CreatePullRequestUseCase::new(dummy_pr_repo);
        let sync_repo_use_case = SyncRepositoryUseCase::new(dummy_repo_repo);

        (app_state, create_pr_use_case, sync_repo_use_case)
    };

    Ok(AppController::new(
        app_state,
        create_pr_use_case,
        sync_repo_use_case,
        config_repo,
    ))
}