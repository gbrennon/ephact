use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph},
};

use std::path::PathBuf;

use crate::application::ports::inbound::list_actions_port::ListActionsPort;
use crate::presentation::handlers::ListActionsHandler;

pub struct ListActionsScreen {
    actions: Vec<String>,
    selected_index: usize,
}

impl ListActionsScreen {
    const TITLE: &'static str = "Actions";
    const EMPTY_MESSAGE: &'static str = "No actions found in repository";
    const FOOTER: &'static str = "Up/Down: Navigate | Esc: Back | q: Quit";
    const INITIAL_SELECTION: usize = 0;
    const SELECTION_STEP: usize = 1;
    const MARGIN: u16 = 2;
    const CONTENT_MIN_HEIGHT: u16 = 5;
    const FOOTER_HEIGHT: u16 = 1;

    pub fn new(actions: Vec<String>) -> Self {
        Self {
            actions,
            selected_index: Self::INITIAL_SELECTION,
        }
    }

    /// Builds the screen by listing the actions under `repository_path`.
    pub fn from_handler(
        port: &dyn ListActionsPort,
        repository_path: PathBuf,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let response = ListActionsHandler::handle(port, repository_path)?;
        Ok(Self::new(response.into_actions()))
    }

    pub fn actions(&self) -> &[String] {
        &self.actions
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn select_next(&mut self) {
        if !self.actions.is_empty()
            && self.selected_index + Self::SELECTION_STEP < self.actions.len()
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
        let block = Block::default()
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1))
            .title(Self::TITLE);
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
        if self.actions.is_empty() {
            self.render_empty(frame, area);
        } else {
            self.render_populated(frame, area);
        }
    }

    fn render_empty(&self, frame: &mut Frame<'_>, area: Rect) {
        let content = Paragraph::new(Self::EMPTY_MESSAGE);
        frame.render_widget(content, area);
    }

    fn render_populated(&self, frame: &mut Frame<'_>, area: Rect) {
        let items = self
            .actions
            .iter()
            .map(|action| Self::action_item(action))
            .collect::<Vec<_>>();
        let list = List::new(items).highlight_style(Self::highlight_style());
        let mut state = ListState::default();
        state.select(Some(self.selected_index));
        frame.render_stateful_widget(list, area, &mut state);
    }

    fn action_item(action: &str) -> ListItem<'static> {
        ListItem::new(Line::from(action.to_string()))
    }

    fn highlight_style() -> Style {
        Style::default()
            .bg(Color::Cyan)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD)
    }

    fn render_footer(&self, frame: &mut Frame<'_>, area: Rect) {
        let footer = Paragraph::new(Self::FOOTER).style(Style::default().fg(Color::DarkGray));
        frame.render_widget(footer, area);
    }
}
