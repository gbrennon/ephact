/// Workflow run summary.
#[derive(Debug, Clone, PartialEq)]
pub struct RunSummaryResponse {
    name: String,
    job_summaries: Vec<crate::application::dtos::responses::JobSummaryResponse>,
    success: bool,
    duration: std::time::Duration,
}

impl RunSummaryResponse {
    pub fn new(
        name: impl Into<String>,
        job_summaries: Vec<crate::application::dtos::responses::JobSummaryResponse>,
        success: bool,
        duration: std::time::Duration,
    ) -> Self {
        Self {
            name: name.into(),
            job_summaries,
            success,
            duration,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn job_summaries(&self) -> &[crate::application::dtos::responses::JobSummaryResponse] {
        &self.job_summaries
    }

    pub fn success(&self) -> bool {
        self.success
    }

    pub fn duration(&self) -> std::time::Duration {
        self.duration
    }

    pub fn into_parts(
        self,
    ) -> (
        String,
        Vec<crate::application::dtos::responses::JobSummaryResponse>,
        bool,
        std::time::Duration,
    ) {
        (self.name, self.job_summaries, self.success, self.duration)
    }
}
