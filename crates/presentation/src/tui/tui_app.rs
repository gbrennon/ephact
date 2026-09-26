use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;

use super::{
    components::{ConfigurationAction, RunConfigurationValues, SplashQuotes},
    screen_manager::{ScreenManager, TuiScreen},
    screens::{HomeScreen, SettingsAction},
};
use crate::{
    application::{
        dtos::responses::{
            RunInputDeclarationResponse, RunSummaryResponse, WorkflowListItemResponse,
        },
        ports::outbound::SettingsStorePort,
    },
    domain::Settings,
};

pub struct TuiApp {
    screens: ScreenManager,
    run_requested: bool,
    cancel_requested: bool,
    configuration_requested: bool,
    splash_quote: &'static str,
}

impl TuiApp {
    const QUIT_KEY: char = 'q';
    const PREVIOUS_KEY: char = 'k';
    const NEXT_KEY: char = 'j';
    const DETAILS_KEY: char = 'd';

    pub fn new(workflows: Vec<WorkflowListItemResponse>) -> Self {
        Self {
            screens: ScreenManager::new(workflows),
            run_requested: false,
            cancel_requested: false,
            configuration_requested: false,
            splash_quote: SplashQuotes::random(),
        }
    }

    pub fn with_actions(mut self, actions: Vec<String>) -> Self {
        self.screens = self.screens.with_actions(actions);
        self
    }

    pub fn with_settings(
        mut self,
        settings: Settings,
        store: Option<Arc<dyn SettingsStorePort>>,
    ) -> Self {
        self.screens = self.screens.with_settings(settings, store);
        self
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
        let (screens, configuration) = self.screens.take_configuration();
        self.screens = screens;
        configuration
    }

    pub fn begin_run_configuration(
        &mut self,
        events: Vec<String>,
        declarations: Vec<RunInputDeclarationResponse>,
    ) {
        self.screens = self.screens.begin_run_configuration(events, declarations);
    }

    pub fn selected_workflow_events(&self) -> Vec<String> {
        self.screens.selected_workflow_events()
    }
    pub fn is_showing_details(&self) -> bool {
        self.screens.is_showing_details()
    }

    pub fn has_run_outcome(&self) -> bool {
        self.screens.has_run_outcome()
    }

    pub fn selected_workflow_name(&self) -> Option<&str> {
        self.screens.selected_workflow_name()
    }

    pub fn home_selection(&self) -> usize {
        self.screens.home_selection()
    }

    pub fn summary_scroll(&self) -> u16 {
        self.screens.summary_scroll()
    }

    pub fn details_scroll(&self) -> u16 {
        self.screens.details_scroll()
    }
    pub fn selected_workflow_index(&self) -> usize {
        self.screens.selected_workflow_index()
    }

    pub fn selected_action_index(&self) -> usize {
        self.screens.selected_action_index()
    }
    pub fn run_outcome(&self) -> Option<&RunSummaryResponse> {
        self.screens.run_outcome()
    }
    pub fn report_configuration_error(&mut self, error: String) {
        self.screens = self.screens.report_configuration_error(error);
    }

    pub fn take_cancel_request(&mut self) -> bool {
        let requested = self.cancel_requested;
        self.cancel_requested = false;
        requested
    }

    pub fn start_run(&mut self) {
        self.screens = self.screens.start_run();
    }

    pub fn finish_run(&mut self) {
        self.screens = self.screens.finish_run();
    }

    pub fn record_run_outcome(&mut self, outcome: RunSummaryResponse) {
        self.screens = self.screens.record_run_outcome(outcome);
    }

    pub fn record_progress(&mut self, line: String) {
        self.screens = self.screens.record_progress(line);
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char(Self::QUIT_KEY)
            && !self.screens.is_workflow_running()
            && !self.screens.is_configuration_editing()
            && !self.screens.is_settings_editing()
        {
            self.screens = self.screens.transition_to(TuiScreen::Exit);
            return;
        }
        match self.screens.current_screen() {
            TuiScreen::Splash => self.transition_to(TuiScreen::Home),
            TuiScreen::Home => self.handle_home_key(key),
            TuiScreen::ListWorkflows => self.handle_list_workflows_key(key),
            TuiScreen::ListActions => self.handle_list_actions_key(key),
            TuiScreen::RunWorkflow => self.handle_run_workflow_key(key),
            TuiScreen::Settings => self.handle_settings_key(key),
            TuiScreen::Exit => {}
        }
    }

    pub fn render(&self, frame: &mut Frame<'_>) {
        self.screens.render(frame, self.splash_quote);
    }

    pub fn screen(&self) -> TuiScreen {
        self.screens.current_screen()
    }

    fn transition_to(&mut self, screen: TuiScreen) {
        self.screens = self.screens.transition_to(screen);
    }

    fn handle_home_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char(Self::PREVIOUS_KEY) => {
                self.screens = self.screens.select_home_previous();
            }
            KeyCode::Down | KeyCode::Char(Self::NEXT_KEY) => {
                self.screens = self.screens.select_home_next();
            }
            KeyCode::Enter => self.open_selected_home_screen(),
            _ => {}
        }
    }

    fn open_selected_home_screen(&mut self) {
        let screen = match self.screens.home_selection() {
            HomeScreen::RUN_WORKFLOW_INDEX => TuiScreen::RunWorkflow,
            HomeScreen::LIST_WORKFLOWS_INDEX => TuiScreen::ListWorkflows,
            HomeScreen::LIST_ACTIONS_INDEX => TuiScreen::ListActions,
            HomeScreen::SETTINGS_INDEX => TuiScreen::Settings,
            _ => return,
        };
        self.screens = self.screens.transition_to(screen);
        if screen == TuiScreen::RunWorkflow {
            self.screens = self.screens.reset_run_workflow();
        }
    }

    fn handle_settings_key(&mut self, key: KeyEvent) {
        let (screens, action) = self.screens.handle_settings_key(key);
        self.screens = screens;
        if matches!(action, SettingsAction::Saved | SettingsAction::Back) {
            self.transition_to(TuiScreen::Home);
        }
    }

    fn handle_list_workflows_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char(Self::PREVIOUS_KEY) => {
                self.screens = self.screens.select_list_workflows_previous();
            }
            KeyCode::Down | KeyCode::Char(Self::NEXT_KEY) => {
                self.screens = self.screens.select_list_workflows_next();
            }
            KeyCode::Esc | KeyCode::Backspace => self.transition_to(TuiScreen::Home),
            _ => {}
        }
    }

    fn handle_list_actions_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char(Self::PREVIOUS_KEY) => {
                self.screens = self.screens.select_list_actions_previous();
            }
            KeyCode::Down | KeyCode::Char(Self::NEXT_KEY) => {
                self.screens = self.screens.select_list_actions_next();
            }
            KeyCode::Esc | KeyCode::Backspace => self.transition_to(TuiScreen::Home),
            _ => {}
        }
    }

    fn handle_run_workflow_key(&mut self, key: KeyEvent) {
        if self.screens.has_configuration_error() {
            self.handle_configuration_key(key);
            return;
        }
        if self.screens.is_showing_details() {
            self.handle_details_key(key);
            return;
        }
        if self.screens.is_workflow_running() {
            self.handle_running_workflow_key(key);
            return;
        }
        self.handle_workflow_picker_key(key);
    }

    fn handle_configuration_key(&mut self, key: KeyEvent) {
        let (screens, action) = self.screens.handle_configuration_key(key);
        self.screens = screens;
        match action {
            ConfigurationAction::Submit => self.configuration_requested = true,
            ConfigurationAction::Cancel => {
                let (screens, _) = self.screens.take_configuration();
                self.screens = screens;
            }
            ConfigurationAction::Continue => {}
        }
    }

    fn handle_details_key(&mut self, key: KeyEvent) {
        self.screens = match key.code {
            KeyCode::Up | KeyCode::Char(Self::PREVIOUS_KEY) => self.screens.scroll_details_up(),
            KeyCode::Down | KeyCode::Char(Self::NEXT_KEY) => self.screens.scroll_details_down(),
            KeyCode::Enter | KeyCode::Char(' ') => self.screens.toggle_details(),
            KeyCode::Esc | KeyCode::Backspace => self.screens.close_details(),
            _ => self.screens.clone(),
        };
    }

    fn handle_running_workflow_key(&mut self, key: KeyEvent) {
        if matches!(key.code, KeyCode::Esc | KeyCode::Backspace) {
            self.cancel_requested = true;
        }
    }

    fn handle_workflow_picker_key(&mut self, key: KeyEvent) {
        if self.screens.has_run_outcome() {
            if key.code == KeyCode::Char(Self::DETAILS_KEY) {
                self.screens = self.screens.open_details();
                return;
            }
            if self.handle_summary_key(key) {
                return;
            }
        }
        self.screens = match key.code {
            KeyCode::Up | KeyCode::Char(Self::PREVIOUS_KEY) => {
                self.screens.select_run_workflow_previous()
            }
            KeyCode::Down | KeyCode::Char(Self::NEXT_KEY) => {
                self.screens.select_run_workflow_next()
            }
            KeyCode::Enter => {
                self.request_run();
                self.screens.clone()
            }
            KeyCode::Esc | KeyCode::Backspace => self.screens.transition_to(TuiScreen::Home),
            _ => self.screens.clone(),
        };
    }

    fn handle_summary_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Up | KeyCode::Char(Self::PREVIOUS_KEY) => {
                self.screens = self.screens.scroll_summary_up();
                true
            }
            KeyCode::Down | KeyCode::Char(Self::NEXT_KEY) => {
                self.screens = self.screens.scroll_summary_down();
                true
            }
            _ => false,
        }
    }

    fn request_run(&mut self) {
        if self.screens.has_workflows() {
            self.run_requested = true;
        }
    }
}

impl Default for TuiApp {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}
