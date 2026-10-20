use std::sync::Arc;

use crate::{
    application::{
        dtos::{requests::ExecuteWorkflowRequest, responses::WorkflowExecutionResponse},
        errors::ExecuteWorkflowError,
        ports::{
            inbound::execute_workflow_port::ExecuteWorkflowPort,
            outbound::{WorkflowCommandHandlerPort, WorkflowSourcePort},
        },
    },
    domain::messages::commands::ExecuteWorkflowPayload,
    workflows::GithubContextBuilder,
};

pub struct WorkflowCommandHandler {
    executor: Box<dyn ExecuteWorkflowPort>,
    context_builder: GithubContextBuilder,
}

impl WorkflowCommandHandler {
    pub fn new(executor: Box<dyn ExecuteWorkflowPort>) -> Self {
        Self {
            executor,
            context_builder: GithubContextBuilder::default(),
        }
    }

    /// Replaces the filesystem source used to load event payloads.
    pub fn with_workflow_source(self, workflow_source: Arc<dyn WorkflowSourcePort>) -> Self {
        Self {
            context_builder: GithubContextBuilder::new(workflow_source),
            ..self
        }
    }

    pub fn handle(
        &self,
        cmd: ExecuteWorkflowPayload,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError> {
        let github_context = self
            .context_builder
            .build_with_payload(&cmd)
            .map_err(ExecuteWorkflowError::Workflow)?;
        let (context, event_payload) = github_context.into_parts();
        let req = ExecuteWorkflowRequest::new(
            cmd.workflow_content().to_string(),
            cmd.repository().path().as_path().to_path_buf(),
            context,
            cmd.run_id().to_string(),
            cmd.allow_repo_writes(),
        )
        .with_event_payload_opt(event_payload.as_deref())
        .with_allow_network(cmd.config().allow_network())
        .with_selected_job_opt(cmd.config().job().map(|job| job.as_str()));
        let req = match cmd.workflow_file_name() {
            Some(file_name) => req.with_file_name(file_name.to_owned()),
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
