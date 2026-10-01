use crate::dtos::requests::PullJobImageRequest;

pub trait PullJobImagePort: Send + Sync {
    fn pull(&self, request: PullJobImageRequest) -> Result<String, Box<dyn std::error::Error>>;
}
