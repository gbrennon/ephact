use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph},
};

use crate::application::dtos::responses::{
    JobSummaryResponse, RunSummaryResponse, WorkflowListItemResponse,
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
}

impl RunWorkflowScreen {
    const PICKER_TITLE: &'static str = "Run Workflow";
    const RESULT_TITLE: &'static str = "Run Summary";
    const EMPTY_MESSAGE: &'static str = "No workflows found in repository";
    const UNNAMED_WORKFLOW: &'static str = "Unnamed workflow";
    const PICKER_FOOTER: &'static str = "Enter: Run | Up/Down: Navigate | Esc: Back | q: Quit";
    const RESULT_FOOTER: &'static str = "Esc: Back | q: Quit";
    const RUNNING_FOOTER: &'static str = "Esc: Cancel";
    const RESULT_NAME_PREFIX: &'static str = "Workflow: ";
    const RESULT_STATUS_PREFIX: &'static str = "Status: ";
    const JOB_PREFIX: &'static str = "  ";
    const JOB_STATUS_SEPARATOR: &'static str = ": ";
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

    pub fn has_workflows(&self) -> bool {
        !self.workflows.is_empty()
    }

    pub fn outcome(&self) -> Option<&RunSummaryResponse> {
        self.outcome.as_ref()
    }

    pub fn record_outcome(&mut self, outcome: RunSummaryResponse) {
        self.outcome = Some(outcome);
    }

    pub fn start_run(&mut self) {
        self.outcome = None;
        self.progress_lines.clear();
        self.running = true;
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
        self.progress_lines.clear();
        self.running = false;
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
        let title = if self.outcome.is_some() && !self.running {
            Self::RESULT_TITLE
        } else {
            Self::PICKER_TITLE
        };
        let block = Block::default()
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1))
            .title(title);
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
        if self.running {
            self.render_running(frame, area);
            return;
        }
        match self.outcome.as_ref() {
            Some(summary) => Self::render_summary(frame, area, summary),
            None => self.render_picker(frame, area),
        }
    }

    fn render_running(&self, frame: &mut Frame<'_>, area: Rect) {
        let content = if self.progress_lines.is_empty() {
            "Workflow is running...".to_string()
        } else {
            self.progress_lines.join("\n")
        };
        frame.render_widget(Paragraph::new(content), area);
    }

    fn render_picker(&self, frame: &mut Frame<'_>, area: Rect) {
        if self.workflows.is_empty() {
            Self::render_empty(frame, area);
        } else {
            self.render_workflow_names(frame, area);
        }
    }

    fn render_empty(frame: &mut Frame<'_>, area: Rect) {
        let content = Paragraph::new(Self::EMPTY_MESSAGE);
        frame.render_widget(content, area);
    }

    fn render_workflow_names(&self, frame: &mut Frame<'_>, area: Rect) {
        let items = self
            .workflows
            .iter()
            .map(Self::name_item)
            .collect::<Vec<_>>();
        let list = List::new(items).highlight_style(Self::highlight_style());
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

    fn render_summary(frame: &mut Frame<'_>, area: Rect, summary: &RunSummaryResponse) {
        let list = List::new(Self::summary_items(summary));
        frame.render_widget(list, area);
    }

    fn summary_items(summary: &RunSummaryResponse) -> Vec<ListItem<'static>> {
        let mut items = vec![
            ListItem::new(Line::from(format!(
                "{}{}",
                Self::RESULT_NAME_PREFIX,
                summary.name()
            ))),
            ListItem::new(Line::from(format!(
                "{}{}",
                Self::RESULT_STATUS_PREFIX,
                Self::status_label(summary.success())
            ))),
        ];
        items.extend(summary.job_summaries().iter().map(Self::job_item));
        items
    }

    fn job_item(job: &JobSummaryResponse) -> ListItem<'static> {
        ListItem::new(Line::from(format!(
            "{}{}{}{}",
            Self::JOB_PREFIX,
            Self::job_label(job),
            Self::JOB_STATUS_SEPARATOR,
            Self::status_label(job.success())
        )))
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

    fn highlight_style() -> Style {
        Style::default()
            .bg(Color::Cyan)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD)
    }

    fn render_footer(&self, frame: &mut Frame<'_>, area: Rect) {
        let footer = Paragraph::new(self.footer_text()).style(Style::default().fg(Color::DarkGray));
        frame.render_widget(footer, area);
    }

    fn footer_text(&self) -> &'static str {
        if self.running {
            Self::RUNNING_FOOTER
        } else if self.outcome.is_some() {
            Self::RESULT_FOOTER
        } else {
            Self::PICKER_FOOTER
        }
    }
}
