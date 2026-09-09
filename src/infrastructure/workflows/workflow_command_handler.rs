use crate::application::ports::inbound::execute_workflow_port::ExecuteWorkflowPort;
use std::error::Error;

use crate::application::commands::ExecuteWorkflowCommand;
use crate::application::dtos::ExecuteWorkflowRequest;
use crate::application::dtos::WorkflowExecution;
use crate::domain::expression::EvalContext;
use super::super::containers::workspace::CONTAINER_WORKSPACE;
use serde_json::{Map, Value};

pub struct WorkflowCommandHandler {
    executor: Box<dyn ExecuteWorkflowPort>,
}

impl WorkflowCommandHandler {
    pub fn new(executor: Box<dyn ExecuteWorkflowPort>) -> Self {
        Self { executor }
    }

    fn build_context(cmd: &ExecuteWorkflowCommand) -> EvalContext {
        let secrets: Map<String, Value> = cmd
            .config()
            .secrets()
            .iter()
            .map(|secret| {
                (
                    secret.name().to_string(),
                    Value::String(secret.value().into()),
                )
            })
            .collect();
        let inputs: Map<String, Value> = cmd
            .config()
            .inputs()
            .iter()
            .map(|input| (input.key().to_string(), Value::String(input.value().into())))
            .collect();
        let event_name = cmd
            .config()
            .event()
            .map_or("workflow_dispatch", |event| event.as_str());

        let mut event = Map::new();
        event.insert("inputs".into(), Value::Object(inputs.clone()));
        let repo_name = cmd.repository().name().as_str().to_string();

        let mut github = Map::new();
        github.insert("event_name".into(), Value::String(event_name.into()));
        github.insert("repository".into(), Value::String(repo_name));
        github.insert(
            "workspace".into(),
            Value::String(CONTAINER_WORKSPACE.into()),
        );
        github.insert("event".into(), Value::Object(event));

        let mut runner = Map::new();
        runner.insert("os".into(), Value::String("Linux".into()));
        runner.insert("arch".into(), Value::String("X64".into()));
        runner.insert("temp".into(), Value::String("/tmp".into()));

        EvalContext::new()
            .with_secrets(Value::Object(secrets))
            .with_inputs(Value::Object(inputs))
            .with_github(Value::Object(github))
            .with_runner(Value::Object(runner))
    }
    pub fn handle(&self, cmd: ExecuteWorkflowCommand) -> Result<WorkflowExecution, Box<dyn Error>> {
        let context = Self::build_context(&cmd);
        let req = ExecuteWorkflowRequest::new(cmd.workflow_content(), cmd.repository().path().as_path(), &context);
        self.executor.execute(req)
    }
}
