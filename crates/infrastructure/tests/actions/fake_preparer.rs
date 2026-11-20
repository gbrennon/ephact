use ephact_infrastructure::application::{
    dtos::{requests::PrepareJobContainerRequest, responses::PreparedJobContainerResponse},
    errors::PrepareJobContainerError,
    ports::outbound::JobContainerPreparerPort,
};

use super::fake_container::FakeContainer;

pub struct FakePreparer;

impl JobContainerPreparerPort for FakePreparer {
    fn prepare(
        &self,
        _: PrepareJobContainerRequest,
    ) -> Result<PreparedJobContainerResponse, PrepareJobContainerError> {
        Ok(PreparedJobContainerResponse::new(
            Box::new(FakeContainer),
            "standalone-action".into(),
        ))
    }
}
