use std::time::Duration;

use crossterm::event::{self, Event, KeyEvent};

pub struct EventReader;

impl EventReader {
    pub fn read_key() -> Result<Option<KeyEvent>, Box<dyn std::error::Error>> {
        if !event::poll(Duration::from_millis(100))? {
            return Ok(None);
        }
        let Event::Key(key) = event::read()? else {
            return Ok(None);
        };
        Ok(Some(key))
    }
}
