#[cfg(test)]
mod tests {

    use std::time::Duration;

    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use ephact::{
        application::dtos::responses::{
            RunInputDeclarationResponse, RunInputSourceResponse, RunSummaryResponse,
            WorkflowListItemResponse,
        },
        presentation::tui::{ScreenManager, TuiApp, TuiScreen},
    };
    use ratatui::{Terminal, backend::TestBackend};

    fn rendered_lines(screens: &ScreenManager, title: &str) -> Vec<String> {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).expect("test terminal");

        terminal
            .draw(|frame| screens.render(frame, title))
            .expect("render screen");

        terminal
            .backend()
            .buffer()
            .content()
            .chunks(80)
            .map(|line| line.iter().map(|cell| cell.symbol()).collect())
            .collect()
    }

    #[test]
    fn screen_transitions_preserve_shared_title_frame() {
        let title = "randomized splash quote";
        let home = ScreenManager::new(Vec::new()).transition_to(TuiScreen::Home);
        let workflows = home.transition_to(TuiScreen::ListWorkflows);

        let home_lines = rendered_lines(&home, title);
        let workflow_lines = rendered_lines(&workflows, title);

        assert!(home_lines.iter().any(|line| line.contains(title)));
        assert!(workflow_lines.iter().any(|line| line.contains(title)));
        assert_eq!(home_lines[..5], workflow_lines[..5]);
        assert!(home_lines[5].contains("What would you like to do?"));
        assert!(workflow_lines[5].contains("Workflows"));
    }
    #[test]
    fn screen_manager_retains_home_and_workflow_selection() {
        let workflows = vec![
            WorkflowListItemResponse::new(Some("CI".to_string()), None, Vec::new()),
            WorkflowListItemResponse::new(Some("Deploy".to_string()), None, Vec::new()),
        ];
        let screens = ScreenManager::new(workflows)
            .transition_to(TuiScreen::RunWorkflow)
            .select_home_next()
            .select_run_workflow_next();

        assert_eq!(screens.home_selection(), 1);
        assert_eq!(screens.selected_workflow_name(), Some("Deploy"));
        assert_eq!(screens.current_screen(), TuiScreen::RunWorkflow);
        assert_eq!(screens.previous_screen(), Some(TuiScreen::Splash));

        let home = screens.transition_to(TuiScreen::Home);

        assert_eq!(screens.current_screen(), TuiScreen::RunWorkflow);
        assert_eq!(home.current_screen(), TuiScreen::Home);
        assert_eq!(home.previous_screen(), Some(TuiScreen::RunWorkflow));
    }

    #[test]
    fn tui_app_records_run_outcome_through_screen_manager() {
        let mut app = TuiApp::new(Vec::new());
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        let summary = RunSummaryResponse::new("CI", vec![], true, Duration::from_secs(1));

        app.record_run_outcome(summary.clone());

        assert!(app.has_run_outcome());
    }

    #[test]
    fn selecting_list_workflows_enters_workflow_screen() {
        let mut app = TuiApp::new(Vec::new());
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));

        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

        assert_eq!(app.screen(), TuiScreen::ListWorkflows);
    }

    #[test]
    fn selecting_list_actions_enters_actions_screen() {
        let mut app = TuiApp::new(Vec::new());
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));

        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

        assert_eq!(app.screen(), TuiScreen::ListActions);
    }

    #[test]
    fn boolean_input_editing_works_through_tui_app() {
        let mut app = TuiApp::new(vec![WorkflowListItemResponse::new(
            Some("CI".to_string()),
            None,
            vec!["push".to_string()],
        )]);
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.begin_run_configuration(
            vec!["push".to_string()],
            vec![RunInputDeclarationResponse::new(
                "include-sysroot",
                RunInputSourceResponse::Workflow,
                None,
                false,
                Some("false".to_string()),
            )],
        );
        app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE));

        let configuration = app
            .take_configured_run_request()
            .expect("boolean input configuration");
        assert_eq!(
            configuration.inputs(),
            &[("include-sysroot".into(), "true".into())]
        );
    }

    #[test]
    fn run_picker_configures_on_enter_and_ignores_details_key() {
        let mut app = TuiApp::new(vec![WorkflowListItemResponse::new(
            Some("CI".to_string()),
            None,
            vec!["push".to_string()],
        )]);
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

        app.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
        assert!(!app.is_showing_details());

        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

        assert!(app.take_run_request());
        assert_eq!(app.screen(), TuiScreen::RunWorkflow);
    }

    #[test]
    fn cancel_key_requests_cancellation_without_leaving_run_screen() {
        let mut app = TuiApp::new(vec![
            ephact::application::dtos::responses::WorkflowListItemResponse::new(
                Some("CI".to_string()),
                None,
                Vec::new(),
            ),
        ]);
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.take_run_request();
        app.start_run();

        app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));

        assert_eq!(app.screen(), TuiScreen::RunWorkflow);
        assert!(app.take_cancel_request());
    }

    #[test]
    fn details_key_opens_failed_run_details() {
        let mut app = TuiApp::new(vec![]);
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.record_run_outcome(RunSummaryResponse::new(
            "CI",
            vec![],
            false,
            Duration::from_secs(1),
        ));

        app.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));

        assert!(app.is_showing_details());
    }

    #[test]
    fn tui_configuration_submits_selected_event() {
        let mut app = TuiApp::new(vec![WorkflowListItemResponse::new(
            Some("CI".to_string()),
            None,
            vec!["push".to_string(), "schedule".to_string()],
        )]);
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.begin_run_configuration(vec!["push".to_string(), "schedule".to_string()], vec![]);
        app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

        let configuration = app
            .take_configured_run_request()
            .expect("configured run request");

        assert_eq!(configuration.event(), "schedule");
    }

    #[test]
    fn summary_navigation_scrolls_completed_run_summary() {
        let mut app = TuiApp::new(Vec::new());
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.record_run_outcome(RunSummaryResponse::new(
            "CI",
            vec![],
            true,
            Duration::from_secs(1),
        ));

        app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));

        assert_eq!(app.summary_scroll(), 1);
    }

    #[test]
    fn details_key_opens_successful_run_details() {
        let mut app = TuiApp::new(vec![]);
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.record_run_outcome(RunSummaryResponse::new(
            "CI",
            vec![],
            true,
            Duration::from_secs(1),
        ));

        app.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));

        assert!(app.is_showing_details());
    }

    #[test]
    fn escape_closes_failed_run_details() {
        let mut app = TuiApp::new(vec![]);
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.record_run_outcome(RunSummaryResponse::new(
            "CI",
            vec![],
            false,
            Duration::from_secs(1),
        ));
        app.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));

        app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));

        assert!(!app.is_showing_details());
    }

    #[test]
    fn details_screen_scroll_stops_at_content_end() {
        let mut app = TuiApp::new(vec![]);
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.record_run_outcome(RunSummaryResponse::new(
            "CI",
            vec![],
            false,
            Duration::from_secs(1),
        ));
        app.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));

        app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));

        assert_eq!(app.details_scroll(), 0);
    }

    #[test]
    fn quit_key_does_not_exit_while_workflow_runs() {
        let mut app = TuiApp::new(vec![
            ephact::application::dtos::responses::WorkflowListItemResponse::new(
                Some("CI".to_string()),
                None,
                Vec::new(),
            ),
        ]);
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.take_run_request();
        app.start_run();

        app.handle_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));

        assert_eq!(app.screen(), TuiScreen::RunWorkflow);
    }
}
