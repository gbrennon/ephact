#[cfg(test)]
mod tests {
    use ephact::presentation::tui::screens::HomeScreen;

    use crate::tui::TuiRenderAssertions;

    #[test]
    fn home_renders_keybind_footer() {
        let home = TuiRenderAssertions::rendered_screen(|frame, area| {
            HomeScreen::render(frame, area, 0);
        });

        let text = TuiRenderAssertions::buffer_text(&home);

        assert!(text.contains("Up/Down/j/k: Move"));
        assert!(text.contains("Enter: Select"));
        assert!(text.contains("q: Quit"));
    }
}
