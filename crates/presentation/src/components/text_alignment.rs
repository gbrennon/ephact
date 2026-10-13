use unicode_width::UnicodeWidthStr;

/// Horizontal alignment used by terminal presentation components.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TextAlignment {
    /// Keep content at the leading edge of its available width.
    #[default]
    Start,
    /// Place content at the center of its available width.
    Center,
}

impl TextAlignment {
    /// Returns the leading padding needed to align content within `container_width`.
    pub fn offset(self, container_width: usize, content_width: usize) -> usize {
        match self {
            Self::Start => 0,
            Self::Center => container_width.saturating_sub(content_width) / 2,
        }
    }

    /// Returns `text` aligned within `container_width` using terminal display widths.
    pub fn line(self, text: &str, container_width: usize) -> String {
        let text = match self {
            Self::Start => text,
            Self::Center => text.trim(),
        };
        let content_width = text.width();
        let leading = self.offset(container_width, content_width);
        let trailing = container_width.saturating_sub(leading + content_width);
        format!("{}{}{}", " ".repeat(leading), text, " ".repeat(trailing))
    }
}
