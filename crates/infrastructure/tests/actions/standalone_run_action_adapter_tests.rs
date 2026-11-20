use std::{collections::HashMap, sync::Arc};

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
#[path = "recording_events.rs"]
mod recording_events;

use failing_action::FailingAction;
use fake_action::FakeAction;
use fake_preparer::FakePreparer;
use recording_events::RecordingEvents;

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
    assert!(matches!(events.first(), Some(Event::ContainerStarted(_))));
    let run_id = match &events[0] {
        Event::ContainerStarted(payload) => payload.run_id().to_owned(),
        _ => unreachable!(),
    };
    assert!(
        matches!(events.get(1), Some(Event::WorkflowRunCompleted(payload))
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
    assert!(
        matches!(events.get(1), Some(Event::WorkflowRunCompleted(payload))
        if !payload.success())
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
