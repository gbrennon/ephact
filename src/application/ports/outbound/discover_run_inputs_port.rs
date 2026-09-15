use crate::application::dtos::requests::DiscoverRunInputsRequest;
use crate::application::dtos::responses::RunInputDeclarationResponse;
use crate::application::errors::DiscoverRunInputsError;

pub trait DiscoverRunInputsPort {
    fn execute(
        &self,
        request: DiscoverRunInputsRequest,
    ) -> Result<Vec<RunInputDeclarationResponse>, DiscoverRunInputsError>;
}
