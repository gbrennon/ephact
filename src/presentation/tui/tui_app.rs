use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;

use crate::application::dtos::responses::WorkflowListItemResponse;

use super::screens::{ListWorkflowsScreen, home::HomeScreen, splash::SplashScreen};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiScreen {
    Splash,
    Home,
    ListWorkflows,
    Exit,
}

pub struct TuiApp {
    screen: TuiScreen,
    home_selection: usize,
    list_workflows_screen: ListWorkflowsScreen,
}

impl TuiApp {
    const INITIAL_HOME_SELECTION: usize = 0;
    const SELECTION_STEP: usize = 1;
    const QUIT_KEY: char = 'q';
    const PREVIOUS_KEY: char = 'k';
    const NEXT_KEY: char = 'j';

    pub fn new(workflows: Vec<WorkflowListItemResponse>) -> Self {
        Self {
            screen: TuiScreen::Splash,
            home_selection: Self::INITIAL_HOME_SELECTION,
            list_workflows_screen: ListWorkflowsScreen::new(workflows),
        }
    }

    pub fn screen(&self) -> TuiScreen {
        self.screen
    }

    pub fn home_selection(&self) -> usize {
        self.home_selection
    }

    pub fn list_workflows_screen(&self) -> &ListWorkflowsScreen {
        &self.list_workflows_screen
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char(Self::QUIT_KEY) {
            self.screen = TuiScreen::Exit;
            return;
        }

        match self.screen {
            TuiScreen::Splash => self.screen = TuiScreen::Home,
            TuiScreen::Home => self.handle_home_key(key),
            TuiScreen::ListWorkflows => self.handle_list_workflows_key(key),
            TuiScreen::Exit => {}
        }
    }

    fn handle_home_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char(Self::PREVIOUS_KEY) => {
                self.home_selection = self.home_selection.saturating_sub(Self::SELECTION_STEP);
            }
            KeyCode::Down | KeyCode::Char(Self::NEXT_KEY) => {
                self.home_selection =
                    (self.home_selection + Self::SELECTION_STEP).min(HomeScreen::LAST_MENU_INDEX);
            }
            KeyCode::Enter if self.home_selection == HomeScreen::LIST_WORKFLOWS_INDEX => {
                self.screen = TuiScreen::ListWorkflows;
            }
            _ => {}
        }
    }

    fn handle_list_workflows_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char(Self::PREVIOUS_KEY) => {
                self.list_workflows_screen.select_previous()
            }
            KeyCode::Down | KeyCode::Char(Self::NEXT_KEY) => {
                self.list_workflows_screen.select_next()
            }
            KeyCode::Esc | KeyCode::Backspace => self.screen = TuiScreen::Home,
            _ => {}
        }
    }

    pub fn render(&self, frame: &mut Frame<'_>) {
        match self.screen {
            TuiScreen::Splash => SplashScreen::render(frame),
            TuiScreen::Home | TuiScreen::Exit => HomeScreen::render(frame, self.home_selection),
            TuiScreen::ListWorkflows => self.list_workflows_screen.render(frame),
        }
    }
}

impl Default for TuiApp {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}
