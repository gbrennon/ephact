use crate::application::dtos::responses::JobSummaryResponse;

/// Outcome of running one job, with the container it ran in.
#[derive(Debug, Clone)]
pub struct JobExecutionResponse {
    job_summary: JobSummaryResponse,
    container_name: String,
}

impl JobExecutionResponse {
    pub fn new(job_summary: JobSummaryResponse, container_name: impl Into<String>) -> Self {
        Self {
            job_summary,
            container_name: container_name.into(),
        }
    }

    pub fn job_summary(&self) -> &JobSummaryResponse {
        &self.job_summary
    }

    pub fn container_name(&self) -> &str {
        &self.container_name
    }

    pub fn into_parts(self) -> (JobSummaryResponse, String) {
        (self.job_summary, self.container_name)
    }
}
