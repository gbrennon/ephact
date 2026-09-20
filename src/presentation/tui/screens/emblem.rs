use ratatui::{
    style::Style,
    text::{Line, Span},
};

use super::color_support::ColorSupport;
use crate::presentation::tui::theme::Theme;

/// Plain single-color emblem used when the terminal lacks 24-bit color.
pub const FALLBACK_EMBLEM: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/project_emblem.txt"
));

/// Builder that renders the project emblem as terminal lines.
///
/// Callers receive fancy 24-bit colored ASCII art when the terminal supports
/// true color, and a single-color rendering of the fallback art otherwise.
pub struct Emblem;

impl Emblem {
    /// Returns emblem lines appropriate for the given color capability.
    pub fn lines_for(support: ColorSupport) -> Vec<Line<'static>> {
        if support.is_true_color() {
            return Self::fancy_lines();
        }
        Self::fallback_lines()
    }

    /// Returns the fallback emblem styled with a single accent color.
    pub fn fallback_lines() -> Vec<Line<'static>> {
        FALLBACK_EMBLEM.lines().map(Self::fallback_line).collect()
    }

    /// Returns the colored emblem lines that use the active-accent palette.
    pub fn fancy_lines() -> Vec<Line<'static>> {
        vec![
            Self::frame_line("◈─────────◈", Theme::active_style()),
            Self::frame_line("╱  ▟▛▜▙  ╲", Theme::node_style()),
            Self::prompt_line(),
            Self::name_line(),
            Self::frame_line("╲  ▜▙▟▛  ╱", Theme::node_style()),
            Self::frame_line("◈─────────◈", Theme::active_style()),
        ]
    }

    fn fallback_line(text: &str) -> Line<'static> {
        Line::from(Span::styled(text.to_string(), Theme::active_style()))
    }

    fn frame_line(text: &'static str, style: Style) -> Line<'static> {
        Line::from(Span::styled(text, style))
    }

    fn prompt_line() -> Line<'static> {
        Line::from(vec![
            Span::styled("◈  ", Theme::node_style()),
            Span::styled("▸ _", Theme::prompt_style()),
            Span::styled("  ◈", Theme::node_style()),
        ])
    }

    fn name_line() -> Line<'static> {
        Line::from(vec![
            Span::styled("◈ ", Theme::node_style()),
            Span::styled("ephact", Theme::wordmark_style()),
            Span::styled(" ◈", Theme::node_style()),
        ])
    }
}

#[test]
fn lines_for_basic_uses_fallback_art() {
    let lines = Emblem::lines_for(ColorSupport::Basic);

    let text: String = lines
        .iter()
        .flat_map(|line| line.spans.iter().map(|span| span.content.as_ref()))
        .collect();
    assert!(text.contains("@---o"));
}

#[test]
fn lines_for_true_color_uses_rgb_styles() {
    use ratatui::style::Color;

    let lines = Emblem::lines_for(ColorSupport::TrueColor);

    let uses_rgb = lines
        .iter()
        .flat_map(|line| line.spans.iter())
        .any(|span| matches!(span.style.fg, Some(Color::Rgb(_, _, _))));
    assert!(uses_rgb);
}

#[test]
fn fancy_lines_include_project_name() {
    let lines = Emblem::fancy_lines();

    let text: String = lines
        .iter()
        .flat_map(|line| line.spans.iter().map(|span| span.content.as_ref()))
        .collect();
    assert!(text.contains("ephact"));
}
