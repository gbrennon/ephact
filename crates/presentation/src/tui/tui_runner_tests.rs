use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{Terminal, backend::TestBackend};

use super::{RunTask, TuiApp, TuiRunner};
use crate::{
    application::{
        dtos::{
            requests::{ListActionsRequest, ListWorkflowsRequest, RunWorkflowRequest},
            responses::{ListActionsResponse, ListWorkflowsResponse, RunSummaryResponse},
        },
        errors::{ApplicationError, ListActionsError, ListWorkflowsError},
        ports::{
            inbound::{ListActionsPort, ListWorkflowsPort, RunWorkflowPort},
            outbound::DomainEventHandlerPort,
        },
    },
    domain::messages::events::{Event, RunFailedPayload, RunStartedPayload},
    infrastructure::logging::FailureLogHandler,
};

struct EmptyListWorkflowsPort;

impl ListWorkflowsPort for EmptyListWorkflowsPort {
    fn execute(
        &self,
        _request: ListWorkflowsRequest,
    ) -> Result<ListWorkflowsResponse, ListWorkflowsError> {
        Ok(ListWorkflowsResponse::new(Vec::new()))
    }
}

struct EmptyListActionsPort;

impl ListActionsPort for EmptyListActionsPort {
    fn execute(
        &self,
        _request: ListActionsRequest,
    ) -> Result<ListActionsResponse, ListActionsError> {
        Ok(ListActionsResponse::new(Vec::new()))
    }
}

struct UnusedRunWorkflowPort;

impl RunWorkflowPort for UnusedRunWorkflowPort {
    fn execute(
        &self,
        _request: RunWorkflowRequest,
    ) -> Pin<Box<dyn Future<Output = Result<RunSummaryResponse, ApplicationError>> + Send + '_>>
    {
        Box::pin(async { Err(ApplicationError::Workflow("unused".to_string())) })
    }
}

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

#[tokio::test]
async fn failed_tui_run_renders_complete_log_path_when_execution_returns_error() {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    let temp_root = std::env::temp_dir().join(format!("ephact-tui-failure-{timestamp}"));
    let handler = FailureLogHandler::with_temp_root(&temp_root);
    let run_id = "run-tui-failure";
    handler.handle(&Event::RunStarted(RunStartedPayload::new(
        run_id.to_string(),
        "/repo/project".to_string(),
    )));
    handler.handle(&Event::RunFailed(RunFailedPayload::new(
        run_id.to_string(),
        "/repo/project".to_string(),
        None,
        "workflow could not be read".to_string(),
    )));
    let expected_path = temp_root
        .join("ephact")
        .join("project")
        .join("failure-run-tui-failure.log");
    assert!(
        expected_path.exists(),
        "expected failure log was not written"
    );
    let runner = TuiRunner::new(
        Arc::new(EmptyListWorkflowsPort),
        Arc::new(EmptyListActionsPort),
        Arc::new(UnusedRunWorkflowPort),
    )
    .with_failure_log_path_store(handler.path_store());
    let mut app = TuiApp::new(Vec::new(), String::new());
    app.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.start_run();
    let handle = tokio::spawn(async { Err("workflow could not be read".to_string()) });
    let mut run_task = Some(RunTask { handle });
    let mut cancelled = false;
    tokio::task::yield_now().await;
    let result = runner
        .process_finished_run(&mut app, &mut run_task, &mut cancelled)
        .await;

    assert!(result.is_ok());
    let text = render_text(&app);
    let normalized_text = text.replace([' ', '│'], "");
    assert!(
        normalized_text.contains(&format!("Failurediagnostics:{}", expected_path.display())),
        "rendered TUI: {text:?}"
    );
    std::fs::remove_dir_all(temp_root).expect("test diagnostics should be removed");
}
