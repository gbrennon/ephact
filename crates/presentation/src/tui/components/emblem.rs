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
use crate::{domain::value_objects::Marker, tui::theme::Theme};

/// Plain single-color emblem used when the terminal lacks 24-bit color.
pub const FALLBACK_EMBLEM: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../assets/project_emblem.txt"
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
