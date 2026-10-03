use std::collections::BTreeMap;

use super::super::containers::workspace::CONTAINER_WORKSPACE;
use crate::{
    application::{
        dtos::{requests::ExecuteWorkflowRequest, responses::WorkflowExecutionResponse},
        errors::ExecuteWorkflowError,
        ports::{
            inbound::execute_workflow_port::ExecuteWorkflowPort,
            outbound::WorkflowCommandHandlerPort,
        },
    },
    domain::{
        messages::commands::ExecuteWorkflowPayload,
        value_objects::{ContextValue, EvaluationContext},
    },
};

pub struct WorkflowCommandHandler {
    executor: Box<dyn ExecuteWorkflowPort>,
}

impl WorkflowCommandHandler {
    pub fn new(executor: Box<dyn ExecuteWorkflowPort>) -> Self {
        Self { executor }
    }

    fn build_context(cmd: &ExecuteWorkflowPayload) -> EvaluationContext {
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
            .with_root("secrets", secrets)
            .with_root("inputs", ContextValue::Mapping(inputs))
            .with_root("github", github)
            .with_root("runner", runner_context())
    }
    pub fn handle(
        &self,
        cmd: ExecuteWorkflowPayload,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError> {
        let context = Self::build_context(&cmd);
        let req = ExecuteWorkflowRequest::new(
            cmd.workflow_content().to_string(),
            cmd.repository().path().as_path().to_path_buf(),
            context,
            cmd.run_id().to_string(),
            cmd.allow_repo_writes(),
        )
        .with_allow_network(cmd.config().allow_network());
        let req = match cmd.workflow_file_name() {
            Some(file_name) => req.with_file_name(file_name),
            None => req,
        };
        self.executor.execute(req)
    }
}

impl WorkflowCommandHandlerPort for WorkflowCommandHandler {
    fn handle(
        &self,
        command: ExecuteWorkflowPayload,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError> {
        WorkflowCommandHandler::handle(self, command)
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
