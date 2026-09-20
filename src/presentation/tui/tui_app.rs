use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, widgets::Block};

use crate::application::dtos::responses::{RunSummaryResponse, WorkflowListItemResponse};

use super::screens::{
    ConfigurationAction, RunConfigurationValues, ScreenManager, home::HomeScreen,
    splash::SplashScreen,
};
use super::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiScreen {
    Splash,
    Home,
    ListWorkflows,
    ListActions,
    RunWorkflow,
    Exit,
}

pub struct TuiApp {
    screen: TuiScreen,
    screens: ScreenManager,
    run_requested: bool,
    cancel_requested: bool,
    configuration_requested: bool,
}

impl TuiApp {
    const QUIT_KEY: char = 'q';
    const PREVIOUS_KEY: char = 'k';
    const NEXT_KEY: char = 'j';
    const DETAILS_KEY: char = 'd';

    pub fn new(workflows: Vec<WorkflowListItemResponse>) -> Self {
        Self {
            screen: TuiScreen::Splash,
            screens: ScreenManager::new(workflows),
            run_requested: false,
            cancel_requested: false,
            configuration_requested: false,
        }
    }

    pub fn home_selection(&self) -> usize {
        self.screens.home_selection()
    }

    pub fn list_workflows_screen(&self) -> &super::screens::ListWorkflowsScreen {
        self.screens.list_workflows()
    }

    pub fn with_actions(mut self, actions: Vec<String>) -> Self {
        self.screens = self.screens.with_actions(actions);
        self
    }

    pub fn list_actions_screen(&self) -> &super::screens::ListActionsScreen {
        self.screens.list_actions()
    }

    pub fn run_workflow_screen(&self) -> &super::screens::RunWorkflowScreen {
        self.screens.run_workflow_screen()
    }

    pub fn take_run_request(&mut self) -> bool {
        let requested = self.run_requested;
        self.run_requested = false;
        requested
    }
    pub fn take_configured_run_request(&mut self) -> Option<RunConfigurationValues> {
        if !self.configuration_requested {
            return None;
        }
        self.configuration_requested = false;
        self.screens.take_configuration()
    }

    pub fn begin_run_configuration(
        &mut self,
        events: Vec<String>,
        declarations: Vec<crate::application::dtos::responses::RunInputDeclarationResponse>,
    ) {
        self.screens.begin_run_configuration(events, declarations);
    }

    pub fn selected_workflow_events(&self) -> Vec<String> {
        self.screens.selected_workflow_events()
    }

    pub fn report_configuration_error(&mut self, error: String) {
        self.screens
            .run_workflow_screen_mut()
            .report_configuration_error(error);
    }

    pub fn take_cancel_request(&mut self) -> bool {
        let requested = self.cancel_requested;
        self.cancel_requested = false;
        requested
    }

    pub fn start_run(&mut self) {
        self.screens.start_run();
    }

    pub fn finish_run(&mut self) {
        self.screens.finish_run();
    }

    pub fn record_run_outcome(&mut self, outcome: RunSummaryResponse) {
        self.screens.record_run_outcome(outcome);
    }

    pub fn record_progress(&mut self, line: String) {
        self.screens.record_progress(line);
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char(Self::QUIT_KEY)
            && !self.screens.run_workflow_screen().is_running()
        {
            self.screen = TuiScreen::Exit;
            return;
        }
        match self.screen {
            TuiScreen::Splash => self.screen = TuiScreen::Home,
            TuiScreen::Home => self.handle_home_key(key),
            TuiScreen::ListWorkflows => self.handle_list_workflows_key(key),
            TuiScreen::ListActions => self.handle_list_actions_key(key),
            TuiScreen::RunWorkflow => self.handle_run_workflow_key(key),
            TuiScreen::Exit => {}
        }
    }

    pub fn render(&self, frame: &mut Frame<'_>) {
        frame.render_widget(Block::default().style(Theme::window_style()), frame.area());
        match self.screen {
            TuiScreen::Splash => SplashScreen::render(frame),
            TuiScreen::Home | TuiScreen::Exit => self.screens.render_home(frame),
            TuiScreen::ListWorkflows => self.screens.render_list_workflows(frame),
            TuiScreen::ListActions => self.screens.render_list_actions(frame),
            TuiScreen::RunWorkflow => self.screens.render_run_workflow(frame),
        }
    }

    pub fn screen(&self) -> TuiScreen {
        self.screen
    }

    fn handle_home_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char(Self::PREVIOUS_KEY) => {
                self.screens.select_home_previous();
            }
            KeyCode::Down | KeyCode::Char(Self::NEXT_KEY) => {
                self.screens.select_home_next();
            }
            KeyCode::Enter => self.open_selected_home_screen(),
            _ => {}
        }
    }

    fn open_selected_home_screen(&mut self) {
        match self.home_selection() {
            HomeScreen::RUN_WORKFLOW_INDEX => self.open_run_workflow(),
            HomeScreen::LIST_WORKFLOWS_INDEX => self.screen = TuiScreen::ListWorkflows,
            HomeScreen::LIST_ACTIONS_INDEX => self.screen = TuiScreen::ListActions,
            _ => {}
        }
    }

    fn open_run_workflow(&mut self) {
        self.screens.run_workflow_screen_mut().reset();
        self.screen = TuiScreen::RunWorkflow;
    }

    fn handle_list_workflows_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char(Self::PREVIOUS_KEY) => {
                self.screens.list_workflows_mut().select_previous()
            }
            KeyCode::Down | KeyCode::Char(Self::NEXT_KEY) => {
                self.screens.list_workflows_mut().select_next()
            }
            KeyCode::Esc | KeyCode::Backspace => self.screen = TuiScreen::Home,
            _ => {}
        }
    }

    fn handle_list_actions_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char(Self::PREVIOUS_KEY) => {
                self.screens.list_actions_mut().select_previous()
            }
            KeyCode::Down | KeyCode::Char(Self::NEXT_KEY) => {
                self.screens.list_actions_mut().select_next()
            }
            KeyCode::Esc | KeyCode::Backspace => self.screen = TuiScreen::Home,
            _ => {}
        }
    }

    fn handle_run_workflow_key(&mut self, key: KeyEvent) {
        if self
            .screens
            .run_workflow_screen()
            .configuration_error()
            .is_some()
            || self
                .screens
                .run_workflow_screen()
                .configuration_footer()
                .is_some()
        {
            self.handle_configuration_key(key);
            return;
        }
        if self.screens.run_workflow_screen().showing_details() {
            self.handle_details_key(key);
            return;
        }
        if self.screens.run_workflow_screen().is_running() {
            self.handle_running_workflow_key(key);
            return;
        }
        self.handle_workflow_picker_key(key);
    }

    fn handle_configuration_key(&mut self, key: KeyEvent) {
        match self.screens.handle_configuration_key(key) {
            ConfigurationAction::Submit => self.configuration_requested = true,
            ConfigurationAction::Cancel => {
                self.screens.take_configuration();
            }
            ConfigurationAction::Continue => {}
        }
    }

    fn handle_details_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char(Self::PREVIOUS_KEY) => {
                self.screens.run_workflow_screen_mut().scroll_details_up();
            }
            KeyCode::Down | KeyCode::Char(Self::NEXT_KEY) => {
                self.screens.run_workflow_screen_mut().scroll_details_down();
            }
            KeyCode::Esc | KeyCode::Backspace => {
                self.screens.run_workflow_screen_mut().close_details();
            }
            _ => {}
        }
    }

    fn handle_running_workflow_key(&mut self, key: KeyEvent) {
        if matches!(key.code, KeyCode::Esc | KeyCode::Backspace) {
            self.cancel_requested = true;
        }
    }

    fn handle_workflow_picker_key(&mut self, key: KeyEvent) {
        if self.screens.run_workflow_screen().outcome().is_some() && self.handle_summary_key(key) {
            return;
        }
        match key.code {
            KeyCode::Up | KeyCode::Char(Self::PREVIOUS_KEY) => {
                self.screens.run_workflow_screen_mut().select_previous()
            }
            KeyCode::Down | KeyCode::Char(Self::NEXT_KEY) => {
                self.screens.run_workflow_screen_mut().select_next()
            }
            KeyCode::Enter => self.request_run(),
            KeyCode::Char(Self::DETAILS_KEY) => {
                self.screens.run_workflow_screen_mut().open_details();
            }
            KeyCode::Esc | KeyCode::Backspace => self.screen = TuiScreen::Home,
            _ => {}
        }
    }

    fn handle_summary_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Up | KeyCode::Char(Self::PREVIOUS_KEY) => {
                self.screens.run_workflow_screen_mut().scroll_summary_up();
                true
            }
            KeyCode::Down | KeyCode::Char(Self::NEXT_KEY) => {
                self.screens.run_workflow_screen_mut().scroll_summary_down();
                true
            }
            _ => false,
        }
    }

    fn request_run(&mut self) {
        if self.screens.run_workflow_screen().has_workflows() {
            self.run_requested = true;
        }
    }
}

impl Default for TuiApp {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}
