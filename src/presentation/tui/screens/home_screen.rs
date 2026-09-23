use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph},
};

use crate::presentation::tui::theme::Theme;

pub struct HomeScreen {}

impl HomeScreen {
    pub const RUN_WORKFLOW_INDEX: usize = 0;
    pub const LIST_WORKFLOWS_INDEX: usize = 1;
    pub const LIST_ACTIONS_INDEX: usize = 2;
    pub const SETTINGS_INDEX: usize = 3;
    pub const LAST_MENU_INDEX: usize = 3;

    const BLOCK_TITLE: &'static str = "What would you like to do?";
    const FOOTER: &'static str = "Up/Down/j/k: Move | Enter: Select | q: Quit";
    const FOOTER_HEIGHT: u16 = 1;
    const CONTENT_MIN_HEIGHT: u16 = 5;
    const MENU_ITEMS: [&'static str; 4] =
        ["Run workflow", "List workflows", "List actions", "Settings"];

    pub fn render(frame: &mut Frame<'_>, area: Rect, selected_index: usize) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Theme::border_style())
            .padding(Padding::horizontal(1))
            .title(Span::styled(Self::BLOCK_TITLE, Theme::title_style()));
        let content_area = block.inner(area);
        frame.render_widget(block, area);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(Self::CONTENT_MIN_HEIGHT),
                Constraint::Length(Self::FOOTER_HEIGHT),
            ])
            .split(content_area);
        Self::render_menu(frame, chunks[0], selected_index);
        Self::render_footer(frame, chunks[1]);
    }

    fn render_menu(frame: &mut Frame<'_>, area: Rect, selected_index: usize) {
        let menu =
            List::new(Self::MENU_ITEMS.map(|item| ListItem::new(Line::from(Span::raw(item)))))
                .style(Theme::body_style())
                .highlight_style(Theme::selection_style());
        let mut state = ListState::default();
        state.select(Some(selected_index));
        frame.render_stateful_widget(menu, area, &mut state);
    }

    fn render_footer(frame: &mut Frame<'_>, area: Rect) {
        frame.render_widget(
            Paragraph::new(Self::FOOTER).style(Theme::muted_style()),
            area,
        );
    }
}
