use ratatui::style::{Color, Modifier, Style};

/// Ghost in the Shell (1995) inspired color theme for every TUI surface.
///
/// Each constant and style is named for the role it plays in the interface so
/// callers express intent instead of raw palette slots.
pub struct Theme;

impl Theme {
    pub const WINDOW_BACKGROUND: Color = Color::Rgb(8, 11, 16);
    pub const PANEL_BACKGROUND: Color = Color::Rgb(22, 28, 38);
    pub const BORDER: Color = Color::Rgb(110, 132, 158);
    pub const TEXT_PRIMARY: Color = Color::Rgb(234, 241, 247);
    pub const TEXT_MUTED: Color = Color::Rgb(162, 179, 195);
    pub const ACTIVE_ACCENT: Color = Color::Rgb(72, 245, 212);
    pub const STRUCTURE_ACCENT: Color = Color::Rgb(58, 130, 232);
    pub const SIGNAL_ACCENT: Color = Color::Rgb(255, 148, 51);
    pub const CRITICAL: Color = Color::Rgb(239, 71, 84);
    pub const SUBTLE_HIGHLIGHT: Color = Color::Rgb(150, 112, 196);

    /// Base style for the window and any surface without a dedicated role.
    pub fn window_style() -> Style {
        Style::default()
            .fg(Self::TEXT_PRIMARY)
            .bg(Self::WINDOW_BACKGROUND)
    }

    /// Style for panel borders and separators.
    pub fn border_style() -> Style {
        Style::default()
            .fg(Self::BORDER)
            .bg(Self::WINDOW_BACKGROUND)
    }

    /// Style for panel and screen titles that must stand out from the field.
    pub fn title_style() -> Style {
        Style::default()
            .fg(Self::SIGNAL_ACCENT)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for section headers grouping related content.
    pub fn section_header_style() -> Style {
        Style::default()
            .fg(Self::ACTIVE_ACCENT)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for the currently selected list row or menu item.
    ///
    /// Fills the focused row with the signal accent so it separates sharply
    /// from the dark field and the cyan and blue used elsewhere.
    pub fn selection_style() -> Style {
        Style::default()
            .bg(Self::SIGNAL_ACCENT)
            .fg(Self::WINDOW_BACKGROUND)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for primary body text.
    pub fn body_style() -> Style {
        Style::default().fg(Self::TEXT_PRIMARY)
    }

    /// Style for secondary or de-emphasized text such as footers and hints.
    pub fn muted_style() -> Style {
        Style::default().fg(Self::TEXT_MUTED)
    }

    /// Style for successful or active state indicators.
    pub fn success_style() -> Style {
        Style::default()
            .fg(Self::ACTIVE_ACCENT)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for the primary attention accent that breaks the cyan and blue field.
    pub fn signal_style() -> Style {
        Style::default()
            .fg(Self::SIGNAL_ACCENT)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for errors and critical state.
    pub fn critical_style() -> Style {
        Style::default()
            .fg(Self::CRITICAL)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for active, digital-feeling accents such as the emblem frame.
    pub fn active_style() -> Style {
        Style::default().fg(Self::ACTIVE_ACCENT)
    }

    /// Style for structural emblem nodes and connectors.
    pub fn node_style() -> Style {
        Style::default().fg(Self::STRUCTURE_ACCENT)
    }

    /// Style for the emblem command prompt glyph.
    pub fn prompt_style() -> Style {
        Style::default()
            .fg(Self::SIGNAL_ACCENT)
            .add_modifier(Modifier::BOLD)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_style_uses_signal_accent_background() {
        let style = Theme::selection_style();

        assert_eq!(style.bg, Some(Theme::SIGNAL_ACCENT));
    }

    #[test]
    fn selection_style_uses_dark_text_for_legibility() {
        let style = Theme::selection_style();

        assert_eq!(style.fg, Some(Theme::WINDOW_BACKGROUND));
    }

    #[test]
    fn title_style_uses_signal_accent() {
        let style = Theme::title_style();

        assert_eq!(style.fg, Some(Theme::SIGNAL_ACCENT));
    }

    #[test]
    fn signal_style_uses_signal_accent() {
        let style = Theme::signal_style();

        assert_eq!(style.fg, Some(Theme::SIGNAL_ACCENT));
    }

    #[test]
    fn muted_style_uses_muted_text() {
        let style = Theme::muted_style();

        assert_eq!(style.fg, Some(Theme::TEXT_MUTED));
    }
}
