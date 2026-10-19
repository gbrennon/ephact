use tokio::task::{JoinError, JoinHandle};

use crate::application::dtos::responses::RunSummaryResponse;

/// Owns an asynchronous workflow run and its identifier.
pub struct RunTask {
    handle: JoinHandle<Result<RunSummaryResponse, String>>,
    run_id: String,
}

impl RunTask {
    /// Creates a task from its asynchronous execution handle and run identifier.
    pub fn new(handle: JoinHandle<Result<RunSummaryResponse, String>>, run_id: String) -> Self {
        Self { handle, run_id }
    }

    /// Aborts the asynchronous workflow run.
    pub fn abort(&self) {
        self.handle.abort();
    }

    /// Reports whether the asynchronous workflow run has completed.
    pub fn is_finished(&self) -> bool {
        self.handle.is_finished()
    }

    /// Returns the workflow run identifier.
    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    /// Waits for the asynchronous workflow run and returns its result.
    pub async fn await_result(self) -> Result<Result<RunSummaryResponse, String>, JoinError> {
        self.handle.await
    }
}
