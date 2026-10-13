use image::ImageReader;
use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::{Line, Span},
};
use ratatui_image::{Image, Resize, picker::Picker};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::{
    components::{EmblemLayout, TextAlignment},
    domain::value_objects::Marker,
    tui::{components::color_support::ColorSupport, theme::Theme},
};

/// Canonical project emblem loaded from the repository asset.
pub const PROJECT_EMBLEM: &str = include_str!("../../../../../assets/project_emblem.txt");

pub struct Emblem<'a> {
    text: &'a str,
}

impl<'a> Emblem<'a> {
    const PROMPT_GLYPHS: [char; 2] = ['>', '_'];
    const NODE_GLYPHS: [char; 2] = ['@', 'o'];
    const MARKER_GLYPH: char = '_';
    const MARKER_SLOT_WIDTH: usize = 2;
    const MARKER_LINE_INDEX: u16 = 2;
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    pub fn width(&self) -> usize {
        self.emblem_width()
    }

    pub fn lines_for(&self, support: ColorSupport, marker: &Marker) -> Vec<Line<'static>> {
        match support.is_true_color() {
            true => self.fancy_lines(marker),
            false => self.fallback_lines(marker),
        }
    }

    pub fn render_graphical_marker(
        &self,
        frame: &mut Frame<'_>,
        area: Rect,
        marker: &Marker,
        support: ColorSupport,
    ) -> bool {
        let path = match (support.is_true_color(), marker) {
            (true, Marker::ImagePath(_) | Marker::GifPath(_)) => marker.value(),
            _ => return false,
        };
        let Ok(reader) = ImageReader::open(path) else {
            return false;
        };
        let Ok(image) = reader.decode() else {
            return false;
        };
        let picker = Picker::from_fontsize((8, 16));
        let image_area = self.marker_area(area);
        let Ok(protocol) = picker.new_protocol(image, image_area, Resize::Fit(None)) else {
            return false;
        };
        frame.render_widget(Image::new(&protocol), image_area);
        true
    }

    fn marker_area(&self, area: Rect) -> Rect {
        let line = self
            .text
            .lines()
            .nth(Self::MARKER_LINE_INDEX as usize)
            .unwrap_or("");
        let layout = EmblemLayout::new(self.text);
        let gutter = layout.gutter();
        let marker_column = line
            .find(Self::MARKER_GLYPH)
            .map(|index| line[..index].chars().count().saturating_sub(gutter))
            .unwrap_or(0);
        let emblem_width = self.emblem_width() as u16;
        let emblem_offset =
            TextAlignment::Center.offset(area.width as usize, emblem_width as usize) as u16;
        Rect::new(
            area.x
                .saturating_add(emblem_offset)
                .saturating_add(marker_column as u16),
            area.y.saturating_add(Self::MARKER_LINE_INDEX),
            Self::MARKER_SLOT_WIDTH as u16,
            1,
        )
    }

    pub fn fallback_lines(&self, marker: &Marker) -> Vec<Line<'static>> {
        self.text
            .lines()
            .map(|line| Self::fallback_line(&self.padded(line, marker)))
            .collect()
    }

    pub fn fancy_lines(&self, marker: &Marker) -> Vec<Line<'static>> {
        self.text
            .lines()
            .map(|line| self.fancy_line(line, marker))
            .collect()
    }

    fn fallback_line(text: &str) -> Line<'static> {
        Line::from(Span::styled(text.to_string(), Theme::active_style()))
    }

    fn fancy_line(&self, text: &str, marker: &Marker) -> Line<'static> {
        let (line, marker_start) = self.padded_with_marker(text, marker);
        Line::from(
            line.chars()
                .enumerate()
                .map(|(index, glyph)| {
                    match marker_start.is_some_and(|start| {
                        index >= start && index < start + Self::MARKER_SLOT_WIDTH
                    }) {
                        true => Span::styled(glyph.to_string(), Theme::prompt_style()),
                        false => Self::fancy_span(glyph),
                    }
                })
                .collect::<Vec<Span<'static>>>(),
        )
    }

    fn padded(&self, text: &str, marker: &Marker) -> String {
        self.padded_with_marker(text, marker).0
    }

    fn padded_with_marker(&self, text: &str, marker: &Marker) -> (String, Option<usize>) {
        let layout = EmblemLayout::new(self.text);
        let source = Self::replace_marker(text, marker);
        let marker_start = source.1.map(|start| start.saturating_sub(layout.gutter()));
        let line = layout.normalize_line(&source.0, self.emblem_width());
        (line, marker_start)
    }

    fn replace_marker(text: &str, marker: &Marker) -> (String, Option<usize>) {
        let Some(byte_index) = text.find(Self::MARKER_GLYPH) else {
            return (text.to_owned(), None);
        };
        let prefix = &text[..byte_index];
        let suffix = &text[byte_index + Self::MARKER_GLYPH.len_utf8()..];
        let marker_text = Self::marker_slot(marker);
        (
            format!("{prefix}{marker_text}{suffix}"),
            Some(prefix.chars().count()),
        )
    }

    fn marker_slot(marker: &Marker) -> String {
        if marker.as_text() == "_" {
            return "_".to_owned();
        }
        let slot = marker
            .as_text()
            .chars()
            .scan(0usize, |width, glyph| {
                let glyph_width = glyph.width().unwrap_or(0);
                (*width + glyph_width <= Self::MARKER_SLOT_WIDTH).then(|| {
                    *width += glyph_width;
                    glyph
                })
            })
            .collect::<String>();
        let slot = match slot.is_empty() {
            true => Self::MARKER_GLYPH.to_string(),
            false => slot,
        };
        let width = slot.width();
        format!("{slot}{}", " ".repeat(Self::MARKER_SLOT_WIDTH - width))
    }

    fn emblem_width(&self) -> usize {
        EmblemLayout::new(self.text).width_with_slot(Self::MARKER_SLOT_WIDTH)
    }

    fn fancy_span(glyph: char) -> Span<'static> {
        Span::styled(glyph.to_string(), Self::glyph_style(glyph))
    }

    fn glyph_style(glyph: char) -> Style {
        match glyph {
            glyph if Self::PROMPT_GLYPHS.contains(&glyph) => Theme::prompt_style(),
            glyph if Self::NODE_GLYPHS.contains(&glyph) => Theme::node_style(),
            _ => Theme::active_style(),
        }
    }
}
