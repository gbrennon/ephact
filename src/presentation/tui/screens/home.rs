use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

pub struct HomeScreen {}

impl HomeScreen {
    pub const RUN_WORKFLOW_INDEX: usize = 0;
    pub const LIST_WORKFLOWS_INDEX: usize = 1;
    pub const LIST_ACTIONS_INDEX: usize = 2;
    pub const LAST_MENU_INDEX: usize = 2;

    const TITLE: &'static str = "EPHACT";
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

        let title = Paragraph::new(Self::TITLE)
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Self::BLOCK_TITLE),
            );
        frame.render_widget(title, chunks[0]);

        let menu =
            List::new(Self::MENU_ITEMS.map(|item| ListItem::new(Line::from(Span::raw(item)))))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(Self::MENU_TITLE),
                )
                .highlight_style(
                    Style::default()
                        .bg(Color::Cyan)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                );
        let mut state = ListState::default();
        state.select(Some(selected_index));
        frame.render_stateful_widget(menu, chunks[1], &mut state);
    }
}
