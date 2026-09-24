#[cfg(test)]
mod tests {
    use ephact::presentation::tui::{components::ColorSupport, screens::SplashScreen};

    use crate::tui::TuiRenderAssertions;

    #[test]
    fn splash_renders_its_keybind_hint() {
        let splash = TuiRenderAssertions::rendered_buffer(|frame| {
            SplashScreen::render_with(frame, ColorSupport::Basic, "test quote");
        });

        let text = TuiRenderAssertions::buffer_text(&splash);

        assert!(text.contains("press any key to continue"));
    }
}
