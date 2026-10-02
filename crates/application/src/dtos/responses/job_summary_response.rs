/// Summary of a job within a workflow run.
#[derive(Debug, Clone, PartialEq)]
pub struct JobSummaryResponse {
    job_id: String,
    name: Option<String>,
    steps: Vec<crate::dtos::responses::StepSummaryResponse>,
    success: bool,
    skip_reason: Option<String>,
}

impl JobSummaryResponse {
    pub fn new(
        job_id: impl Into<String>,
        name: Option<String>,
        steps: Vec<crate::dtos::responses::StepSummaryResponse>,
        success: bool,
    ) -> Self {
        Self {
            job_id: job_id.into(),
            name,
            steps,
            success,
            skip_reason: None,
        }
    }

    pub fn job_id(&self) -> &str {
        &self.job_id
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn steps(&self) -> &[crate::dtos::responses::StepSummaryResponse] {
        &self.steps
    }

    pub fn success(&self) -> bool {
        self.success
    }

    pub fn is_skipped(&self) -> bool {
        self.skip_reason.is_some()
    }

    pub fn skip_reason(&self) -> Option<&str> {
        self.skip_reason.as_deref()
    }

    pub fn with_skip_reason(mut self, reason: impl Into<String>) -> Self {
        self.skip_reason = Some(reason.into());
        self
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
        Vec<crate::dtos::responses::StepSummaryResponse>,
        bool,
        Option<String>,
    ) {
        (
            self.job_id,
            self.name,
            self.steps,
            self.success,
            self.skip_reason,
        )
    }
}
