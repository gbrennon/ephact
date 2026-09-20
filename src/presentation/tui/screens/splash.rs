use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use super::color_support::ColorSupport;
use super::emblem::Emblem;
use crate::presentation::tui::theme::Theme;

/// Splash screen shown before the home menu.
///
/// Renders a centered emblem, the project wordmark, a subtitle, and a hint.
/// The emblem and wordmark upgrade to 24-bit color when the terminal supports
/// it, and degrade to a single-color fallback otherwise.
pub struct SplashScreen;

impl SplashScreen {
    const TITLE: &'static str = "EPHACT";
    const SUBTITLE: &'static str = "run workflows locally in containers";
    const HINT: &'static str = "press any key to continue";
    const BLOCK_TITLE: &'static str = "welcome";
    const WIDTH_PERCENT: u16 = 60;
    const HEIGHT_PERCENT: u16 = 70;

    /// Renders the splash using the color capability detected from the environment.
    pub fn render(frame: &mut Frame<'_>) {
        Self::render_with(frame, ColorSupport::from_env());
    }

    /// Renders the splash using an explicit color capability.
    pub fn render_with(frame: &mut Frame<'_>, support: ColorSupport) {
        let area = Self::centered_area(frame.area(), Self::WIDTH_PERCENT, Self::HEIGHT_PERCENT);
        let paragraph = Paragraph::new(Self::body_lines(support))
            .alignment(Alignment::Center)
            .style(Theme::window_style())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Theme::border_style())
                    .title(Span::styled(Self::BLOCK_TITLE, Theme::title_style())),
            );
        frame.render_widget(paragraph, area);
    }

    fn body_lines(support: ColorSupport) -> Vec<Line<'static>> {
        let mut lines = vec![Line::from("")];
        lines.extend(Emblem::lines_for(support));
        lines.push(Line::from(""));
        lines.push(Self::title_line(support));
        lines.push(Self::subtitle_line());
        lines.push(Line::from(""));
        lines.push(Self::hint_line());
        lines
    }

    fn title_line(_support: ColorSupport) -> Line<'static> {
        Line::from(Span::styled(Self::TITLE, Theme::title_style()))
    }

    fn subtitle_line() -> Line<'static> {
        Line::from(Span::styled(Self::SUBTITLE, Theme::muted_style()))
    }

    fn hint_line() -> Line<'static> {
        Line::from(Span::styled(
            Self::HINT,
            Theme::muted_style().add_modifier(Modifier::DIM),
        ))
    }

    fn centered_area(area: Rect, width_percent: u16, height_percent: u16) -> Rect {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - height_percent) / 2),
                Constraint::Percentage(height_percent),
                Constraint::Percentage((100 - height_percent) / 2),
            ])
            .split(area);
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - width_percent) / 2),
                Constraint::Percentage(width_percent),
                Constraint::Percentage((100 - width_percent) / 2),
            ])
            .split(vertical[1])[1]
    }
}
