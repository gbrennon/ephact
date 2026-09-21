use crate::application::{
    dtos::{requests::DiscoverRunInputsRequest, responses::RunInputDeclarationResponse},
    errors::DiscoverRunInputsError,
};

pub trait DiscoverRunInputsPort {
    fn execute(
        &self,
        request: DiscoverRunInputsRequest,
    ) -> Result<Vec<RunInputDeclarationResponse>, DiscoverRunInputsError>;
}
