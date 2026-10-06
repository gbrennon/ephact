use crate::dtos::responses::JobSummaryResponse;

/// Summary of a completed workflow execution.
#[derive(Debug, Clone)]
pub struct WorkflowExecutionResponse {
    workflow_name: String,
    job_summaries: Vec<JobSummaryResponse>,
    container_names: Vec<String>,
    success: bool,
}

impl WorkflowExecutionResponse {
    /// Creates a response from workflow, job, container, and success data.
    pub fn new(
        workflow_name: impl Into<String>,
        job_summaries: Vec<JobSummaryResponse>,
        container_names: Vec<String>,
        success: bool,
    ) -> Self {
        Self {
            workflow_name: workflow_name.into(),
            job_summaries,
            container_names,
            success,
        }
    }

    /// Returns the workflow name.
    pub fn workflow_name(&self) -> &str {
        &self.workflow_name
    }

    /// Returns the job execution summaries.
    pub fn job_summaries(&self) -> &[JobSummaryResponse] {
        &self.job_summaries
    }

    /// Returns the created container names.
    pub fn container_names(&self) -> &[String] {
        &self.container_names
    }

    /// Returns whether the workflow succeeded.
    pub fn success(&self) -> bool {
        self.success
    }

    /// Consumes the response and returns all execution data.
    pub fn into_parts(self) -> (String, Vec<JobSummaryResponse>, Vec<String>, bool) {
        (
            self.workflow_name,
            self.job_summaries,
            self.container_names,
            self.success,
        )
    }
}
