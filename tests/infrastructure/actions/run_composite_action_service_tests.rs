#[cfg(test)]
mod tests {
    use ephact::{
        application::ports::outbound::run_composite_action_port::RunCompositeActionPort,
        infrastructure::actions::run_composite_action_service::RunCompositeActionService,
    };
    use std::{
        collections::HashMap,
        path::{Path, PathBuf},
        sync::Arc,
    };

    use ephact::application::dtos::requests::ExecuteActionRequest;
    use ephact::application::dtos::requests::RunCompositeActionRequest;
    use ephact::application::dtos::responses::ExecResultResponse;
    use ephact::domain::{entities::Step, errors::StepError, value_objects::EvaluationContext};

    use crate::common::fakes::{
        fake_run_composite_step_port::FakeRunCompositeStepPort, stub_container::StubContainer,
    };
    use ephact::infrastructure::workflows::yaml::StepYaml;

    fn steps(yaml: &str) -> Vec<Step> {
        serde_yaml::from_str::<Vec<StepYaml>>(yaml)
            .unwrap()
            .into_iter()
            .map(StepYaml::into_domain)
            .collect()
    }

    fn action_request() -> ExecuteActionRequest {
        ExecuteActionRequest::new(
            "./actions/outer",
            serde_yaml::from_str::<StepYaml>("uses: ./actions/outer\n")
                .unwrap()
                .into_domain(),
            PathBuf::from("/repo"),
            HashMap::new(),
            EvaluationContext::new(),
            Arc::new(StubContainer),
        )
    }

    fn result(exit_code: i64, stdout: &str) -> ExecResultResponse {
        ExecResultResponse::new(exit_code, stdout, String::new())
    }

    #[test]
    fn execute_runs_every_step_and_concatenates_their_output() {
        let runner = FakeRunCompositeStepPort::queueing(vec![result(0, "one"), result(0, "two")]);
        let service = RunCompositeActionService::new(Box::new(runner.clone()));
        let request_owner = action_request();

        let response = service
            .execute(RunCompositeActionRequest::new(
                &steps("- run: one\n- run: two\n"),
                &HashMap::new(),
                Path::new("/repo/actions/outer"),
                &request_owner,
            ))
            .unwrap();

        assert_eq!(response.exit_code(), 0);
        assert_eq!(response.stdout(), "onetwo");
        assert_eq!(runner.steps().len(), 2);
    }

    #[test]
    fn execute_stops_at_the_first_failing_step() {
        let runner = FakeRunCompositeStepPort::queueing(vec![result(3, "one"), result(0, "two")]);
        let service = RunCompositeActionService::new(Box::new(runner.clone()));
        let request_owner = action_request();

        let response = service
            .execute(RunCompositeActionRequest::new(
                &steps("- run: one\n- run: two\n"),
                &HashMap::new(),
                Path::new("/repo/actions/outer"),
                &request_owner,
            ))
            .unwrap();

        assert_eq!(response.exit_code(), 3);
        assert_eq!(response.stdout(), "one");
        assert_eq!(runner.steps().len(), 1);
    }

    #[test]
    fn execute_carries_earlier_output_into_a_step_error() {
        let runner = FakeRunCompositeStepPort::failing(
            StepError::new("boom".to_string())
                .with_stdout("partial".to_string())
                .with_stderr("bad".to_string()),
        );
        let service = RunCompositeActionService::new(Box::new(runner));
        let request_owner = action_request();

        let error = service
            .execute(RunCompositeActionRequest::new(
                &steps("- run: one\n"),
                &HashMap::new(),
                Path::new("/repo/actions/outer"),
                &request_owner,
            ))
            .unwrap_err();

        assert_eq!(error.message(), "boom");
        assert_eq!(error.stdout(), "partial");
        assert_eq!(error.stderr(), "bad");
    }

    #[test]
    fn execute_exposes_the_actions_inputs_to_its_steps() {
        let runner = FakeRunCompositeStepPort::queueing(vec![result(0, "")]);
        let service = RunCompositeActionService::new(Box::new(runner.clone()));
        let request_owner = action_request();
        let mut inputs = HashMap::new();
        inputs.insert("mode".to_string(), "staging".to_string());

        service
            .execute(RunCompositeActionRequest::new(
                &steps("- run: deploy ${{ inputs.mode }}\n"),
                &inputs,
                Path::new("/repo/actions/outer"),
                &request_owner,
            ))
            .unwrap();

        assert_eq!(runner.steps()[0].run(), Some("deploy staging"));
    }
}
