/// Describes a request to read values a step exported for later steps.
#[derive(Debug, Clone, Copy, Default)]
pub struct ReadStepOutputExportsRequest;

impl ReadStepOutputExportsRequest {
    pub fn new() -> Self {
        Self
    }
}
