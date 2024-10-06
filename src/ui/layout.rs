use crate::ui::util::{centered_rect, inner_area};
use crate::App;
use crate::InputMode;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph};
use ratatui::{
    style::{Color, Style},
    Frame,
};

pub fn ui(f: &mut Frame, app: &App) {
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

    let repository_block = Block::default()
        .title("Config")
        .padding(Padding::new(1, 0, 1, 0))
        .borders(Borders::ALL);
    let text = vec![
        Line::from(Span::raw(format!(
            "Name: {}",
            app.github_repository.get_name()
        ))),
        Line::from(Span::raw(format!(
            "URL: {}",
            app.github_repository.get_url()
        ))),
        Line::from(Span::raw(format!("Owner: {}", app.repo_owner))),
        Line::from(Span::raw(format!("Repo: {}", app.repo_name))),
        Line::from(Span::raw(format!(
            "Default Branch: {}",
            app.github_repository.get_default_branch()
        ))),
    ];
    let paragraph = Paragraph::new(text)
        .block(repository_block)
        .style(Style::default());

    let repo_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[0]);

    f.render_widget(paragraph, repo_area[0]);

    let description_lines = app.pull_request.description.lines().count();
    let description_height = description_lines.min(20) + 2;
    let form_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .vertical_margin(2)
        .horizontal_margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(description_height as u16),
                Constraint::Length(1),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(chunks[1]);
    let form_block = Block::default()
        .title("Create")
        .padding(Padding::proportional(1))
        .borders(Borders::ALL);
    f.render_widget(form_block, chunks[1]);
    let fields = vec![
        ("Title", &app.pull_request.title),
        ("Description", &app.pull_request.description),
        ("Description", &app.pull_request.description),
        ("Source Branch", &app.pull_request.source_branch),
        ("Target Branch", &app.pull_request.target_branch),
    ];

    for (i, (name, value)) in fields.iter().enumerate() {
        let (text, style) = match app.input_mode {
            InputMode::Normal => (
                format!("{}: {}", name, if value.is_empty() { "" } else { value }),
                if i == app.current_field {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                },
            ),
            InputMode::Editing => (
                format!("{}: {}", name, value),
                if i == app.current_field {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                },
            ),
            InputMode::Creating => (format!("{}: {}", name, value), Style::default()),
        };

        if i == 2 {
            f.render_widget(&app.description_text_area, form_layout[i]);
        } else {
            let paragraph = Paragraph::new(Span::styled(text, style));
            f.render_widget(paragraph, form_layout[i]);
        }
    }

    render_message(f, "", Color::default(), chunks[2]);

    if let Some(ref error_message) = app.error_message {
        render_message(f, error_message, Color::Red, chunks[2]);
    }
    if let Some(ref success_message) = app.success_message {
        render_message(f, success_message, Color::Green, chunks[2]);
    }

    // Instructions
    let instructions = match app.input_mode {
        InputMode::Normal => {
            "[Normal mode] \n Press [n] to create PR, [e] to edit PR, [s] to sync with GitHub or [q] to quit"
        }
        InputMode::Editing => "[Editing mode] \n Press [Esc] to back, [Tab]/[BackTab] to move to next or previous field, [Enter] to send",
        InputMode::Creating => {
            "[Confirm mode] \n Press [Enter] to confirm, Press [e] to continue editing, Press [q] to quit"
        }

    };
    let instructions_paragraph = Paragraph::new(instructions).style(Style::default());
    f.render_widget(instructions_paragraph, chunks[3]);

    if app.show_confirm_popup {
        let popup_block = Block::default()
            .title("Pull Request Confirmation")
            .borders(Borders::ALL)
            .style(Style::default());

        let area = centered_rect(35, 12, f.area());
        f.render_widget(Clear, area);
        f.render_widget(popup_block, area);

        let popup_text = vec![
            Line::from(format!(
                "Please confirm PR creation from {} to {} ",
                app.pull_request.source_branch, app.pull_request.target_branch
            )),
            Line::from(""),
            Line::from("Press [y] to confirm or [n] to cancel"),
        ];

        let popup_paragraph = Paragraph::new(popup_text)
            .block(Block::default().borders(Borders::NONE))
            .alignment(ratatui::layout::Alignment::Left);

        f.render_widget(popup_paragraph, inner_area(area));
    }

    if app.show_pat_popup {
        let area = centered_rect(50, 15, f.area());
        f.render_widget(Clear, area);

        let popup_text = vec![
            Line::from(app.config_pat.as_str())
                .style(Style::default().bg(Color::Black).fg(Color::White)),
            Line::from(""),
            Line::from("Press [enter] to confirm or [esc] to cancel"),
        ];

        let pat_input = Paragraph::new(popup_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Enter you Github PAT:"),
            )
            .style(Style::default().bg(Color::White).fg(Color::Black));

        // Centrar el modal en la pantalla
        f.render_widget(pat_input, inner_area(area));
    }
}

fn render_message(f: &mut Frame, message: &str, color: Color, area: Rect) {
    let paragraph = Paragraph::new(Span::from(Span::styled(
        message,
        Style::default().fg(color),
    )))
    .block(Block::default().borders(Borders::ALL).title("Ouput"));

    f.render_widget(paragraph, area);
}
