use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use octocrab::models::pulls::PullRequest as OctocrabPullRequest;
use octocrab::Error as OctocrabError;
use octocrab::Octocrab;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame, Terminal,
};
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

    fn enter_edit_mode(&mut self) {
        self.input_mode = InputMode::Editing;
        self.current_field = 0;
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
    }

    async fn create_github_pull_request(&self) -> Result<OctocrabPullRequest, PullRequestError> {
        let github_token = std::env::var("GITHUB_TOKEN").expect("GitHub token not set");
        let octocrab = Octocrab::builder().personal_token(github_token).build()?;
        let repo_owner = "djego";
        let repo_name = "exchange-rate-next";

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

    fn clear_result(&mut self) {
        self.error_message = None;
        self.success_message = None;
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
                        app.enter_edit_mode();
                    }
                    KeyCode::Char('c') => {
                        app.preview_pull_request();
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
                        } else if current_field_index == 3 {
                            app.preview_pull_request();
                        } else {
                            app.current_field = (app.current_field + 1) % 4;
                        }
                    }
                    KeyCode::Tab => {
                        app.current_field = (app.current_field + 1) % 4;
                    }
                    _ => {}
                },
                InputMode::Creating => match key.code {
                    KeyCode::Enter => {
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
                                app.reset();
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
                    KeyCode::Char('n') => {
                        app.reset();
                        app.clear_result();
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

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(5),
                Constraint::Percentage(40),
                Constraint::Percentage(50),
                Constraint::Percentage(5),
            ]
            .as_ref(),
        )
        .split(f.area());

    let block = Block::default()
        .title("PRT: Pull Request TUI")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(block, f.area());

    let repo_block = Block::default()
        .title("Context")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let repo_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(100), // Ajusta según el tamaño que prefieras
            ]
            .as_ref(),
        )
        .split(chunks[0]); // Aquí puedes usar `chunks[0]` para colocar el bloque en el área adecuada.

    f.render_widget(repo_block, repo_area[0]);

    let description_lines = app.pull_request.description.lines().count();
    let description_height = description_lines.min(15) + 2;
    let form_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(description_height as u16),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(chunks[1]);
    let form_block = Block::default().title("Create").borders(Borders::ALL);
    f.render_widget(form_block, chunks[1]);
    let fields = vec![
        ("Title", &app.pull_request.title),
        ("Description", &app.pull_request.description),
        ("Source Branch", &app.pull_request.source_branch),
        ("Target Branch", &app.pull_request.target_branch),
    ];

    for (i, (name, value)) in fields.iter().enumerate() {
        let (text, style) = match app.input_mode {
            InputMode::Normal => (
                format!("{}: {}", name, if value.is_empty() { "" } else { value }),
                Style::default().fg(if i == app.current_field {
                    Color::Yellow
                } else {
                    Color::White
                }),
            ),
            InputMode::Editing => (
                format!("{}: {}", name, value),
                if i == app.current_field {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::White)
                },
            ),
            InputMode::Creating => (
                format!("{}: {}", name, value),
                Style::default().fg(Color::White),
            ),
        };

        if i == 1 {
            let paragraph = Paragraph::new(Text::from(text).style(style))
                .block(Block::default())
                .wrap(Wrap { trim: true });
            f.render_widget(paragraph, form_layout[i]);
        } else {
            let paragraph = Paragraph::new(Span::styled(text, style));
            f.render_widget(paragraph, form_layout[i]);
        }
    }

    if let Some(ref error_message) = app.error_message {
        let error_paragraph = Paragraph::new(Span::from(Span::styled(
            error_message,
            Style::default().fg(Color::Red),
        )))
        .block(Block::default().borders(Borders::ALL).title("Error"));
        f.render_widget(error_paragraph, chunks[2]);
    }

    if let Some(ref success_message) = app.success_message {
        let success_paragraph = Paragraph::new(Text::from(Text::styled(
            success_message,
            Style::default().fg(Color::Green),
        )))
        .block(Block::default().borders(Borders::ALL).title("Success"));
        f.render_widget(success_paragraph, chunks[2]);
    }
    // Instructions
    let instructions = match app.input_mode {
        InputMode::Normal => "Press 'e' to edit Title, 'c' to create PR, 'q' to quit",
        InputMode::Editing => "Editing mode. Press 'Esc' to exit, 'Tab' to move to next field",
        InputMode::Creating => {
            "Press 'n' to create a new PR, 'e' to edit the PR, 'Enter' to submit"
        }
    };
    let instructions_paragraph =
        Paragraph::new(instructions).style(Style::default().fg(Color::Gray));
    f.render_widget(instructions_paragraph, chunks[3]);

    if app.show_popup {
        let popup_block = Block::default()
            .title("Pull Request Created")
            .borders(Borders::ALL);

        let area = centered_rect(60, 20, f.area());
        f.render_widget(Clear, area);
        f.render_widget(popup_block, area);

        let popup_text = vec![
            Line::from(Span::styled(
                "Pull Request Details:",
                Style::default().fg(Color::Green),
            )),
            Line::from(""),
            Line::from(format!("Title: {}", app.pull_request.title)),
            Line::from(format!("Description: {}", app.pull_request.description)),
            Line::from(format!("Source Branch: {}", app.pull_request.source_branch)),
            Line::from(format!("Target Branch: {}", app.pull_request.target_branch)),
            Line::from(""),
            Line::from(Span::styled(
                "Press 'n' to create a new PR",
                Style::default().fg(Color::Yellow),
            )),
        ];

        let popup_paragraph = Paragraph::new(popup_text)
            .block(Block::default().borders(Borders::NONE))
            .alignment(ratatui::layout::Alignment::Left);

        f.render_widget(popup_paragraph, inner_area(area));
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn inner_area(area: Rect) -> Rect {
    let inner = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(0)].as_ref())
        .split(area);
    inner[0]
}
