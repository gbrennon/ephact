use crate::{
    application::{dtos::requests::LoadWorkflowRequest, errors::LoadWorkflowError},
    domain::aggregates::Workflow,
};
/// Loads and parses a workflow document.
pub trait WorkflowLoaderPort: Send + Sync {
    fn load(&self, request: LoadWorkflowRequest) -> Result<Workflow, LoadWorkflowError>;
}
