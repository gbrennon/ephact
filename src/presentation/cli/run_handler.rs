use std::time::Duration;

use super::run_args::RunArgs;
use crate::application::{
    dtos::{RunAllWorkflowsRequest, RunSummary, RunWorkflowRequest},
    ports::inbound::{
        run_all_workflows_port::RunAllWorkflowsPort, run_workflow_port::RunWorkflowPort,
    },
};

/// Handles the `run` subcommand by dispatching parsed CLI arguments to the
/// application port.
///
/// Live progress is rendered by the event handler registered in the
/// container; this handler only prints the final GitHub-Actions-like run
/// summary and interprets the result for the process exit code.
pub struct RunHandler;

impl RunHandler {
    /// Executes the `run` subcommand: converts CLI args to domain objects,
    pub fn handle(
        args: RunArgs,
        run_workflow_port: &dyn RunWorkflowPort,
        run_all_workflows_port: &dyn RunAllWorkflowsPort,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (config, repository) = args.to_domain()?;
        let summary = Self::execute(
            config,
            repository,
            run_workflow_port,
            run_all_workflows_port,
        )?;
        eprint!("{}", Self::render(&summary));
        Self::interpret_result(&summary)
    }

    fn execute(
        config: crate::domain::value_objects::ActRunConfig,
        repository: crate::domain::Repository,
        run_workflow_port: &dyn RunWorkflowPort,
        run_all_workflows_port: &dyn RunAllWorkflowsPort,
    ) -> Result<RunSummary, Box<dyn std::error::Error>> {
        if config.all_workflows() {
            run_all_workflows_port.execute(RunAllWorkflowsRequest::new(config, repository))
        } else {
            run_workflow_port.execute(RunWorkflowRequest::new(config, repository))
        }
    }

    fn interpret_result(summary: &RunSummary) -> Result<(), Box<dyn std::error::Error>> {
        if summary.success {
            Ok(())
        } else {
            Err("workflow failed".into())
        }
    }

    /// Renders the final run summary: a status header with the total
    /// duration followed by one status line per job, in the style of the
    /// GitHub Actions run summary page.
    pub fn render(summary: &RunSummary) -> String {
        let mut out = String::new();
        let status = if summary.success {
            "succeeded"
        } else {
            "failed"
        };
        out.push_str(&format!(
            "Run '{}' {} in {}\n",
            summary.name,
            status,
            Self::format_duration(summary.duration),
        ));
        out.push_str(&Self::render_job_summary_lines(summary));
        out
    }

    fn render_job_summary_lines(summary: &RunSummary) -> String {
        if summary.job_summaries.is_empty() {
            return String::new();
        }
        let mut out = String::from("\nSummary\n");
        for job in &summary.job_summaries {
            let status = if job.success { "ok" } else { "failed" };
            out.push_str(&format!("  [{status}] {}\n", Self::job_label(job)));
        }
        out
    }

    fn job_label(job: &crate::application::dtos::JobSummary) -> String {
        match &job.name {
            Some(name) => format!("{} ({name})", job.job_id),
            None => job.job_id.clone(),
        }
    }

    fn format_duration(duration: Duration) -> String {
        let total_seconds = duration.as_secs();
        if total_seconds < 60 {
            return format!("{total_seconds}s");
        }
        format!("{}m {}s", total_seconds / 60, total_seconds % 60)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::dtos::JobSummary;

    fn job(job_id: &str, name: Option<&str>, success: bool) -> JobSummary {
        JobSummary {
            job_id: job_id.into(),
            name: name.map(Into::into),
            steps: vec![],
            success,
        }
    }

    fn summary(success: bool, jobs: Vec<JobSummary>, duration: Duration) -> RunSummary {
        RunSummary {
            name: "test".into(),
            job_summaries: jobs,
            success,
            duration,
        }
    }

    #[test]
    fn render_reports_a_succeeded_run_with_its_duration() {
        let rendered = Rendered::of(&summary(true, vec![], Duration::from_secs(5)));
        assert_eq!(rendered.line(0), "Run 'test' succeeded in 5s");
    }

    #[test]
    fn render_reports_a_failed_run() {
        let rendered = Rendered::of(&summary(false, vec![], Duration::ZERO));
        assert_eq!(rendered.line(0), "Run 'test' failed in 0s");
    }

    #[test]
    fn render_formats_durations_over_a_minute() {
        let rendered = Rendered::of(&summary(true, vec![], Duration::from_secs(83)));
        assert_eq!(rendered.line(0), "Run 'test' succeeded in 1m 23s");
    }

    #[test]
    fn render_lists_every_job_with_its_status_in_the_summary() {
        let rendered = Rendered::of(&summary(
            false,
            vec![
                job("build", Some("Build"), true),
                job("validate", None, false),
            ],
            Duration::ZERO,
        ));
        assert_eq!(rendered.line(1), "");
        assert_eq!(rendered.line(2), "Summary");
        assert_eq!(rendered.line(3), "  [ok] build (Build)");
        assert_eq!(rendered.line(4), "  [failed] validate");
    }

    #[test]
    fn render_omits_the_summary_block_when_the_run_has_no_jobs() {
        let rendered = Rendered::of(&summary(true, vec![], Duration::ZERO));
        assert_eq!(rendered.lines().count(), 1);
    }

    struct Rendered {
        text: String,
    }

    impl Rendered {
        fn of(summary: &RunSummary) -> Self {
            Self {
                text: RunHandler::render(summary),
            }
        }

        fn line(&self, index: usize) -> &str {
            self.text.lines().nth(index).unwrap()
        }

        fn lines(&self) -> std::str::Lines<'_> {
            self.text.lines()
        }
    }
}
