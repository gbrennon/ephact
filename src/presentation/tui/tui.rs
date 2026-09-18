use std::io::stdout;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{DefaultTerminal, Terminal, backend::CrosstermBackend};

use super::app::{App, Screen};
use super::event;

struct TerminalGuard;

impl TerminalGuard {
    fn enter() -> Result<Self, Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        let guard = Self;
        execute!(stdout(), EnterAlternateScreen)?;
        Ok(guard)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
    }
}

pub struct Tui;

impl Tui {
    fn init_terminal() -> Result<DefaultTerminal, Box<dyn std::error::Error>> {
        let backend = CrosstermBackend::new(stdout());
        Ok(Terminal::new(backend)?)
    }

    fn run_event_loop(
        terminal: &mut DefaultTerminal,
        app: &mut App,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while app.screen() != Screen::Exit {
            terminal.draw(|frame| app.render(frame))?;
            app.handle_key(event::EventReader::read_key()?);
        }

        Ok(())
    }

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let _guard = TerminalGuard::enter()?;
        let mut terminal = Self::init_terminal()?;
        let mut app = App::new();
        Self::run_event_loop(&mut terminal, &mut app)
    }
}
