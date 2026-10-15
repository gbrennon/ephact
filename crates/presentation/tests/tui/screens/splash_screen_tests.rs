#[cfg(test)]
mod tests {
    use ephact::presentation::tui::{components::ColorSupport, screens::SplashScreen};
    use ratatui::{Terminal, backend::TestBackend};

    use crate::tui::tests::TuiRenderAssertions;

    #[test]
    fn splash_renders_its_keybind_hint() {
        let splash = TuiRenderAssertions::rendered_buffer(|frame| {
            SplashScreen::render_with(
                frame,
                ephact::PROJECT_EMBLEM,
                ColorSupport::Basic,
                "test quote",
                "press any key to continue",
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
                        "press any key to continue",
                    );
                })
                .expect("render splash");

            let buffer = terminal.backend().buffer();
            let first_expected = ephact::PROJECT_EMBLEM.lines().next().unwrap().trim_start();
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
                assert!(
                    actual.contains(expected.trim_start()),
                    "width {width}: {actual:?}"
                );
            }
        }
    }

    #[test]
    fn splash_renders_asset_geometry_without_row_drift() {
        let width = 190;
        let backend = TestBackend::new(width, 40);
        let mut terminal = Terminal::new(backend).expect("test terminal");
        terminal
            .draw(|frame| {
                SplashScreen::render_with(
                    frame,
                    ephact::PROJECT_EMBLEM,
                    ColorSupport::Basic,
                    "test quote",
                    "press any key to continue",
                );
            })
            .expect("render splash");

        let buffer = terminal.backend().buffer();
        let first_expected = ephact::PROJECT_EMBLEM.lines().next().unwrap();
        let first_row = (0..40)
            .find(|row| {
                let actual = (0..width)
                    .map(|column| buffer[(column, *row)].symbol())
                    .collect::<String>();
                actual.contains(first_expected.trim_start())
            })
            .expect("find first emblem row");
        let row_offsets = ephact::PROJECT_EMBLEM
            .lines()
            .enumerate()
            .map(|(offset, expected)| {
                let actual = (0..width)
                    .map(|column| buffer[(column, first_row + offset as u16)].symbol())
                    .collect::<String>();
                let visible = expected.trim_start();
                let actual_start = actual.find(visible).expect("find rendered emblem row");
                let expected_start = expected
                    .chars()
                    .take_while(|glyph| glyph.is_whitespace())
                    .count();

                actual_start as isize - expected_start as isize
            })
            .collect::<Vec<_>>();

        assert!(
            row_offsets.windows(2).all(|pair| pair[0] == pair[1]),
            "emblem rows drifted relative to the asset: {row_offsets:?}"
        );
    }

    #[test]
    fn splash_centers_title_on_emblem_axis() {
        let width = 190;
        let backend = TestBackend::new(width, 40);
        let mut terminal = Terminal::new(backend).expect("test terminal");
        terminal
            .draw(|frame| {
                SplashScreen::render_with(
                    frame,
                    ephact::PROJECT_EMBLEM,
                    ColorSupport::Basic,
                    "test quote",
                    "press any key to continue",
                );
            })
            .expect("render splash");

        let buffer = terminal.backend().buffer();
        let emblem_line = ephact::PROJECT_EMBLEM.lines().next().unwrap().trim_start();
        let emblem_row = (0..40)
            .find_map(|row| {
                let actual = (0..width)
                    .map(|column| buffer[(column, row)].symbol())
                    .collect::<String>();
                actual.find(emblem_line).map(|start| (actual, start))
            })
            .expect("find rendered emblem");
        let title_start = (0..40)
            .find_map(|row| {
                let actual = (0..width)
                    .map(|column| buffer[(column, row)].symbol())
                    .collect::<String>();
                actual.find("EPHACT")
            })
            .expect("find rendered title");

        let emblem_center = 2 * emblem_row.1 + emblem_line.chars().count() - 1;
        let title_center = 2 * title_start + "EPHACT".chars().count() - 1;
        assert_eq!(title_center, emblem_center);
    }
}
