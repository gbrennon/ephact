/// Marker request for reading path entries exported by a step.
pub struct ReadStepPathExportsRequest;

impl ReadStepPathExportsRequest {
    /// Creates an empty request.
    pub fn new() -> Self {
        Self
    }
}

impl Default for ReadStepPathExportsRequest {
    /// Creates an empty request.
    fn default() -> Self {
        Self::new()
    }
}
