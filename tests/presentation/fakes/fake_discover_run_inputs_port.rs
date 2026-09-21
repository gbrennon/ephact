use ephact::application::{
    dtos::{requests::DiscoverRunInputsRequest, responses::RunInputDeclarationResponse},
    errors::DiscoverRunInputsError,
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
    ) -> Result<Vec<RunInputDeclarationResponse>, DiscoverRunInputsError> {
        Ok(Vec::new())
    }
}
