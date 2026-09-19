#[cfg(test)]
mod tests {
    use ephact::application::dtos::responses::WorkflowListItemResponse;
    use ephact::presentation::tui::screens::ListWorkflowsScreen;
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn selecting_next_stops_at_last_workflow() {
        let workflows = vec![
            WorkflowListItemResponse::new(Some("first".into()), None, vec![]),
            WorkflowListItemResponse::new(Some("second".into()), None, vec![]),
        ];
        let mut screen = ListWorkflowsScreen::new(workflows);

        screen.select_next();
        screen.select_next();

        assert_eq!(screen.selected_index(), 1);
    }

    #[test]
    fn render_empty_screen_shows_repository_message() {
        let screen = ListWorkflowsScreen::new(Vec::new());
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).expect("test terminal");

        terminal
            .draw(|frame| screen.render(frame))
            .expect("render screen");

        let text = terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|cell| cell.symbol())
            .collect::<String>();
        assert!(text.contains("No workflows found in repository"));
    }
}
