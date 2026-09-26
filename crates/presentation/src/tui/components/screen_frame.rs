use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    text::Span,
    widgets::{Block, Borders, Padding, Paragraph},
};

use crate::tui::theme::Theme;

/// Renders the shared outer frame around each interactive TUI screen.
pub struct ScreenFrame;

impl ScreenFrame {
    const BLOCK_TITLE: &'static str = "ephact";
    const MARGIN: u16 = 2;
    const TITLE_HEIGHT: u16 = 3;
    const CONTENT_MIN_HEIGHT: u16 = 5;

    /// Renders the application title and returns the area for the active screen.
    pub fn render(frame: &mut Frame<'_>, title: &str) -> Rect {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(Self::MARGIN)
            .constraints([
                Constraint::Length(Self::TITLE_HEIGHT),
                Constraint::Min(Self::CONTENT_MIN_HEIGHT),
            ])
            .split(frame.area());
        let title = Paragraph::new(title).style(Theme::title_style()).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Theme::border_style())
                .padding(Padding::horizontal(1))
                .title(Span::styled(Self::BLOCK_TITLE, Theme::title_style())),
        );
        frame.render_widget(title, chunks[0]);
        chunks[1]
    }
}
