use std::sync::Arc;

use crossterm::event::KeyEvent;
use ratatui::{Frame, widgets::Block};

use super::{
    components::{ConfigurationAction, RunConfigurationValues, ScreenFrame},
    screens::{
        HomeScreen, ListActionsScreen, ListWorkflowsScreen, RunWorkflowScreen, SettingsAction,
        SettingsScreen, SplashScreen,
    },
    theme::Theme,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiScreen {
    Splash,
    Home,
    ListWorkflows,
    ListActions,
    RunWorkflow,
    Settings,
    Exit,
}

#[derive(Clone)]
pub struct ScreenManager {
    current_screen: TuiScreen,
    previous_screen: Option<TuiScreen>,
    home_selection: usize,
    list_workflows: ListWorkflowsScreen,
    list_actions: ListActionsScreen,
    run_workflow: RunWorkflowScreen,
    settings: SettingsScreen,
}

impl ScreenManager {
    const INITIAL_HOME_SELECTION: usize = 0;
    const SELECTION_STEP: usize = 1;

    pub fn new(workflows: Vec<WorkflowListItemResponse>) -> Self {
        Self {
            current_screen: TuiScreen::Splash,
            previous_screen: None,
            home_selection: Self::INITIAL_HOME_SELECTION,
            list_workflows: ListWorkflowsScreen::new(workflows.clone()),
            list_actions: ListActionsScreen::new(Vec::new()),
            run_workflow: RunWorkflowScreen::new(workflows),
            settings: SettingsScreen::new(Settings::default(), None),
        }
    }

    pub fn current_screen(&self) -> TuiScreen {
        self.current_screen
    }

    pub fn previous_screen(&self) -> Option<TuiScreen> {
        self.previous_screen
    }

    pub fn transition_to(&self, screen: TuiScreen) -> Self {
        if self.current_screen == screen {
            return self.clone();
        }
        let mut next = self.clone();
        next.previous_screen = Some(self.current_screen);
        next.current_screen = screen;
        next
    }

    pub fn with_actions(mut self, actions: Vec<String>) -> Self {
        self.list_actions = ListActionsScreen::new(actions);
        self
    }

    pub fn with_settings(
        mut self,
        settings: Settings,
        store: Option<Arc<dyn SettingsStorePort>>,
    ) -> Self {
        self.settings = SettingsScreen::new(settings, store);
        self
    }

    pub fn home_selection(&self) -> usize {
        self.home_selection
    }

    pub fn select_home_previous(&self) -> Self {
        let mut next = self.clone();
        next.home_selection = self.home_selection.saturating_sub(Self::SELECTION_STEP);
        next
    }

    pub fn select_home_next(&self) -> Self {
        let mut next = self.clone();
        next.home_selection =
            (self.home_selection + Self::SELECTION_STEP).min(HomeScreen::LAST_MENU_INDEX);
        next
    }

    pub fn render(&self, frame: &mut Frame<'_>, splash_quote: &str) {
        frame.render_widget(Block::default().style(Theme::window_style()), frame.area());
        if self.current_screen == TuiScreen::Splash {
            SplashScreen::render(frame, splash_quote);
            return;
        }
        let content_area = ScreenFrame::render(frame, splash_quote);
        match self.current_screen {
            TuiScreen::Home | TuiScreen::Exit => {
                HomeScreen::render(frame, content_area, self.home_selection)
            }
            TuiScreen::ListWorkflows => self.list_workflows.render(frame, content_area),
            TuiScreen::ListActions => self.list_actions.render(frame, content_area),
            TuiScreen::RunWorkflow => self.run_workflow.render(frame, content_area),
            TuiScreen::Settings => self.settings.render(frame, content_area),
            TuiScreen::Splash => {}
        }
    }

    pub fn handle_settings_key(&self, key: KeyEvent) -> (Self, SettingsAction) {
        let mut next = self.clone();
        let action = next.settings.handle_key(key);
        (next, action)
    }

    pub fn select_list_workflows_previous(&self) -> Self {
        let mut next = self.clone();
        next.list_workflows.select_previous();
        next
    }

    pub fn select_list_workflows_next(&self) -> Self {
        let mut next = self.clone();
        next.list_workflows.select_next();
        next
    }

    pub fn select_list_actions_previous(&self) -> Self {
        let mut next = self.clone();
        next.list_actions.select_previous();
        next
    }

    pub fn select_list_actions_next(&self) -> Self {
        let mut next = self.clone();
        next.list_actions.select_next();
        next
    }

    pub fn reset_run_workflow(&self) -> Self {
        let mut next = self.clone();
        next.run_workflow.reset();
        next
    }

    pub fn select_run_workflow_previous(&self) -> Self {
        let mut next = self.clone();
        next.run_workflow.select_previous();
        next
    }

    pub fn select_run_workflow_next(&self) -> Self {
        let mut next = self.clone();
        next.run_workflow.select_next();
        next
    }

    pub fn begin_run_configuration(
        &self,
        events: Vec<String>,
        declarations: Vec<RunInputDeclarationResponse>,
    ) -> Self {
        let mut next = self.clone();
        next.run_workflow.begin_configuration(events, declarations);
        next
    }

    pub fn take_configuration(&self) -> (Self, Option<RunConfigurationValues>) {
        let mut next = self.clone();
        let configuration = next.run_workflow.take_configuration();
        (next, configuration)
    }

    pub fn selected_workflow_events(&self) -> Vec<String> {
        self.run_workflow.selected_workflow_events()
    }

    pub fn selected_workflow_name(&self) -> Option<&str> {
        self.run_workflow.selected_workflow_name()
    }

    pub fn has_workflows(&self) -> bool {
        self.run_workflow.has_workflows()
    }

    pub fn is_workflow_running(&self) -> bool {
        self.run_workflow.is_running()
    }

    pub fn has_configuration_error(&self) -> bool {
        self.run_workflow.configuration_error().is_some()
            || self.run_workflow.configuration_footer().is_some()
    }

    pub fn is_showing_details(&self) -> bool {
        self.run_workflow.showing_details()
    }

    pub fn summary_scroll(&self) -> u16 {
        self.run_workflow.summary_scroll()
    }

    pub fn details_scroll(&self) -> u16 {
        self.run_workflow.details_scroll()
    }

    pub fn selected_workflow_index(&self) -> usize {
        self.list_workflows.selected_index()
    }

    pub fn selected_action_index(&self) -> usize {
        self.list_actions.selected_index()
    }

    pub fn has_run_outcome(&self) -> bool {
        self.run_workflow.outcome().is_some()
    }

    pub fn run_outcome(&self) -> Option<&RunSummaryResponse> {
        self.run_workflow.outcome()
    }

    pub fn handle_configuration_key(&self, key: KeyEvent) -> (Self, ConfigurationAction) {
        let mut next = self.clone();
        let action = next.run_workflow.handle_configuration_key(key);
        (next, action)
    }

    pub fn report_configuration_error(&self, error: String) -> Self {
        let mut next = self.clone();
        next.run_workflow.report_configuration_error(error);
        next
    }

    pub fn scroll_details_up(&self) -> Self {
        let mut next = self.clone();
        next.run_workflow.scroll_details_up();
        next
    }

    pub fn scroll_details_down(&self) -> Self {
        let mut next = self.clone();
        next.run_workflow.scroll_details_down();
        next
    }

    pub fn toggle_details(&self) -> Self {
        let mut next = self.clone();
        next.run_workflow.toggle_details();
        next
    }

    pub fn close_details(&self) -> Self {
        let mut next = self.clone();
        next.run_workflow.close_details();
        next
    }

    pub fn scroll_summary_up(&self) -> Self {
        let mut next = self.clone();
        next.run_workflow.scroll_summary_up();
        next
    }

    pub fn scroll_summary_down(&self) -> Self {
        let mut next = self.clone();
        next.run_workflow.scroll_summary_down();
        next
    }

    pub fn open_details(&self) -> Self {
        let mut next = self.clone();
        next.run_workflow.open_details();
        next
    }

    pub fn start_run(&self) -> Self {
        let mut next = self.clone();
        next.run_workflow.start_run();
        next
    }

    pub fn finish_run(&self) -> Self {
        let mut next = self.clone();
        next.run_workflow.finish_run();
        next
    }

    pub fn record_run_outcome(&self, outcome: RunSummaryResponse) -> Self {
        let mut next = self.clone();
        next.run_workflow.record_outcome(outcome);
        next
    }

    pub fn record_progress(&self, line: String) -> Self {
        let mut next = self.clone();
        next.run_workflow.record_progress(line);
        next
    }
}
