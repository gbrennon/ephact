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
        job_id: String,
        name: Option<String>,
        steps: Vec<crate::dtos::responses::StepSummaryResponse>,
        success: bool,
    ) -> Self {
        Self {
            job_id,
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

    pub fn with_skip_reason(self, reason: String) -> Self {
        Self {
            skip_reason: Some(reason),
            ..self
        }
    }

    pub fn with_name(self, name: Option<String>) -> Self {
        Self { name, ..self }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chaining_replaces_name_and_skip_reason() {
        let response = JobSummaryResponse::new(
            "job".to_owned(),
            Some("initial".to_owned()),
            Vec::new(),
            true,
        )
        .with_name(Some("renamed".to_owned()))
        .with_name(Some("final".to_owned()))
        .with_skip_reason("first".to_owned())
        .with_skip_reason("second".to_owned());

        assert_eq!(response.name(), Some("final"));
        assert!(response.is_skipped());
        assert_eq!(response.skip_reason(), Some("second"));
    }
}
