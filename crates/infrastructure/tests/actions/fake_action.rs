use ephact_infrastructure::application::{
    dtos::{requests::RunActionRequest, responses::ExecuteActionResponse},
    errors::RunActionError,
    ports::inbound::RunActionPort,
};

pub struct FakeAction;

impl RunActionPort for FakeAction {
    fn execute(&self, _: RunActionRequest) -> Result<ExecuteActionResponse, RunActionError> {
        Ok(ExecuteActionResponse::note("done".into()))
    }
}
