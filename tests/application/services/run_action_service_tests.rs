#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf, sync::Arc};

    use ephact::application::dtos::requests::{
        RunActionExecutionInput, RunActionRequest, RunActionRequestInput,
    };
    use ephact::application::dtos::responses::ExecuteActionResponse;
    use ephact::application::ports::inbound::RunActionPort;
    use ephact::application::services::run_action_service::RunActionService;
    use ephact::domain::services::evaluation_context_mapper::EvaluationContextMapper;
    use ephact::domain::services::step_factory::StepFactory;
    use ephact::domain::value_objects::EvaluationContext;

    use crate::common::fakes::{fake_command_bus::FakeCommandBus, stub_container::StubContainer};
    use ephact::infrastructure::workflows::yaml::StepYaml;

    #[test]
    fn execute_delegates_action_execution_to_command_bus() {
        let command_bus = FakeCommandBus::new().with_action_result(ExecuteActionResponse::new(
            0,
            "action executed".to_string(),
            String::new(),
        ));

        let container = Arc::new(StubContainer);
        let service = RunActionService::new(Box::new(command_bus.clone()), container);
        let step = serde_yaml::from_str::<StepYaml>("uses: actions/checkout@v4")
            .unwrap()
            .into_domain();

        let request = RunActionRequest::new(RunActionRequestInput::new(
            "actions/checkout@v4",
            StepFactory::to_text(&step).unwrap(),
            RunActionExecutionInput::new(
                PathBuf::from("/repo"),
                HashMap::new(),
                EvaluationContextMapper::to_parts(&EvaluationContext::new()),
            ),
        ));
        let response = service.execute(request).unwrap();

        assert_eq!(response.exit_code(), 0);
        assert_eq!(response.stdout(), "action executed");
        assert_eq!(command_bus.dispatched_actions.lock().len(), 1);
    }
}
