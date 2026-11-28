use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ephact_application::ports::outbound::SettingsStorePort;
use ephact_domain::{NavigationMode, Settings};
use ephact_infrastructure::TomlSettingsStore;
use ephact_presentation::tui::{
    TuiApp, TuiScreen,
    navigation_bindings::{NavigationAction, NavigationBindings},
};
use tempfile::tempdir;

#[cfg(test)]
mod tests {
    use super::*;

    enum Key {
        Plain(KeyCode),
        Control(char),
    }

    struct NavigationModesE2e;

    impl NavigationModesE2e {
        fn press(app: &mut TuiApp, key: Key) {
            let (code, modifiers) = match key {
                Key::Plain(code) => (code, KeyModifiers::NONE),
                Key::Control(character) => (KeyCode::Char(character), KeyModifiers::CONTROL),
            };
            app.handle_key(KeyEvent::new(code, modifiers));
        }

        fn open_settings(app: &mut TuiApp) {
            Self::press(app, Key::Plain(KeyCode::Char(' ')));
            Self::press(app, Key::Plain(KeyCode::Down));
            Self::press(app, Key::Plain(KeyCode::Down));
            Self::press(app, Key::Plain(KeyCode::Enter));
        }
    }

    #[test]
    fn official_vim_and_emacs_bindings_map_to_semantic_actions() {
        let mut vim = NavigationBindings::new(NavigationMode::Vim);
        assert_eq!(
            vim.map(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE)),
            None
        );
        assert_eq!(
            vim.map(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE)),
            Some(NavigationAction::Top)
        );
        assert_eq!(
            vim.map(KeyEvent::new(KeyCode::Char('G'), KeyModifiers::NONE)),
            Some(NavigationAction::Bottom)
        );
        assert_eq!(
            vim.map(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL)),
            Some(NavigationAction::PageDown)
        );
        assert_eq!(
            vim.map(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL)),
            Some(NavigationAction::PageUp)
        );

        let mut emacs = NavigationBindings::new(NavigationMode::Emacs);
        assert_eq!(
            emacs.map(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::CONTROL)),
            Some(NavigationAction::Right)
        );
        assert_eq!(
            emacs.map(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::CONTROL)),
            Some(NavigationAction::Left)
        );
        assert_eq!(
            emacs.map(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL)),
            Some(NavigationAction::Top)
        );
        assert_eq!(
            emacs.map(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::CONTROL)),
            Some(NavigationAction::Bottom)
        );
        assert_eq!(
            emacs.map(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::CONTROL)),
            Some(NavigationAction::PageDown)
        );
        assert_eq!(
            emacs.map(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::ALT)),
            Some(NavigationAction::PageUp)
        );
    }

    #[test]
    fn vim_mode_disables_arrows_but_keeps_vim_navigation() {
        let mut app = TuiApp::new(Vec::new(), String::new()).with_settings(
            Settings::default().with_navigation_mode(NavigationMode::Vim),
            None,
        );
        NavigationModesE2e::press(&mut app, Key::Plain(KeyCode::Char(' ')));
        assert_eq!(app.screen(), TuiScreen::Home);
        assert_eq!(app.home_selection(), 0);

        NavigationModesE2e::press(&mut app, Key::Plain(KeyCode::Down));
        assert_eq!(app.home_selection(), 0);
        NavigationModesE2e::press(&mut app, Key::Plain(KeyCode::Char('j')));
        assert_eq!(app.home_selection(), 1);
    }

    #[test]
    fn help_key_opens_and_closes_mode_aware_navigation_help() {
        let mut app = TuiApp::new(Vec::new(), String::new());
        NavigationModesE2e::press(&mut app, Key::Plain(KeyCode::Char(' ')));
        NavigationModesE2e::press(&mut app, Key::Plain(KeyCode::Char('?')));
        assert!(app.is_navigation_help_visible());
        NavigationModesE2e::press(&mut app, Key::Plain(KeyCode::Esc));
        assert!(!app.is_navigation_help_visible());
    }

    #[test]
    fn emacs_mode_disables_arrows_and_uses_control_navigation() {
        let mut app = TuiApp::new(Vec::new(), String::new()).with_settings(
            Settings::default().with_navigation_mode(NavigationMode::Emacs),
            None,
        );
        NavigationModesE2e::press(&mut app, Key::Plain(KeyCode::Char(' ')));
        NavigationModesE2e::press(&mut app, Key::Plain(KeyCode::Down));
        assert_eq!(app.home_selection(), 0);
        NavigationModesE2e::press(&mut app, Key::Control('n'));
        assert_eq!(app.home_selection(), 1);
    }

    #[test]
    fn settings_screen_exposes_the_three_persisted_modes() {
        let directory = tempdir().expect("temporary settings directory");
        let store = Arc::new(TomlSettingsStore::new(directory.path().join("config.toml")));
        let mut app =
            TuiApp::new(Vec::new(), String::new()).with_settings(Settings::default(), Some(store));
        NavigationModesE2e::open_settings(&mut app);
        assert_eq!(app.screen(), TuiScreen::Settings);

        for _ in 0..12 {
            NavigationModesE2e::press(&mut app, Key::Plain(KeyCode::Down));
        }
        NavigationModesE2e::press(&mut app, Key::Plain(KeyCode::Enter));
        NavigationModesE2e::press(&mut app, Key::Plain(KeyCode::Right));
        NavigationModesE2e::press(&mut app, Key::Plain(KeyCode::Enter));
        NavigationModesE2e::press(&mut app, Key::Plain(KeyCode::Char('s')));
        assert_eq!(app.screen(), TuiScreen::Home);
        assert_eq!(app.home_selection(), 2);
        NavigationModesE2e::press(&mut app, Key::Plain(KeyCode::Up));
        assert_eq!(app.home_selection(), 2);
        NavigationModesE2e::press(&mut app, Key::Plain(KeyCode::Char('k')));
        assert_eq!(app.home_selection(), 1);
    }

    #[test]
    fn navigation_mode_round_trips_through_the_real_settings_store() {
        let directory = tempdir().expect("temporary settings directory");
        let store = TomlSettingsStore::new(directory.path().join("config.toml"));
        let settings = Settings::default().with_navigation_mode(NavigationMode::Emacs);

        store.write_settings(&settings).expect("write settings");
        let restored = store.read_settings().expect("read settings");

        assert_eq!(restored.navigation_mode(), NavigationMode::Emacs);
    }
}
