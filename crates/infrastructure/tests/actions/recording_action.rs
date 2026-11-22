use std::sync::{Arc, Mutex};

use ephact_infrastructure::application::{
    dtos::responses::ExecuteActionResponse, errors::RunActionError, ports::inbound::RunActionPort,
};

pub struct RecordingAction {
    pub refs: Arc<Mutex<Vec<String>>>,
}

impl RunActionPort for RecordingAction {
    fn execute(
        &self,
        request: ephact_infrastructure::application::dtos::requests::RunActionRequest,
    ) -> Result<ExecuteActionResponse, RunActionError> {
        self.refs
            .lock()
            .unwrap()
            .push(request.action_ref().to_owned());
        Ok(ExecuteActionResponse::note("done".to_owned()))
    }
}
