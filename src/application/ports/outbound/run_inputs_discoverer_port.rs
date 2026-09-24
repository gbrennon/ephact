use crate::application::{
    dtos::{requests::DiscoverRunInputsRequest, responses::RunInputDeclarationResponse},
    errors::DiscoverRunInputsError,
};
/// Discovers input declarations required by a run.
pub trait RunInputsDiscovererPort {
    fn discover(
        &self,
        request: DiscoverRunInputsRequest,
    ) -> Result<Vec<RunInputDeclarationResponse>, DiscoverRunInputsError>;
}
