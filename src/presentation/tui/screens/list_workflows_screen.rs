use std::path::PathBuf;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph},
};

use crate::{
    application::{
        dtos::responses::WorkflowListItemResponse,
        ports::inbound::list_workflows_port::ListWorkflowsPort,
    },
    presentation::{handlers::ListWorkflowsHandler, tui::theme::Theme},
};

#[derive(Clone)]
pub struct ListWorkflowsScreen {
    workflows: Vec<WorkflowListItemResponse>,
    selected_index: usize,
}

impl ListWorkflowsScreen {
    const TITLE: &'static str = "Workflows";
    const EMPTY_MESSAGE: &'static str = "No workflows found in repository";
    const UNNAMED_WORKFLOW: &'static str = "Unnamed workflow";
    const MISSING_FILE: &'static str = "-";
    const MISSING_EVENTS: &'static str = "-";
    const EVENT_SEPARATOR: &'static str = ", ";
    const FOOTER: &'static str = "Up/Down/j/k: Move | Esc/Bksp: Back | q: Quit";
    const INITIAL_SELECTION: usize = 0;
    const SELECTION_STEP: usize = 1;
    const CONTENT_MIN_HEIGHT: u16 = 5;
    const FOOTER_HEIGHT: u16 = 1;

    pub fn new(workflows: Vec<WorkflowListItemResponse>) -> Self {
        Self {
            workflows,
            selected_index: Self::INITIAL_SELECTION,
        }
    }

    /// Builds the screen by listing the workflows under `repository_path`.
    pub fn from_handler(
        port: &dyn ListWorkflowsPort,
        repository_path: PathBuf,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let response = ListWorkflowsHandler::handle(port, repository_path)?;
        Ok(Self::new(response.into_workflows()))
    }

    pub fn workflows(&self) -> &[WorkflowListItemResponse] {
        &self.workflows
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
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
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Theme::border_style())
            .padding(Padding::horizontal(1))
            .title(Span::styled(Self::TITLE, Theme::title_style()));
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
        if self.workflows.is_empty() {
            self.render_empty(frame, area);
        } else {
            self.render_populated(frame, area);
        }
    }

    fn render_empty(&self, frame: &mut Frame<'_>, area: Rect) {
        let content = Paragraph::new(Self::EMPTY_MESSAGE).style(Theme::muted_style());
        frame.render_widget(content, area);
    }

    fn render_populated(&self, frame: &mut Frame<'_>, area: Rect) {
        let items = self
            .workflows
            .iter()
            .map(Self::workflow_item)
            .collect::<Vec<_>>();
        let list = List::new(items)
            .style(Theme::body_style())
            .highlight_style(Self::highlight_style());
        let mut state = ListState::default();
        state.select(Some(self.selected_index));
        frame.render_stateful_widget(list, area, &mut state);
    }

    fn workflow_item(workflow: &WorkflowListItemResponse) -> ListItem<'static> {
        let name = workflow.name().unwrap_or(Self::UNNAMED_WORKFLOW);
        let file = workflow.file().unwrap_or(Self::MISSING_FILE);
        let events = Self::format_events(workflow);
        ListItem::new(Line::from(format!("{name}  ({file})  [{events}]")))
    }

    fn format_events(workflow: &WorkflowListItemResponse) -> String {
        if workflow.events().is_empty() {
            Self::MISSING_EVENTS.to_string()
        } else {
            workflow.events().join(Self::EVENT_SEPARATOR)
        }
    }

    fn highlight_style() -> Style {
        Theme::selection_style()
    }

    fn render_footer(&self, frame: &mut Frame<'_>, area: Rect) {
        let footer = Paragraph::new(Self::FOOTER).style(Theme::muted_style());
        frame.render_widget(footer, area);
    }
}
