/// Marker request for reading environment entries exported by a step.
pub struct ReadStepEnvExportsRequest;

impl ReadStepEnvExportsRequest {
    /// Creates an empty request.
    pub fn new() -> Self {
        Self
    }
}

impl Default for ReadStepEnvExportsRequest {
    /// Creates an empty request.
    fn default() -> Self {
        Self::new()
    }
}
