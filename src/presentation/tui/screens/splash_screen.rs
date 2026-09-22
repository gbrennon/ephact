use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph},
};

use crate::presentation::tui::{
    components::{ColorSupport, Emblem},
    theme::Theme,
};

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
    const WIDTH_PERCENT: u16 = 60;
    const HEIGHT_PERCENT: u16 = 70;
    const HORIZONTAL_PADDING: u16 = 4;
    const VERTICAL_PADDING: u16 = 1;

    /// Renders the splash with `title`, using the color capability from the environment.
    pub fn render(frame: &mut Frame<'_>, title: &str) {
        Self::render_with(frame, ColorSupport::from_env(), title);
    }

    /// Renders the splash with `title` and an explicit color capability.
    pub fn render_with(frame: &mut Frame<'_>, support: ColorSupport, title: &str) {
        let area = Self::centered_area(frame.area(), Self::WIDTH_PERCENT, Self::HEIGHT_PERCENT);
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Theme::border_style())
            .style(Theme::window_style())
            .padding(Padding::symmetric(
                Self::HORIZONTAL_PADDING,
                Self::VERTICAL_PADDING,
            ))
            .title(Span::styled(title.to_string(), Theme::title_style()));
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let lines = Self::body_lines(support);
        let content = Self::vertically_centered(inner, lines.len() as u16);
        frame.render_widget(
            Paragraph::new(lines)
                .alignment(Alignment::Center)
                .style(Theme::window_style()),
            content,
        );
    }

    fn body_lines(support: ColorSupport) -> Vec<Line<'static>> {
        let mut lines = Emblem::lines_for(support);
        lines.push(Line::from(""));
        lines.push(Self::title_line(support));
        lines.push(Line::from(""));
        lines.push(Self::subtitle_line());
        lines.push(Line::from(""));
        lines.push(Self::hint_line());
        lines
    }

    fn vertically_centered(area: Rect, content_height: u16) -> Rect {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(content_height),
                Constraint::Fill(1),
            ])
            .split(area)[1]
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
