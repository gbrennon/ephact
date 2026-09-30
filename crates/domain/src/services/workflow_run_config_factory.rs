use crate::{
    services::workflow_run_config_input::WorkflowRunConfigInput, value_objects::WorkflowRunConfig,
};

pub struct WorkflowRunConfigFactory;

impl WorkflowRunConfigFactory {
    pub fn create(input: WorkflowRunConfigInput) -> WorkflowRunConfig {
        input.into_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_builds_config_from_input() {
        let input = WorkflowRunConfigInput::default().with_event(Some("push".into()));

        let config = WorkflowRunConfigFactory::create(input);

        assert_eq!(config.event().unwrap().as_str(), "push");
    }
}
