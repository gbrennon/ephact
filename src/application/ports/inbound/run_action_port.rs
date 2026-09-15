use crate::application::dtos::requests::RunActionRequest;
use crate::application::dtos::responses::ExecuteActionResponse;
use crate::application::errors::RunActionError;

pub trait RunActionPort {
    fn execute(&self, request: RunActionRequest) -> Result<ExecuteActionResponse, RunActionError>;
}
