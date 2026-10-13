#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use ephact::presentation::tui::components::{ConfigurationAction, RunConfiguration};

    #[test]
    fn empty_event_configuration_cannot_submit() {
        let mut configuration = RunConfiguration::new(Vec::new(), Vec::new());

        let action =
            configuration.handle_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE));

        assert_eq!(action, ConfigurationAction::Continue);
        assert!(configuration.error().is_some());
    }

    #[test]
    fn empty_event_configuration_has_no_values() {
        let configuration = RunConfiguration::new(Vec::new(), Vec::new());

        assert!(configuration.values().is_none());
    }

    #[test]
    fn configuration_error_dismisses_on_escape() {
        let mut configuration = RunConfiguration::new(Vec::new(), Vec::new());
        configuration.report_error("Error: Workflow declares no supported events".to_owned());

        let action = configuration.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));

        assert_eq!(action, ConfigurationAction::Cancel);
        assert!(configuration.error().is_none());
    }

    #[test]
    fn empty_event_configuration_does_not_enter_input_editing() {
        let mut configuration = RunConfiguration::new(Vec::new(), Vec::new());

        let action = configuration.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        let follow_up =
            configuration.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));

        assert_eq!(action, ConfigurationAction::Continue);
        assert_eq!(follow_up, ConfigurationAction::Continue);
        assert!(configuration.error().is_none());
    }
}
