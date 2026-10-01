use crate::{
    domain::errors::StepError,
    dtos::{requests::CollectActionFilesRequest, responses::CollectActionFilesResponse},
};

pub trait CollectActionFilesPort: Send + Sync {
    fn collect(
        &self,
        request: CollectActionFilesRequest,
    ) -> Result<CollectActionFilesResponse, StepError>;
}
