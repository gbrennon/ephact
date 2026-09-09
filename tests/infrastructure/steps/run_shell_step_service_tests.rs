use std::sync::Arc;

use ephact::{
    application::ports::outbound::run_shell_step_port::RunShellStepPort,
    infrastructure::steps::run_shell_step_service::RunShellStepService,
};
use std::collections::HashMap;

use ephact::application::dtos::ContainerConfig;
use ephact::application::dtos::ExecResult;
use ephact::application::dtos::RunShellStepRequest;
use ephact::application::dtos::RunnerContext;
use ephact::application::ports::outbound::ContainerRuntimePort;
use ephact::application::ports::outbound::container_port::ContainerPort;
use ephact::domain::workflow::Step;

use crate::common::fakes::{
    fake_event_bus::FakeEventBus, fake_runtime::FakeRuntime,
    stub_failing_container::StubFailingContainer,
};

fn container(runtime: &dyn ContainerRuntimePort) -> Box<dyn ContainerPort> {
    runtime
        .create_container(&ContainerConfig::new(
            "image",
            None,
            HashMap::new(),
            vec![],
            None,
            None,
            None,
            None,
            None,
            RunnerContext::default(),
        ))
        .unwrap()
}

fn step_from(yaml: &str) -> Step {
    serde_yaml::from_str(yaml).unwrap()
}

#[test]
fn execute_runs_the_steps_script_through_bash() {
    let runtime = FakeRuntime::new();
    runtime
        .exec_results
        .lock()
        .push(ExecResult::new(0, "hi\n", String::new()));
    let container = container(&runtime);
    let step = step_from("run: echo hi\n");

    let result = RunShellStepService::new(Arc::new(FakeEventBus::new()))
        .execute(RunShellStepRequest::new(
            &step,
            container.as_ref(),
            &HashMap::new(),
        ))
        .unwrap();

    assert_eq!(result.exit_code(), 0);
    assert_eq!(result.stdout(), "hi\n");
    assert_eq!(runtime.executed_scripts(), vec!["echo hi".to_string()]);
}

#[test]
fn execute_lets_the_steps_own_env_override_the_passed_env() {
    let runtime = FakeRuntime::new();
    let container = container(&runtime);
    let step = step_from("run: echo hi\nenv:\n  MODE: step\n");
    let mut env = HashMap::new();
    env.insert("MODE".to_string(), "job".to_string());

    RunShellStepService::new(Arc::new(FakeEventBus::new()))
        .execute(RunShellStepRequest::new(&step, container.as_ref(), &env))
        .unwrap();

    let environments = runtime.exec_environments.lock();
    assert_eq!(
        environments[0].get("MODE").map(String::as_str),
        Some("step")
    );
}

#[test]
fn execute_errors_when_the_step_has_neither_run_nor_uses() {
    let runtime = FakeRuntime::new();
    let container = container(&runtime);
    let step = step_from("name: nothing to run\n");

    let error = RunShellStepService::new(Arc::new(FakeEventBus::new()))
        .execute(RunShellStepRequest::new(
            &step,
            container.as_ref(),
            &HashMap::new(),
        ))
        .unwrap_err();

    assert_eq!(error.message(), "step has neither `run` nor `uses` defined");
}

#[test]
fn execute_reports_a_container_failure_as_a_step_error() {
    let step = step_from("run: echo hi\n");
    let container = StubFailingContainer;

    let error = RunShellStepService::new(Arc::new(FakeEventBus::new()))
        .execute(RunShellStepRequest::new(&step, &container, &HashMap::new()))
        .unwrap_err();

    assert!(
        error.message().contains("exec refused"),
        "{}",
        error.message()
    );
}
