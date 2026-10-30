#[derive(Debug, Clone, PartialEq, Default)]
pub(super) struct JobIdentity {
    /// The display name of the job.
    name: Option<String>,

    /// Jobs that must complete successfully before this job runs.
    needs: Vec<String>,
}

impl JobIdentity {
    pub(super) fn new(name: Option<String>, needs: Vec<String>) -> Self {
        Self { name, needs }
    }

    pub(super) fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub(super) fn needs(&self) -> &[String] {
        &self.needs
    }

    pub(super) fn depends_on(&self, job_id: &str) -> bool {
        self.needs.iter().any(|dependency| dependency == job_id)
    }
}
