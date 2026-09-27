use crate::{
    services::workflow_run_config_input::WorkflowRunConfigInput, value_objects::WorkflowRunConfig,
};

pub struct WorkflowRunConfigFactory;

impl WorkflowRunConfigFactory {
    pub fn create(input: WorkflowRunConfigInput) -> WorkflowRunConfig {
        input.into_config()
    }
}
