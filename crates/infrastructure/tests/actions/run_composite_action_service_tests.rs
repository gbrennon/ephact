use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use ephact::{
    application::{
        dtos::{
            requests::{
                ExecuteActionExecutionInput, ExecuteActionRequest, ExecuteActionRequestInput,
                RunCompositeActionRequest,
            },
            responses::ExecResultResponse,
        },
        ports::outbound::{
            StepTextCodecPort, composite_action_runner_port::CompositeActionRunnerPort,
            container_port::ContainerPort,
        },
    },
    domain::{
        entities::Step,
        errors::StepError,
        value_objects::{ContextValue, EvaluationContext},
    },
    infrastructure::{
        actions::execution::run_composite_action_service::RunCompositeActionService,
        steps::JsonStepTextCodec, workflows::actions::StepYaml,
    },
};

use crate::common::fakes::{
    fake_run_composite_step_port::FakeRunCompositeStepPort, stub_container::StubContainer,
};

fn steps(yaml: &str) -> Vec<Step> {
    serde_yaml::from_str::<Vec<StepYaml>>(yaml)
        .unwrap()
        .into_iter()
        .map(StepYaml::into_domain)
        .collect()
}

fn action_request(container: &dyn ContainerPort) -> ExecuteActionRequest {
    action_request_with_context(container, EvaluationContext::new())
}

fn action_request_with_context(
    _container: &dyn ContainerPort,
    context: EvaluationContext,
) -> ExecuteActionRequest {
    ExecuteActionRequest::new(ExecuteActionRequestInput::new(
        "./actions/outer",
        JsonStepTextCodec
            .encode(
                &serde_yaml::from_str::<StepYaml>("uses: ./actions/outer\n")
                    .unwrap()
                    .into_domain(),
            )
            .unwrap(),
        ExecuteActionExecutionInput::new(PathBuf::from("/repo"), HashMap::new(), context),
    ))
}

fn result(exit_code: i64, stdout: &str) -> ExecResultResponse {
    result_with_output(exit_code, stdout, "")
}

fn result_with_output(exit_code: i64, stdout: &str, stderr: &str) -> ExecResultResponse {
    ExecResultResponse::new(exit_code, stdout, stderr)
}

#[test]
fn execute_runs_every_step_and_concatenates_their_output() {
    let runner = FakeRunCompositeStepPort::queueing(vec![result(0, "one"), result(0, "two")]);
    let service = RunCompositeActionService::new(Box::new(runner.clone()));
    let container = StubContainer;
    let request_owner = action_request(&container);

    let response = service
        .run(
            RunCompositeActionRequest::new(
                &steps("- run: one\n- run: two\n"),
                &HashMap::new(),
                Path::new("/repo/actions/outer"),
                &request_owner,
            ),
            Arc::new(container),
        )
        .unwrap();

    assert_eq!(response.exit_code(), 0);
    assert_eq!(response.stdout(), "onetwo");
    assert_eq!(runner.steps().len(), 2);
}

#[test]
fn execute_stops_at_the_first_failing_step() {
    let runner = FakeRunCompositeStepPort::queueing(vec![result(3, "one"), result(0, "two")]);
    let service = RunCompositeActionService::new(Box::new(runner.clone()));
    let container = StubContainer;
    let request_owner = action_request(&container);

    let response = service
        .run(
            RunCompositeActionRequest::new(
                &steps("- run: one\n- run: two\n"),
                &HashMap::new(),
                Path::new("/repo/actions/outer"),
                &request_owner,
            ),
            Arc::new(container),
        )
        .unwrap();

    assert_eq!(response.exit_code(), 3);
    assert_eq!(response.stdout(), "one");
    assert_eq!(runner.steps().len(), 1);
}

#[test]
fn execute_carries_earlier_output_into_a_step_error() {
    let runner = FakeRunCompositeStepPort::sequence(vec![
        Ok(result_with_output(0, "one", "first")),
        Err(StepError::new("boom".to_string())
            .with_stdout("partial".to_string())
            .with_stderr("bad".to_string())),
    ]);
    let service = RunCompositeActionService::new(Box::new(runner));
    let container = StubContainer;
    let request_owner = action_request(&container);

    let error = service
        .run(
            RunCompositeActionRequest::new(
                &steps("- run: one\n- run: two\n"),
                &HashMap::new(),
                Path::new("/repo/actions/outer"),
                &request_owner,
            ),
            Arc::new(container),
        )
        .unwrap_err();

    assert_eq!(error.message(), "boom");
    assert_eq!(error.stdout(), "onepartial");
    assert_eq!(error.stderr(), "firstbad");
}

#[test]
fn execute_exposes_the_actions_inputs_to_its_steps() {
    let runner = FakeRunCompositeStepPort::queueing(vec![result(0, "")]);
    let service = RunCompositeActionService::new(Box::new(runner.clone()));
    let container = StubContainer;
    let request_owner = action_request(&container);
    let mut inputs = HashMap::new();
    inputs.insert("mode".to_string(), "staging".to_string());

    service
        .run(
            RunCompositeActionRequest::new(
                &steps("- run: deploy ${{ inputs.mode }}\n"),
                &inputs,
                Path::new("/repo/actions/outer"),
                &request_owner,
            ),
            Arc::new(container),
        )
        .unwrap();

    assert_eq!(runner.steps()[0].run(), Some("deploy staging"));
}

#[test]
fn execute_skips_steps_when_their_condition_is_false() {
    let runner = FakeRunCompositeStepPort::queueing(vec![result(0, "linux")]);
    let service = RunCompositeActionService::new(Box::new(runner.clone()));
    let container = StubContainer;
    let context = EvaluationContext::new().with_root(
        "runner",
        ContextValue::mapping([("os".to_owned(), ContextValue::text("Linux"))]),
    );
    let request_owner = action_request_with_context(&container, context);

    let response = service
        .run(
            RunCompositeActionRequest::new(
                &steps(
                    "- if: runner.os == 'Windows'\n  run: echo windows\n\
                     - run: echo linux\n",
                ),
                &HashMap::new(),
                Path::new("/repo/actions/outer"),
                &request_owner,
            ),
            Arc::new(container),
        )
        .unwrap();

    assert_eq!(response.exit_code(), 0);
    assert_eq!(runner.steps().len(), 1);
    assert_eq!(runner.steps()[0].run(), Some("echo linux"));
}
