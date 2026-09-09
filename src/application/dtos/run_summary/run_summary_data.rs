/// Workflow run summary.
#[derive(Debug, Clone, PartialEq)]
pub struct RunSummary {
    name: String,
    job_summaries: Vec<crate::application::dtos::run_summary::job_summary::JobSummary>,
    success: bool,
    duration: std::time::Duration,
}

impl RunSummary {
    pub fn new(
        name: impl Into<String>,
        job_summaries: Vec<crate::application::dtos::run_summary::job_summary::JobSummary>,
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

    pub fn job_summaries(
        &self,
    ) -> &[crate::application::dtos::run_summary::job_summary::JobSummary] {
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
        Vec<crate::application::dtos::run_summary::job_summary::JobSummary>,
        bool,
        std::time::Duration,
    ) {
        (self.name, self.job_summaries, self.success, self.duration)
    }
}
