use crate::dtos::requests::PullJobImageRequest;

/// Makes a job image available through the container runtime.
pub trait PullJobImagePort: Send + Sync {
    /// Pulls the requested image and returns its image reference on success.
    ///
    /// # Errors
    ///
    /// Returns an error when the runtime cannot pull the image.
    fn pull(&self, request: PullJobImageRequest) -> Result<String, Box<dyn std::error::Error>>;
}
