#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::Path};

    use ephact::application::dtos::requests::ExecuteStepRequest;
    use ephact::application::dtos::responses::ExecResultResponse;
    use ephact::application::dtos::responses::ExecuteActionResponse;
    use ephact::application::ports::inbound::execute_step_port::ExecuteStepPort;
    use ephact::application::services::execute_step_service::ExecuteStepService;
    use ephact::domain::entities::Step;
    use ephact::domain::errors::StepError;
    use ephact::domain::value_objects::ContextValue;
    use ephact::domain::value_objects::EvaluationContext;

    use crate::common::fakes::{
        fake_command_bus::FakeCommandBus, fake_run_shell_step_port::FakeRunShellStepPort,
        stub_container::StubContainer,
    };
    use ephact::infrastructure::workflows::yaml::StepYaml;

    fn step_from(yaml: &str) -> Step {
        serde_yaml::from_str::<StepYaml>(yaml)
            .unwrap()
            .into_domain()
    }

    fn shell_result(stdout: &str) -> ExecResultResponse {
        ExecResultResponse::new(0, stdout.to_string(), String::new())
    }

    fn action_response() -> ExecuteActionResponse {
        ExecuteActionResponse::new(0, "action\n".to_string(), String::new())
    }

    fn service(shell: FakeRunShellStepPort, command_bus: FakeCommandBus) -> ExecuteStepService {
        ExecuteStepService::new(Box::new(shell), Box::new(command_bus))
    }

    #[test]
    fn execute_runs_a_run_step_through_the_shell_runner() {
        let shell = FakeRunShellStepPort::returning(ExecResultResponse::new(
            3,
            "out".to_string(),
            "err".to_string(),
        ));
        let service = service(shell.clone(), FakeCommandBus::new());
        let step = step_from("run: echo hi\n");
        let container = StubContainer;

        let executed = service
            .execute(ExecuteStepRequest::new(
                &step,
                &EvaluationContext::new(),
                &container,
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
        let container = StubContainer;

        let executed = service
            .execute(ExecuteStepRequest::new(
                &step,
                &EvaluationContext::new(),
                &container,
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
        let container = StubContainer;

        service
            .execute(ExecuteStepRequest::new(
                &step,
                &EvaluationContext::new(),
                &container,
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
        let inputs = ContextValue::mapping([("mode".to_string(), ContextValue::text("staging"))]);
        let context = EvaluationContext::new().with_inputs(inputs);
        let container = StubContainer;

        service
            .execute(ExecuteStepRequest::new(
                &step,
                &context,
                &container,
                Path::new("/repo"),
                &HashMap::new(),
            ))
            .unwrap();

        assert_eq!(shell.steps()[0].run(), Some("deploy staging"));
    }

    #[test]
    fn execute_reports_an_interpolation_failure() {
        let service = service(
            FakeRunShellStepPort::returning(shell_result("")),
            FakeCommandBus::new(),
        );
        let step = step_from("run: deploy ${{ }}\n");
        let container = StubContainer;

        let error = service
            .execute(ExecuteStepRequest::new(
                &step,
                &EvaluationContext::new(),
                &container,
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
        let container = StubContainer;

        let error = service
            .execute(ExecuteStepRequest::new(
                &step,
                &EvaluationContext::new(),
                &container,
                Path::new("/repo"),
                &HashMap::new(),
            ))
            .unwrap_err();

        assert_eq!(error.message(), "boom");
        assert_eq!(error.stdout(), "partial");
        assert_eq!(error.stderr(), "bad");
    }
}
