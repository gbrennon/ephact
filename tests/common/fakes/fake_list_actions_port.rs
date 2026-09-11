#![allow(dead_code)]
use ephact::application::dtos::requests::ListActionsRequest;
use ephact::application::dtos::responses::ListActionsResponse;
use ephact::application::ports::inbound::list_actions_port::ListActionsPort;

#[derive(Clone)]
pub struct FakeListActionsPort;

impl FakeListActionsPort {
    pub fn new() -> Self {
        Self
    }
}

impl ListActionsPort for FakeListActionsPort {
    fn execute(
        &self,
        _request: ListActionsRequest,
    ) -> Result<ListActionsResponse, Box<dyn std::error::Error>> {
        Ok(ListActionsResponse::new(vec![]))
    }
}
