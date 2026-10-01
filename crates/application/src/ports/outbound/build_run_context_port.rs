use crate::dtos::{requests::BuildRunContextRequest, responses::BuildRunContextResponse};

pub trait BuildRunContextPort: Send + Sync {
    fn build(&self, request: BuildRunContextRequest) -> BuildRunContextResponse;
}
