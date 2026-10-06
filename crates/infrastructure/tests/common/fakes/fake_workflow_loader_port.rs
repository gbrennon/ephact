use std::sync::Arc;

use ephact::{
    application::{
        errors::LoadWorkflowError, ports::outbound::workflow_loader_port::WorkflowLoaderPort,
    },
    domain::aggregates::Workflow,
    infrastructure::workflows::actions::WorkflowYaml,
};
use parking_lot::Mutex;

/// Parses a prepared YAML document instead of reading one from disk.
#[derive(Clone)]
pub struct FakeWorkflowLoaderPort {
    yaml: Result<String, String>,
    loaded_contents: Arc<Mutex<Vec<String>>>,
}

impl FakeWorkflowLoaderPort {
    pub fn holding(yaml: &str) -> Self {
        Self {
            yaml: Ok(yaml.to_string()),
            loaded_contents: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn failing(message: &str) -> Self {
        Self {
            yaml: Err(message.to_string()),
            loaded_contents: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn loaded_contents(&self) -> Vec<String> {
        self.loaded_contents.lock().clone()
    }
}

impl WorkflowLoaderPort for FakeWorkflowLoaderPort {
    fn load(&self, workflow_content: &str, file_name: &str) -> Result<Workflow, LoadWorkflowError> {
        self.loaded_contents
            .lock()
            .push(workflow_content.to_string());
        match &self.yaml {
            Ok(yaml) => serde_yaml::from_str::<WorkflowYaml>(yaml)
                .map(|workflow| workflow.into_domain().with_file(file_name))
                .map_err(|error| LoadWorkflowError::Parse(error.to_string())),
            Err(message) => Err(LoadWorkflowError::Message(message.clone())),
        }
    }
}
