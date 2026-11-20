use crate::application::{
    dtos::{requests::DiscoverRunInputsRequest, responses::RunInputDeclarationResponse},
    errors::DiscoverRunInputsError,
    ports::outbound::RunInputsDiscovererPort,
};

pub struct EmptyRunInputDiscovery;

impl RunInputsDiscovererPort for EmptyRunInputDiscovery {
    fn discover(
        &self,
        _request: DiscoverRunInputsRequest,
    ) -> Result<Vec<RunInputDeclarationResponse>, DiscoverRunInputsError> {
        Ok(Vec::new())
    }
}
