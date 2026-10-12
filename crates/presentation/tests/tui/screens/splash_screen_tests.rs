#[cfg(test)]
mod tests {
    use ephact::presentation::tui::{components::ColorSupport, screens::SplashScreen};
    use ratatui::{Terminal, backend::TestBackend};

    use crate::tui::TuiRenderAssertions;

    #[test]
    fn splash_renders_its_keybind_hint() {
        let splash = TuiRenderAssertions::rendered_buffer(|frame| {
            SplashScreen::render_with(
                frame,
                ephact::PROJECT_EMBLEM,
                ColorSupport::Basic,
                "test quote",
            );
        });

        let text = TuiRenderAssertions::buffer_text(&splash);

        assert!(text.contains("press any key to continue"));
    }

    #[test]
    fn splash_preserves_emblem_at_narrow_widths() {
        for width in [40, 30, 20] {
            let backend = TestBackend::new(width, 15);
            let mut terminal = Terminal::new(backend).expect("test terminal");
            terminal
                .draw(|frame| {
                    SplashScreen::render_with(
                        frame,
                        ephact::PROJECT_EMBLEM,
                        ColorSupport::Basic,
                        "test quote",
                    );
                })
                .expect("render splash");

            let buffer = terminal.backend().buffer();
            let first_expected = ephact::PROJECT_EMBLEM.lines().next().unwrap();
            let first_row = (0..15).find(|row| {
                let actual = (0..width)
                    .map(|column| buffer[(column, *row)].symbol())
                    .collect::<String>();
                actual.contains(first_expected)
            });
            let first_row = first_row.expect("find first emblem row");
            for (offset, expected) in ephact::PROJECT_EMBLEM.lines().enumerate() {
                let row = first_row + offset as u16;
                assert!(row < 15, "width {width}: emblem exceeds terminal height");
                let actual = (0..width)
                    .map(|column| buffer[(column, row)].symbol())
                    .collect::<String>();
                assert!(actual.contains(expected), "width {width}: {actual:?}");
            }
        }
    }
}
