#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use ephact::{
        application::dtos::responses::{RunInputDeclarationResponse, RunInputSourceResponse},
        presentation::tui::{
            components::{Renderable, ScreenFrame},
            screens::RunActionScreen,
        },
    };
    use ratatui::{Terminal, backend::TestBackend};

    fn rendered_text(screen: &RunActionScreen) -> String {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).expect("test terminal");

        terminal
            .draw(|frame| {
                let area = ScreenFrame.render(frame, frame.area(), "test quote");
                screen.render(frame, area);
            })
            .expect("render screen");

        terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|cell| cell.symbol())
            .collect()
    }

    #[test]
    fn action_screen_prefills_declared_defaults() {
        let mut screen = RunActionScreen::new("./.github/actions/demo".to_owned());
        screen.begin_configuration(vec![RunInputDeclarationResponse::new(
            "greeting".to_owned(),
            RunInputSourceResponse::Action("./.github/actions/demo".to_owned()),
            None,
            false,
            Some("hello".to_owned()),
        )]);

        assert!(rendered_text(&screen).contains("Input: greeting = hello"));
    }

    #[test]
    fn action_screen_rejects_missing_required_input() {
        let mut screen = RunActionScreen::new("./.github/actions/demo".to_owned());
        screen.begin_configuration(vec![RunInputDeclarationResponse::new(
            "token".to_owned(),
            RunInputSourceResponse::Action("./.github/actions/demo".to_owned()),
            None,
            true,
            None,
        )]);

        screen.handle_configuration_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        screen.handle_configuration_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE));

        assert!(rendered_text(&screen).contains("Error: token is required"));
    }

    #[test]
    fn action_screen_renders_status_and_scrollable_details() {
        let mut screen = RunActionScreen::new("./.github/actions/demo".to_owned());
        screen.record_outcome(
            ephact::application::dtos::responses::ExecuteActionResponse::builder()
                .exit_code(2)
                .stdout("output".to_owned())
                .stderr("failure log".to_owned())
                .build(),
        );

        assert!(rendered_text(&screen).contains("Status: FAILED"));
        assert!(rendered_text(&screen).contains("Exit code: 2"));
        screen.open_details();
        let details = rendered_text(&screen);
        assert!(details.contains("stdout:"));
        assert!(details.contains("failure log"));
    }
}
