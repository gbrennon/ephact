use ephact_infrastructure::application::{
    dtos::{requests::RunActionRequest, responses::ExecuteActionResponse},
    errors::RunActionError,
    ports::inbound::RunActionPort,
};

pub struct FailingAction;

impl RunActionPort for FailingAction {
    fn execute(&self, _: RunActionRequest) -> Result<ExecuteActionResponse, RunActionError> {
        Ok(ExecuteActionResponse::builder()
            .exit_code(7)
            .stderr("failed".into())
            .build())
    }
}
