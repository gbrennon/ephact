use std::path::PathBuf;

use ephact::{
    domain::{errors::ActionError, value_objects::RemoteActionReference},
    infrastructure::actions::ActionFetcherPort,
};

#[derive(Clone)]
pub struct FailingPipelineActionFetcher {
    action_directory: PathBuf,
}

impl FailingPipelineActionFetcher {
    pub fn mirroring(action_directory: PathBuf) -> Self {
        Self { action_directory }
    }
}

impl ActionFetcherPort for FailingPipelineActionFetcher {
    fn fetch(&self, _reference: &RemoteActionReference) -> Result<PathBuf, ActionError> {
        Ok(self.action_directory.clone())
    }

    fn clone_box(&self) -> Box<dyn ActionFetcherPort> {
        Box::new(self.clone())
    }
}
