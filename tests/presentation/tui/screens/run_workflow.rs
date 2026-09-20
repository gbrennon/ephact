#[cfg(test)]
mod tests {
    use std::time::Duration;

    use ephact::application::dtos::responses::{
        JobSummaryResponse, RunSummaryResponse, WorkflowListItemResponse,
    };
    use ephact::presentation::tui::screens::RunWorkflowScreen;
    use ratatui::{Terminal, backend::TestBackend};

    fn workflows() -> Vec<WorkflowListItemResponse> {
        vec![
            WorkflowListItemResponse::new(Some("Build".into()), None, vec![]),
            WorkflowListItemResponse::new(Some("CI".into()), None, vec![]),
        ]
    }

    fn rendered_text(screen: &RunWorkflowScreen) -> String {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).expect("test terminal");

        terminal
            .draw(|frame| screen.render(frame))
            .expect("render screen");

        terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|cell| cell.symbol())
            .collect()
    }

    fn rendered_line_containing(screen: &RunWorkflowScreen, text: &str) -> String {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).expect("test terminal");

        terminal
            .draw(|frame| screen.render(frame))
            .expect("render screen");

        for line in terminal.backend().buffer().content().chunks(80) {
            let rendered: String = line.iter().map(|cell| cell.symbol()).collect();
            if rendered.contains(text) {
                return rendered;
            }
        }
        panic!("rendered text not found");
    }

    #[test]
    fn picker_footer_is_inside_the_screen_border() {
        let screen = RunWorkflowScreen::new(workflows());

        let line = rendered_line_containing(&screen, "Enter: Run");
        let footer_start = line.find("Enter: Run").expect("footer text");

        assert!(line[..footer_start].ends_with("│ "));
        assert!(line[footer_start..].contains('│'));
    }

    #[test]
    fn selecting_next_stops_at_last_workflow() {
        let mut screen = RunWorkflowScreen::new(workflows());

        screen.select_next();
        screen.select_next();

        assert_eq!(screen.selected_index(), 1);
    }

    #[test]
    fn selected_workflow_name_returns_highlighted_workflow() {
        let mut screen = RunWorkflowScreen::new(workflows());

        screen.select_next();

        assert_eq!(screen.selected_workflow_name(), Some("CI"));
    }

    #[test]
    fn picker_renders_workflow_names() {
        let screen = RunWorkflowScreen::new(workflows());

        let text = rendered_text(&screen);

        assert!(text.contains("Build"));
        assert!(text.contains("CI"));
    }

    #[test]
    fn empty_picker_renders_repository_message() {
        let screen = RunWorkflowScreen::new(Vec::new());

        let text = rendered_text(&screen);

        assert!(text.contains("No workflows found in repository"));
    }

    #[test]
    fn recording_outcome_renders_run_summary_with_status() {
        let mut screen = RunWorkflowScreen::new(workflows());
        let job = JobSummaryResponse::new("build", Some("build".into()), vec![], true);
        let summary = RunSummaryResponse::new("CI", vec![job], true, Duration::from_secs(1));

        screen.record_outcome(summary);

        let text = rendered_text(&screen);
        assert!(text.contains("Run Summary"));
        assert!(text.contains("SUCCESS"));
    }

    #[test]
    fn running_state_renders_streamed_progress_lines() {
        let mut screen = RunWorkflowScreen::new(workflows());
        screen.start_run();
        screen.record_progress("    Step 'compile': running...".to_string());

        let text = rendered_text(&screen);

        assert!(text.contains("Step 'compile': running..."));
    }

    #[test]
    fn reset_returns_screen_to_picker() {
        let mut screen = RunWorkflowScreen::new(workflows());
        screen.record_outcome(RunSummaryResponse::new(
            "CI",
            vec![],
            true,
            Duration::from_secs(1),
        ));

        screen.reset();

        assert!(screen.outcome().is_none());
    }

    #[test]
    fn running_state_renders_cancellation_prompt() {
        let mut screen = RunWorkflowScreen::new(workflows());

        screen.start_run();

        let text = rendered_text(&screen);
        assert!(screen.is_running());
        assert!(text.contains("Workflow is running..."));
        assert!(text.contains("Esc: Cancel"));
    }
}
