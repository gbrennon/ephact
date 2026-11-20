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
            outbound::{
                ContainerPort, DomainEventPublisherPort, JobContainerPreparerPort,
                StepTextCodecPort,
            },
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
    assert!(
        events
            .iter()
            .any(|event| matches!(event, Event::ContainerStarted(_)))
    );
    assert!(
        events.iter().any(
            |event| matches!(event, Event::WorkflowRunCompleted(payload) if payload.success())
        )
    );
}
