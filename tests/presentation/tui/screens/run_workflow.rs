#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use ephact::application::dtos::responses::{
        JobSummaryResponse, RunInputDeclarationResponse, RunInputSourceResponse,
        RunSummaryResponse, StepSummaryDetails, StepSummaryResponse, StepSummaryResponseInput,
        WorkflowListItemResponse,
    };
    use ephact::domain::value_objects::StepType;
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

    fn failed_summary() -> RunSummaryResponse {
        let step = StepSummaryResponse::new(StepSummaryResponseInput::new(
            "compile",
            StepType::Run,
            StepSummaryDetails::new(
                Some(1),
                false,
                Duration::from_secs(1),
                "",
                "compiler failed",
            ),
        ));
        let job = JobSummaryResponse::new("build", Some("build".into()), vec![step], false);
        RunSummaryResponse::new("CI", vec![job], false, Duration::from_secs(1))
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
        screen.record_progress("Step 'compile': running...".to_string());

        let text = rendered_text(&screen);

        assert!(text.contains("Step 'compile': running..."));
    }

    #[test]
    fn failed_summary_offers_details_without_indenting_jobs() {
        let mut screen = RunWorkflowScreen::new(workflows());
        screen.record_outcome(failed_summary());

        let text = rendered_text(&screen);

        assert!(text.contains("d: Details"));
        assert!(text.contains("build: FAILED"));
        assert!(!text.contains("  build: FAILED"));
    }

    #[test]
    fn successful_summary_offers_details() {
        let mut screen = RunWorkflowScreen::new(workflows());
        screen.record_outcome(RunSummaryResponse::new(
            "CI",
            vec![],
            true,
            Duration::from_secs(1),
        ));

        let text = rendered_text(&screen);

        assert!(text.contains("d: Details"));
    }

    #[test]
    fn failure_details_render_step_output() {
        let mut screen = RunWorkflowScreen::new(workflows());
        screen.record_outcome(failed_summary());

        assert!(screen.open_details());
        let text = rendered_text(&screen);

        assert!(text.contains("Run Details"));
        assert!(text.contains("Step: compile [FAILED]"));
        assert!(text.contains("Exit code: 1"));
        assert!(text.contains("stderr: compiler failed"));
    }

    #[test]
    fn summary_scrolls_with_navigation_keys() {
        let mut screen = RunWorkflowScreen::new(workflows());
        screen.record_outcome(RunSummaryResponse::new(
            "CI",
            vec![],
            true,
            Duration::from_secs(1),
        ));

        screen.scroll_summary_down();

        assert_eq!(screen.summary_scroll(), 1);
    }

    #[test]
    fn successful_summary_opens_details() {
        let mut screen = RunWorkflowScreen::new(workflows());
        screen.record_outcome(RunSummaryResponse::new(
            "CI",
            vec![],
            true,
            Duration::from_secs(1),
        ));

        assert!(screen.open_details());
        assert!(screen.showing_details());
    }

    #[test]
    fn configuration_collects_event_and_required_input() {
        let mut screen = RunWorkflowScreen::new(workflows());
        screen.begin_configuration(
            vec!["push".to_string(), "schedule".to_string()],
            vec![RunInputDeclarationResponse::new(
                "environment",
                RunInputSourceResponse::Workflow,
                None,
                true,
                None,
            )],
        );

        screen.handle_configuration_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        screen.handle_configuration_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        screen.handle_configuration_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        screen.handle_configuration_key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE));
        screen.handle_configuration_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE));
        screen.handle_configuration_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
        screen.handle_configuration_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

        assert_eq!(
            screen.handle_configuration_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE)),
            ephact::presentation::tui::screens::ConfigurationAction::Submit
        );
        let configuration = screen.take_configuration().expect("configuration");
        assert_eq!(configuration.event(), "schedule");
        assert_eq!(
            configuration.inputs(),
            &[("environment".to_string(), "pro".to_string())]
        );
    }

    #[test]
    fn configuration_rejects_missing_required_input() {
        let mut screen = RunWorkflowScreen::new(workflows());
        screen.begin_configuration(
            vec!["push".to_string()],
            vec![RunInputDeclarationResponse::new(
                "environment",
                RunInputSourceResponse::Workflow,
                None,
                true,
                None,
            )],
        );
        screen.handle_configuration_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));

        assert_eq!(
            screen.handle_configuration_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE)),
            ephact::presentation::tui::screens::ConfigurationAction::Continue
        );
        assert!(screen.configuration_error().is_some());
    }

    #[test]
    fn failure_details_scroll_in_both_directions() {
        let mut screen = RunWorkflowScreen::new(workflows());
        screen.record_outcome(failed_summary());
        screen.open_details();

        screen.scroll_details_down();
        assert_eq!(screen.details_scroll(), 1);

        screen.scroll_details_up();
        screen.scroll_details_up();
        assert_eq!(screen.details_scroll(), 0);
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
