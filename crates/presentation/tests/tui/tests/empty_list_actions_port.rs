use crate::application::{
    dtos::{requests::ListActionsRequest, responses::ListActionsResponse},
    errors::ListActionsError,
    ports::inbound::ListActionsPort,
};

pub struct EmptyListActionsPort;

impl ListActionsPort for EmptyListActionsPort {
    fn execute(
        &self,
        _request: ListActionsRequest,
    ) -> Result<ListActionsResponse, ListActionsError> {
        Ok(ListActionsResponse::new(Vec::new()))
    }
}
