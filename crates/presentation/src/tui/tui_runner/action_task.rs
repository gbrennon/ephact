use tokio::task::JoinHandle;

use crate::application::dtos::responses::ExecuteActionResponse;

pub struct ActionTask {
    handle: JoinHandle<Result<ExecuteActionResponse, String>>,
}

impl ActionTask {
    pub(crate) fn new(handle: JoinHandle<Result<ExecuteActionResponse, String>>) -> Self {
        Self { handle }
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.handle.is_finished()
    }

    pub(crate) fn abort(&self) {
        self.handle.abort();
    }

    pub(crate) fn into_handle(self) -> JoinHandle<Result<ExecuteActionResponse, String>> {
        self.handle
    }
}
