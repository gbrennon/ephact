use crate::application::dtos::JobSummary;

#[derive(Debug, Clone)]
pub struct WorkflowExecution {
    workflow_name: String,
    job_summaries: Vec<JobSummary>,
    container_names: Vec<String>,
    success: bool,
}

impl WorkflowExecution {
    pub fn new(
        workflow_name: impl Into<String>,
        job_summaries: Vec<JobSummary>,
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

    pub fn workflow_name(&self) -> &str {
        &self.workflow_name
    }

    pub fn job_summaries(&self) -> &[JobSummary] {
        &self.job_summaries
    }

    pub fn container_names(&self) -> &[String] {
        &self.container_names
    }

    pub fn success(&self) -> bool {
        self.success
    }

    pub fn into_parts(self) -> (String, Vec<JobSummary>, Vec<String>, bool) {
        (
            self.workflow_name,
            self.job_summaries,
            self.container_names,
            self.success,
        )
    }
}
