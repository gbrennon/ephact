use std::{collections::HashMap, path::Path, sync::Arc};

use ephact::application::dtos::ExecResult;
use ephact::application::dtos::ExecuteActionResponse;
use ephact::application::dtos::ExecuteStepRequest;
use ephact::application::ports::inbound::execute_step_port::ExecuteStepPort;
use ephact::application::services::execute_step_service::ExecuteStepService;
use ephact::domain::errors::StepError;
use ephact::domain::expression::EvalContext;
use ephact::domain::workflow::Step;
use serde_json::Value;

use crate::common::fakes::{
    fake_command_bus::FakeCommandBus, fake_run_shell_step_port::FakeRunShellStepPort,
    stub_container::StubContainer,
};

fn step_from(yaml: &str) -> Step {
    serde_yaml::from_str(yaml).unwrap()
}

fn shell_result(stdout: &str) -> ExecResult {
    ExecResult::new(0, stdout.to_string(), String::new())
}

fn action_response() -> ExecuteActionResponse {
    ExecuteActionResponse::new(0, "action\n".to_string(), String::new())
}

fn service(shell: FakeRunShellStepPort, command_bus: FakeCommandBus) -> ExecuteStepService {
    ExecuteStepService::new(Box::new(shell), Arc::new(command_bus))
}

#[test]
fn execute_runs_a_run_step_through_the_shell_runner() {
    let shell =
        FakeRunShellStepPort::returning(ExecResult::new(3, "out".to_string(), "err".to_string()));
    let service = service(shell.clone(), FakeCommandBus::new());
    let step = step_from("run: echo hi\n");

    let executed = service
        .execute(ExecuteStepRequest::new(
            &step,
            &EvalContext::new(),
            Arc::new(StubContainer),
            Path::new("/repo"),
            &HashMap::new(),
        ))
        .unwrap();

    assert_eq!(executed.response().exit_code(), 3);
    assert_eq!(executed.response().stdout(), "out");
    assert_eq!(executed.response().stderr(), "err");
    assert_eq!(shell.steps().len(), 1);
}

#[test]
fn execute_publishes_an_action_command_for_a_uses_step() {
    let command_bus = FakeCommandBus::new().with_action_result(action_response());
    let service = service(
        FakeRunShellStepPort::returning(shell_result("")),
        command_bus.clone(),
    );
    let step = step_from("uses: ./actions/greet\n");
    let mut env = HashMap::new();
    env.insert("MODE".to_string(), "staging".to_string());

    let executed = service
        .execute(ExecuteStepRequest::new(
            &step,
            &EvalContext::new(),
            Arc::new(StubContainer),
            Path::new("/repo"),
            &env,
        ))
        .unwrap();

    assert_eq!(executed.response().stdout(), "action\n");
    let dispatched = command_bus.dispatched_actions.lock();
    assert_eq!(dispatched.len(), 1);
    assert_eq!(dispatched[0].action_ref(), "./actions/greet");
    assert_eq!(dispatched[0].repo_path(), Path::new("/repo"));
    assert_eq!(dispatched[0].env(), &env);
}

#[test]
fn execute_does_not_publish_an_action_command_for_a_run_step() {
    let command_bus = FakeCommandBus::new();
    let service = service(
        FakeRunShellStepPort::returning(shell_result("")),
        command_bus.clone(),
    );
    let step = step_from("run: echo hi\n");

    service
        .execute(ExecuteStepRequest::new(
            &step,
            &EvalContext::new(),
            Arc::new(StubContainer),
            Path::new("/repo"),
            &HashMap::new(),
        ))
        .unwrap();

    assert!(command_bus.dispatched_actions.lock().is_empty());
}

#[test]
fn execute_resolves_expressions_before_running_the_step() {
    let shell = FakeRunShellStepPort::returning(shell_result(""));
    let service = service(shell.clone(), FakeCommandBus::new());
    let step = step_from("run: deploy ${{ inputs.mode }}\n");
    let mut context = EvalContext::new();
    let mut inputs = serde_json::Map::new();
    inputs.insert("mode".to_string(), Value::String("staging".into()));
    context = context.with_inputs(Value::Object(inputs));

    service
        .execute(ExecuteStepRequest::new(
            &step,
            &context,
            Arc::new(StubContainer),
            Path::new("/repo"),
            &HashMap::new(),
        ))
        .unwrap();

    assert_eq!(shell.steps()[0].run().as_deref(), Some("deploy staging"));
}

#[test]
fn execute_reports_an_interpolation_failure() {
    let service = service(
        FakeRunShellStepPort::returning(shell_result("")),
        FakeCommandBus::new(),
    );
    let step = step_from("run: deploy ${{ }}\n");

    let error = service
        .execute(ExecuteStepRequest::new(
            &step,
            &EvalContext::new(),
            Arc::new(StubContainer),
            Path::new("/repo"),
            &HashMap::new(),
        ))
        .unwrap_err();

    assert!(
        error
            .message()
            .starts_with("failed to resolve expressions:"),
        "{}",
        error.message()
    );
}

#[test]
fn execute_propagates_a_collaborator_error_unchanged() {
    let service = service(
        FakeRunShellStepPort::failing(
            StepError::new("boom")
                .with_stdout("partial")
                .with_stderr("bad"),
        ),
        FakeCommandBus::new(),
    );
    let step = step_from("run: echo hi\n");

    let error = service
        .execute(ExecuteStepRequest::new(
            &step,
            &EvalContext::new(),
            Arc::new(StubContainer),
            Path::new("/repo"),
            &HashMap::new(),
        ))
        .unwrap_err();

    assert_eq!(error.message(), "boom");
    assert_eq!(error.stdout(), "partial");
    assert_eq!(error.stderr(), "bad");
}
