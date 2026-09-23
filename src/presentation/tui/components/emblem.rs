use image::ImageReader;
use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::{Line, Span},
};
use ratatui_image::{Image, Resize, picker::Picker};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use super::color_support::ColorSupport;
use crate::{domain::value_objects::Marker, presentation::tui::theme::Theme};

/// Plain single-color emblem used when the terminal lacks 24-bit color.
pub const FALLBACK_EMBLEM: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/project_emblem.txt"
));

const PROMPT_GLYPHS: [char; 2] = ['>', '_'];
const NODE_GLYPHS: [char; 2] = ['@', 'o'];
const MARKER_GLYPH: char = '_';
const MARKER_SLOT_WIDTH: usize = 2;
const MARKER_LINE_INDEX: u16 = 2;

/// Builder that renders the project emblem as terminal lines.
///
/// Callers receive fancy 24-bit colored ASCII art when the terminal supports
/// true color, and a single-color rendering of the fallback art otherwise.
pub struct Emblem;

impl Emblem {
    /// Returns emblem lines appropriate for the given color capability.
    pub fn lines_for(support: ColorSupport, marker: &Marker) -> Vec<Line<'static>> {
        if support.is_true_color() {
            return Self::fancy_lines(marker);
        }
        Self::fallback_lines(marker)
    }

    /// Returns the fallback emblem styled with a single accent color.
    pub fn render_graphical_marker(
        frame: &mut Frame<'_>,
        area: Rect,
        marker: &Marker,
        support: ColorSupport,
    ) -> bool {
        if !support.is_true_color() || !matches!(marker, Marker::ImagePath(_) | Marker::GifPath(_))
        {
            return false;
        }
        let path = marker.value();
        let Ok(reader) = ImageReader::open(path) else {
            return false;
        };
        let Ok(image) = reader.decode() else {
            return false;
        };
        let picker = Picker::from_fontsize((8, 16));
        let image_area = Self::marker_area(area);
        let Ok(protocol) = picker.new_protocol(image, image_area, Resize::Fit(None)) else {
            return false;
        };
        frame.render_widget(Image::new(&protocol), image_area);
        true
    }

    fn marker_area(area: Rect) -> Rect {
        let line = FALLBACK_EMBLEM
            .lines()
            .nth(MARKER_LINE_INDEX as usize)
            .unwrap_or("");
        let gutter = Self::left_gutter();
        let marker_column = line
            .find(MARKER_GLYPH)
            .map(|index| line[..index].chars().count().saturating_sub(gutter))
            .unwrap_or(0);
        let emblem_width = Self::emblem_width() as u16;
        Rect::new(
            area.x
                .saturating_add(area.width.saturating_sub(emblem_width) / 2)
                .saturating_add(marker_column as u16),
            area.y.saturating_add(MARKER_LINE_INDEX),
            MARKER_SLOT_WIDTH as u16,
            1,
        )
    }

    pub fn fallback_lines(marker: &Marker) -> Vec<Line<'static>> {
        FALLBACK_EMBLEM
            .lines()
            .map(|line| Self::fallback_line(line, marker))
            .collect()
    }

    /// Returns the colored canonical emblem lines using the active-accent palette.
    pub fn fancy_lines(marker: &Marker) -> Vec<Line<'static>> {
        FALLBACK_EMBLEM
            .lines()
            .map(|line| Self::fancy_line(line, marker))
            .collect()
    }

    fn fallback_line(text: &str, marker: &Marker) -> Line<'static> {
        Line::from(Span::styled(
            Self::padded(text, marker),
            Theme::active_style(),
        ))
    }

    fn fancy_line(text: &str, marker: &Marker) -> Line<'static> {
        let (line, marker_start) = Self::padded_with_marker(text, marker);
        Line::from(
            line.chars()
                .enumerate()
                .map(|(index, glyph)| {
                    if marker_start
                        .is_some_and(|start| index >= start && index < start + MARKER_SLOT_WIDTH)
                    {
                        Span::styled(glyph.to_string(), Theme::prompt_style())
                    } else {
                        Self::fancy_span(glyph)
                    }
                })
                .collect::<Vec<Span<'static>>>(),
        )
    }

    fn padded(text: &str, marker: &Marker) -> String {
        Self::padded_with_marker(text, marker).0
    }

    fn padded_with_marker(text: &str, marker: &Marker) -> (String, Option<usize>) {
        let gutter = Self::left_gutter();
        let source = Self::replace_marker(text, marker);
        let marker_start = source.1.map(|start| start.saturating_sub(gutter));
        let trimmed = source.0.chars().skip(gutter).collect::<String>();
        let width = Self::emblem_width().saturating_sub(gutter);
        let padding = width.saturating_sub(trimmed.width());
        (format!("{trimmed}{}", " ".repeat(padding)), marker_start)
    }

    fn replace_marker(text: &str, marker: &Marker) -> (String, Option<usize>) {
        let Some(byte_index) = text.find(MARKER_GLYPH) else {
            return (text.to_owned(), None);
        };
        let prefix = &text[..byte_index];
        let suffix = &text[byte_index + MARKER_GLYPH.len_utf8()..];
        let marker_text = Self::marker_slot(marker);
        (
            format!("{prefix}{marker_text}{suffix}"),
            Some(prefix.chars().count()),
        )
    }

    fn marker_slot(marker: &Marker) -> String {
        let text = marker.as_text();
        let mut slot = String::new();
        let mut width = 0;
        for glyph in text.chars() {
            let glyph_width = glyph.width().unwrap_or(0);
            if width + glyph_width > MARKER_SLOT_WIDTH {
                break;
            }
            slot.push(glyph);
            width += glyph_width;
        }
        if slot.is_empty() {
            slot.push(MARKER_GLYPH);
            width = 1;
        }
        slot.push_str(&" ".repeat(MARKER_SLOT_WIDTH - width));
        slot
    }

    fn emblem_width() -> usize {
        FALLBACK_EMBLEM
            .lines()
            .map(str::width)
            .max()
            .unwrap_or(0)
            .saturating_add(MARKER_SLOT_WIDTH - 1)
    }

    fn left_gutter() -> usize {
        FALLBACK_EMBLEM
            .lines()
            .map(|line| {
                line.chars()
                    .take_while(|glyph| glyph.is_whitespace())
                    .count()
            })
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
    use std::fs::File;

    use image::{ImageBuffer, Rgba, codecs::gif::GifEncoder};
    use ratatui::{Terminal, backend::TestBackend};
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn lines_for_basic_uses_fallback_art() {
        let lines = Emblem::lines_for(ColorSupport::Basic, &Marker::default());

        let text: String = lines
            .iter()
            .flat_map(|line| line.spans.iter().map(|span| span.content.as_ref()))
            .collect();
        assert!(text.contains("@---o"));
    }

    #[test]
    fn lines_for_true_color_uses_rgb_styles() {
        use ratatui::style::Color;

        let lines = Emblem::lines_for(ColorSupport::TrueColor, &Marker::default());

        let uses_rgb = lines
            .iter()
            .flat_map(|line| line.spans.iter())
            .any(|span| matches!(span.style.fg, Some(Color::Rgb(_, _, _))));
        assert!(uses_rgb);
    }

    #[test]
    fn fancy_lines_render_canonical_emblem_art() {
        let lines = Emblem::fancy_lines(&Marker::default());

        let text: String = lines
            .iter()
            .flat_map(|line| line.spans.iter().map(|span| span.content.as_ref()))
            .collect();
        assert!(text.contains("@---o"));
        assert!(!text.contains("ephact"));
    }

    #[test]
    fn marker_replaces_prompt_without_changing_line_count_or_slot_width() {
        let default_lines = Emblem::fallback_lines(&Marker::default());
        let custom_lines = Emblem::fallback_lines(&Marker::custom_text("🚀"));

        assert_eq!(default_lines.len(), custom_lines.len());
        let default_text = default_lines[2]
            .spans
            .iter()
            .map(|span| span.content.as_ref())
            .collect::<String>();
        let custom_text = custom_lines[2]
            .spans
            .iter()
            .map(|span| span.content.as_ref())
            .collect::<String>();
        assert_eq!(
            unicode_width::UnicodeWidthStr::width(default_text.as_str()),
            unicode_width::UnicodeWidthStr::width(custom_text.as_str())
        );
        assert!(custom_text.contains("🚀"));
    }

    #[test]
    fn graphical_markers_fall_back_to_the_default_prompt_on_basic_terminals() {
        let lines = Emblem::lines_for(ColorSupport::Basic, &Marker::image_path("marker.png"));
        let text = lines[2]
            .spans
            .iter()
            .map(|span| span.content.as_ref())
            .collect::<String>();

        assert!(text.contains("_"));
        assert!(!text.contains("marker.png"));
    }

    #[test]
    fn image_marker_renders_in_fixed_area_when_graphics_are_supported() {
        let directory = tempdir().expect("temporary directory");
        let path = directory.path().join("marker.png");
        ImageBuffer::from_pixel(2, 2, Rgba([255u8, 0, 0, 255]))
            .save(&path)
            .expect("write image");
        let marker = Marker::image_path(path.to_string_lossy());
        let backend = TestBackend::new(10, 5);
        let mut terminal = Terminal::new(backend).expect("test terminal");

        let area = Rect::new(0, 0, 10, 5);
        terminal
            .draw(|frame| {
                assert!(Emblem::render_graphical_marker(
                    frame,
                    area,
                    &marker,
                    ColorSupport::TrueColor,
                ));
            })
            .expect("render image marker");
        let marker_area = Emblem::marker_area(area);
        assert_eq!(marker_area, Rect::new(5, 2, 2, 1));
        let symbol = terminal.backend().buffer()[(marker_area.x, marker_area.y)].symbol();
        assert!(symbol.contains("_G"));
    }

    #[test]
    fn gif_marker_renders_in_fixed_area_when_graphics_are_supported() {
        let directory = tempdir().expect("temporary directory");
        let path = directory.path().join("marker.gif");
        let file = File::create(&path).expect("create gif");
        let mut encoder = GifEncoder::new(file);
        encoder
            .encode_frame(image::Frame::new(ImageBuffer::from_pixel(
                2,
                2,
                Rgba([0u8, 255, 0, 255]),
            )))
            .expect("write gif");
        let marker = Marker::gif_path(path.to_string_lossy());
        let backend = TestBackend::new(10, 5);
        let mut terminal = Terminal::new(backend).expect("test terminal");

        let area = Rect::new(0, 0, 10, 5);
        terminal
            .draw(|frame| {
                assert!(Emblem::render_graphical_marker(
                    frame,
                    area,
                    &marker,
                    ColorSupport::TrueColor,
                ));
            })
            .expect("render gif marker");
        let marker_area = Emblem::marker_area(area);
        assert_eq!(marker_area, Rect::new(5, 2, 2, 1));
        let symbol = terminal.backend().buffer()[(marker_area.x, marker_area.y)].symbol();
        assert!(symbol.contains("_G"));
    }

    #[test]
    fn gif_marker_falls_back_when_graphics_are_unsupported() {
        let marker = Marker::gif_path("marker.gif");
        let backend = TestBackend::new(10, 5);
        let mut terminal = Terminal::new(backend).expect("test terminal");

        terminal
            .draw(|frame| {
                assert!(!Emblem::render_graphical_marker(
                    frame,
                    Rect::new(0, 0, 10, 5),
                    &marker,
                    ColorSupport::Basic,
                ));
            })
            .expect("render fallback");
    }
}
