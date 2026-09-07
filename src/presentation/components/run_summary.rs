use crate::application::dtos::{JobSummary, RunSummary, run_summary::step_summary::StepSummary};

use super::component::Component;

pub struct RunSummaryComponent<'a> {
    summary: &'a RunSummary,
}

impl<'a> RunSummaryComponent<'a> {
    pub fn new(summary: &'a RunSummary) -> Self {
        Self { summary }
    }

    fn job_label(job: &JobSummary) -> String {
        match &job.name {
            Some(name) => format!("{} ({name})", job.job_id),
            None => job.job_id.clone(),
        }
    }

    fn failed_step_lines(job: &JobSummary) -> Vec<String> {
        job.steps
            .iter()
            .filter(|step| step.exit_code != Some(0))
            .flat_map(Self::step_lines)
            .collect()
    }

    fn step_lines(step: &StepSummary) -> Vec<String> {
        let outcome = match step.exit_code {
            Some(code) => format!("exit code: {code}"),
            None => "no exit code".to_string(),
        };
        let mut lines = vec![format!("    Step '{}' failed ({outcome})", step.name)];
        Self::append_output(&mut lines, "stdout", &step.stdout);
        Self::append_output(&mut lines, "stderr", &step.stderr);
        lines
    }

    fn append_output(lines: &mut Vec<String>, label: &str, output: &str) {
        if output.is_empty() {
            return;
        }
        lines.extend(output.lines().map(|line| format!("      {label}: {line}")));
    }
}

impl Component for RunSummaryComponent<'_> {
    fn render(&self) -> String {
        let mut output = String::from("Summary");
        for job in &self.summary.job_summaries {
            let status = if job.success { "ok" } else { "failed" };
            output.push_str(&format!("\n  [{status}] {}", Self::job_label(job)));
            for line in Self::failed_step_lines(job) {
                output.push('\n');
                output.push_str(&line);
            }
        }
        output
    }
}
