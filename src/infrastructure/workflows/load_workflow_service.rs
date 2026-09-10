use crate::application::ports::outbound::load_workflow_port::LoadWorkflowPort;
use std::error::Error;

use crate::{
    application::dtos::LoadWorkflowRequest, domain::aggregates::Workflow,
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

impl LoadWorkflowPort for LoadWorkflowService {
    fn execute(&self, request: LoadWorkflowRequest<'_>) -> Result<Workflow, Box<dyn Error>> {
        let parsed: WorkflowYaml = serde_yaml::from_str(request.workflow_content())?;
        Ok(parsed.into_domain())
    }
}
