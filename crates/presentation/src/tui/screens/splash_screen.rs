use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph},
};

use crate::{
    domain::value_objects::Marker,
    tui::{
        components::{ColorSupport, Emblem},
        theme::Theme,
    },
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
    pub fn render(frame: &mut Frame<'_>, emblem: &str, title: &str) {
        Self::render_with_marker(
            frame,
            emblem,
            ColorSupport::from_env(),
            title,
            &Marker::default(),
        );
    }

    /// Renders the splash with `title` and an explicit color capability.
    pub fn render_with(frame: &mut Frame<'_>, emblem: &str, support: ColorSupport, title: &str) {
        Self::render_with_marker(frame, emblem, support, title, &Marker::default());
    }

    /// Renders the splash with the configured marker and terminal capability.
    pub fn render_with_marker(
        frame: &mut Frame<'_>,
        emblem: &str,
        support: ColorSupport,
        title: &str,
        marker: &Marker,
    ) {
        let lines = Self::body_lines(emblem, support, marker);
        let emblem_width = Emblem::new(emblem).width() as u16;
        let area = Self::centered_area(
            frame.area(),
            Self::WIDTH_PERCENT,
            Self::HEIGHT_PERCENT,
            emblem_width
                .saturating_add(2)
                .saturating_add(Self::HORIZONTAL_PADDING * 2),
            (lines.len() as u16)
                .saturating_add(2)
                .saturating_add(Self::VERTICAL_PADDING * 2),
        );
        let horizontal_padding = Self::horizontal_padding(area.width, emblem_width);
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Theme::border_style())
            .style(Theme::window_style())
            .padding(Padding::symmetric(
                horizontal_padding,
                Self::VERTICAL_PADDING,
            ))
            .title(Span::styled(title.to_string(), Theme::title_style()));
        let inner = block.inner(area);
        frame.render_widget(block, area);
        let content = Self::vertically_centered(inner, lines.len() as u16);
        frame.render_widget(
            Paragraph::new(lines)
                .alignment(Alignment::Center)
                .style(Theme::window_style()),
            content,
        );
        Emblem::new(emblem).render_graphical_marker(frame, content, marker, support);
    }

    fn body_lines(emblem: &str, support: ColorSupport, marker: &Marker) -> Vec<Line<'static>> {
        let mut lines = Emblem::new(emblem).lines_for(support, marker);
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

    fn centered_area(
        area: Rect,
        width_percent: u16,
        height_percent: u16,
        minimum_width: u16,
        minimum_height: u16,
    ) -> Rect {
        let width = (area.width * width_percent / 100)
            .max(minimum_width)
            .min(area.width);
        let height = (area.height * height_percent / 100)
            .max(minimum_height)
            .min(area.height);
        Rect::new(
            area.x + (area.width - width) / 2,
            area.y + (area.height - height) / 2,
            width,
            height,
        )
    }

    fn horizontal_padding(area_width: u16, content_width: u16) -> u16 {
        Self::HORIZONTAL_PADDING.min(area_width.saturating_sub(2).saturating_sub(content_width) / 2)
    }
}
