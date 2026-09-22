use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Padding},
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
    const MENU_ITEMS: [&'static str; 4] =
        ["Run workflow", "List workflows", "List actions", "Settings"];

    pub fn render(frame: &mut Frame<'_>, area: Rect, selected_index: usize) {
        let menu =
            List::new(Self::MENU_ITEMS.map(|item| ListItem::new(Line::from(Span::raw(item)))))
                .style(Theme::body_style())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Theme::border_style())
                        .padding(Padding::horizontal(1))
                        .title(Span::styled(Self::BLOCK_TITLE, Theme::title_style())),
                )
                .highlight_style(Theme::selection_style());
        let mut state = ListState::default();
        state.select(Some(selected_index));
        frame.render_stateful_widget(menu, area, &mut state);
    }
}
