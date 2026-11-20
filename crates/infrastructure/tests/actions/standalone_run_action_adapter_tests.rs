use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use ephact_infrastructure::{
    actions::{RunActionFactory, StandaloneRunActionAdapter},
    application::{
        dtos::{
            requests::{PrepareJobContainerRequest, RunActionRequest, RunActionRequestInput},
            responses::{
                ExecResultResponse, ExecuteActionResponse, PreparedJobContainerResponse,
                RunnerContextResponse,
            },
        },
        errors::{PrepareJobContainerError, RunActionError},
        ports::{
            inbound::RunActionPort,
            outbound::{ContainerPort, DomainEventPublisherPort, JobContainerPreparerPort},
        },
    },
    domain::{
        entities::FileEntry,
        errors::ContainerError,
        messages::events::{Event, OutputStream},
    },
    steps::JsonStepTextCodec,
};

struct FakeContainer;
impl ContainerPort for FakeContainer {
    fn exec(
        &self,
        _: &[String],
        _: Option<&str>,
        _: &HashMap<String, String>,
    ) -> Result<ExecResultResponse, ContainerError> {
        unimplemented!()
    }
    fn exec_streaming(
        &self,
        _: ephact_infrastructure::application::dtos::requests::ExecOptions<'_>,
        _: &mut dyn FnMut(OutputStream, &str),
    ) -> Result<ExecResultResponse, ContainerError> {
        unimplemented!()
    }
    fn copy_to(&self, _: &str, _: &[FileEntry]) -> Result<(), ContainerError> {
        unimplemented!()
    }
    fn copy_from(&self, _: &str) -> Result<Vec<FileEntry>, ContainerError> {
        unimplemented!()
    }
    fn remove(&self) -> Result<(), ContainerError> {
        unimplemented!()
    }
    fn get_runner_context(&self) -> Result<RunnerContextResponse, ContainerError> {
        unimplemented!()
    }
}

struct FakePreparer;
impl JobContainerPreparerPort for FakePreparer {
    fn prepare(
        &self,
        _: PrepareJobContainerRequest,
    ) -> Result<PreparedJobContainerResponse, PrepareJobContainerError> {
        Ok(PreparedJobContainerResponse::new(
            Box::new(FakeContainer),
            "standalone-action".into(),
        ))
    }
}

struct FakeAction;
impl RunActionPort for FakeAction {
    fn execute(&self, _: RunActionRequest) -> Result<ExecuteActionResponse, RunActionError> {
        Ok(ExecuteActionResponse::note("done".into()))
    }
}

struct FailingAction;
impl RunActionPort for FailingAction {
    fn execute(&self, _: RunActionRequest) -> Result<ExecuteActionResponse, RunActionError> {
        Ok(ExecuteActionResponse::builder()
            .exit_code(7)
            .stderr("failed".into())
            .build())
    }
}

#[derive(Default)]
struct RecordingEvents(Mutex<Vec<Event>>);
impl DomainEventPublisherPort for RecordingEvents {
    fn publish(&self, event: Event) {
        self.0.lock().unwrap().push(event);
    }
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
