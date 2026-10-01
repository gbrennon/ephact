use crate::dtos::{
    requests::BuildActionInputEnvironmentRequest, responses::BuildActionInputEnvironmentResponse,
};

pub trait BuildActionInputEnvironmentPort: Send + Sync {
    fn build(
        &self,
        request: BuildActionInputEnvironmentRequest,
    ) -> BuildActionInputEnvironmentResponse;
}
