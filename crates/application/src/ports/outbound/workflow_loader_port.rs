use crate::{
    domain::aggregates::Workflow, dtos::requests::LoadWorkflowRequest, errors::LoadWorkflowError,
};
/// Loads and parses a workflow document.
pub trait WorkflowLoaderPort: Send + Sync {
    fn load(&self, request: LoadWorkflowRequest) -> Result<Workflow, LoadWorkflowError>;
}
