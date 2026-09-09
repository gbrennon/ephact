/// Request DTO for the
/// [`PullJobImagePort`](crate::application::ports::inbound::pull_job_image_port::PullJobImagePort)
/// inbound port.
pub struct PullJobImageRequest<'a> {
    /// Runner label the job declared, when it declared one.
    pub runs_on: Option<&'a str>,
}

impl<'a> PullJobImageRequest<'a> {
    /// Creates a new request.
    pub fn new(runs_on: Option<&'a str>) -> Self {
        Self { runs_on }
    }

    /// Runner label the job declared, when it declared one.
    pub fn runs_on(&self) -> Option<&'a str> {
        self.runs_on
    }
}
