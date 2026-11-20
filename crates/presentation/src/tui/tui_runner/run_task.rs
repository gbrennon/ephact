use tokio::task::JoinHandle;

use crate::application::dtos::responses::RunSummaryResponse;

pub struct RunTask {
    pub(crate) handle: JoinHandle<Result<RunSummaryResponse, String>>,
    pub(crate) run_id: String,
}
