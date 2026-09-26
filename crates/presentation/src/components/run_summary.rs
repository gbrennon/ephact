use super::component::Component;
use crate::application::dtos::responses::{
    JobSummaryResponse, RunSummaryResponse, StepSummaryResponse,
};

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
        if step.is_skipped() {
            return "skipped";
        }
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
                let status = Self::step_status(step);
                let reason = step
                    .skip_reason()
                    .map(|value| format!(": {value}"))
                    .unwrap_or_default();
                output.push_str(&format!(
                    "\n    [{status}] Step '{}'{}",
                    step.name(),
                    reason
                ));
            }
        }
        output
    }
}
