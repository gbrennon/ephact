use crate::application::dtos::requests::LoadWorkflowRequest;
use crate::application::errors::LoadWorkflowError;
use crate::domain::aggregates::Workflow;

pub trait LoadWorkflowPort: Send + Sync {
    fn execute(&self, request: LoadWorkflowRequest) -> Result<Workflow, LoadWorkflowError>;
}
