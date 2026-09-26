use ephact::application::{
    dtos::{requests::DiscoverRunInputsRequest, responses::RunInputDeclarationResponse},
    errors::DiscoverRunInputsError,
    ports::outbound::RunInputsDiscovererPort,
};
pub struct FakeRunInputsDiscovererPort;

impl FakeRunInputsDiscovererPort {
    pub fn new() -> Self {
        Self
    }
}

impl RunInputsDiscovererPort for FakeRunInputsDiscovererPort {
    fn discover(
        &self,
        _request: DiscoverRunInputsRequest,
    ) -> Result<Vec<RunInputDeclarationResponse>, DiscoverRunInputsError> {
        Ok(Vec::new())
    }
}
