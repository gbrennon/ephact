#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use ephact::{
        domain::Settings,
        presentation::tui::screens::{SettingsAction, SettingsScreen},
    };

    use crate::fakes::fake_settings_store::FakeSettingsStore;

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[test]
    fn settings_save_can_retry_after_canceling_invalid_retention() {
        let store = Arc::new(FakeSettingsStore::new(Settings::default()));
        let mut screen = SettingsScreen::new(Settings::default(), Some(store.clone()));
        for _ in 0..9 {
            screen.handle_key(key(KeyCode::Down));
        }

        screen.handle_key(key(KeyCode::Enter));
        screen.handle_key(key(KeyCode::Backspace));
        screen.handle_key(key(KeyCode::Backspace));
        screen.handle_key(key(KeyCode::Char('0')));
        screen.handle_key(key(KeyCode::Enter));
        screen.handle_key(key(KeyCode::Esc));

        assert_eq!(
            screen.handle_key(key(KeyCode::Char('s'))),
            SettingsAction::Saved
        );
        assert_eq!(store.writes().len(), 1);
    }
}
