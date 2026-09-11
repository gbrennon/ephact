/// Summary of a job within a workflow run.
#[derive(Debug, Clone, PartialEq)]
pub struct JobSummaryResponse {
    job_id: String,
    name: Option<String>,
    steps: Vec<crate::application::dtos::responses::StepSummaryResponse>,
    success: bool,
}

impl JobSummaryResponse {
    pub fn new(
        job_id: impl Into<String>,
        name: Option<String>,
        steps: Vec<crate::application::dtos::responses::StepSummaryResponse>,
        success: bool,
    ) -> Self {
        Self {
            job_id: job_id.into(),
            name,
            steps,
            success,
        }
    }

    pub fn job_id(&self) -> &str {
        &self.job_id
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn steps(&self) -> &[crate::application::dtos::responses::StepSummaryResponse] {
        &self.steps
    }

    pub fn success(&self) -> bool {
        self.success
    }

    pub fn with_name(mut self, name: Option<String>) -> Self {
        self.name = name;
        self
    }

    pub fn into_parts(
        self,
    ) -> (
        String,
        Option<String>,
        Vec<crate::application::dtos::responses::StepSummaryResponse>,
        bool,
    ) {
        (self.job_id, self.name, self.steps, self.success)
    }
}
