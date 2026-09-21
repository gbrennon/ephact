use ratatui::Frame;

use super::{
    ConfigurationAction, ListActionsScreen, ListWorkflowsScreen, RunConfigurationValues,
    RunWorkflowScreen, home::HomeScreen,
};
use crate::application::dtos::responses::{
    RunInputDeclarationResponse, RunSummaryResponse, WorkflowListItemResponse,
};

pub struct ScreenManager {
    home_selection: usize,
    list_workflows: ListWorkflowsScreen,
    list_actions: ListActionsScreen,
    run_workflow: RunWorkflowScreen,
}

impl ScreenManager {
    const INITIAL_HOME_SELECTION: usize = 0;
    const SELECTION_STEP: usize = 1;

    pub fn new(workflows: Vec<WorkflowListItemResponse>) -> Self {
        Self {
            home_selection: Self::INITIAL_HOME_SELECTION,
            list_workflows: ListWorkflowsScreen::new(workflows.clone()),
            list_actions: ListActionsScreen::new(Vec::new()),
            run_workflow: RunWorkflowScreen::new(workflows),
        }
    }

    pub fn with_actions(mut self, actions: Vec<String>) -> Self {
        self.list_actions = ListActionsScreen::new(actions);
        self
    }

    pub fn home_selection(&self) -> usize {
        self.home_selection
    }

    pub fn select_home_previous(&mut self) {
        self.home_selection = self.home_selection.saturating_sub(Self::SELECTION_STEP);
    }

    pub fn select_home_next(&mut self) {
        self.home_selection =
            (self.home_selection + Self::SELECTION_STEP).min(HomeScreen::LAST_MENU_INDEX);
    }

    pub fn list_workflows(&self) -> &ListWorkflowsScreen {
        &self.list_workflows
    }

    pub fn list_workflows_mut(&mut self) -> &mut ListWorkflowsScreen {
        &mut self.list_workflows
    }

    pub fn list_actions(&self) -> &ListActionsScreen {
        &self.list_actions
    }

    pub fn list_actions_mut(&mut self) -> &mut ListActionsScreen {
        &mut self.list_actions
    }

    pub fn run_workflow_screen(&self) -> &RunWorkflowScreen {
        &self.run_workflow
    }

    pub fn run_workflow_screen_mut(&mut self) -> &mut RunWorkflowScreen {
        &mut self.run_workflow
    }

    pub fn render_home(&self, frame: &mut Frame<'_>) {
        HomeScreen::render(frame, self.home_selection);
    }

    pub fn render_list_workflows(&self, frame: &mut Frame<'_>) {
        self.list_workflows.render(frame);
    }

    pub fn render_list_actions(&self, frame: &mut Frame<'_>) {
        self.list_actions.render(frame);
    }

    pub fn render_run_workflow(&self, frame: &mut Frame<'_>) {
        self.run_workflow.render(frame);
    }

    pub fn begin_run_configuration(
        &mut self,
        events: Vec<String>,
        declarations: Vec<RunInputDeclarationResponse>,
    ) {
        self.run_workflow.begin_configuration(events, declarations);
    }

    pub fn take_configuration(&mut self) -> Option<RunConfigurationValues> {
        self.run_workflow.take_configuration()
    }

    pub fn selected_workflow_events(&self) -> Vec<String> {
        self.run_workflow.selected_workflow_events()
    }

    pub fn handle_configuration_key(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> ConfigurationAction {
        self.run_workflow.handle_configuration_key(key)
    }

    pub fn start_run(&mut self) {
        self.run_workflow.start_run();
    }

    pub fn finish_run(&mut self) {
        self.run_workflow.finish_run();
    }

    pub fn record_run_outcome(&mut self, outcome: RunSummaryResponse) {
        self.run_workflow.record_outcome(outcome);
    }

    pub fn record_progress(&mut self, line: String) {
        self.run_workflow.record_progress(line);
    }
}
