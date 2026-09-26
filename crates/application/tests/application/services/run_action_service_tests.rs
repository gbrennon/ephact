#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf, sync::Arc};

    use ephact::{
        application::{
            dtos::{
                requests::{RunActionExecutionInput, RunActionRequest, RunActionRequestInput},
                responses::ExecuteActionResponse,
            },
            ports::{inbound::RunActionPort, outbound::StepTextCodecPort},
            services::run_action_service::RunActionService,
        },
        domain::{
            services::evaluation_context_mapper::EvaluationContextMapper,
            value_objects::EvaluationContext,
        },
        infrastructure::{steps::JsonStepTextCodec, workflows::yaml::StepYaml},
    };

    use crate::common::fakes::{fake_command_bus::FakeCommandBus, stub_container::StubContainer};

    #[test]
    fn execute_delegates_action_execution_to_command_bus() {
        let command_bus = FakeCommandBus::new().with_action_result(ExecuteActionResponse::new(
            0,
            "action executed".to_string(),
            String::new(),
        ));

        let container = Arc::new(StubContainer);
        let service = RunActionService::new(
            Box::new(command_bus.clone()),
            container,
            Arc::new(JsonStepTextCodec),
        );
        let step = serde_yaml::from_str::<StepYaml>("uses: actions/checkout@v4")
            .unwrap()
            .into_domain();

        let request = RunActionRequest::new(RunActionRequestInput::new(
            "actions/checkout@v4",
            JsonStepTextCodec.encode(&step).unwrap(),
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
