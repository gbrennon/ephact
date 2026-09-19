use std::env;
use std::time::Duration;

use ephact::{application::dtos::responses::RunSummaryResponse, presentation::tui::RunHandler};

use crate::{
    common::fakes::stub_run_workflow_port::StubRunWorkflowPort,
    fakes::recording_run_workflow_port::RecordingRunWorkflowPort,
};

fn run_summary(success: bool) -> RunSummaryResponse {
    RunSummaryResponse::new("CI", vec![], success, Duration::from_secs(1))
}

fn current_repository_path() -> std::path::PathBuf {
    env::current_dir()
        .expect("current directory")
        .canonicalize()
        .expect("canonical repository path")
}

#[test]
fn handler_returns_summary_from_port() {
    let summary = run_summary(true);
    let port = RecordingRunWorkflowPort::new(summary.clone());

    let response = RunHandler::handle(
        &port,
        env::current_dir().expect("current directory"),
        Some("CI".to_string()),
    )
    .expect("repository should be valid");

    assert_eq!(response, summary);
}

#[test]
fn handler_executes_port_with_selected_workflow_and_safe_defaults() {
    let port = RecordingRunWorkflowPort::new(run_summary(true));

    RunHandler::handle(
        &port,
        env::current_dir().expect("current directory"),
        Some("CI".to_string()),
    )
    .expect("repository should be valid");

    let request = port.recorded_request().expect("recorded request");
    assert_eq!(request.repository_path(), current_repository_path());
    assert_eq!(request.workflow(), Some("CI"));
    assert!(request.job().is_none());
    assert!(request.event().is_none());
    assert!(request.inputs().is_empty());
    assert!(request.secrets().is_empty());
    assert!(!request.all_workflows());
    assert!(!request.allow_repo_writes());
    assert!(!request.allow_real_container());
    assert!(!request.allow_real_fetcher());
    assert!(!request.allow_network());
    assert!(!request.run_id().is_empty());
}

#[test]
fn handler_executes_port_without_workflow_when_selection_is_unnamed() {
    let port = RecordingRunWorkflowPort::new(run_summary(true));

    RunHandler::handle(&port, env::current_dir().expect("current directory"), None)
        .expect("repository should be valid");

    let request = port.recorded_request().expect("recorded request");
    assert!(request.workflow().is_none());
    assert!(!request.all_workflows());
}

#[test]
fn handler_returns_error_on_invalid_repo_path() {
    let port = RecordingRunWorkflowPort::new(run_summary(true));

    let result = RunHandler::handle(&port, "/definitely/not/a/repository".into(), None);

    assert!(result.is_err());
}

#[test]
fn handler_returns_error_when_port_fails() {
    let port = StubRunWorkflowPort {
        result: Err("workflow failed".to_string()),
    };

    let result = RunHandler::handle(
        &port,
        env::current_dir().expect("current directory"),
        Some("CI".to_string()),
    );

    assert!(result.is_err());
}
