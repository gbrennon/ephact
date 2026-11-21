#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, mpsc},
        time::Duration,
    };

    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use ephact::{
        application::{
            dtos::responses::RunSummaryResponse, ports::outbound::DomainEventHandlerPort,
        },
        domain::{
            Settings,
            messages::events::{Event, StepStartedPayload},
            value_objects::Permissions,
        },
        presentation::{
            cli::RunProgressHandler,
            tui::{TuiApp, TuiLoopState, TuiRunner, TuiScreen},
        },
    };

    use crate::{
        common::fakes::{
            fake_list_actions_port::FakeListActionsPort,
            fake_list_workflows_port::FakeListWorkflowsPort,
        },
        fakes::recording_run_workflow_port::RecordingRunWorkflowPort,
    };

    #[path = "blocking_run_action_port.rs"]
    mod blocking_run_action_port;
    #[path = "recording_discovery.rs"]
    mod recording_discovery;
    #[path = "recording_run_action_port.rs"]
    mod recording_run_action_port;

    use blocking_run_action_port::BlockingRunActionPort;
    use recording_discovery::RecordingDiscovery;
    use recording_run_action_port::RecordingRunActionPort;

    fn action_app() -> TuiApp {
        let mut app = TuiApp::new(Vec::new(), ephact::PROJECT_EMBLEM.to_owned())
            .with_actions(vec!["./.github/actions/demo".to_owned()]);
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app
    }

    fn rendered_text(app: &TuiApp) -> String {
        let backend = ratatui::backend::TestBackend::new(100, 20);
        let mut terminal = ratatui::Terminal::new(backend).expect("test terminal");
        terminal.draw(|frame| app.render(frame)).expect("render");
        terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|cell| cell.symbol())
            .collect()
    }

    fn ci_summary() -> RunSummaryResponse {
        RunSummaryResponse::new("CI".to_owned(), vec![], true, Duration::from_secs(1))
    }

    #[tokio::test]
    async fn event_loop_workflow_path_forwards_execution_permissions() {
        let workflow_port = Arc::new(RecordingRunWorkflowPort::new(ci_summary()));
        let settings = Settings::default().with_permissions(
            Permissions::default()
                .with_allow_repo_writes(true)
                .with_allow_real_container(true)
                .with_allow_real_fetcher(true)
                .with_allow_network(true),
        );
        let runner = TuiRunner::new(
            Arc::new(FakeListWorkflowsPort::new()),
            Arc::new(FakeListActionsPort::new()),
            workflow_port.clone(),
        )
        .with_settings(settings, None);
        let mut app = TuiApp::new(
            vec![
                ephact::application::dtos::responses::WorkflowListItemResponse::new(
                    Some("CI".to_owned()),
                    None,
                    vec!["push".to_owned()],
                ),
            ],
            ephact::PROJECT_EMBLEM.to_owned(),
        );
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.begin_run_configuration(vec!["push".to_owned()], Vec::new());
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        let mut state = TuiLoopState::new();

        for _ in 0..10 {
            runner
                .process_event_loop_step(&mut app, &mut state)
                .await
                .expect("workflow should process");
            if workflow_port.recorded_request().is_some() {
                break;
            }
            tokio::task::yield_now().await;
        }

        let request = workflow_port.recorded_request().expect("workflow request");
        assert!(request.allow_repo_writes());
        assert!(request.allow_real_container());
        assert!(request.allow_real_fetcher());
        assert!(request.allow_network());
    }

    #[tokio::test]
    async fn event_loop_action_path_discovers_submits_and_renders_outcome() {
        let action_port = Arc::new(RecordingRunActionPort::new());
        let discovery = Arc::new(RecordingDiscovery::new());
        let (handler, progress) = RunProgressHandler::with_tui_stream(true);
        let runner = TuiRunner::new(
            Arc::new(FakeListWorkflowsPort::new()),
            Arc::new(FakeListActionsPort::new()),
            Arc::new(RecordingRunWorkflowPort::new(ci_summary())),
        )
        .with_action_port(action_port.clone())
        .with_input_discovery(discovery.clone())
        .with_progress_stream(progress);
        let mut app = action_app();
        let mut state = TuiLoopState::new();

        runner
            .process_event_loop_step(&mut app, &mut state)
            .await
            .expect("discovery should be processed");
        assert_eq!(discovery.requests().as_slice(), &["./.github/actions/demo"]);

        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        runner
            .process_event_loop_step(&mut app, &mut state)
            .await
            .expect("configured action should be submitted");
        assert!(app.is_action_running());

        handler.handle(&Event::StepStarted(StepStartedPayload::new(
            "workflow".to_owned(),
            "job".to_owned(),
            "progress".to_owned(),
        )));
        for _ in 0..10 {
            tokio::time::sleep(Duration::from_millis(1)).await;
            runner
                .process_event_loop_step(&mut app, &mut state)
                .await
                .expect("completed action should be rendered");
            if !app.is_action_running() {
                break;
            }
        }

        assert_eq!(app.screen(), TuiScreen::RunAction);
        assert!(rendered_text(&app).contains("action completed"));
        assert_eq!(action_port.requests().len(), 1);

        runner
            .process_event_loop_step(&mut app, &mut state)
            .await
            .expect("the completed action should remain stable on the next tick");
        assert_eq!(
            discovery.requests().len(),
            1,
            "completed actions must not rediscover inputs on the next tick"
        );
        assert!(rendered_text(&app).contains("action completed"));

        app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        assert_eq!(app.screen(), TuiScreen::Home);
    }

    #[tokio::test]
    async fn event_loop_cancellation_waits_for_action_and_back_navigation_returns_home() {
        let (started_tx, started_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let action_port = Arc::new(BlockingRunActionPort::new(started_tx, release_rx));
        let (handler, progress) = RunProgressHandler::with_tui_stream(true);
        progress.activate_tui();
        let runner = TuiRunner::new(
            Arc::new(FakeListWorkflowsPort::new()),
            Arc::new(FakeListActionsPort::new()),
            Arc::new(RecordingRunWorkflowPort::new(ci_summary())),
        )
        .with_action_port(action_port)
        .with_input_discovery(Arc::new(RecordingDiscovery::new()))
        .with_progress_stream(progress);
        let mut app = action_app();
        let mut state = TuiLoopState::new();
        runner
            .process_event_loop_step(&mut app, &mut state)
            .await
            .expect("discovery should be processed");
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        runner
            .process_event_loop_step(&mut app, &mut state)
            .await
            .expect("action should start");
        started_rx
            .recv_timeout(Duration::from_secs(1))
            .expect("action should reach the recording port");
        handler.handle(&Event::StepStarted(StepStartedPayload::new(
            "workflow".to_owned(),
            "job".to_owned(),
            "progress".to_owned(),
        )));
        runner
            .process_event_loop_step(&mut app, &mut state)
            .await
            .expect("progress should be routed while action runs");
        assert!(rendered_text(&app).contains("Step 'progress': running..."));

        app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        runner
            .process_event_loop_step(&mut app, &mut state)
            .await
            .expect("cancellation should be processed");
        assert!(matches!(state, TuiLoopState::ActionCancelling { .. }));
        assert!(!app.is_action_running());
        assert!(rendered_text(&app).contains("CANCELLED"));

        release_tx.send(()).expect("release action");
        for _ in 0..10 {
            tokio::time::sleep(Duration::from_millis(1)).await;
            runner
                .process_event_loop_step(&mut app, &mut state)
                .await
                .expect("completed cancellation should render outcome");
            if !app.is_action_running() {
                break;
            }
        }
        assert!(rendered_text(&app).contains("CANCELLED"));
        runner
            .process_event_loop_step(&mut app, &mut state)
            .await
            .expect("the cancelled action should remain stable on the next tick");
        assert!(rendered_text(&app).contains("CANCELLED"));

        app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        assert_eq!(app.screen(), TuiScreen::Home);
    }

    #[tokio::test]
    async fn event_loop_missing_action_port_renders_error_outcome() {
        let runner = TuiRunner::new(
            Arc::new(FakeListWorkflowsPort::new()),
            Arc::new(FakeListActionsPort::new()),
            Arc::new(RecordingRunWorkflowPort::new(ci_summary())),
        );
        let mut app = action_app();
        let mut state = TuiLoopState::new();
        runner
            .process_event_loop_step(&mut app, &mut state)
            .await
            .expect("discovery should be processed");
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        runner
            .process_event_loop_step(&mut app, &mut state)
            .await
            .expect("missing action port should be rendered");

        assert!(rendered_text(&app).contains("action execution is not configured"));

        runner
            .process_event_loop_step(&mut app, &mut state)
            .await
            .expect("missing-port result should remain stable on the next tick");
        assert!(rendered_text(&app).contains("action execution is not configured"));
    }

    #[tokio::test]
    async fn run_workflow_executes_run_workflow_port_with_selected_name() {
        let run_port = Arc::new(RecordingRunWorkflowPort::new(ci_summary()));
        let runner = TuiRunner::new(
            Arc::new(FakeListWorkflowsPort::new()),
            Arc::new(FakeListActionsPort::new()),
            run_port.clone(),
        );

        let summary = runner
            .run_workflow(Some("CI".to_string()))
            .await
            .expect("run should succeed");

        assert_eq!(summary, ci_summary());
        assert_eq!(
            run_port
                .recorded_request()
                .expect("recorded request")
                .workflow(),
            Some("CI")
        );
    }

    #[tokio::test]
    async fn run_action_executes_only_selected_action() {
        let action_port = Arc::new(RecordingRunActionPort::new());
        let workflow_port = Arc::new(RecordingRunWorkflowPort::new(ci_summary()));
        let runner = TuiRunner::new(
            Arc::new(FakeListWorkflowsPort::new()),
            Arc::new(FakeListActionsPort::new()),
            workflow_port.clone(),
        )
        .with_action_port(action_port.clone());

        let response = runner
            .run_action(
                "./.github/actions/demo".to_owned(),
                vec![("name".to_owned(), "Ada".to_owned())],
            )
            .await
            .expect("action should succeed");

        assert_eq!(response.stdout(), "action completed");
        let requests = action_port.requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].action_ref(), "./.github/actions/demo");
        assert!(requests[0].step().contains("\"name\":\"Ada\""));
        assert!(workflow_port.recorded_request().is_none());
    }

    #[tokio::test]
    async fn run_workflow_does_not_start_nested_runtime() {
        let run_port = Arc::new(RecordingRunWorkflowPort::new(ci_summary()));
        let runner = TuiRunner::new(
            Arc::new(FakeListWorkflowsPort::new()),
            Arc::new(FakeListActionsPort::new()),
            run_port,
        );

        let result = runner.run_workflow(Some("CI".to_string())).await;

        assert!(result.is_ok());
    }
}
