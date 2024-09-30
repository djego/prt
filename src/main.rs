mod core;
mod ui;
use crate::core::config::{load_config, save_config};
use crate::core::input_mode::InputMode;
use crate::ui::layout::ui;
use core::app::App;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dotenv::dotenv;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> Result<(), io::Error> {
    dotenv().ok();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let config = load_config();

    let mut app = App::new();
    let runtime = tokio::runtime::Runtime::new().unwrap();

    if config.is_none() {
        app.show_pat_popup = true;
    }

    loop {
        terminal.draw(|f| ui(f, &app))?;
        if let Event::Key(key) = event::read()? {
            if app.show_pat_popup {
                match key.code {
                    KeyCode::Char(c) => {
                        app.config_pat.push(c);
                    }
                    KeyCode::Backspace => {
                        app.config_pat.pop();
                    }
                    KeyCode::Enter => {
                        if !app.config_pat.is_empty() {
                            save_config(&app.config_pat).expect("Failed to save the configuration");
                            app.show_pat_popup = false;
                        } else {
                            app.set_error("PAT cannot be empty!".to_string());
                        }
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
                continue;
            }

            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('e') => {
                        app.enter_edit_mode(app.current_field);
                    }
                    KeyCode::Char('n') => {
                        app.reset();
                        app.clear_success();
                        app.enter_edit_mode(0);
                    }
                    KeyCode::Down => {
                        app.current_field = (app.current_field + 1) % 3;
                    }
                    KeyCode::Up => {
                        app.current_field = (app.current_field + 2) % 3;
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
                            app.confirm_pull_request();
                        }
                    }
                    KeyCode::Tab => {
                        app.current_field = (app.current_field + 1) % 3;
                    }
                    KeyCode::BackTab => {
                        app.current_field = (app.current_field + 2) % 3;
                    }
                    _ => {}
                },
                InputMode::Creating => match key.code {
                    KeyCode::Enter | KeyCode::Char('y') => {
                        app.input_mode = InputMode::Normal;
                        app.show_confirm_popup = false;
                        let result = runtime
                            .block_on(app.create_github_pull_request(app.config_pat.clone()));
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
                    KeyCode::Char('e') | KeyCode::Char('n') => {
                        app.input_mode = InputMode::Editing;
                        app.show_confirm_popup = false;
                    }
                    KeyCode::Char('q') => {
                        app.input_mode = InputMode::Normal;
                        app.show_confirm_popup = false;
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
