use crate::ui::util::{centered_rect, inner_area}; // Importa las funciones utilitarias
use crate::App; // Importa la estructura App
use crate::InputMode;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
use ratatui::{
    style::{Color, Style},
    Frame,
};

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(15),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
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
        .title("Current repository")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded);

    let repo_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[0]);

    f.render_widget(repository_block, repo_area[0]);

    let description_lines = app.pull_request.description.lines().count();
    let description_height = description_lines.min(20) + 2;
    let form_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(description_height as u16),
                Constraint::Length(1),
                Constraint::Length(1),
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
        InputMode::Normal => {
            "Press [n] to create PR, [e] to edit PR, [c] to modify context or [q] to quit"
        }
        InputMode::Editing => "[Editing mode] \n Press [Esc] to exit, [Tab]/[BackTab] to move to next or previous field, [Enter] to send",
        InputMode::Creating => {
            "[Creating mode] \n Press [Enter] to confirm, Press [e] to continue editing, Press [q] to quit"
        }
    };
    let instructions_paragraph =
        Paragraph::new(instructions).style(Style::default().fg(Color::Gray));
    f.render_widget(instructions_paragraph, chunks[3]);

    if app.show_popup {
        let popup_block = Block::default()
            .title("Pull Request Confirmation")
            .borders(Borders::ALL);

        let area = centered_rect(90, 15, f.area());
        f.render_widget(Clear, area);
        f.render_widget(popup_block, area);

        let popup_text = vec![
            Line::from(Span::styled(
                "Please confirm the creation of the following pull request:",
                Style::default().fg(Color::Green),
            )),
            Line::from(""),
            Line::from(format!("Title: {}", app.pull_request.title)),
            Line::from(format!(
                "Branches: {} -> {}",
                app.pull_request.source_branch, app.pull_request.target_branch
            )),
            Line::from(""),
        ];

        let popup_paragraph = Paragraph::new(popup_text)
            .block(Block::default().borders(Borders::NONE))
            .alignment(ratatui::layout::Alignment::Left);

        f.render_widget(popup_paragraph, inner_area(area));
    }
}
