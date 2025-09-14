mod domain;
mod application;
mod infrastructure;
mod presentation;

use crate::infrastructure::ui::{ratatui_renderer::RatatUIRenderer, event_handler::EventHandler};
use crate::presentation::controllers::app_controller::create_app_controller;
use crate::presentation::views::main_view::render_main_view;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut renderer = RatatUIRenderer::new(backend)?;
    let event_handler = EventHandler::new();

    // Create application controller with all dependencies
    let mut app_controller = create_app_controller().await?;

    // Main application loop
    loop {
        // Render the UI
        renderer.get_terminal().draw(|f| {
            render_main_view(f, app_controller.get_app_state());
        })?;

        // Handle events
        if let Some(event) = event_handler.poll_event(std::time::Duration::from_millis(100))? {
            let should_exit = app_controller.handle_event(event).await?;
            if should_exit {
                break;
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(renderer.get_terminal().backend_mut(), LeaveAlternateScreen)?;
    renderer.show_cursor()?;

    Ok(())
}
