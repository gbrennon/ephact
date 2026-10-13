use unicode_width::UnicodeWidthStr;

/// Geometry shared by CLI and TUI rendering of the project emblem.
pub struct EmblemLayout<'a> {
    text: &'a str,
}

impl<'a> EmblemLayout<'a> {
    /// Creates layout geometry from the canonical emblem text.
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    /// Returns the visible emblem width after removing its common left gutter.
    pub fn width(&self) -> usize {
        self.text
            .lines()
            .map(str::width)
            .max()
            .unwrap_or(0)
            .saturating_sub(self.gutter())
    }

    /// Returns the common leading whitespace shared by all emblem rows.
    pub fn gutter(&self) -> usize {
        self.text
            .lines()
            .map(|line| {
                line.chars()
                    .take_while(|glyph| glyph.is_whitespace())
                    .count()
            })
            .min()
            .unwrap_or(0)
    }

    /// Returns the width needed for the emblem plus a fixed-width inline slot.
    pub fn width_with_slot(&self, slot_width: usize) -> usize {
        self.width().saturating_add(slot_width.saturating_sub(1))
    }

    /// Removes the shared gutter and pads one row to `width` display columns.
    pub fn normalize_line(&self, line: &str, width: usize) -> String {
        let trimmed = line.chars().skip(self.gutter()).collect::<String>();
        let padding = width.saturating_sub(trimmed.width());
        format!("{trimmed}{}", " ".repeat(padding))
    }

    /// Returns all emblem rows with a common visible width.
    pub fn lines(&self) -> Vec<String> {
        let width = self.width();
        self.text
            .lines()
            .map(|line| self.normalize_line(line, width))
            .collect()
    }
}
