use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;

use crate::application::dtos::responses::WorkflowListItemResponse;

use super::screens::{
    ListActionsScreen, ListWorkflowsScreen, home::HomeScreen, splash::SplashScreen,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiScreen {
    Splash,
    Home,
    ListWorkflows,
    ListActions,
    Exit,
}

pub struct TuiApp {
    screen: TuiScreen,
    home_selection: usize,
    list_workflows_screen: ListWorkflowsScreen,
    list_actions_screen: ListActionsScreen,
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
            list_actions_screen: ListActionsScreen::new(Vec::new()),
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
    pub fn with_actions(mut self, actions: Vec<String>) -> Self {
        self.list_actions_screen = ListActionsScreen::new(actions);
        self
    }

    pub fn list_actions_screen(&self) -> &ListActionsScreen {
        &self.list_actions_screen
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(Self::QUIT_KEY) => {
                self.screen = TuiScreen::Exit;
            }
            _ => match self.screen {
                TuiScreen::Splash => self.screen = TuiScreen::Home,
                TuiScreen::Home => self.handle_home_key(key),
                TuiScreen::ListWorkflows => self.handle_list_workflows_key(key),
                TuiScreen::ListActions => self.handle_list_actions_key(key),
                TuiScreen::Exit => {}
            },
        }
    }

    fn handle_home_key(&mut self, key: KeyEvent) {
        match (key.code, self.home_selection) {
            (KeyCode::Up | KeyCode::Char(Self::PREVIOUS_KEY), _) => {
                self.home_selection = self.home_selection.saturating_sub(Self::SELECTION_STEP);
            }
            (KeyCode::Down | KeyCode::Char(Self::NEXT_KEY), _) => {
                self.home_selection =
                    (self.home_selection + Self::SELECTION_STEP).min(HomeScreen::LAST_MENU_INDEX);
            }
            (KeyCode::Enter, HomeScreen::LIST_WORKFLOWS_INDEX) => {
                self.screen = TuiScreen::ListWorkflows;
            }
            (KeyCode::Enter, HomeScreen::LIST_ACTIONS_INDEX) => {
                self.screen = TuiScreen::ListActions;
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
    fn handle_list_actions_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char(Self::PREVIOUS_KEY) => {
                self.list_actions_screen.select_previous()
            }
            KeyCode::Down | KeyCode::Char(Self::NEXT_KEY) => self.list_actions_screen.select_next(),
            KeyCode::Esc | KeyCode::Backspace => self.screen = TuiScreen::Home,
            _ => {}
        }
    }

    pub fn render(&self, frame: &mut Frame<'_>) {
        match self.screen {
            TuiScreen::Splash => SplashScreen::render(frame),
            TuiScreen::Home | TuiScreen::Exit => HomeScreen::render(frame, self.home_selection),
            TuiScreen::ListWorkflows => self.list_workflows_screen.render(frame),
            TuiScreen::ListActions => self.list_actions_screen.render(frame),
        }
    }
}

impl Default for TuiApp {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}
