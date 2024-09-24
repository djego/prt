mod ui;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use octocrab::models::pulls::PullRequest as OctocrabPullRequest;
use octocrab::Error as OctocrabError;
use octocrab::Octocrab;

use crate::ui::layout::ui;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use thiserror::Error;

struct PullRequest {
    title: String,
    description: String,
    source_branch: String,
    target_branch: String,
}

enum InputMode {
    Normal,
    Editing,
    Creating,
}

#[derive(Debug, Error)]
enum PullRequestError {
    #[error("GitHub API error: {0}")]
    ApiError(#[from] OctocrabError),

    #[error("Validation failed")]
    PullRequestValidationFailed(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

struct App {
    error_message: Option<String>,
    success_message: Option<String>,
    pull_request: PullRequest,
    input_mode: InputMode,
    current_field: usize,
    show_popup: bool,
}

impl App {
    fn new() -> App {
        App {
            pull_request: PullRequest {
                title: String::new(),
                description: String::new(),
                source_branch: String::new(),
                target_branch: String::new(),
            },
            input_mode: InputMode::Normal,
            current_field: 0,
            show_popup: false,
            error_message: None,
            success_message: None,
        }
    }

    fn get_current_field_mut(&mut self) -> &mut String {
        match self.current_field {
            0 => &mut self.pull_request.title,
            1 => &mut self.pull_request.description,
            2 => &mut self.pull_request.source_branch,
            3 => &mut self.pull_request.target_branch,
            _ => unreachable!(),
        }
    }

    fn enter_edit_mode(&mut self, index: usize) {
        self.input_mode = InputMode::Editing;
        self.current_field = index;
    }

    fn preview_pull_request(&mut self) {
        self.input_mode = InputMode::Creating;
        self.show_popup = true;
    }

    fn reset(&mut self) {
        self.pull_request = PullRequest {
            title: String::new(),
            description: String::new(),
            source_branch: String::new(),
            target_branch: String::new(),
        };
        self.input_mode = InputMode::Normal;
        self.current_field = 0;
        self.show_popup = false;
        self.error_message = None;
        self.success_message = None;
    }

    async fn create_github_pull_request(&self) -> Result<OctocrabPullRequest, PullRequestError> {
        let github_token = std::env::var("GITHUB_TOKEN").expect("GitHub token not set");
        let octocrab = Octocrab::builder().personal_token(github_token).build()?;
        let repo_owner = "djego";
        let repo_name = "prt";

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
            .pulls(repo_owner, repo_name)
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
                if let OctocrabError::GitHub { source, .. } = &e {
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

    fn set_error(&mut self, error: String) {
        self.error_message = Some(error);
    }

    fn set_success(&mut self, success: String) {
        self.success_message = Some(success);
    }
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('e') => {
                        app.enter_edit_mode(app.current_field);
                    }
                    KeyCode::Char('n') => {
                        app.reset();
                        app.enter_edit_mode(0);
                    }
                    KeyCode::Down => {
                        app.current_field = (app.current_field + 1) % 4;
                    }
                    KeyCode::Up => {
                        app.current_field = (app.current_field + 3) % 4;
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char(c) => {
                        let current_field = app.get_current_field_mut();
                        current_field.push(c);
                    }
                    KeyCode::Backspace => {
                        let current_field = app.get_current_field_mut();
                        current_field.pop();
                    }
                    KeyCode::Enter => {
                        let current_field_index = app.current_field;
                        let current_field = app.get_current_field_mut();
                        if current_field_index == 1 {
                            current_field.push('\n');
                        } else {
                            app.preview_pull_request();
                        }
                    }
                    KeyCode::Tab => {
                        app.current_field = (app.current_field + 1) % 4;
                    }
                    KeyCode::BackTab => {
                        app.current_field = (app.current_field + 3) % 4;
                    }
                    _ => {}
                },
                InputMode::Creating => match key.code {
                    KeyCode::Enter => {
                        app.input_mode = InputMode::Normal;
                        app.show_popup = false;
                        let result = tokio::runtime::Runtime::new()
                            .unwrap()
                            .block_on(app.create_github_pull_request());

                        match result {
                            Ok(pr) => {
                                let url_str = match pr.html_url {
                                    Some(ref url) => url.to_string(),
                                    None => "No URL available".to_string(),
                                };
                                app.set_success(format!(
                                    "Pull request created successfully! \n Url: {}",
                                    url_str
                                ));
                            }
                            Err(e) => {
                                app.set_error(format!("Failed to create pull request: {}", e));
                            }
                        }
                    }
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                        app.show_popup = false;
                    }
                    KeyCode::Char('q') => {
                        app.input_mode = InputMode::Normal;
                        app.show_popup = false;
                    }
                    _ => {}
                },
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
