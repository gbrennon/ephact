use std::{
    collections::HashMap,
    fs,
    sync::{Arc, Mutex},
};

use ephact_infrastructure::{
    actions::{RunActionFactory, StandaloneRunActionAdapter},
    application::{
        dtos::requests::{RunActionRequest, RunActionRequestInput},
        ports::inbound::RunActionPort,
    },
    domain::messages::events::Event,
    steps::JsonStepTextCodec,
};

#[path = "failing_action.rs"]
mod failing_action;
#[path = "fake_action.rs"]
mod fake_action;
#[path = "fake_container.rs"]
mod fake_container;
#[path = "fake_preparer.rs"]
mod fake_preparer;
#[path = "recording_action.rs"]
mod recording_action;
#[path = "recording_events.rs"]
mod recording_events;

use failing_action::FailingAction;
use fake_action::FakeAction;
use fake_preparer::FakePreparer;
use recording_action::RecordingAction;
use recording_events::RecordingEvents;

fn request_at(repository_path: impl Into<std::path::PathBuf>) -> RunActionRequest {
    let codec = JsonStepTextCodec;
    RunActionRequest::new(
        RunActionRequestInput::for_action(
            "./.forgejo/actions/ci-validate".into(),
            HashMap::new(),
            repository_path.into(),
            &codec,
        )
        .unwrap(),
    )
}

fn request() -> RunActionRequest {
    let codec = JsonStepTextCodec;
    RunActionRequest::new(
        RunActionRequestInput::for_action(
            "./action".into(),
            HashMap::new(),
            "/repo".into(),
            &codec,
        )
        .unwrap(),
    )
}

#[test]
fn prepares_container_runs_factory_action_and_publishes_lifecycle_events() {
    let events = Arc::new(RecordingEvents::default());
    let factory: RunActionFactory = Box::new(|_| Box::new(FakeAction));
    let adapter = StandaloneRunActionAdapter::new(Box::new(FakePreparer), factory, events.clone());

    let response = adapter.execute(request()).expect("action should run");

    assert_eq!(response.stdout(), "done");
    let events = events.0.lock().unwrap();
    let run_id = match events.first() {
        Some(Event::RunStarted(payload)) => payload.run_id().to_owned(),
        _ => panic!("expected RunStarted first"),
    };
    assert!(matches!(events.get(1), Some(Event::ContainerStarted(_))));
    assert!(matches!(events.get(2), Some(Event::StepFinished(payload))
        if payload.success() && payload.stdout() == "done"));
    assert!(
        matches!(events.get(3), Some(Event::WorkflowRunCompleted(payload))
        if payload.success()
            && payload.run_id() == run_id
            && payload.repository_path() == "/repo"
            && payload.container_names() == ["standalone-action"])
    );
}

#[test]
fn nonzero_action_response_publishes_failed_lifecycle_event() {
    let events = Arc::new(RecordingEvents::default());
    let factory: RunActionFactory = Box::new(|_| Box::new(FailingAction));
    let adapter = StandaloneRunActionAdapter::new(Box::new(FakePreparer), factory, events.clone());

    let response = adapter
        .execute(request())
        .expect("action response should return");

    assert_eq!(response.exit_code(), 7);
    let events = events.0.lock().unwrap();
    assert!(matches!(events.first(), Some(Event::RunStarted(_))));
    assert!(matches!(events.get(1), Some(Event::ContainerStarted(_))));
    assert!(matches!(events.get(2), Some(Event::StepFinished(payload))
        if !payload.success()
            && payload.exit_code() == Some(7)
            && payload.stdout().is_empty()
            && payload.stderr() == "failed"));
    assert!(
        matches!(events.get(3), Some(Event::WorkflowRunCompleted(payload))
        if !payload.success())
    );
}

#[test]
fn standalone_action_runs_repository_dependency_setup_first() {
    let repository = tempfile::tempdir().expect("temporary repository");
    let dependency_directory = repository
        .path()
        .join(".forgejo/actions/install-dependencies");
    fs::create_dir_all(&dependency_directory).expect("dependency directory");
    fs::write(
        dependency_directory.join("action.yml"),
        "name: dependencies",
    )
    .expect("dependency action");
    fs::create_dir_all(repository.path().join(".forgejo/actions/ci-validate"))
        .expect("selected action directory");
    fs::write(
        repository
            .path()
            .join(".forgejo/actions/ci-validate/action.yml"),
        "name: validate",
    )
    .expect("selected action");

    let refs = Arc::new(Mutex::new(Vec::new()));
    let factory_refs = refs.clone();
    let factory: RunActionFactory = Box::new(move |_| {
        Box::new(RecordingAction {
            refs: factory_refs.clone(),
        })
    });
    let events = Arc::new(RecordingEvents::default());
    let adapter = StandaloneRunActionAdapter::new(Box::new(FakePreparer), factory, events);

    adapter
        .execute(request_at(repository.path()))
        .expect("action should run");

    assert_eq!(
        *refs.lock().unwrap(),
        vec![
            "./.forgejo/actions/install-dependencies".to_owned(),
            "./.forgejo/actions/ci-validate".to_owned(),
        ]
    );
}

#[test]
fn standalone_action_run_ids_are_unique_for_consecutive_runs() {
    let events = Arc::new(RecordingEvents::default());
    let factory: RunActionFactory = Box::new(|_| Box::new(FakeAction));
    let adapter = StandaloneRunActionAdapter::new(Box::new(FakePreparer), factory, events.clone());

    adapter.execute(request()).expect("first action should run");
    adapter
        .execute(request())
        .expect("second action should run");

    let events = events.0.lock().unwrap();
    let ids: Vec<_> = events
        .iter()
        .filter_map(|event| match event {
            Event::ContainerStarted(payload) => Some(payload.run_id().to_owned()),
            _ => None,
        })
        .collect();
    assert_eq!(ids.len(), 2);
    assert_ne!(ids[0], ids[1]);
}
