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
                        app.clear_message();
                        app.enter_edit_mode(app.current_field);
                    }
                    KeyCode::Char('n') => {
                        app.reset();
                        app.clear_message();
                        app.enter_edit_mode(0);
                    }
                    KeyCode::Down => {
                        app.current_field = (app.current_field + 1) % 4;
                    }
                    KeyCode::Up => {
                        app.current_field = (app.current_field + 3) % 4;
                    }
                    KeyCode::Char('s') => {
                        let result = runtime.block_on(app.fetch_github_repo_info());
                        match result {
                            Ok(repo) => {
                                if let Some(link) = repo.html_url {
                                    app.github_repository.set_url(link.to_string());
                                }
                                if let Some(branch) = repo.default_branch {
                                    app.github_repository.set_default_branch(branch.clone());
                                    app.pull_request.target_branch = branch;
                                    app.set_success(
                                        "Repository information fetched successfully!".to_string(),
                                    );
                                }
                                app.github_repository.set_name(repo.name.clone());
                            }
                            Err(e) => {
                                app.set_error(format!("Error {:?}", e));
                            }
                        }
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char(c) => {
                        if app.is_editing_description() {
                            app.description_text_area.input(key);
                        } else {
                            let current_field = app.get_current_field_mut();
                            current_field.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        if app.is_editing_description() {
                            app.description_text_area.input(key);
                        } else {
                            let current_field = app.get_current_field_mut();
                            current_field.pop();
                        }
                    }
                    KeyCode::Enter => {
                        let current_field_index = app.current_field;
                        let current_field = app.get_current_field_mut();
                        if current_field_index == 1 {
                            current_field.push('\n');
                            app.description_text_area.input(key);
                        } else {
                            app.confirm_pull_request();
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
                    KeyCode::Enter | KeyCode::Char('y') => {
                        app.input_mode = InputMode::Normal;
                        app.show_confirm_popup = false;
                        app.pull_request.description = app.description_text_area.lines().join("\n");

                        let result = runtime.block_on(app.create_github_pull_request());
                        match result {
                            Ok(pr) => {
                                let url_str = match pr.html_url {
                                    Some(ref url) => url.to_string(),
                                    None => "No URL available".to_string(),
                                };
                                app.reset();
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
