#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use ephact::{
        domain::{InterfaceMode, Settings},
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
    }
    #[test]
    fn editing_interface_shows_alternatives_and_highlights_current_value() {
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
    }

    #[test]
    fn settings_is_reachable_from_home_menu() {
        let mut app = TuiApp::new(Vec::new());

        open_settings(&mut app);

        assert_eq!(app.screen(), TuiScreen::Settings);
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
