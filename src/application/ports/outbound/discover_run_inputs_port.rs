use std::error::Error;

use crate::application::dtos::{DiscoverRunInputsRequest, RunInputDeclaration};

pub trait DiscoverRunInputsPort {
    fn execute(
        &self,
        request: DiscoverRunInputsRequest,
    ) -> Result<Vec<RunInputDeclaration>, Box<dyn Error>>;
}
