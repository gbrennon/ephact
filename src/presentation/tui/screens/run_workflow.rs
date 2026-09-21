use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph, Wrap},
};

use super::run_configuration::{ConfigurationAction, RunConfiguration, RunConfigurationValues};
use crate::{
    application::dtos::responses::{
        JobSummaryResponse, RunInputDeclarationResponse, RunSummaryResponse,
        WorkflowListItemResponse,
    },
    presentation::tui::theme::Theme,
};

/// Screen that lets the user pick a workflow, run it, and read the summary.
///
/// Before a run the screen shows a selectable list of workflow names. Once a
/// run finishes, [`RunWorkflowScreen::record_outcome`] stores the summary and
/// the screen renders the per-job result instead of the picker.
pub struct RunWorkflowScreen {
    workflows: Vec<WorkflowListItemResponse>,
    selected_index: usize,
    outcome: Option<RunSummaryResponse>,
    progress_lines: Vec<String>,
    running: bool,
    showing_details: bool,
    details_scroll: u16,
    summary_scroll: u16,
    configuration: Option<RunConfiguration>,
}

impl RunWorkflowScreen {
    const PICKER_TITLE: &'static str = "Run Workflow";
    const RESULT_TITLE: &'static str = "Run Summary";
    const EMPTY_MESSAGE: &'static str = "No workflows found in repository";
    const UNNAMED_WORKFLOW: &'static str = "Unnamed workflow";
    const PICKER_FOOTER: &'static str = "Enter: Run | Up/Down: Navigate | Esc: Back | q: Quit";
    const RUNNING_FOOTER: &'static str = "Esc: Cancel";
    const RESULT_NAME_PREFIX: &'static str = "Workflow: ";
    const RESULT_STATUS_PREFIX: &'static str = "Status: ";
    const JOB_STATUS_SEPARATOR: &'static str = ": ";
    const DETAILS_TITLE: &'static str = "Run Details";
    const DETAILS_EMPTY_MESSAGE: &'static str = "No step details available";
    const SUCCESS_LABEL: &'static str = "SUCCESS";
    const FAILURE_LABEL: &'static str = "FAILED";
    const INITIAL_SELECTION: usize = 0;
    const SELECTION_STEP: usize = 1;
    const MARGIN: u16 = 2;
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
            details_scroll: 0,
            summary_scroll: 0,
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
            .map(|configuration| configuration.values())
    }

    pub fn configuration_footer(&self) -> Option<&'static str> {
        self.configuration.as_ref().map(RunConfiguration::footer)
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
        self.showing_details = false;
        self.details_scroll = 0;
        self.summary_scroll = 0;
    }

    pub fn open_details(&mut self) -> bool {
        if self.outcome.is_some() {
            self.showing_details = true;
            self.details_scroll = 0;
        }
        self.showing_details
    }

    pub fn close_details(&mut self) {
        self.showing_details = false;
        self.details_scroll = 0;
    }

    pub fn scroll_details_up(&mut self) {
        self.details_scroll = self.details_scroll.saturating_sub(1);
    }

    pub fn scroll_details_down(&mut self) {
        self.details_scroll = self.details_scroll.saturating_add(1);
    }

    pub fn details_scroll(&self) -> u16 {
        self.details_scroll
    }

    pub fn scroll_summary_up(&mut self) {
        self.summary_scroll = self.summary_scroll.saturating_sub(1);
    }

    pub fn scroll_summary_down(&mut self) {
        self.summary_scroll = self.summary_scroll.saturating_add(1);
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
        self.showing_details = false;
        self.summary_scroll = 0;
    }

    pub fn record_progress(&mut self, line: String) {
        self.progress_lines.push(line);
    }

    pub fn finish_run(&mut self) {
        self.running = false;
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
        self.showing_details = false;
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

    pub fn render(&self, frame: &mut Frame<'_>) {
        let area = frame.area().inner(Margin::new(Self::MARGIN, Self::MARGIN));
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
        let content = if self.progress_lines.is_empty() {
            "Workflow is running...".to_string()
        } else {
            self.progress_lines.join("\n")
        };
        frame.render_widget(Paragraph::new(content).style(Theme::body_style()), area);
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
        let content = Paragraph::new(Self::detail_lines(summary))
            .style(Theme::body_style())
            .wrap(Wrap { trim: true })
            .scroll((self.details_scroll, 0));
        frame.render_widget(content, area);
    }

    fn detail_lines(summary: &RunSummaryResponse) -> Vec<Line<'static>> {
        summary
            .job_summaries()
            .iter()
            .flat_map(Self::job_detail_lines)
            .collect()
    }

    fn job_detail_lines(job: &JobSummaryResponse) -> Vec<Line<'static>> {
        let mut lines = vec![Line::from(format!("Job: {}", Self::job_label(job)))];
        for step in job.steps() {
            let status = if step.exit_code().is_some_and(|code| code != 0) {
                Self::FAILURE_LABEL
            } else {
                Self::SUCCESS_LABEL
            };
            lines.push(Line::from(format!("Step: {} [{status}]", step.name())));
            if let Some(exit_code) = step.exit_code() {
                lines.push(Line::from(format!("Exit code: {exit_code}")));
            }
            Self::append_output_line(&mut lines, "stdout", step.stdout());
            Self::append_output_line(&mut lines, "stderr", step.stderr());
        }
        lines
    }

    fn append_output_line(lines: &mut Vec<Line<'static>>, label: &str, output: &str) {
        if !output.is_empty() {
            lines.push(Line::from(format!("{label}: {output}")));
        }
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

    fn render_summary(&self, frame: &mut Frame<'_>, area: Rect, summary: &RunSummaryResponse) {
        let content = Paragraph::new(Self::summary_lines(summary))
            .style(Theme::body_style())
            .scroll((self.summary_scroll, 0));
        frame.render_widget(content, area);
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
            "Up/Down: Scroll | Esc: Back | q: Quit"
        } else if self.outcome.is_some() {
            "Up/Down: Scroll | Esc: Back | d: Details | q: Quit"
        } else {
            Self::PICKER_FOOTER
        }
    }
}
