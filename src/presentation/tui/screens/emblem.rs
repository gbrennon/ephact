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

const PROMPT_GLYPHS: [char; 2] = ['>', '_'];
const NODE_GLYPHS: [char; 2] = ['@', 'o'];

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

    /// Returns the colored canonical emblem lines using the active-accent palette.
    pub fn fancy_lines() -> Vec<Line<'static>> {
        FALLBACK_EMBLEM.lines().map(Self::fancy_line).collect()
    }

    fn fallback_line(text: &str) -> Line<'static> {
        Line::from(Span::styled(Self::padded(text), Theme::active_style()))
    }

    fn fancy_line(text: &str) -> Line<'static> {
        Line::from(
            Self::padded(text)
                .chars()
                .map(Self::fancy_span)
                .collect::<Vec<Span<'static>>>(),
        )
    }

    fn padded(text: &str) -> String {
        let gutter = Self::left_gutter();
        let width = Self::emblem_width().saturating_sub(gutter);
        let trimmed = text.get(gutter..).unwrap_or("");
        format!("{trimmed:<width$}")
    }

    fn emblem_width() -> usize {
        FALLBACK_EMBLEM.lines().map(str::len).max().unwrap_or(0)
    }

    fn left_gutter() -> usize {
        FALLBACK_EMBLEM
            .lines()
            .map(|line| line.len() - line.trim_start().len())
            .min()
            .unwrap_or(0)
    }

    fn fancy_span(glyph: char) -> Span<'static> {
        Span::styled(glyph.to_string(), Self::glyph_style(glyph))
    }

    fn glyph_style(glyph: char) -> Style {
        if PROMPT_GLYPHS.contains(&glyph) {
            return Theme::prompt_style();
        }
        if NODE_GLYPHS.contains(&glyph) {
            return Theme::node_style();
        }
        Theme::active_style()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn fancy_lines_render_canonical_emblem_art() {
        let lines = Emblem::fancy_lines();

        let text: String = lines
            .iter()
            .flat_map(|line| line.spans.iter().map(|span| span.content.as_ref()))
            .collect();
        assert!(text.contains("@---o"));
        assert!(!text.contains("ephact"));
    }
}
