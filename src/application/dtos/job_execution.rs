use crate::application::dtos::JobSummary;

/// Outcome of running one job, with the container it ran in.
#[derive(Debug, Clone)]
pub struct JobExecution {
    job_summary: JobSummary,
    container_name: String,
}

impl JobExecution {
    pub fn new(job_summary: JobSummary, container_name: impl Into<String>) -> Self {
        Self {
            job_summary,
            container_name: container_name.into(),
        }
    }

    pub fn job_summary(&self) -> &JobSummary {
        &self.job_summary
    }

    pub fn container_name(&self) -> &str {
        &self.container_name
    }

    pub fn into_parts(self) -> (JobSummary, String) {
        (self.job_summary, self.container_name)
    }
}
