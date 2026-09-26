/// Request DTO for the
/// [`PullJobImagePort`](crate::ports::inbound::pull_job_image_port::PullJobImagePort)
/// inbound port.
pub struct PullJobImageRequest {
    /// Runner label the job declared, when it declared one.
    runs_on: Option<String>,
}

impl PullJobImageRequest {
    /// Creates a new request.
    pub fn new(runs_on: Option<String>) -> Self {
        Self { runs_on }
    }

    /// Runner label the job declared, when it declared one.
    pub fn runs_on(&self) -> Option<&str> {
        self.runs_on.as_deref()
    }
}
