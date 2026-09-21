#[cfg(test)]
mod tests {
    use ephact::{
        application::dtos::responses::WorkflowListItemResponse,
        presentation::tui::screens::ListWorkflowsScreen,
    };
    use ratatui::{Terminal, backend::TestBackend};

    use crate::common::fakes::fake_list_workflows_port::FakeListWorkflowsPort;

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

    #[test]
    fn from_handler_populates_workflows_from_port() {
        let workflows = vec![WorkflowListItemResponse::new(
            Some("CI".into()),
            Some("ci.yml".into()),
            vec!["push".into()],
        )];
        let port = FakeListWorkflowsPort::with_workflows(workflows.clone());

        let screen = ListWorkflowsScreen::from_handler(
            &port,
            std::env::current_dir().expect("current directory"),
        )
        .expect("repository should be valid");

        assert_eq!(screen.workflows(), workflows);
    }
}
