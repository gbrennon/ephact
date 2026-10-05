/// Requests pulling one container image by its registry reference.
pub struct PullJobImageRequest {
    image: String,
}

impl PullJobImageRequest {
    /// Creates a request for the supplied container image.
    pub fn new(image: String) -> Self {
        Self { image }
    }

    /// Returns the exact container image reference to pull.
    pub fn image(&self) -> &str {
        &self.image
    }
}
