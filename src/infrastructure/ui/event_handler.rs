use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub struct EventHandler;

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

    pub fn read_event(&self) -> Result<Event, std::io::Error> {
        event::read()
    }
}