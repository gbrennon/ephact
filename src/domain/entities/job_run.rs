use std::collections::HashMap;

use crate::domain::entities::Job;

/// A single job run within a stage.
///
/// When a job has a matrix strategy, it expands into multiple `JobRun` instances,
/// one per matrix combination.
#[derive(Debug, Clone, PartialEq)]
pub struct JobRun {
    /// The workflow this run belongs to.
    workflow_name: Option<String>,

    /// The identifier of the job within the workflow.
    job_id: String,

    /// The job definition to execute.
    job: Job,

    /// Optional matrix values for this specific run.
    matrix_values: Option<HashMap<String, String>>,
}

impl JobRun {
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
    /// Returns one matrix value by dimension name.
    pub fn matrix_value(&self, name: &str) -> Option<&str> {
        self.matrix_values
            .as_ref()
            .and_then(|values| values.get(name))
            .map(String::as_str)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_exposes_fields() {
        let run = JobRun::new(Some("workflow".into()), "job".into(), Job::default(), None);

        assert_eq!(run.workflow_name(), Some("workflow"));
        assert_eq!(run.job_id(), "job");
        assert!(run.job().steps().is_empty());
        assert!(run.matrix_values().is_none());
    }
    #[test]
    fn finds_matrix_value_by_name() {
        let run = JobRun::new(
            Some("workflow".into()),
            "job".into(),
            Job::default(),
            Some(HashMap::from([("os".into(), "linux".into())])),
        );

        assert_eq!(run.matrix_value("os"), Some("linux"));
        assert_eq!(run.matrix_value("missing"), None);
    }
}
