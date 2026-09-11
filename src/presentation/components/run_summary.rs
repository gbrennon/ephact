use crate::application::dtos::responses::JobSummaryResponse;
use crate::application::dtos::responses::RunSummaryResponse;
use crate::application::dtos::responses::StepSummaryResponse;

use super::component::Component;

pub struct RunSummaryComponent<'a> {
    summary: &'a RunSummaryResponse,
}

impl<'a> RunSummaryComponent<'a> {
    pub fn new(summary: &'a RunSummaryResponse) -> Self {
        Self { summary }
    }

    fn job_label(job: &JobSummaryResponse) -> String {
        match &job.name() {
            Some(name) => format!("{} ({name})", job.job_id()),
            None => job.job_id().to_string(),
        }
    }

    fn step_status(step: &StepSummaryResponse) -> &'static str {
        match step.exit_code() {
            Some(0) => "ok",
            Some(_) => "failed",
            None => "error",
        }
    }
}

impl Component for RunSummaryComponent<'_> {
    fn render(&self) -> String {
        let mut output = format!("Summary\nWorkflow: {}", self.summary.name());
        for job in self.summary.job_summaries() {
            let status = if job.success() { "ok" } else { "failed" };
            output.push_str(&format!("\n  [{status}] {}", Self::job_label(job)));
            for step in job.steps() {
                output.push_str(&format!(
                    "\n    [{}] Step '{}'",
                    Self::step_status(step),
                    step.name()
                ));
            }
        }
        output
    }
}
