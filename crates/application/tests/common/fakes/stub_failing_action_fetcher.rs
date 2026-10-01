use std::path::PathBuf;

use ephact::{
    application::ports::outbound::ActionFetcherPort,
    domain::{errors::ActionError, value_objects::RemoteActionReference},
};

#[derive(Clone)]
pub struct StubFailingActionFetcher;

impl ActionFetcherPort for StubFailingActionFetcher {
    fn fetch(&self, _reference: &RemoteActionReference) -> Result<PathBuf, ActionError> {
        Err(ActionError::FetchFailed("source is unreachable".to_owned()))
    }

    fn clone_box(&self) -> Box<dyn ActionFetcherPort> {
        Box::new(self.clone())
    }
}
