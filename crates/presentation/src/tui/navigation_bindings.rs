use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::domain::NavigationMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationAction {
    Previous,
    Next,
    Left,
    Right,
    PageUp,
    PageDown,
    Top,
    Bottom,
    Help,
}

#[derive(Debug, Clone, Copy)]
pub struct NavigationBindings {
    mode: NavigationMode,
    pending_vim_g: bool,
}

impl NavigationBindings {
    pub fn help_lines(mode: NavigationMode) -> Vec<String> {
        match mode {
            NavigationMode::Standard => vec![
                "Arrows or h/j/k/l: move".to_owned(),
                "PageUp/PageDown: scroll".to_owned(),
            ],
            NavigationMode::Vim => vec![
                "h/j/k/l: move".to_owned(),
                "gg: top | G: bottom".to_owned(),
                "Ctrl-d/Ctrl-u: page down/up".to_owned(),
            ],
            NavigationMode::Emacs => vec![
                "Ctrl-f/b: right/left | Ctrl-n/p: down/up".to_owned(),
                "Ctrl-a/e: top/bottom".to_owned(),
                "Ctrl-v/Meta-v: page down/up".to_owned(),
            ],
        }
    }

    pub fn new(mode: NavigationMode) -> Self {
        Self {
            mode,
            pending_vim_g: false,
        }
    }

    pub fn mode(&self) -> NavigationMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: NavigationMode) {
        self.mode = mode;
        self.pending_vim_g = false;
    }

    pub fn map(&mut self, key: KeyEvent) -> Option<NavigationAction> {
        if key.code == KeyCode::Char('?') && key.modifiers.is_empty() {
            self.pending_vim_g = false;
            return Some(NavigationAction::Help);
        }
        match self.mode {
            NavigationMode::Standard => Self::map_standard(key),
            NavigationMode::Vim => self.map_vim(key),
            NavigationMode::Emacs => Self::map_emacs(key),
        }
    }

    fn map_standard(key: KeyEvent) -> Option<NavigationAction> {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => Some(NavigationAction::Previous),
            KeyCode::Down | KeyCode::Char('j') => Some(NavigationAction::Next),
            KeyCode::Left | KeyCode::Char('h') => Some(NavigationAction::Left),
            KeyCode::Right | KeyCode::Char('l') => Some(NavigationAction::Right),
            KeyCode::PageUp => Some(NavigationAction::PageUp),
            KeyCode::PageDown => Some(NavigationAction::PageDown),
            _ => None,
        }
    }

    fn map_vim(&mut self, key: KeyEvent) -> Option<NavigationAction> {
        if self.pending_vim_g {
            self.pending_vim_g = false;
            return (key.modifiers.is_empty() && key.code == KeyCode::Char('g'))
                .then_some(NavigationAction::Top);
        }
        if key.modifiers.is_empty() && key.code == KeyCode::Char('g') {
            self.pending_vim_g = true;
            return None;
        }
        match (key.code, key.modifiers) {
            (KeyCode::Char('h'), KeyModifiers::NONE) => Some(NavigationAction::Left),
            (KeyCode::Char('j'), KeyModifiers::NONE) => Some(NavigationAction::Next),
            (KeyCode::Char('k'), KeyModifiers::NONE) => Some(NavigationAction::Previous),
            (KeyCode::Char('l'), KeyModifiers::NONE) => Some(NavigationAction::Right),
            (KeyCode::Char('G'), KeyModifiers::NONE)
            | (KeyCode::Char('G'), KeyModifiers::SHIFT)
            | (KeyCode::Char('g'), KeyModifiers::SHIFT) => Some(NavigationAction::Bottom),
            (KeyCode::Char('d'), KeyModifiers::CONTROL) => Some(NavigationAction::PageDown),
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => Some(NavigationAction::PageUp),
            _ => None,
        }
    }

    fn map_emacs(key: KeyEvent) -> Option<NavigationAction> {
        match (key.code, key.modifiers) {
            (KeyCode::Char('f'), KeyModifiers::CONTROL) => Some(NavigationAction::Right),
            (KeyCode::Char('b'), KeyModifiers::CONTROL) => Some(NavigationAction::Left),
            (KeyCode::Char('n'), KeyModifiers::CONTROL) => Some(NavigationAction::Next),
            (KeyCode::Char('p'), KeyModifiers::CONTROL) => Some(NavigationAction::Previous),
            (KeyCode::Char('a'), KeyModifiers::CONTROL) => Some(NavigationAction::Top),
            (KeyCode::Char('e'), KeyModifiers::CONTROL) => Some(NavigationAction::Bottom),
            (KeyCode::Char('v'), KeyModifiers::CONTROL) => Some(NavigationAction::PageDown),
            (KeyCode::Char('v'), KeyModifiers::ALT) => Some(NavigationAction::PageUp),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
        KeyEvent::new(code, modifiers)
    }

    #[test]
    fn standard_uses_arrows_and_existing_letter_aliases() {
        let mut bindings = NavigationBindings::new(NavigationMode::Standard);
        assert_eq!(
            bindings.map(key(KeyCode::Up, KeyModifiers::NONE)),
            Some(NavigationAction::Previous)
        );
        assert_eq!(
            bindings.map(key(KeyCode::Char('j'), KeyModifiers::NONE)),
            Some(NavigationAction::Next)
        );
    }

    #[test]
    fn vim_ignores_arrows_and_supports_official_motions() {
        let mut bindings = NavigationBindings::new(NavigationMode::Vim);
        assert_eq!(bindings.map(key(KeyCode::Up, KeyModifiers::NONE)), None);
        assert_eq!(
            bindings.map(key(KeyCode::Char('j'), KeyModifiers::NONE)),
            Some(NavigationAction::Next)
        );
        assert_eq!(
            bindings.map(key(KeyCode::Char('g'), KeyModifiers::NONE)),
            None
        );
        assert_eq!(
            bindings.map(key(KeyCode::Char('g'), KeyModifiers::NONE)),
            Some(NavigationAction::Top)
        );
        assert_eq!(
            bindings.map(key(KeyCode::Char('G'), KeyModifiers::SHIFT)),
            Some(NavigationAction::Bottom)
        );
        assert_eq!(
            bindings.map(key(KeyCode::Char('d'), KeyModifiers::CONTROL)),
            Some(NavigationAction::PageDown)
        );
    }

    #[test]
    fn emacs_ignores_arrows_and_supports_official_motions() {
        let mut bindings = NavigationBindings::new(NavigationMode::Emacs);
        assert_eq!(bindings.map(key(KeyCode::Down, KeyModifiers::NONE)), None);
        assert_eq!(
            bindings.map(key(KeyCode::Char('n'), KeyModifiers::CONTROL)),
            Some(NavigationAction::Next)
        );
        assert_eq!(
            bindings.map(key(KeyCode::Char('p'), KeyModifiers::CONTROL)),
            Some(NavigationAction::Previous)
        );
        assert_eq!(
            bindings.map(key(KeyCode::Char('v'), KeyModifiers::ALT)),
            Some(NavigationAction::PageUp)
        );
    }

    #[test]
    fn help_key_is_available_in_every_mode() {
        for mode in NavigationMode::ALL {
            let mut bindings = NavigationBindings::new(mode);
            assert_eq!(
                bindings.map(key(KeyCode::Char('?'), KeyModifiers::NONE)),
                Some(NavigationAction::Help)
            );
        }
    }
}
