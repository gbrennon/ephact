#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use ephact::presentation::tui::tui_app::{TuiApp, TuiScreen};

    #[test]
    fn selecting_list_workflows_enters_workflow_screen() {
        let mut app = TuiApp::new(Vec::new());
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));

        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

        assert_eq!(app.screen(), TuiScreen::ListWorkflows);
    }
}
