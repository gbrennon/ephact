use crate::{
    application::{
        dtos::requests::LoadWorkflowRequest, errors::LoadWorkflowError,
        ports::outbound::workflow_loader_port::WorkflowLoaderPort,
    },
    domain::aggregates::Workflow,
    infrastructure::workflows::yaml::WorkflowYaml,
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
        let parsed: WorkflowYaml =
            serde_yaml::from_str(request.workflow_content()).map_err(LoadWorkflowError::Parse)?;
        Ok(parsed.into_domain())
    }
}
