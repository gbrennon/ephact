use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::presentation::tui::theme::Theme;

pub struct HomeScreen {}

impl HomeScreen {
    pub const RUN_WORKFLOW_INDEX: usize = 0;
    pub const LIST_WORKFLOWS_INDEX: usize = 1;
    pub const LIST_ACTIONS_INDEX: usize = 2;
    pub const LAST_MENU_INDEX: usize = 2;

    const EPHEMERAL_TITLE: &'static str = "EPHEMERAL WORKFLOW RUNNER";
    const BLOCK_TITLE: &'static str = "ephact";
    const MENU_TITLE: &'static str = "What would you like to do?";
    const MENU_ITEMS: [&'static str; 3] = ["Run workflow", "List workflows", "List actions"];
    const MARGIN: u16 = 2;
    const TITLE_HEIGHT: u16 = 3;
    const CONTENT_MIN_HEIGHT: u16 = 5;

    pub fn render(frame: &mut Frame<'_>, selected_index: usize) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(Self::MARGIN)
            .constraints([
                Constraint::Length(Self::TITLE_HEIGHT),
                Constraint::Min(Self::CONTENT_MIN_HEIGHT),
            ])
            .split(frame.area());

        let title = Paragraph::new(Self::EPHEMERAL_TITLE)
            .style(Theme::title_style())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Theme::border_style())
                    .title(Span::styled(Self::BLOCK_TITLE, Theme::title_style())),
            );
        frame.render_widget(title, chunks[0]);

        let menu =
            List::new(Self::MENU_ITEMS.map(|item| ListItem::new(Line::from(Span::raw(item)))))
                .style(Theme::body_style())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Theme::border_style())
                        .title(Span::styled(
                            Self::MENU_TITLE,
                            Theme::section_header_style(),
                        )),
                )
                .highlight_style(Theme::selection_style());
        let mut state = ListState::default();
        state.select(Some(selected_index));
        frame.render_stateful_widget(menu, chunks[1], &mut state);
    }
}
