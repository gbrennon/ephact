use std::{path::Path, sync::Arc};

use ephact::{
    application::{
        dtos::{RunSummary, RunWorkflowRequest, WorkflowExecution},
        ports::inbound::RunWorkflowPort,
        services::run_workflow_service::RunWorkflowService,
    },
    domain::{ActRunConfig, RepoPath, Repository, RepositoryName, events::DomainEvent},
};

use crate::common::fakes::{
    fake_command_bus::FakeCommandBus,
    fake_detect_workflow_trigger_port::FakeDetectWorkflowTriggerPort, fake_event_bus::FakeEventBus,
    fake_workflow_source::FakeWorkflowSource,
};

fn make_repo(path: &Path) -> Repository {
    let git_dir = path.join(".git");
    if !git_dir.exists() {
        std::fs::create_dir_all(&git_dir).ok();
    }
    let repo_path = RepoPath::new(path.to_path_buf()).unwrap();
    let name = RepositoryName::new("test-repo".to_string()).unwrap();
    Repository::new(repo_path, name)
}

#[test]
fn execute_runs_workflow_and_publishes_event() {
    let temp = tempfile::tempdir().unwrap();
    let repo = make_repo(temp.path());

    let workflow_source =
        FakeWorkflowSource::new().with_workflow_content("name: CI\non: pull_request\njobs: {}");
    let command_bus = Arc::new(
        FakeCommandBus::new().with_workflow_result(WorkflowExecution::new(
            "CI".to_string(),
            Vec::new(),
            vec!["test-container-1".to_string()],
            true,
        )),
    );
    let event_bus = Arc::new(FakeEventBus::new());

    let service = RunWorkflowService::new(
        Box::new(workflow_source),
        command_bus.clone(),
        event_bus.clone(),
        Arc::new(FakeDetectWorkflowTriggerPort::always_triggering()),
    );
    let request = RunWorkflowRequest::new(ActRunConfig::new(), repo);

    let summary: RunSummary = service.execute(request).unwrap();

    assert_eq!(summary.name(), "CI");
    assert!(summary.success());
    assert_eq!(command_bus.dispatched_workflows.lock().len(), 1);
    let dispatched = command_bus.dispatched_workflows.lock();
    assert_eq!(
        dispatched[0].config().event().map(|event| event.as_str()),
        Some("pull_request")
    );

    let events = event_bus.events();
    assert_eq!(events.len(), 1);
    let DomainEvent::ActRunCompleted(payload) = &events[0] else {
        panic!("expected ActRunCompleted event");
    };
    assert!(payload.success());
    assert_eq!(
        payload.container_names(),
        vec!["test-container-1".to_string()]
    );
}

#[test]
fn execute_rejects_workflows_without_pull_request_event() {
    let temp = tempfile::tempdir().unwrap();
    let repo = make_repo(temp.path());
    let workflow_source =
        FakeWorkflowSource::new().with_workflow_content("name: CI\non: merge_group\njobs: {}");
    let command_bus = Arc::new(FakeCommandBus::new());
    let event_bus = Arc::new(FakeEventBus::new());
    let service = RunWorkflowService::new(
        Box::new(workflow_source),
        command_bus.clone(),
        event_bus,
        Arc::new(FakeDetectWorkflowTriggerPort::never_triggering()),
    );
    let request = RunWorkflowRequest::new(ActRunConfig::new(), repo);

    let error = service.execute(request).unwrap_err();

    assert!(error.to_string().contains("pull_request"));
    assert!(command_bus.dispatched_workflows.lock().is_empty());
}
