#[cfg(test)]
mod tests {
    use std::fs::File;

    use ephact::{
        domain::value_objects::Marker,
        presentation::tui::components::{ColorSupport, Emblem},
    };
    use image::{ImageBuffer, Rgba, codecs::gif::GifEncoder};
    use ratatui::{Terminal, backend::TestBackend, layout::Rect};
    use tempfile::tempdir;

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
        let marker_area = Rect::new(5, 2, 2, 1);
        let cell = &terminal.backend().buffer()[(marker_area.x, marker_area.y)];

        assert_ne!(cell, &ratatui::buffer::Cell::EMPTY);
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
        let marker_area = Rect::new(5, 2, 2, 1);
        let cell = &terminal.backend().buffer()[(marker_area.x, marker_area.y)];

        assert_ne!(cell, &ratatui::buffer::Cell::EMPTY);
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
