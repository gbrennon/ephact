use crate::{
    application::{
        dtos::requests::LoadWorkflowRequest, errors::LoadWorkflowError,
        ports::outbound::workflow_loader_port::WorkflowLoaderPort,
    },
    domain::aggregates::Workflow,
    workflows::workflow_document::WorkflowDocument,
};

pub struct LoadWorkflowService;

impl LoadWorkflowService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LoadWorkflowService {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkflowLoaderPort for LoadWorkflowService {
    fn load(&self, request: LoadWorkflowRequest) -> Result<Workflow, LoadWorkflowError> {
        let workflow = WorkflowDocument::parse(request.workflow_content())
            .map_err(|error| LoadWorkflowError::Parse(error.to_string()))?
            .into_domain()
            .with_file(request.file_name().to_string());
        Ok(workflow)
    }
}
