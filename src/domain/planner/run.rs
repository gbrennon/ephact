use std::collections::HashMap;

use crate::domain::workflow::Job;

/// A single job run within a stage.
///
/// When a job has a matrix strategy, it expands into multiple `Run` instances,
/// one per matrix combination.
#[derive(Debug, Clone, PartialEq)]
pub struct Run {
    /// The workflow this run belongs to.
    workflow_name: Option<String>,

    /// The identifier of the job within the workflow.
    job_id: String,

    /// The job definition to execute.
    job: Job,

    /// Optional matrix values for this specific run.
    matrix_values: Option<HashMap<String, String>>,
}

impl Run {
    #[must_use]
    pub fn new(
        workflow_name: Option<String>,
        job_id: String,
        job: Job,
        matrix_values: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            workflow_name,
            job_id,
            job,
            matrix_values,
        }
    }

    #[must_use]
    pub fn workflow_name(&self) -> Option<&str> {
        self.workflow_name.as_deref()
    }

    #[must_use]
    pub fn job_id(&self) -> &str {
        &self.job_id
    }

    #[must_use]
    pub fn job(&self) -> &Job {
        &self.job
    }

    #[must_use]
    pub fn matrix_values(&self) -> Option<&HashMap<String, String>> {
        self.matrix_values.as_ref()
    }
}
