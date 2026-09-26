use crate::{
    dtos::{requests::RunActionRequest, responses::ExecuteActionResponse},
    errors::RunActionError,
};

pub trait RunActionPort {
    fn execute(&self, request: RunActionRequest) -> Result<ExecuteActionResponse, RunActionError>;
}
