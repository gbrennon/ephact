use crate::{
    application::{dtos::requests::LoadWorkflowRequest, errors::LoadWorkflowError},
    domain::aggregates::Workflow,
};

pub trait LoadWorkflowPort: Send + Sync {
    fn execute(&self, request: LoadWorkflowRequest) -> Result<Workflow, LoadWorkflowError>;
}
