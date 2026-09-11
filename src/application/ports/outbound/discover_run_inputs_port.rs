use std::error::Error;

use crate::application::dtos::requests::DiscoverRunInputsRequest;
use crate::application::dtos::responses::RunInputDeclarationResponse;

pub trait DiscoverRunInputsPort {
    fn execute(
        &self,
        request: DiscoverRunInputsRequest,
    ) -> Result<Vec<RunInputDeclarationResponse>, Box<dyn Error>>;
}
