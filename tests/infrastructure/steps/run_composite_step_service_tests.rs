#[cfg(test)]
mod tests {
    use ephact::infrastructure::steps::run_composite_step_port::RunCompositeStepPort;
    use std::{
        collections::HashMap,
        path::{Path, PathBuf},
    };

    use ephact::application::dtos::requests::ExecuteActionRequest;
    use ephact::application::dtos::requests::RunCompositeStepRequest;
    use ephact::application::dtos::responses::ExecuteActionResponse;
    use ephact::domain::entities::Step;
    use ephact::domain::value_objects::EvaluationContext;
    use ephact::infrastructure::steps::run_composite_step_service::RunCompositeStepService;
    use ephact::infrastructure::steps::run_shell_step_service::RunShellStepService;

    use crate::common::fakes::{
        fake_command_bus::FakeCommandBus, fake_event_bus::FakeEventBus,
        stub_failing_container::StubFailingContainer,
        stub_recording_container::StubRecordingContainer,
    };
    use ephact::infrastructure::workflows::yaml::StepYaml;

    fn step_from(yaml: &str) -> Step {
        serde_yaml::from_str::<StepYaml>(yaml)
            .unwrap()
            .into_domain()
    }

    fn action_request<'a>(
        container: &'a dyn ephact::application::ports::outbound::container_port::ContainerPort,
    ) -> ExecuteActionRequest<'a> {
        ExecuteActionRequest::new(
            "./actions/outer".to_string(),
            step_from("uses: ./actions/outer\n"),
            PathBuf::from("/repo"),
            HashMap::new(),
            EvaluationContext::new(),
            container,
        )
    }

    fn action_response() -> ExecuteActionResponse {
        ExecuteActionResponse::new(0, "nested\n".to_string(), String::new())
    }

    fn service(command_bus: FakeCommandBus) -> RunCompositeStepService {
        RunCompositeStepService::new(
            Box::new(RunShellStepService::new(Box::new(FakeEventBus::new()))),
            Box::new(command_bus),
        )
    }

    #[test]
    fn execute_runs_a_run_step_with_the_action_path_exposed() {
        let container = StubRecordingContainer::new();
        let request = action_request(&container);
        let step = step_from("run: echo hi\n");
        let service = service(FakeCommandBus::new());

        service
            .execute(RunCompositeStepRequest::new(
                &step,
                Path::new("/repo/actions/outer"),
                &request,
                &EvaluationContext::new(),
            ))
            .unwrap();

        assert_eq!(
            container.exec_environments()[0]
                .get("GITHUB_ACTION_PATH")
                .map(String::as_str),
            Some("/repo/actions/outer")
        );
    }

    #[test]
    fn execute_publishes_an_action_command_for_a_uses_step() {
        let container = StubRecordingContainer::new();
        let request = action_request(&container);
        let step = step_from("uses: ./actions/inner\n");
        let command_bus = FakeCommandBus::new().with_action_result(action_response());
        let service = service(command_bus.clone());

        let result = service
            .execute(RunCompositeStepRequest::new(
                &step,
                Path::new("/repo/actions/outer"),
                &request,
                &EvaluationContext::new(),
            ))
            .unwrap();

        assert_eq!(result.stdout(), "nested\n");
        let dispatched = command_bus.dispatched_actions.lock();
        assert_eq!(dispatched.len(), 1);
        assert_eq!(dispatched[0].action_ref(), "./actions/inner");
        assert_eq!(dispatched[0].repo_path(), Path::new("/repo"));
        assert!(container.executed_commands().is_empty());
    }

    #[test]
    fn execute_propagates_a_shell_runner_failure() {
        let container = StubFailingContainer;
        let request = action_request(&container);
        let step = step_from("run: echo hi\n");
        let service = service(FakeCommandBus::new());

        let error = service
            .execute(RunCompositeStepRequest::new(
                &step,
                Path::new("/repo/actions/outer"),
                &request,
                &EvaluationContext::new(),
            ))
            .unwrap_err();

        assert!(
            error.message().contains("exec refused"),
            "{}",
            error.message()
        );
    }
}
