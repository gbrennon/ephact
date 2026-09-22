use std::env;

/// Terminal color capability used to select the emblem presentation.
///
/// `TrueColor` guarantees the terminal advertises 24-bit color, so callers may
/// render RGB gradients. `Basic` guarantees only limited color, so callers must
/// fall back to a single named color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSupport {
    TrueColor,
    Basic,
}

impl ColorSupport {
    /// Detects the color capability from the current process environment.
    ///
    /// Reads `COLORTERM` and `TERM` and returns the capability the terminal
    /// advertises without mutating any global state.
    pub fn from_env() -> Self {
        Self::detect(
            env::var("COLORTERM").ok().as_deref(),
            env::var("TERM").ok().as_deref(),
        )
    }

    /// Derives the color capability from raw `COLORTERM` and `TERM` values.
    ///
    /// Returns `TrueColor` when either variable advertises 24-bit color and
    /// `Basic` otherwise.
    pub fn detect(colorterm: Option<&str>, term: Option<&str>) -> Self {
        if Self::colorterm_is_true_color(colorterm) || Self::term_is_true_color(term) {
            return Self::TrueColor;
        }
        Self::Basic
    }

    /// Reports whether the terminal supports 24-bit color.
    pub fn is_true_color(self) -> bool {
        matches!(self, Self::TrueColor)
    }

    fn colorterm_is_true_color(colorterm: Option<&str>) -> bool {
        matches!(colorterm, Some(value) if value.contains("truecolor") || value.contains("24bit"))
    }

    fn term_is_true_color(term: Option<&str>) -> bool {
        matches!(term, Some(value) if value.contains("truecolor") || value.contains("direct"))
    }
}

#[test]
fn detect_truecolor_colorterm_reports_true_color() {
    let support = ColorSupport::detect(Some("truecolor"), Some("xterm-256color"));

    assert_eq!(support, ColorSupport::TrueColor);
}

#[test]
fn detect_24bit_colorterm_reports_true_color() {
    let support = ColorSupport::detect(Some("24bit"), None);

    assert_eq!(support, ColorSupport::TrueColor);
}

#[test]
fn detect_direct_term_reports_true_color() {
    let support = ColorSupport::detect(None, Some("xterm-direct"));

    assert_eq!(support, ColorSupport::TrueColor);
}

#[test]
fn detect_without_true_color_hints_reports_basic() {
    let support = ColorSupport::detect(None, Some("xterm-256color"));

    assert_eq!(support, ColorSupport::Basic);
}

#[test]
fn is_true_color_reflects_variant() {
    assert!(ColorSupport::TrueColor.is_true_color());
    assert!(!ColorSupport::Basic.is_true_color());
}
