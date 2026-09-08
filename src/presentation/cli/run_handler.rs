use super::run_args::RunArgs;
use crate::application::{
    dtos::{RunAllWorkflowsRequest, RunSummary, RunWorkflowRequest},
    ports::inbound::{
        run_all_workflows_port::RunAllWorkflowsPort, run_workflow_port::RunWorkflowPort,
    },
};
use crate::presentation::components::{
    box_component::BoxComponent, component::Component, run_summary::RunSummaryComponent,
    terminal::Terminal,
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
        terminal: &dyn Terminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (rendered, success) =
            Self::handle_with_output(args, run_workflow_port, run_all_workflows_port, terminal)?;
        print!("{rendered}");
        Self::result_for(success)
    }

    pub fn handle_with_output(
        args: RunArgs,
        run_workflow_port: &dyn RunWorkflowPort,
        run_all_workflows_port: &dyn RunAllWorkflowsPort,
        terminal: &dyn Terminal,
    ) -> Result<(String, bool), Box<dyn std::error::Error>> {
        let (config, repository) = args.to_domain()?;
        let summary = Self::execute(
            config,
            repository,
            run_workflow_port,
            run_all_workflows_port,
        )?;
        let rendered = BoxComponent::new(RunSummaryComponent::new(&summary), terminal).render();
        Ok((rendered, summary.success))
    }

    fn result_for(success: bool) -> Result<(), Box<dyn std::error::Error>> {
        if success {
            Ok(())
        } else {
            Err("workflow failed; see the run summary for failed steps".into())
        }
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

    pub fn render(summary: &RunSummary) -> String {
        RunSummaryComponent::new(summary).render()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::{
        application::dtos::{JobSummary, run_summary::step_summary::StepSummary},
        domain::workflow::StepType,
    };

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
    fn render_starts_with_summary_heading() {
        let rendered = Rendered::of(&summary(true, vec![], Duration::from_secs(5)));
        assert_eq!(rendered.line(0), "Summary");
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
        assert_eq!(rendered.line(0), "Summary");
        assert_eq!(rendered.line(1), "Workflow: test");
        assert_eq!(rendered.line(2), "  [ok] build (Build)");
        assert_eq!(rendered.line(3), "  [failed] validate");
    }
    #[test]
    fn render_includes_workflow_and_every_step_status() {
        let summary = RunSummary {
            name: "Build".into(),
            job_summaries: vec![JobSummary {
                job_id: "compile".into(),
                name: Some("Compile".into()),
                steps: vec![
                    StepSummary {
                        name: "Checkout".into(),
                        step_type: StepType::Run,
                        exit_code: Some(0),
                        continue_on_error: false,
                        duration: Duration::ZERO,
                        stdout: String::new(),
                        stderr: String::new(),
                    },
                    StepSummary {
                        name: "Build".into(),
                        step_type: StepType::Run,
                        exit_code: Some(1),
                        continue_on_error: false,
                        duration: Duration::ZERO,
                        stdout: String::new(),
                        stderr: String::new(),
                    },
                ],
                success: false,
            }],
            success: false,
            duration: Duration::ZERO,
        };

        let rendered = RunHandler::render(&summary);

        assert!(rendered.contains("Workflow: Build"));
        assert!(rendered.contains("[ok] Step 'Checkout'"));
        assert!(rendered.contains("[failed] Step 'Build'"));
    }

    #[test]
    fn render_reports_failed_step_status_without_output_details() {
        let summary = RunSummary {
            name: "test".into(),
            job_summaries: vec![JobSummary {
                job_id: "lint".into(),
                name: Some("Lint".into()),
                steps: vec![StepSummary {
                    name: "Clippy".into(),
                    step_type: StepType::Run,
                    exit_code: Some(101),
                    continue_on_error: false,
                    duration: Duration::ZERO,
                    stdout: String::new(),
                    stderr: "clippy failed".into(),
                }],
                success: false,
            }],
            success: false,
            duration: Duration::from_secs(2),
        };

        let rendered = RunHandler::render(&summary);

        assert!(rendered.contains("[failed] Step 'Clippy'"));
        assert!(!rendered.contains("clippy failed"));
    }

    #[test]
    fn render_includes_summary_heading_without_jobs() {
        let rendered = Rendered::of(&summary(true, vec![], Duration::ZERO));
        assert_eq!(rendered.lines().count(), 2);
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
