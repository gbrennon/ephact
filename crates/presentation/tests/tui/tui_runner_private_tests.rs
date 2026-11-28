#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use ratatui::{Terminal, backend::TestBackend};

    use super::super::{RunTask, TuiApp, TuiLoopState, TuiRunner};
    use crate::application::dtos::responses::{ExecuteActionResponse, WorkflowListItemResponse};

    #[path = "empty_list_actions_port.rs"]
    mod empty_list_actions_port;
    #[path = "empty_list_workflows_port.rs"]
    mod empty_list_workflows_port;
    #[path = "failure_log_scenario.rs"]
    mod failure_log_scenario;
    #[path = "unused_run_workflow_port.rs"]
    mod unused_run_workflow_port;

    use empty_list_actions_port::EmptyListActionsPort;
    use empty_list_workflows_port::EmptyListWorkflowsPort;
    use failure_log_scenario::FailureLogScenario;
    use unused_run_workflow_port::UnusedRunWorkflowPort;

    #[path = "tui_runner_active_path_test.rs"]
    mod active_path_test;

    #[path = "tui_runner_keyed_path_test.rs"]
    mod keyed_path_test;

    #[path = "tui_runner_cancellation_test.rs"]
    mod cancellation_test;

    fn render_text(app: &TuiApp) -> String {
        let backend = TestBackend::new(100, 20);
        let mut terminal = Terminal::new(backend).expect("test terminal should be created");
        terminal
            .draw(|frame| app.render(frame))
            .expect("test screen should render");
        terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|cell| cell.symbol().to_string())
            .collect()
    }

    #[test]
    fn loop_state_starts_idle() {
        let state = TuiLoopState::new();

        assert!(matches!(state, TuiLoopState::Idle));
    }

    fn action_app() -> TuiApp {
        let mut app =
            TuiApp::new(Vec::new(), String::new()).with_actions(vec!["actions/demo".to_owned()]);
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app
    }

    #[test]
    fn backing_out_of_action_configuration_resets_discovery_state() {
        let runner = TuiRunner::new(
            Arc::new(EmptyListWorkflowsPort),
            Arc::new(EmptyListActionsPort),
            Arc::new(UnusedRunWorkflowPort),
        );
        let mut app = action_app();
        app.begin_action_configuration(Vec::new());
        app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        let mut state = TuiLoopState::ActionInputConfigurationPending;

        runner
            .process_action_input_discovery(&mut app, &mut state)
            .expect("back navigation should be handled");

        assert!(matches!(state, TuiLoopState::Idle));
    }

    #[test]
    fn cancelled_workflow_configuration_releases_pending_state() {
        let runner = TuiRunner::new(
            Arc::new(EmptyListWorkflowsPort),
            Arc::new(EmptyListActionsPort),
            Arc::new(UnusedRunWorkflowPort),
        );
        let mut app = TuiApp::new(
            vec![WorkflowListItemResponse::new(
                Some("CI".to_owned()),
                None,
                vec!["push".to_owned()],
            )],
            String::new(),
        );
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.begin_run_configuration(vec!["push".to_owned()], vec![]);
        app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        let mut state = TuiLoopState::InputConfigurationPending;

        runner
            .process_configured_run_request(&mut app, &mut state)
            .expect("cancelled configuration should be handled");

        assert!(matches!(state, TuiLoopState::Idle));
    }

    #[tokio::test]
    async fn action_join_failure_is_rendered_as_action_outcome() {
        let runner = TuiRunner::new(
            Arc::new(EmptyListWorkflowsPort),
            Arc::new(EmptyListActionsPort),
            Arc::new(UnusedRunWorkflowPort),
        );
        let mut app = action_app();
        app.start_action_run();
        let handle = tokio::spawn(async move {
            panic!("action task panicked");
            #[allow(unreachable_code)]
            Ok::<ExecuteActionResponse, String>(ExecuteActionResponse::note("unused".into()))
        });
        let mut state = TuiLoopState::ActionRunning {
            task: super::super::ActionTask::new(handle),
        };
        tokio::task::yield_now().await;

        runner
            .process_finished_action(&mut app, &mut state)
            .await
            .expect("join failures should become action outcomes");

        assert!(render_text(&app).contains("Exit code: 1"));
    }
}
