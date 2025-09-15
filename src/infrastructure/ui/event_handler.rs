use crossterm::event::{self, Event};
use std::time::Duration;

pub struct EventHandler;

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn poll_event(&self, timeout: Duration) -> Result<Option<Event>, std::io::Error> {
        if event::poll(timeout)? {
            Ok(Some(event::read()?))
        } else {
            Ok(None)
        }
    }

    #[allow(dead_code)]
    pub fn read_event(&self) -> Result<Event, std::io::Error> {
        event::read()
    }
}