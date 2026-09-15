use ephact::application::dtos::requests::DiscoverRunInputsRequest;
use ephact::application::dtos::responses::RunInputDeclarationResponse;
use ephact::application::errors::DiscoverRunInputsError;
use ephact::application::ports::outbound::DiscoverRunInputsPort;
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
