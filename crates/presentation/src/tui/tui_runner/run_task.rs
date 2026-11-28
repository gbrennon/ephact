use tokio::task::JoinHandle;

use crate::application::dtos::responses::RunSummaryResponse;

pub struct RunTask {
    handle: JoinHandle<Result<RunSummaryResponse, String>>,
    run_id: String,
}

impl RunTask {
    pub(crate) fn new(
        handle: JoinHandle<Result<RunSummaryResponse, String>>,
        run_id: String,
    ) -> Self {
        Self { handle, run_id }
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.handle.is_finished()
    }

    pub(crate) fn abort(&self) {
        self.handle.abort();
    }

    pub(crate) fn run_id(&self) -> &str {
        &self.run_id
    }

    pub(crate) fn into_parts(self) -> (JoinHandle<Result<RunSummaryResponse, String>>, String) {
        (self.handle, self.run_id)
    }
}
