use std::time::{Duration, Instant};

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph, Wrap},
};

use crate::{
    application::dtos::responses::{
        JobSummaryResponse, RunInputDeclarationResponse, RunSummaryResponse,
        WorkflowListItemResponse,
    },
    presentation::tui::{
        components::{
            ConfigurationAction, RunConfiguration, RunConfigurationValues, RunDetailsView,
        },
        theme::Theme,
    },
};
/// Screen that lets the user pick a workflow, run it, and read the summary.
///
/// Before a run the screen shows a selectable list of workflow names. Once a
/// run finishes, [`RunWorkflowScreen::record_outcome`] stores the summary and
/// the screen renders the per-job result instead of the picker.
#[derive(Clone)]
pub struct RunWorkflowScreen {
    workflows: Vec<WorkflowListItemResponse>,
    selected_index: usize,
    outcome: Option<RunSummaryResponse>,
    progress_lines: Vec<String>,
    running: bool,
    showing_details: bool,
    details: RunDetailsView,
    summary_scroll: u16,
    run_started_at: Option<Instant>,
    configuration: Option<RunConfiguration>,
}

impl RunWorkflowScreen {
    const PICKER_TITLE: &'static str = "Run Workflow";
    const RESULT_TITLE: &'static str = "Run Summary";
    const EMPTY_MESSAGE: &'static str = "No workflows found in repository";
    const UNNAMED_WORKFLOW: &'static str = "Unnamed workflow";
    const PICKER_FOOTER: &'static str =
        "Up/Down/j/k: Move | Enter: Configure | Esc/Bksp: Back | q: Quit";
    const RUNNING_FOOTER: &'static str = "Esc/Bksp: Cancel";
    const RESULT_NAME_PREFIX: &'static str = "Workflow: ";
    const RESULT_STATUS_PREFIX: &'static str = "Status: ";
    const JOB_STATUS_SEPARATOR: &'static str = ": ";
    const DETAILS_TITLE: &'static str = "Run Details";
    const DETAILS_EMPTY_MESSAGE: &'static str = "No step details available";
    const SUCCESS_LABEL: &'static str = "SUCCESS";
    const FAILURE_LABEL: &'static str = "FAILED";
    const INITIAL_SELECTION: usize = 0;
    const SELECTION_STEP: usize = 1;
    const CONTENT_MIN_HEIGHT: u16 = 5;
    const FOOTER_HEIGHT: u16 = 1;
    pub fn new(workflows: Vec<WorkflowListItemResponse>) -> Self {
        Self {
            workflows,
            selected_index: Self::INITIAL_SELECTION,
            outcome: None,
            progress_lines: Vec::new(),
            running: false,
            showing_details: false,
            details: RunDetailsView::new(),
            summary_scroll: 0,
            run_started_at: None,
            configuration: None,
        }
    }

    pub fn workflows(&self) -> &[WorkflowListItemResponse] {
        &self.workflows
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    /// Returns the name of the currently highlighted workflow, if any.
    pub fn selected_workflow_name(&self) -> Option<&str> {
        self.workflows
            .get(self.selected_index)
            .and_then(|workflow| workflow.name())
    }

    pub fn selected_workflow_events(&self) -> Vec<String> {
        self.workflows
            .get(self.selected_index)
            .map(|workflow| workflow.events().to_vec())
            .unwrap_or_default()
    }

    pub fn has_workflows(&self) -> bool {
        !self.workflows.is_empty()
    }

    pub fn outcome(&self) -> Option<&RunSummaryResponse> {
        self.outcome.as_ref()
    }

    pub fn begin_configuration(
        &mut self,
        events: Vec<String>,
        declarations: Vec<RunInputDeclarationResponse>,
    ) {
        self.configuration = Some(RunConfiguration::new(events, declarations));
    }

    pub fn handle_configuration_key(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> ConfigurationAction {
        self.configuration
            .as_mut()
            .map(|configuration| configuration.handle_key(key))
            .unwrap_or(ConfigurationAction::Cancel)
    }

    pub fn take_configuration(&mut self) -> Option<RunConfigurationValues> {
        self.configuration
            .take()
            .and_then(|configuration| configuration.values())
    }

    pub fn configuration_footer(&self) -> Option<&'static str> {
        self.configuration.as_ref().map(RunConfiguration::footer)
    }

    pub fn configuration_is_editing(&self) -> bool {
        self.configuration
            .as_ref()
            .map(RunConfiguration::is_editing)
            .unwrap_or(false)
    }

    pub fn configuration_error(&self) -> Option<&str> {
        self.configuration
            .as_ref()
            .and_then(RunConfiguration::error)
    }

    pub fn report_configuration_error(&mut self, error: String) {
        if let Some(configuration) = self.configuration.as_mut() {
            configuration.report_error(error);
        }
    }

    pub fn record_outcome(&mut self, outcome: RunSummaryResponse) {
        self.outcome = Some(outcome);
        self.configuration = None;
        self.running = false;
        self.run_started_at = None;
        self.showing_details = false;
        self.details.reset();
        self.summary_scroll = 0;
    }

    pub fn open_details(&mut self) -> bool {
        if self.outcome.is_some() {
            self.showing_details = true;
            self.details.reset();
        }
        self.showing_details
    }

    pub fn close_details(&mut self) {
        self.showing_details = false;
        self.details.reset();
    }

    pub fn scroll_details_up(&mut self) {
        self.details.move_up();
    }

    pub fn scroll_details_down(&mut self) {
        if let Some(summary) = self.outcome.as_ref() {
            self.details.move_down(summary);
        }
    }

    pub fn toggle_details(&mut self) {
        if let Some(summary) = self.outcome.as_ref() {
            self.details.toggle(summary);
        }
    }

    pub fn scroll_summary_up(&mut self) {
        self.summary_scroll = self.summary_scroll.saturating_sub(1);
    }

    pub fn scroll_summary_down(&mut self) {
        self.summary_scroll = self.summary_scroll.saturating_add(1);
    }

    pub fn details_scroll(&self) -> u16 {
        self.details.cursor() as u16
    }
    pub fn summary_scroll(&self) -> u16 {
        self.summary_scroll
    }

    pub fn showing_details(&self) -> bool {
        self.showing_details
    }

    pub fn start_run(&mut self) {
        self.outcome = None;
        self.configuration = None;
        self.progress_lines.clear();
        self.running = true;
        self.run_started_at = Some(Instant::now());
        self.showing_details = false;
        self.details.reset();
        self.summary_scroll = 0;
    }

    pub fn record_progress(&mut self, line: String) {
        self.progress_lines.extend(normalize_lines(&line));
    }

    pub fn finish_run(&mut self) {
        self.running = false;
        self.run_started_at = None;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Discards any prior run outcome, returning the screen to the picker.
    pub fn reset(&mut self) {
        self.outcome = None;
        self.configuration = None;
        self.progress_lines.clear();
        self.running = false;
        self.run_started_at = None;
        self.showing_details = false;
        self.details.reset();
        self.summary_scroll = 0;
    }

    pub fn select_next(&mut self) {
        if !self.workflows.is_empty()
            && self.selected_index + Self::SELECTION_STEP < self.workflows.len()
        {
            self.selected_index += Self::SELECTION_STEP;
        }
    }

    pub fn select_previous(&mut self) {
        if self.selected_index >= Self::SELECTION_STEP {
            self.selected_index -= Self::SELECTION_STEP;
        }
    }

    pub fn render(&self, frame: &mut Frame<'_>, area: Rect) {
        let title = if self.showing_details {
            Self::DETAILS_TITLE
        } else if self.configuration.is_some() {
            "Run Configuration"
        } else if self.outcome.is_some() && !self.running {
            Self::RESULT_TITLE
        } else {
            Self::PICKER_TITLE
        };
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Theme::border_style())
            .padding(Padding::horizontal(1))
            .title(Span::styled(title, Theme::title_style()));
        let content_area = block.inner(area);
        frame.render_widget(block, area);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(Self::CONTENT_MIN_HEIGHT),
                Constraint::Length(Self::FOOTER_HEIGHT),
            ])
            .split(content_area);
        self.render_content(frame, chunks[0]);
        self.render_footer(frame, chunks[1]);
    }

    fn render_content(&self, frame: &mut Frame<'_>, area: Rect) {
        if self.showing_details {
            self.render_details(frame, area);
            return;
        }
        if let Some(configuration) = self.configuration.as_ref() {
            configuration.render(frame, area);
            return;
        }
        if self.running {
            self.render_running(frame, area);
            return;
        }
        match self.outcome.as_ref() {
            Some(summary) => self.render_summary(frame, area, summary),
            None => self.render_picker(frame, area),
        }
    }

    fn render_running(&self, frame: &mut Frame<'_>, area: Rect) {
        let elapsed = self
            .run_started_at
            .map(|started| started.elapsed())
            .unwrap_or(Duration::ZERO);
        let spinner = ["|", "/", "-", "\\"][elapsed.as_millis() as usize / 120 % 4];
        let mut lines = vec![Line::from(format!(
            "Workflow is running... {spinner} ({:.1}s)",
            elapsed.as_secs_f32()
        ))];
        lines.extend(
            self.progress_lines
                .iter()
                .rev()
                .take(20)
                .rev()
                .cloned()
                .map(Line::from),
        );
        frame.render_widget(
            Paragraph::new(lines)
                .style(Theme::body_style())
                .wrap(Wrap { trim: true }),
            area,
        );
    }

    fn render_picker(&self, frame: &mut Frame<'_>, area: Rect) {
        if self.workflows.is_empty() {
            Self::render_empty(frame, area);
        } else {
            self.render_workflow_names(frame, area);
        }
    }

    fn render_empty(frame: &mut Frame<'_>, area: Rect) {
        let content = Paragraph::new(Self::EMPTY_MESSAGE).style(Theme::muted_style());
        frame.render_widget(content, area);
    }

    fn render_details(&self, frame: &mut Frame<'_>, area: Rect) {
        let Some(summary) = self.outcome.as_ref() else {
            frame.render_widget(
                Paragraph::new(Self::DETAILS_EMPTY_MESSAGE).style(Theme::muted_style()),
                area,
            );
            return;
        };
        self.details.render(frame, area, summary);
    }

    fn render_workflow_names(&self, frame: &mut Frame<'_>, area: Rect) {
        let items = self
            .workflows
            .iter()
            .map(Self::name_item)
            .collect::<Vec<_>>();
        let list = List::new(items)
            .style(Theme::body_style())
            .highlight_style(Self::highlight_style());
        let mut state = ListState::default();
        state.select(Some(self.selected_index));
        frame.render_stateful_widget(list, area, &mut state);
    }

    fn name_item(workflow: &WorkflowListItemResponse) -> ListItem<'static> {
        ListItem::new(Line::from(
            workflow
                .name()
                .unwrap_or(Self::UNNAMED_WORKFLOW)
                .to_string(),
        ))
    }

    fn summary_lines(summary: &RunSummaryResponse) -> Vec<Line<'static>> {
        let mut lines = vec![
            Line::from(Span::styled(
                format!("{}{}", Self::RESULT_NAME_PREFIX, summary.name()),
                Theme::body_style(),
            )),
            Self::status_line(Self::RESULT_STATUS_PREFIX, summary.success()),
        ];
        lines.extend(summary.job_summaries().iter().map(Self::job_line));
        lines
    }

    fn job_line(job: &JobSummaryResponse) -> Line<'static> {
        let prefix = format!("{}{}", Self::job_label(job), Self::JOB_STATUS_SEPARATOR);
        Self::status_line(&prefix, job.success())
    }
    fn render_summary(&self, frame: &mut Frame<'_>, area: Rect, summary: &RunSummaryResponse) {
        let lines = Self::summary_lines(summary);
        let max_scroll = lines.len().saturating_sub(area.height as usize) as u16;
        let content = Paragraph::new(lines)
            .style(Theme::body_style())
            .scroll((self.summary_scroll.min(max_scroll), 0));
        frame.render_widget(content, area);
    }

    fn status_line(prefix: &str, success: bool) -> Line<'static> {
        Line::from(vec![
            Span::styled(prefix.to_string(), Theme::body_style()),
            Span::styled(Self::status_label(success), Self::status_style(success)),
        ])
    }

    fn job_label(job: &JobSummaryResponse) -> String {
        job.name().unwrap_or(job.job_id()).to_string()
    }

    fn status_label(success: bool) -> &'static str {
        if success {
            Self::SUCCESS_LABEL
        } else {
            Self::FAILURE_LABEL
        }
    }

    fn status_style(success: bool) -> Style {
        if success {
            Theme::success_style()
        } else {
            Theme::critical_style()
        }
    }

    fn highlight_style() -> Style {
        Theme::selection_style()
    }

    fn render_footer(&self, frame: &mut Frame<'_>, area: Rect) {
        let footer = Paragraph::new(
            self.configuration_footer()
                .unwrap_or_else(|| self.footer_text()),
        )
        .style(Theme::muted_style());
        frame.render_widget(footer, area);
    }

    fn footer_text(&self) -> &'static str {
        if self.running {
            Self::RUNNING_FOOTER
        } else if self.showing_details {
            "Up/Down/j/k: Scroll | Enter/Space: Fold | Esc/Bksp: Back | q: Quit"
        } else if self.outcome.is_some() {
            "Up/Down/j/k: Scroll | d: Details | Esc/Bksp: Back | q: Quit"
        } else {
            Self::PICKER_FOOTER
        }
    }
}

fn normalize_lines(text: &str) -> Vec<String> {
    let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
    normalized
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|character| {
                    if character.is_control() {
                        ' '
                    } else {
                        character
                    }
                })
                .collect()
        })
        .collect()
}
