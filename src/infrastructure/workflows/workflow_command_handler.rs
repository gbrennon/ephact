use crate::application::ports::inbound::execute_workflow_port::ExecuteWorkflowPort;
use std::error::Error;

use super::super::containers::workspace::CONTAINER_WORKSPACE;
use crate::application::dtos::requests::ExecuteWorkflowRequest;
use crate::application::dtos::responses::WorkflowExecutionResponse;
use crate::domain::messages::commands::ExecuteWorkflowCommand;
use crate::domain::value_objects::ContextValue;
use crate::domain::value_objects::EvaluationContext;
use std::collections::BTreeMap;

pub struct WorkflowCommandHandler {
    executor: Box<dyn ExecuteWorkflowPort>,
}

impl WorkflowCommandHandler {
    pub fn new(executor: Box<dyn ExecuteWorkflowPort>) -> Self {
        Self { executor }
    }

    fn build_context(cmd: &ExecuteWorkflowCommand) -> EvaluationContext {
        let secrets = ContextValue::mapping(cmd.config().secrets().iter().map(|secret| {
            (
                secret.name().to_string(),
                ContextValue::text(secret.value()),
            )
        }));
        let inputs: BTreeMap<String, ContextValue> = cmd
            .config()
            .inputs()
            .iter()
            .map(|input| (input.key().to_string(), ContextValue::text(input.value())))
            .collect();
        let event_name = cmd
            .config()
            .event()
            .map_or("workflow_dispatch", |event| event.as_str());

        let event =
            ContextValue::mapping([("inputs".to_owned(), ContextValue::Mapping(inputs.clone()))]);

        let github = ContextValue::mapping([
            ("event_name".to_owned(), ContextValue::text(event_name)),
            (
                "repository".to_owned(),
                ContextValue::text(cmd.repository().name().as_str()),
            ),
            (
                "workspace".to_owned(),
                ContextValue::text(CONTAINER_WORKSPACE),
            ),
            ("event".to_owned(), event),
        ]);

        EvaluationContext::new()
            .with_secrets(secrets)
            .with_inputs(ContextValue::Mapping(inputs))
            .with_github(github)
            .with_runner(runner_context())
    }
    pub fn handle(
        &self,
        cmd: ExecuteWorkflowCommand,
    ) -> Result<WorkflowExecutionResponse, Box<dyn Error>> {
        let context = Self::build_context(&cmd);
        let req = ExecuteWorkflowRequest::new(
            cmd.workflow_content(),
            cmd.repository().path().as_path(),
            &context,
        );
        self.executor.execute(req)
    }
}

/// Returns the runner facts every workflow run sees in the `runner` context.
fn runner_context() -> ContextValue {
    ContextValue::mapping([
        ("os".to_owned(), ContextValue::text("Linux")),
        ("arch".to_owned(), ContextValue::text("X64")),
        ("temp".to_owned(), ContextValue::text("/tmp")),
    ])
}
