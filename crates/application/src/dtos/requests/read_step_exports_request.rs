/// Marker request for reading values exported by a step.
pub struct ReadStepExportsRequest;

impl ReadStepExportsRequest {
    /// Creates an empty request.
    pub fn new() -> Self {
        Self
    }
}

impl Default for ReadStepExportsRequest {
    /// Creates an empty request.
    fn default() -> Self {
        Self::new()
    }
}
