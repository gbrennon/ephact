use std::{
    path::Path,
    task::{Context, Poll, Waker},
};

use ephact::{
    application::{
        dtos::{
            requests::RunWorkflowRequest,
            responses::{RunSummaryResponse, WorkflowExecutionResponse},
        },
        errors::ApplicationError,
        ports::inbound::RunWorkflowPort,
    },
    domain::{
        RepoPath, Repository, RepositoryName, WorkflowRunConfig,
        messages::commands::Event,
        value_objects::{WorkflowEvent, WorkflowPath},
    },
    infrastructure::workflows::execution::run_workflow_service::RunWorkflowService,
};

use crate::common::fakes::{
    fake_command_bus::FakeCommandBus,
    fake_detect_workflow_trigger_port::FakeDetectWorkflowTriggerPort, fake_event_bus::FakeEventBus,
    fake_workflow_source::FakeWorkflowSource,
};

fn execute_workflow(
    service: &RunWorkflowService,
    request: RunWorkflowRequest,
) -> Result<RunSummaryResponse, ApplicationError> {
    let mut future = service.execute(request);
    let mut context = Context::from_waker(Waker::noop());

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

fn make_repo(path: &Path) -> Repository {
    let git_dir = path.join(".git");
    if !git_dir.exists() {
        std::fs::create_dir_all(&git_dir).ok();
    }
    let repo_path = RepoPath::new(path.to_path_buf()).unwrap();
    let name = RepositoryName::new("test-repo".to_string()).unwrap();
    Repository::new(repo_path, name)
}

fn primitive_request(config: WorkflowRunConfig, repository: Repository) -> RunWorkflowRequest {
    RunWorkflowRequest::from_domain(&repository, &config, "test-run")
}

#[test]
fn execute_runs_workflow_and_publishes_lifecycle_events() {
    let temp = tempfile::tempdir().unwrap();
    let repo = make_repo(temp.path());

    let workflow_source = FakeWorkflowSource::new()
        .with_workflow_content("name: CI\non: pull_request\njobs: {}")
        .with_workflow_file_name("ci.yml");
    let command_bus = FakeCommandBus::new().with_workflow_result(WorkflowExecutionResponse::new(
        "CI".to_string(),
        Vec::new(),
        vec!["test-container-1".to_string()],
        true,
    ));
    let event_bus = FakeEventBus::new();
    let config = WorkflowRunConfig::new().with_event(WorkflowEvent::new("pull_request".to_owned()));
    let run_id = "test-run";
    let repository_path = temp.path().display().to_string();

    let service = RunWorkflowService::new(
        Box::new(workflow_source),
        Box::new(command_bus.clone()),
        Box::new(event_bus.clone()),
        Box::new(FakeDetectWorkflowTriggerPort::always_triggering()),
    );
    let request = primitive_request(config, repo);

    let summary: RunSummaryResponse = execute_workflow(&service, request).unwrap();

    assert_eq!(summary.name(), "CI");
    assert!(summary.success());
    let dispatched = command_bus.dispatched_workflows.lock();
    assert_eq!(dispatched.len(), 1);
    assert_eq!(dispatched[0].workflow_file_name(), Some("ci.yml"));

    assert_run_lifecycle_events(
        &event_bus.events(),
        run_id,
        &repository_path,
        vec!["test-container-1".to_string()],
    );
}

fn assert_run_lifecycle_events(
    events: &[Event],
    run_id: &str,
    repository_path: &str,
    container_names: Vec<String>,
) {
    assert_eq!(events.len(), 2);
    let Event::RunStarted(payload) = &events[0] else {
        panic!("expected RunStarted event");
    };
    assert_eq!(payload.run_id(), run_id);
    assert_eq!(payload.repository_path(), repository_path);
    let Event::WorkflowRunCompleted(payload) = &events[1] else {
        panic!("expected WorkflowRunCompleted event");
    };
    assert_eq!(payload.run_id(), run_id);
    assert_eq!(payload.repository_path(), repository_path);
    assert!(payload.success());
    assert_eq!(payload.container_names(), container_names);
}

#[test]
fn execute_publishes_run_failed_when_workflow_read_fails() {
    let temp = tempfile::tempdir().unwrap();
    let repo = make_repo(temp.path());
    let source = FakeWorkflowSource::new().failing_read_workflow("cannot read workflow");
    let event_bus = FakeEventBus::new();
    let config = WorkflowRunConfig::new();
    let run_id = "test-run";

    let service = RunWorkflowService::new(
        Box::new(source),
        Box::new(FakeCommandBus::new()),
        Box::new(event_bus.clone()),
        Box::new(FakeDetectWorkflowTriggerPort::always_triggering()),
    );

    let error = execute_workflow(&service, primitive_request(config, repo)).unwrap_err();

    assert_eq!(error.to_string(), "cannot read workflow");
    let events = event_bus.events();
    assert_eq!(events.len(), 2);
    let Event::RunFailed(payload) = &events[1] else {
        panic!("expected RunFailed event");
    };
    assert_eq!(payload.run_id(), run_id);
    assert_eq!(payload.error(), "cannot read workflow");
}

#[test]
fn execute_publishes_run_failed_when_event_is_missing() {
    let temp = tempfile::tempdir().unwrap();
    let repo = make_repo(temp.path());
    let workflow_source =
        FakeWorkflowSource::new().with_workflow_content("name: CI\non: merge_group\njobs: {}");
    let event_bus = FakeEventBus::new();
    let command_bus = FakeCommandBus::new();
    let config = WorkflowRunConfig::new();
    let run_id = "test-run";
    let service = RunWorkflowService::new(
        Box::new(workflow_source),
        Box::new(command_bus.clone()),
        Box::new(event_bus.clone()),
        Box::new(FakeDetectWorkflowTriggerPort::never_triggering()),
    );

    let error = execute_workflow(&service, primitive_request(config, repo)).unwrap_err();

    assert_eq!(error.to_string(), "workflow event must be specified");
    let events = event_bus.events();
    assert_eq!(events.len(), 2);
    let Event::RunFailed(payload) = &events[1] else {
        panic!("expected RunFailed event");
    };
    assert_eq!(payload.run_id(), run_id);
    assert_eq!(payload.error(), "workflow event must be specified");
    assert!(command_bus.dispatched_workflows.lock().is_empty());
}

#[test]
fn execute_rejects_workflows_without_an_explicit_event() {
    let temp = tempfile::tempdir().unwrap();
    let repo = make_repo(temp.path());
    let workflow_source =
        FakeWorkflowSource::new().with_workflow_content("name: CI\non: merge_group\njobs: {}");
    let command_bus = FakeCommandBus::new();
    let event_bus = FakeEventBus::new();
    let service = RunWorkflowService::new(
        Box::new(workflow_source),
        Box::new(command_bus.clone()),
        Box::new(event_bus),
        Box::new(FakeDetectWorkflowTriggerPort::never_triggering()),
    );
    let request = primitive_request(WorkflowRunConfig::new(), repo);

    let error = execute_workflow(&service, request).unwrap_err();

    assert_eq!(error.to_string(), "workflow event must be specified");
    assert!(command_bus.dispatched_workflows.lock().is_empty());
}

#[test]
fn execute_rejects_named_workflow_without_an_explicit_event() {
    let temp = tempfile::tempdir().unwrap();
    let repo = make_repo(temp.path());
    let workflow_source = FakeWorkflowSource::new();
    let command_bus = FakeCommandBus::new();
    let event_bus = FakeEventBus::new();
    let service = RunWorkflowService::new(
        Box::new(workflow_source),
        Box::new(command_bus.clone()),
        Box::new(event_bus),
        Box::new(FakeDetectWorkflowTriggerPort::never_triggering()),
    );
    let config = WorkflowRunConfig::new().with_workflow(WorkflowPath::new("ci.yml".to_owned()));

    let error = execute_workflow(&service, primitive_request(config, repo)).unwrap_err();

    assert_eq!(error.to_string(), "workflow event must be specified");
    assert!(command_bus.dispatched_workflows.lock().is_empty());
}
