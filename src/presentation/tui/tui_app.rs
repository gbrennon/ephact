use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, widgets::Block};

use crate::application::dtos::responses::{RunSummaryResponse, WorkflowListItemResponse};

use super::screens::{
    ConfigurationAction, ListActionsScreen, ListWorkflowsScreen, RunConfigurationValues,
    RunWorkflowScreen, home::HomeScreen, splash::SplashScreen,
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
    home_selection: usize,
    list_workflows_screen: ListWorkflowsScreen,
    list_actions_screen: ListActionsScreen,
    run_workflow_screen: RunWorkflowScreen,
    run_requested: bool,
    cancel_requested: bool,
    configuration_requested: bool,
}

impl TuiApp {
    const INITIAL_HOME_SELECTION: usize = 0;
    const SELECTION_STEP: usize = 1;
    const QUIT_KEY: char = 'q';
    const PREVIOUS_KEY: char = 'k';
    const NEXT_KEY: char = 'j';
    const DETAILS_KEY: char = 'd';

    pub fn new(workflows: Vec<WorkflowListItemResponse>) -> Self {
        Self {
            screen: TuiScreen::Splash,
            home_selection: Self::INITIAL_HOME_SELECTION,
            list_workflows_screen: ListWorkflowsScreen::new(workflows.clone()),
            list_actions_screen: ListActionsScreen::new(Vec::new()),
            run_workflow_screen: RunWorkflowScreen::new(workflows),
            run_requested: false,
            cancel_requested: false,
            configuration_requested: false,
        }
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

    pub fn run_workflow_screen(&self) -> &RunWorkflowScreen {
        &self.run_workflow_screen
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
        self.run_workflow_screen.take_configuration()
    }

    pub fn begin_run_configuration(
        &mut self,
        events: Vec<String>,
        declarations: Vec<crate::application::dtos::responses::RunInputDeclarationResponse>,
    ) {
        self.run_workflow_screen
            .begin_configuration(events, declarations);
    }

    pub fn selected_workflow_events(&self) -> Vec<String> {
        self.run_workflow_screen.selected_workflow_events()
    }

    pub fn report_configuration_error(&mut self, error: String) {
        self.run_workflow_screen.report_configuration_error(error);
    }

    pub fn take_cancel_request(&mut self) -> bool {
        let requested = self.cancel_requested;
        self.cancel_requested = false;
        requested
    }

    pub fn start_run(&mut self) {
        self.run_workflow_screen.start_run();
    }

    pub fn finish_run(&mut self) {
        self.run_workflow_screen.finish_run();
    }

    pub fn record_run_outcome(&mut self, outcome: RunSummaryResponse) {
        self.run_workflow_screen.record_outcome(outcome);
        self.run_workflow_screen.finish_run();
    }

    pub fn record_progress(&mut self, line: String) {
        self.run_workflow_screen.record_progress(line);
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char(Self::QUIT_KEY) && !self.run_workflow_screen.is_running() {
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
            TuiScreen::Home | TuiScreen::Exit => HomeScreen::render(frame, self.home_selection),
            TuiScreen::ListWorkflows => self.list_workflows_screen.render(frame),
            TuiScreen::ListActions => self.list_actions_screen.render(frame),
            TuiScreen::RunWorkflow => self.run_workflow_screen.render(frame),
        }
    }

    pub fn screen(&self) -> TuiScreen {
        self.screen
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
            KeyCode::Enter => self.open_selected_home_screen(),
            _ => {}
        }
    }

    fn open_selected_home_screen(&mut self) {
        match self.home_selection {
            HomeScreen::RUN_WORKFLOW_INDEX => self.open_run_workflow(),
            HomeScreen::LIST_WORKFLOWS_INDEX => self.screen = TuiScreen::ListWorkflows,
            HomeScreen::LIST_ACTIONS_INDEX => self.screen = TuiScreen::ListActions,
            _ => {}
        }
    }

    fn open_run_workflow(&mut self) {
        self.run_workflow_screen.reset();
        self.screen = TuiScreen::RunWorkflow;
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

    fn handle_run_workflow_key(&mut self, key: KeyEvent) {
        if self.run_workflow_screen.configuration_error().is_some()
            || self.run_workflow_screen.configuration_footer().is_some()
        {
            self.handle_configuration_key(key);
            return;
        }
        if self.run_workflow_screen.showing_details() {
            self.handle_details_key(key);
            return;
        }
        if self.run_workflow_screen.is_running() {
            self.handle_running_workflow_key(key);
            return;
        }
        self.handle_workflow_picker_key(key);
    }

    fn handle_configuration_key(&mut self, key: KeyEvent) {
        match self.run_workflow_screen.handle_configuration_key(key) {
            ConfigurationAction::Submit => self.configuration_requested = true,
            ConfigurationAction::Cancel => {
                self.run_workflow_screen.take_configuration();
            }
            ConfigurationAction::Continue => {}
        }
    }

    fn handle_details_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char(Self::PREVIOUS_KEY) => {
                self.run_workflow_screen.scroll_details_up();
            }
            KeyCode::Down | KeyCode::Char(Self::NEXT_KEY) => {
                self.run_workflow_screen.scroll_details_down();
            }
            KeyCode::Esc | KeyCode::Backspace => {
                self.run_workflow_screen.close_details();
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
        match key.code {
            KeyCode::Up | KeyCode::Char(Self::PREVIOUS_KEY) => {
                self.run_workflow_screen.select_previous()
            }
            KeyCode::Down | KeyCode::Char(Self::NEXT_KEY) => self.run_workflow_screen.select_next(),
            KeyCode::Enter => self.request_run(),
            KeyCode::Char(Self::DETAILS_KEY) => {
                self.run_workflow_screen.open_details();
            }
            KeyCode::Esc | KeyCode::Backspace => self.screen = TuiScreen::Home,
            _ => {}
        }
    }

    fn request_run(&mut self) {
        if self.run_workflow_screen.has_workflows() {
            self.run_requested = true;
        }
    }
}

impl Default for TuiApp {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}
