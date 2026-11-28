use std::{path::PathBuf, sync::Arc};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Clear, Paragraph},
};

use super::{
    components::{ConfigurationAction, RunConfigurationValues},
    navigation_bindings::{NavigationAction, NavigationBindings},
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
    components::SplashQuotes,
    domain::Settings,
    infrastructure::HostSshForwardingSettingsPort,
};

pub struct TuiApp {
    screens: ScreenManager,
    run_requested: bool,
    cancel_requested: bool,
    configuration_requested: bool,
    action_configuration_requested: bool,
    action_selection_requested: bool,
    splash_quote: &'static str,
    navigation: NavigationBindings,
    help_visible: bool,
}

impl TuiApp {
    const QUIT_KEY: char = 'q';
    const PREVIOUS_KEY: char = 'k';
    const NEXT_KEY: char = 'j';
    const DETAILS_KEY: char = 'd';

    fn centered_rect(width_percent: u16, height_percent: u16, area: Rect) -> Rect {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - height_percent) / 2),
                Constraint::Percentage(height_percent),
                Constraint::Percentage((100 - height_percent) / 2),
            ])
            .split(area);
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - width_percent) / 2),
                Constraint::Percentage(width_percent),
                Constraint::Percentage((100 - width_percent) / 2),
            ])
            .split(vertical[1])[1]
    }

    pub fn new(workflows: Vec<WorkflowListItemResponse>, emblem: String) -> Self {
        Self {
            screens: ScreenManager::new(workflows, emblem),
            run_requested: false,
            cancel_requested: false,
            configuration_requested: false,
            action_configuration_requested: false,
            action_selection_requested: false,
            splash_quote: SplashQuotes::random(),
            navigation: NavigationBindings::new(crate::domain::NavigationMode::Standard),
            help_visible: false,
        }
    }

    pub fn with_actions(self, actions: Vec<String>) -> Self {
        Self {
            screens: self.screens.with_actions(actions),
            ..self
        }
    }

    pub fn with_settings(
        mut self,
        settings: Settings,
        store: Option<Arc<dyn SettingsStorePort>>,
    ) -> Self {
        self.navigation.set_mode(settings.navigation_mode());
        self.screens = self.screens.with_settings(settings, store);
        self
    }

    pub fn with_host_ssh_settings(self, settings: Arc<dyn HostSshForwardingSettingsPort>) -> Self {
        Self {
            screens: self.screens.with_host_ssh_settings(settings),
            ..self
        }
    }
    pub fn with_optional_host_ssh_settings(
        self,
        settings: Option<Arc<dyn HostSshForwardingSettingsPort>>,
    ) -> Self {
        match settings {
            Some(settings) => self.with_host_ssh_settings(settings),
            None => self,
        }
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

    pub fn take_action_selection_request(&mut self) -> bool {
        let requested = self.action_selection_requested;
        self.action_selection_requested = false;
        requested
    }

    pub fn take_action_run_request(&mut self) -> Option<(String, RunConfigurationValues)> {
        if !self.action_configuration_requested {
            return None;
        }
        self.action_configuration_requested = false;
        let action = self.screens.selected_action_for_run()?;
        let (screens, configuration) = self.screens.take_action_configuration();
        self.screens = screens;
        configuration.map(|configuration| (action, configuration))
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

    pub fn selected_action(&self) -> Option<&str> {
        self.screens.selected_action()
    }

    pub fn begin_action_configuration(&mut self, declarations: Vec<RunInputDeclarationResponse>) {
        self.screens = self.screens.begin_action_configuration(declarations);
    }

    pub fn report_action_configuration_error(&mut self, error: String) {
        self.screens = self.screens.report_action_configuration_error(error);
    }

    pub fn is_action_configuration_ready(&self) -> bool {
        self.screens.action_configuration_footer().is_some()
            || self.screens.has_action_configuration_error()
    }

    pub fn start_action_run(&mut self) {
        self.screens = self.screens.start_action_run();
    }

    pub fn record_action_progress(&mut self, line: String) {
        self.screens = self.screens.record_action_progress(line);
    }

    pub fn record_action_outcome(
        &mut self,
        outcome: crate::application::dtos::responses::ExecuteActionResponse,
    ) {
        self.screens = self.screens.record_action_outcome(outcome);
    }

    pub fn record_action_cancelled(&mut self) {
        self.screens = self.screens.record_action_cancelled();
    }

    pub fn run_outcome(&self) -> Option<&RunSummaryResponse> {
        self.screens.run_outcome()
    }
    pub fn report_configuration_error(&mut self, error: String) {
        self.screens = self.screens.report_configuration_error(error);
    }

    pub fn has_configuration_error(&self) -> bool {
        self.screens.has_configuration_error()
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
        self.record_run_outcome_with_failure_log_path(outcome, None);
    }

    pub fn record_run_outcome_with_failure_log_path(
        &mut self,
        outcome: RunSummaryResponse,
        failure_log_path: Option<PathBuf>,
    ) {
        self.screens = self
            .screens
            .record_run_outcome_with_failure_log_path(outcome, failure_log_path);
    }

    pub fn record_progress(&mut self, line: String) {
        self.screens = self.screens.record_progress(line);
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if self.help_visible {
            if matches!(
                key.code,
                KeyCode::Char('?') | KeyCode::Esc | KeyCode::Backspace
            ) {
                self.help_visible = false;
            }
            return;
        }
        if self.navigation_is_active() {
            if let Some(action) = self.navigation.map(key) {
                if action == NavigationAction::Help {
                    self.help_visible = true;
                } else {
                    self.apply_navigation_action(action);
                }
                return;
            }
            if self.navigation.mode() != crate::domain::NavigationMode::Standard
                && matches!(
                    key.code,
                    KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right
                )
            {
                return;
            }
        }
        if key.code == KeyCode::Char(Self::QUIT_KEY)
            && !self.screens.is_workflow_running()
            && !self.screens.is_action_running()
            && !self.screens.is_configuration_editing()
            && !self.screens.is_action_configuration_editing()
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
            TuiScreen::RunAction => self.handle_run_action_key(key),
            TuiScreen::Settings => self.handle_settings_key(key),
            TuiScreen::Exit => {}
        }
    }

    pub fn render(&self, frame: &mut Frame<'_>) {
        self.screens.render(frame, self.splash_quote);
        if self.help_visible {
            self.render_navigation_help(frame);
        }
    }

    fn navigation_is_active(&self) -> bool {
        !self.screens.is_workflow_running()
            && !self.screens.is_action_running()
            && !self.screens.is_configuration_editing()
            && !self.screens.has_configuration_error()
            && !self.screens.is_action_configuration_editing()
            && !self.screens.has_action_configuration_error()
            && !self.screens.action_configuration_footer().is_some()
            && !self.screens.is_settings_editing()
    }

    fn apply_navigation_action(&mut self, action: NavigationAction) {
        match action {
            NavigationAction::Previous => self.apply_navigation_step(false),
            NavigationAction::Next => self.apply_navigation_step(true),
            NavigationAction::Left | NavigationAction::Right => {}
            NavigationAction::PageUp => self.apply_repeated_navigation(false, 10),
            NavigationAction::PageDown => self.apply_repeated_navigation(true, 10),
            NavigationAction::Top => self.apply_repeated_navigation(false, 1_000),
            NavigationAction::Bottom => self.apply_repeated_navigation(true, 1_000),
            NavigationAction::Help => self.help_visible = true,
        }
    }

    fn apply_repeated_navigation(&mut self, next: bool, count: usize) {
        for _ in 0..count {
            self.apply_navigation_step(next);
        }
    }

    fn apply_navigation_step(&mut self, next: bool) {
        match self.screens.current_screen() {
            TuiScreen::Home => {
                self.screens = if next {
                    self.screens.select_home_next()
                } else {
                    self.screens.select_home_previous()
                };
            }
            TuiScreen::ListWorkflows => {
                self.screens = if next {
                    self.screens.select_list_workflows_next()
                } else {
                    self.screens.select_list_workflows_previous()
                };
            }
            TuiScreen::ListActions => {
                self.screens = if next {
                    self.screens.select_list_actions_next()
                } else {
                    self.screens.select_list_actions_previous()
                };
            }
            TuiScreen::RunWorkflow => {
                if self.screens.is_showing_details() {
                    self.screens = if next {
                        self.screens.scroll_details_down()
                    } else {
                        self.screens.scroll_details_up()
                    };
                } else if self.screens.has_run_outcome() {
                    self.screens = if next {
                        self.screens.scroll_summary_down()
                    } else {
                        self.screens.scroll_summary_up()
                    };
                } else {
                    self.screens = if next {
                        self.screens.select_run_workflow_next()
                    } else {
                        self.screens.select_run_workflow_previous()
                    };
                }
            }
            TuiScreen::RunAction => {
                if self.screens.is_showing_action_details() {
                    self.screens = if next {
                        self.screens.scroll_action_summary_down()
                    } else {
                        self.screens.scroll_action_summary_up()
                    };
                } else if self.screens.is_action_picker() {
                    self.screens = if next {
                        self.screens.select_list_actions_next()
                    } else {
                        self.screens.select_list_actions_previous()
                    };
                }
            }
            TuiScreen::Settings => {
                let key = if next { KeyCode::Down } else { KeyCode::Up };
                self.handle_settings_key(KeyEvent::new(key, KeyModifiers::NONE));
            }
            TuiScreen::Splash | TuiScreen::Exit => {}
        }
    }

    fn render_navigation_help(&self, frame: &mut Frame<'_>) {
        let area = Self::centered_rect(60, 35, frame.area());
        let mut lines = vec![format!(
            "Navigation mode: {}",
            self.navigation.mode().label()
        )];
        lines.extend(NavigationBindings::help_lines(self.navigation.mode()));
        lines.push("?: close help".to_owned());
        frame.render_widget(Clear, area);
        frame.render_widget(
            Paragraph::new(lines.join("\n"))
                .block(Block::default().borders(Borders::ALL).title("Keybindings"))
                .wrap(ratatui::widgets::Wrap { trim: true }),
            area,
        );
    }

    pub fn screen(&self) -> TuiScreen {
        self.screens.current_screen()
    }

    pub fn is_navigation_help_visible(&self) -> bool {
        self.help_visible
    }

    pub fn is_action_picker(&self) -> bool {
        self.screens.is_action_picker()
    }

    pub fn is_action_running(&self) -> bool {
        self.screens.is_action_running()
    }

    pub fn has_action_outcome(&self) -> bool {
        self.screens.has_action_outcome()
    }

    pub fn is_showing_action_details(&self) -> bool {
        self.screens.is_showing_action_details()
    }

    pub fn open_action_details(&mut self) {
        self.screens = self.screens.open_action_details();
    }

    pub fn close_action_details(&mut self) {
        self.screens = self.screens.close_action_details();
    }

    pub fn scroll_action_summary_up(&mut self) {
        self.screens = self.screens.scroll_action_summary_up();
    }

    pub fn scroll_action_summary_down(&mut self) {
        self.screens = self.screens.scroll_action_summary_down();
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
            HomeScreen::RUN_ACTION_INDEX => TuiScreen::RunAction,
            HomeScreen::SETTINGS_INDEX => TuiScreen::Settings,
            _ => return,
        };
        self.screens = self.screens.transition_to(screen);
        if screen == TuiScreen::RunWorkflow {
            self.screens = self.screens.reset_run_workflow();
        } else if screen == TuiScreen::RunAction {
            self.screens = self.screens.open_action_picker();
        }
    }

    fn handle_settings_key(&mut self, key: KeyEvent) {
        let (screens, action) = self.screens.handle_settings_key(key);
        self.screens = screens;
        if matches!(action, SettingsAction::Saved) {
            self.navigation.set_mode(self.screens.navigation_mode());
        }
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
            KeyCode::Enter if self.screens.selected_action().is_some() => {
                self.screens = self.screens.open_selected_action();
                self.action_selection_requested = true;
                self.transition_to(TuiScreen::RunAction);
            }
            _ => {}
        }
    }

    fn handle_run_action_key(&mut self, key: KeyEvent) {
        if self.screens.is_action_picker() {
            match key.code {
                KeyCode::Up | KeyCode::Char(Self::PREVIOUS_KEY) => {
                    self.screens = self.screens.select_list_actions_previous();
                }
                KeyCode::Down | KeyCode::Char(Self::NEXT_KEY) => {
                    self.screens = self.screens.select_list_actions_next();
                }
                KeyCode::Enter if self.screens.selected_action().is_some() => {
                    self.screens = self.screens.confirm_action_selection();
                    self.action_selection_requested = true;
                }
                KeyCode::Esc | KeyCode::Backspace => self.transition_to(TuiScreen::Home),
                _ => {}
            }
            return;
        }
        if self.screens.has_action_configuration_error() {
            self.handle_action_configuration_key(key);
            return;
        }
        if self.screens.is_showing_action_details() {
            match key.code {
                KeyCode::Up | KeyCode::Char(Self::PREVIOUS_KEY) => {
                    self.scroll_action_summary_up();
                }
                KeyCode::Down | KeyCode::Char(Self::NEXT_KEY) => {
                    self.scroll_action_summary_down();
                }
                KeyCode::Esc | KeyCode::Backspace => self.close_action_details(),
                _ => {}
            }
            return;
        }
        if self.screens.is_action_running() {
            if matches!(key.code, KeyCode::Esc | KeyCode::Backspace) {
                self.cancel_requested = true;
            }
            return;
        }
        if self.screens.action_configuration_footer().is_some() {
            self.handle_action_configuration_key(key);
            return;
        }
        if self.screens.has_action_outcome() && key.code == KeyCode::Char(Self::DETAILS_KEY) {
            self.open_action_details();
            return;
        }
        if matches!(key.code, KeyCode::Esc | KeyCode::Backspace) {
            let destination = self.screens.previous_screen().unwrap_or(TuiScreen::Home);
            self.transition_to(destination);
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

    fn handle_action_configuration_key(&mut self, key: KeyEvent) {
        let (screens, action) = self.screens.handle_action_configuration_key(key);
        self.screens = screens;
        match action {
            ConfigurationAction::Submit => self.action_configuration_requested = true,
            ConfigurationAction::Cancel => {
                let (screens, _) = self.screens.take_action_configuration();
                self.screens = screens.transition_to(TuiScreen::Home);
            }
            ConfigurationAction::Continue => {}
        }
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
