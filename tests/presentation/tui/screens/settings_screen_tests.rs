#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use ephact::{
        application::dtos::responses::{
            RunInputDeclarationResponse, RunInputSourceResponse, WorkflowListItemResponse,
        },
        domain::{InterfaceMode, Marker, Settings},
        presentation::tui::{TuiApp, TuiScreen, components::ScreenFrame, screens::SettingsScreen},
    };
    use ratatui::{Terminal, backend::TestBackend};

    use crate::fakes::fake_settings_store::FakeSettingsStore;

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn save_key() -> KeyEvent {
        key(KeyCode::Char(char::from(b"s"[0])))
    }

    fn open_settings(app: &mut TuiApp) {
        app.handle_key(key(KeyCode::Enter));
        app.handle_key(key(KeyCode::Down));

        app.handle_key(key(KeyCode::Down));
        app.handle_key(key(KeyCode::Down));
        app.handle_key(key(KeyCode::Enter));
    }

    fn render_app_text(app: &TuiApp) -> String {
        let backend = TestBackend::new(100, 24);
        let mut terminal = Terminal::new(backend).expect("test terminal");
        terminal
            .draw(|frame| app.render(frame))
            .expect("render app");
        terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|cell| cell.symbol())
            .collect()
    }

    fn render_text(screen: &SettingsScreen) -> String {
        let backend = TestBackend::new(100, 24);
        let mut terminal = Terminal::new(backend).expect("test terminal");
        terminal
            .draw(|frame| {
                let area = ScreenFrame::render(frame, "test quote");
                screen.render(frame, area);
            })
            .expect("render settings");
        terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|cell| cell.symbol())
            .collect()
    }

    #[test]
    fn settings_render_inside_themed_frame() {
        let screen = SettingsScreen::new(Settings::default(), None);
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).expect("test terminal");

        terminal
            .draw(|frame| {
                let area = ScreenFrame::render(frame, "test quote");
                screen.render(frame, area);
            })
            .expect("render settings");
        let buffer = terminal.backend().buffer();
        let text: String = buffer.content().iter().map(|cell| cell.symbol()).collect();
        assert_eq!(buffer[(2, 2)].symbol(), "┌");
        assert!(text.contains("test quote"));
        assert!(text.contains("Settings"));
        assert!(text.contains("Up/Down/j/k: Move"));
        assert!(text.contains("Enter: Edit"));
        assert!(text.contains("s: Save"));
        assert!(text.contains("Esc/Bksp: Back"));
        assert!(text.contains("q: Quit"));
    }

    #[test]
    fn editing_settings_renders_editing_keybinds() {
        let mut screen = SettingsScreen::new(Settings::default(), None);
        screen.handle_key(key(KeyCode::Enter));
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).expect("test terminal");

        terminal
            .draw(|frame| {
                let area = ScreenFrame::render(frame, "test quote");
                screen.render(frame, area);
            })
            .expect("render editing settings");

        let row = (0..24)
            .map(|y| {
                (0..80)
                    .map(|x| terminal.backend().buffer()[(x, y)].symbol())
                    .collect::<String>()
            })
            .find(|line| line.contains("default-interface"))
            .expect("interface setting row");

        assert!(row.contains("[tui] cli"));
        assert!(screen.footer().contains("Left/Right: Choose"));
        assert!(screen.footer().contains("Enter: Confirm"));
        assert!(screen.footer().contains("Esc: Cancel"));
        assert!(screen.footer().contains("q: Quit"));
    }

    #[test]
    fn marker_setting_is_the_tenth_row_and_previews_presets() {
        let mut screen = SettingsScreen::new(Settings::default(), None);
        assert!(render_text(&screen).contains("marker = _"));

        for _ in 0..9 {
            screen.handle_key(key(KeyCode::Down));
        }
        screen.handle_key(key(KeyCode::Enter));

        let editing = render_text(&screen);
        assert!(
            editing.contains("marker = > [_] $ # ❯ ➜ ▶ Custom"),
            "{editing}"
        );
        assert!(screen.footer().contains("Left/Right: Choose"));

        screen.handle_key(key(KeyCode::Left));
        assert_eq!(screen.settings().marker().as_text(), ">");
        assert!(!screen.footer().contains("Type: Custom"));
        assert!(render_text(&screen).contains("marker = [>] _ $ # ❯ ➜ ▶ Custom"));
        screen.handle_key(key(KeyCode::Right));
        assert_eq!(screen.settings().marker().as_text(), "_");
        assert!(!screen.footer().contains("Type: Custom"));
    }

    #[test]
    fn marker_custom_editor_accepts_unicode_shows_cursor_and_cancels() {
        let mut screen = SettingsScreen::new(Settings::default(), None);
        for _ in 0..9 {
            screen.handle_key(key(KeyCode::Down));
        }
        screen.handle_key(key(KeyCode::Enter));
        for _ in 0..6 {
            screen.handle_key(key(KeyCode::Right));
        }
        screen.handle_key(key(KeyCode::Char('q')));
        screen.handle_key(key(KeyCode::Char('🚀')));

        assert!(
            render_text(&screen).contains("marker = Custom: q🚀 |"),
            "{}",
            render_text(&screen)
        );
        assert!(screen.footer().contains("Type: Custom"));
        screen.handle_key(key(KeyCode::Backspace));
        screen.handle_key(key(KeyCode::Char('🚀')));
        screen.handle_key(key(KeyCode::Esc));

        assert_eq!(screen.settings().marker(), &Marker::default());
    }

    #[test]
    fn marker_custom_editor_confirms_unicode_value() {
        let mut screen = SettingsScreen::new(Settings::default(), None);
        for _ in 0..9 {
            screen.handle_key(key(KeyCode::Down));
        }
        screen.handle_key(key(KeyCode::Enter));
        for _ in 0..6 {
            screen.handle_key(key(KeyCode::Right));
        }
        screen.handle_key(key(KeyCode::Char('🚀')));
        screen.handle_key(key(KeyCode::Enter));

        assert_eq!(screen.settings().marker().as_text(), "🚀");
    }

    #[test]
    fn settings_is_reachable_from_home_menu() {
        let mut app = TuiApp::new(Vec::new());

        open_settings(&mut app);

        assert_eq!(app.screen(), TuiScreen::Settings);
    }

    #[test]
    fn selected_marker_controls_the_text_input_form() {
        let store = Arc::new(FakeSettingsStore::new(Settings::default()));
        let workflow =
            WorkflowListItemResponse::new(Some("CI".to_string()), None, vec!["push".to_string()]);
        let mut app =
            TuiApp::new(vec![workflow]).with_settings(Settings::default(), Some(store.clone()));
        open_settings(&mut app);
        for _ in 0..9 {
            app.handle_key(key(KeyCode::Down));
        }
        app.handle_key(key(KeyCode::Enter));
        for _ in 0..3 {
            app.handle_key(key(KeyCode::Right));
        }
        app.handle_key(key(KeyCode::Enter));
        app.handle_key(save_key());

        assert_eq!(store.writes()[0].marker().as_text(), "❯");

        for _ in 0..3 {
            app.handle_key(key(KeyCode::Up));
        }
        app.handle_key(key(KeyCode::Enter));
        app.begin_run_configuration(
            vec!["push".to_string()],
            vec![RunInputDeclarationResponse::new(
                "rustc-version",
                RunInputSourceResponse::Workflow,
                None,
                false,
                Some("stable".to_string()),
            )],
        );
        app.handle_key(key(KeyCode::Down));
        app.handle_key(key(KeyCode::Enter));

        assert!(render_app_text(&app).contains("stable❯"));
    }

    #[test]
    fn settings_save_persists_edited_interface() {
        let store = Arc::new(FakeSettingsStore::new(Settings::default()));
        let mut app =
            TuiApp::new(Vec::new()).with_settings(Settings::default(), Some(store.clone()));
        open_settings(&mut app);
        app.handle_key(key(KeyCode::Enter));
        app.handle_key(key(KeyCode::Right));
        app.handle_key(key(KeyCode::Enter));
        app.handle_key(save_key());

        assert_eq!(app.screen(), TuiScreen::Home);
        assert_eq!(store.writes().len(), 1);
        assert_eq!(store.writes()[0].default_interface(), InterfaceMode::Cli);
    }

    #[test]
    fn settings_cancel_does_not_write() {
        let store = Arc::new(FakeSettingsStore::new(Settings::default()));
        let mut app =
            TuiApp::new(Vec::new()).with_settings(Settings::default(), Some(store.clone()));
        open_settings(&mut app);
        app.handle_key(key(KeyCode::Enter));
        app.handle_key(key(KeyCode::Right));
        app.handle_key(key(KeyCode::Esc));
        app.handle_key(key(KeyCode::Esc));

        assert_eq!(app.screen(), TuiScreen::Home);
        assert!(store.writes().is_empty());
    }

    #[test]
    fn settings_save_failure_keeps_settings_screen_open() {
        let store = Arc::new(FakeSettingsStore::failing(Settings::default(), "disk full"));
        let mut app = TuiApp::new(Vec::new()).with_settings(Settings::default(), Some(store));
        open_settings(&mut app);
        app.handle_key(save_key());

        assert_eq!(app.screen(), TuiScreen::Settings);
    }

    #[test]
    fn settings_save_persists_custom_marker() {
        let store = Arc::new(FakeSettingsStore::new(Settings::default()));
        let mut app =
            TuiApp::new(Vec::new()).with_settings(Settings::default(), Some(store.clone()));
        open_settings(&mut app);

        for _ in 0..9 {
            app.handle_key(key(KeyCode::Down));
        }
        app.handle_key(key(KeyCode::Enter));
        for _ in 0..6 {
            app.handle_key(key(KeyCode::Right));
        }
        app.handle_key(key(KeyCode::Char('q')));
        app.handle_key(key(KeyCode::Char('🚀')));
        app.handle_key(key(KeyCode::Enter));
        app.handle_key(save_key());

        assert_eq!(app.screen(), TuiScreen::Home);
        assert_eq!(store.writes().len(), 1);
        assert_eq!(store.writes()[0].marker(), &Marker::custom_text("q🚀"));
    }

    #[test]
    fn settings_save_persists_toggled_boolean_settings() {
        let store = Arc::new(FakeSettingsStore::new(Settings::default()));
        let mut app =
            TuiApp::new(Vec::new()).with_settings(Settings::default(), Some(store.clone()));
        open_settings(&mut app);

        app.handle_key(key(KeyCode::Down));
        app.handle_key(key(KeyCode::Enter));
        app.handle_key(key(KeyCode::Right));
        app.handle_key(key(KeyCode::Enter));
        app.handle_key(save_key());

        assert_eq!(app.screen(), TuiScreen::Home);
        assert_eq!(store.writes().len(), 1);
        assert!(store.writes()[0].allow_repo_writes());
        assert_eq!(store.writes()[0].default_interface(), InterfaceMode::Tui);
        assert!(!store.writes()[0].allow_network());
    }

    #[test]
    fn settings_save_persists_multiple_modified_settings() {
        let store = Arc::new(FakeSettingsStore::new(Settings::default()));
        let mut app =
            TuiApp::new(Vec::new()).with_settings(Settings::default(), Some(store.clone()));
        open_settings(&mut app);

        app.handle_key(key(KeyCode::Enter));
        app.handle_key(key(KeyCode::Right));
        app.handle_key(key(KeyCode::Enter));
        app.handle_key(key(KeyCode::Down));
        app.handle_key(key(KeyCode::Down));
        app.handle_key(key(KeyCode::Down));
        app.handle_key(key(KeyCode::Down));
        app.handle_key(key(KeyCode::Enter));
        app.handle_key(key(KeyCode::Right));
        app.handle_key(key(KeyCode::Enter));
        app.handle_key(key(KeyCode::Down));
        app.handle_key(key(KeyCode::Down));
        app.handle_key(key(KeyCode::Enter));
        app.handle_key(key(KeyCode::Right));
        app.handle_key(key(KeyCode::Enter));
        app.handle_key(save_key());

        assert_eq!(app.screen(), TuiScreen::Home);
        assert_eq!(store.writes().len(), 1);
        assert_eq!(store.writes()[0].default_interface(), InterfaceMode::Cli);
        assert!(store.writes()[0].allow_network());
        assert!(store.writes()[0].verbose());
        assert!(!store.writes()[0].allow_repo_writes());
    }

    #[test]
    fn settings_canceling_boolean_edit_with_escape_reverts_value() {
        let store = Arc::new(FakeSettingsStore::new(Settings::default()));
        let mut app =
            TuiApp::new(Vec::new()).with_settings(Settings::default(), Some(store.clone()));
        open_settings(&mut app);

        app.handle_key(key(KeyCode::Down));
        app.handle_key(key(KeyCode::Down));
        app.handle_key(key(KeyCode::Down));
        app.handle_key(key(KeyCode::Down));
        app.handle_key(key(KeyCode::Enter));
        app.handle_key(key(KeyCode::Right));
        app.handle_key(key(KeyCode::Esc));
        app.handle_key(save_key());

        assert_eq!(app.screen(), TuiScreen::Home);
        assert_eq!(store.writes().len(), 1);
        assert!(!store.writes()[0].allow_network());
    }
}
