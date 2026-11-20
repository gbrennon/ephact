use tokio::task::JoinHandle;

use crate::application::dtos::responses::ExecuteActionResponse;

pub struct ActionTask {
    pub(crate) handle: JoinHandle<Result<ExecuteActionResponse, String>>,
}
