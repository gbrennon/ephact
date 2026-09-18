use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;

use super::screen::{home::HomeScreen, splash::SplashScreen};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Splash,
    Home,
    Exit,
}

pub struct App {
    screen: Screen,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Splash,
        }
    }

    pub fn screen(&self) -> Screen {
        self.screen
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('q') {
            self.screen = Screen::Exit;
        } else if self.screen == Screen::Splash {
            self.screen = Screen::Home;
        }
    }

    pub fn render(&self, frame: &mut Frame<'_>) {
        match self.screen {
            Screen::Splash => SplashScreen::render(frame),
            Screen::Home | Screen::Exit => HomeScreen::render(frame),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
