use std::time::Duration;

use crossterm::event::{self, Event, KeyEvent};

pub fn read_key() -> Result<KeyEvent, Box<dyn std::error::Error>> {
    loop {
        if event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
        {
            return Ok(key);
        }
    }
}
