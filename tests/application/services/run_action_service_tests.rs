use std::{collections::HashMap, path::PathBuf, sync::Arc};

use ephact::{
    application::{
        dtos::{ExecuteActionResponse, RunActionRequest},
        ports::inbound::RunActionPort,
        services::run_action_service::RunActionService,
    },
    domain::value_objects::EvaluationContext,
};

use crate::common::fakes::{fake_command_bus::FakeCommandBus, stub_container::StubContainer};
use ephact::infrastructure::workflows::yaml::StepYaml;

#[test]
fn execute_delegates_action_execution_to_command_bus() {
    let command_bus = Arc::new(FakeCommandBus::new().with_action_result(
        ExecuteActionResponse::new(0, "action executed".to_string(), String::new()),
    ));

    let service = RunActionService::new(command_bus.clone());
    let step = serde_yaml::from_str::<StepYaml>("uses: actions/checkout@v4")
        .unwrap()
        .into_domain();

    let request = RunActionRequest::new(
        "actions/checkout@v4".into(),
        step,
        PathBuf::from("/repo"),
        HashMap::new(),
        EvaluationContext::new(),
        Arc::new(StubContainer),
    );

    let response = service.execute(request).unwrap();

    assert_eq!(response.exit_code(), 0);
    assert_eq!(response.stdout(), "action executed");
    assert_eq!(command_bus.dispatched_actions.lock().len(), 1);
}
