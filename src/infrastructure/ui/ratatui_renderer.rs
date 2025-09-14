use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::Stdout;

pub type TerminalBackend = CrosstermBackend<Stdout>;

pub struct RatatUIRenderer {
    terminal: Terminal<TerminalBackend>,
}

impl RatatUIRenderer {
    pub fn new(backend: TerminalBackend) -> Result<Self, std::io::Error> {
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    pub fn get_terminal(&mut self) -> &mut Terminal<TerminalBackend> {
        &mut self.terminal
    }

    pub fn clear(&mut self) -> Result<(), std::io::Error> {
        self.terminal.clear()
    }

    pub fn show_cursor(&mut self) -> Result<(), std::io::Error> {
        self.terminal.show_cursor()
    }
}