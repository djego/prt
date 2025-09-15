use crate::presentation::state::app_state::AppState;
use crate::presentation::state::input_state::InputMode;
use crate::presentation::views::components::centered_rect::centered_rect;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph};
use ratatui::{
    style::{Color, Style},
    Frame,
};

pub fn render_main_view(f: &mut Frame, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
                Constraint::Percentage(5),
            ]
            .as_ref(),
        )
        .split(f.area());

    let block = Block::default()
        .title("PRT: Pull Request TUI")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded);
    f.render_widget(block, f.area());

    // Repository info section
    render_repository_info(f, app_state, chunks[0]);

    // Form section
    render_form(f, app_state, chunks[1]);

    // Messages section
    render_messages(f, app_state, chunks[2]);

    // Instructions section
    render_instructions(f, app_state, chunks[3]);

    // Popups
    render_popups(f, app_state);
}

fn render_repository_info(f: &mut Frame, app_state: &AppState, area: Rect) {
    let repository_block = Block::default()
        .title("Github Config")
        .padding(Padding::new(1, 0, 1, 0))
        .borders(Borders::ALL);

    let text = if let Some(repo) = &app_state.repository {
        vec![
            Line::from(Span::raw(format!("Owner: {}", repo.owner))),
            Line::from(Span::raw(format!("Repo: {}", repo.name))),
            Line::from(Span::raw(format!("URL: {}", repo.url))),
            Line::from(Span::raw(format!("Default Branch: {}", repo.default_branch))),
        ]
    } else {
        vec![
            Line::from(Span::raw("Owner: -")),
            Line::from(Span::raw("Repo: -")),
            Line::from(Span::raw("URL: -")),
            Line::from(Span::raw("Default Branch: -")),
        ]
    };

    let paragraph = Paragraph::new(text)
        .block(repository_block)
        .style(Style::default());

    let repo_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(area);

    f.render_widget(paragraph, repo_area[0]);
}

fn render_form(f: &mut Frame, app_state: &AppState, area: Rect) {
    let description_lines = app_state.pull_request.description.lines().count();
    let description_height = description_lines.min(20) + 3;
    let form_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .vertical_margin(2)
        .horizontal_margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(description_height as u16),
                Constraint::Length(1),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(area);

    let form_block = Block::default()
        .title("Create")
        .padding(Padding::proportional(1))
        .borders(Borders::ALL);
    f.render_widget(form_block, area);

    let fields = [
        ("Title", &app_state.pull_request.title),
        ("Description", &app_state.pull_request.description),
        ("Source Branch", &app_state.pull_request.source_branch),
        ("Target Branch", &app_state.pull_request.target_branch),
    ];

    for (i, (name, value)) in fields.iter().enumerate() {
        let (text, style) = match app_state.input_mode {
            InputMode::Normal => (
                format!("{}: {}", name, if value.is_empty() { "" } else { value }),
                if i == app_state.current_field {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                },
            ),
            InputMode::Editing => (
                format!("{}: {}", name, value),
                if i == app_state.current_field {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                },
            ),
            InputMode::Creating => (format!("{}: {}", name, value), Style::default()),
        };

        let mut description_text = app_state.description_text_area.clone();
        description_text.set_cursor_style(Style::default().fg(Color::Red));

        if app_state.input_mode == InputMode::Normal && i == app_state.current_field {
            description_text.set_block(
                Block::default()
                    .title("Description")
                    .style(Style::default().fg(Color::Yellow)),
            );
        } else if app_state.input_mode == InputMode::Editing && i == app_state.current_field {
            description_text.set_block(
                Block::default()
                    .title("Description")
                    .style(Style::default().fg(Color::Green)),
            );
            description_text.set_cursor_style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(ratatui::style::Modifier::REVERSED),
            );
        } else {
            description_text.set_block(
                Block::default()
                    .title("Description")
                    .style(Style::default()),
            );
        }

        if i == 1 {
            f.render_widget(&description_text, form_layout[i]);
        } else {
            let paragraph = Paragraph::new(Span::styled(text, style));
            f.render_widget(paragraph, form_layout[i]);
        }
    }
}

fn render_messages(f: &mut Frame, app_state: &AppState, area: Rect) {
    let message = if let Some(ref error_message) = app_state.error_message {
        (error_message.as_str(), Color::Red)
    } else if let Some(ref success_message) = app_state.success_message {
        (success_message.as_str(), Color::Green)
    } else {
        ("", Color::default())
    };

    let paragraph = Paragraph::new(Span::styled(
        message.0,
        Style::default().fg(message.1),
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Output")
            .padding(Padding::new(1, 0, 1, 0)),
    );

    f.render_widget(paragraph, area);
}

fn render_instructions(f: &mut Frame, app_state: &AppState, area: Rect) {
    let instructions = match app_state.input_mode {
        InputMode::Normal => {
            if !app_state.pull_request.description.is_empty() || !app_state.pull_request.title.is_empty() {
                "[Normal mode] \n Press [s] to sync with GitHub, [n] to create PR, [e] to edit PR or [Esc] to quit"
            } else {
                "[Normal mode] \n Press [s] to sync with GitHub, [n] to create PR or [Esc] to quit"
            }
        }
        InputMode::Editing => "[Editing mode] \n Press [Tab]/[BackTab] to move to next or previous field, [Enter] to send or [Esc] to back",
        InputMode::Creating => {
            "[Confirm mode] \n Press [Enter] to confirm, Press [e] to continue editing, Press [Esc] to cancel"
        }
    };

    let instructions_paragraph = Paragraph::new(instructions).style(Style::default());
    f.render_widget(instructions_paragraph, area);
}

fn render_popups(f: &mut Frame, app_state: &AppState) {
    if app_state.show_confirm_popup {
        render_confirm_popup(f, app_state);
    }

    if app_state.show_pat_popup {
        render_pat_popup(f, app_state);
    }

    if app_state.show_exit_popup {
        render_exit_popup(f);
    }
}

fn render_confirm_popup(f: &mut Frame, app_state: &AppState) {
    let popup_block = Block::default()
        .title("Pull Request Confirmation")
        .borders(Borders::ALL)
        .style(Style::default());

    let area_confirm_popup = centered_rect(60, 12, f.area());
    f.render_widget(Clear, area_confirm_popup);
    f.render_widget(popup_block, area_confirm_popup);

    let popup_text = vec![
        Line::from(format!(
            "Please confirm PR creation from {} to {} ",
            app_state.pull_request.source_branch, app_state.pull_request.target_branch
        )),
        Line::from(""),
        Line::from("Press [y] to confirm or [n] to cancel"),
    ];

    let popup_paragraph = Paragraph::new(popup_text)
        .block(Block::default().borders(Borders::NONE))
        .alignment(ratatui::layout::Alignment::Center);

    let inner = centered_rect(58, 10, area_confirm_popup);
    f.render_widget(popup_paragraph, inner);
}

fn render_pat_popup(f: &mut Frame, app_state: &AppState) {
    let area = centered_rect(50, 15, f.area());
    f.render_widget(Clear, area);
    let mut pat_input_text = app_state.pat_input.clone();
    pat_input_text.set_block(
        Block::default()
            .title("Insert Github PAT")
            .padding(Padding::new(1, 1, 0, 0))
            .style(Style::default())
            .borders(Borders::ALL),
    );
    pat_input_text.set_cursor_style(
        Style::default()
            .fg(Color::Green)
            .add_modifier(ratatui::style::Modifier::REVERSED),
    );
    pat_input_text.set_placeholder_text("Enter your Github PAT here");
    pat_input_text.set_mask_char('*');

    let inner = centered_rect(48, 13, area);
    f.render_widget(&pat_input_text, inner);
}

fn render_exit_popup(f: &mut Frame) {
    let popup_block = Block::default()
        .title("Exit Confirmation")
        .borders(Borders::ALL)
        .style(Style::default());

    let exit_area = centered_rect(40, 12, f.area());
    f.render_widget(Clear, exit_area);
    f.render_widget(popup_block, exit_area);

    let popup_text = vec![
        Line::from("Are you sure you want to exit?".to_string()),
        Line::from(""),
        Line::from("Press [y] to confirm or [n] to cancel"),
    ];

    let popup_paragraph = Paragraph::new(popup_text)
        .block(Block::default().borders(Borders::NONE))
        .alignment(ratatui::layout::Alignment::Center);

    let inner = centered_rect(38, 10, exit_area);
    f.render_widget(popup_paragraph, inner);
}