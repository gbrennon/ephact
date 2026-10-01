use std::path::PathBuf;

use crate::domain::{errors::ActionError, value_objects::RemoteActionReference};

pub trait ActionFetcherPort: Send + Sync {
    fn fetch(&self, reference: &RemoteActionReference) -> Result<PathBuf, ActionError>;
    fn clone_box(&self) -> Box<dyn ActionFetcherPort>;
}

impl Clone for Box<dyn ActionFetcherPort> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
