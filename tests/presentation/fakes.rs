use ephact::application::{
    dtos::{DiscoverRunInputsRequest, RunInputDeclaration},
    ports::outbound::DiscoverRunInputsPort,
};

pub struct FakeDiscoverRunInputsPort;

impl FakeDiscoverRunInputsPort {
    pub fn new() -> Self {
        Self
    }
}

impl DiscoverRunInputsPort for FakeDiscoverRunInputsPort {
    fn execute(
        &self,
        _request: DiscoverRunInputsRequest,
    ) -> Result<Vec<RunInputDeclaration>, Box<dyn std::error::Error>> {
        Ok(Vec::new())
    }
}
