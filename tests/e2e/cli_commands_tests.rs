#[cfg(test)]
mod tests {
    use std::time::Duration;

    use ephact::{
        application::{
            dtos::{
                JobSummary, ListActionsRequest, ListActionsResponse, ListWorkflowsRequest,
                ListWorkflowsResponse, RunAllWorkflowsRequest, RunSummary, RunWorkflowRequest,
                ShowProjectBrandingInfoResponse, WorkflowListItem,
                run_summary::step_summary::StepSummary,
            },
            ports::inbound::{
                ListActionsPort, ListWorkflowsPort, RunAllWorkflowsPort, RunWorkflowPort,
                ShowProjectBrandingInfoPort,
            },
        },
        domain::workflow::StepType,
        presentation::{cli::Cli, components::terminal::Terminal},
    };

    use super::super::support::workflow_repository::WorkflowRepository;

    struct FixedTerminal;

    impl Terminal for FixedTerminal {
        fn dimensions(&self) -> (usize, usize) {
            (100, 40)
        }
    }

    struct BrandingFake;

    impl ShowProjectBrandingInfoPort for BrandingFake {
        fn execute(&self) -> Result<ShowProjectBrandingInfoResponse, Box<dyn std::error::Error>> {
            Ok(ShowProjectBrandingInfoResponse {
                name: "ephact".into(),
                description: "test runner".into(),
                version: "test".into(),
                emblem: "*".into(),
            })
        }
    }

    struct WorkflowListFake;

    impl ListWorkflowsPort for WorkflowListFake {
        fn execute(
            &self,
            _request: ListWorkflowsRequest,
        ) -> Result<ListWorkflowsResponse, Box<dyn std::error::Error>> {
            Ok(ListWorkflowsResponse::new(vec![
                WorkflowListItem::new(Some("Build".into()), Some("build.yml".into())),
                WorkflowListItem::new(Some("Release".into()), Some("release.yml".into())),
            ]))
        }
    }

    struct ActionListFake;

    impl ListActionsPort for ActionListFake {
        fn execute(
            &self,
            _request: ListActionsRequest,
        ) -> Result<ListActionsResponse, Box<dyn std::error::Error>> {
            Ok(ListActionsResponse::new(vec![
                "custom/build-action".into(),
                "custom/release-action".into(),
            ]))
        }
    }

    struct RunFake {
        summary: RunSummary,
    }

    impl RunWorkflowPort for RunFake {
        fn execute(
            &self,
            _request: RunWorkflowRequest,
        ) -> Result<RunSummary, Box<dyn std::error::Error>> {
            Ok(self.summary.clone())
        }
    }

    impl RunAllWorkflowsPort for RunFake {
        fn execute(
            &self,
            _request: RunAllWorkflowsRequest,
        ) -> Result<RunSummary, Box<dyn std::error::Error>> {
            Ok(self.summary.clone())
        }
    }

    fn cli(summary: RunSummary) -> Cli {
        Cli::new(
            Box::new(RunFake {
                summary: summary.clone(),
            }),
            Box::new(RunFake { summary }),
            Box::new(WorkflowListFake),
            Box::new(ActionListFake),
            Box::new(BrandingFake),
        )
    }

    fn repository() -> WorkflowRepository {
        WorkflowRepository::named("cli-e2e")
            .with_workflow("build.yml", "name: Build")
            .with_workflow("release.yml", "name: Release")
            .with_action(".forgejo/actions/build", "name: Build")
            .with_action(".github/actions/release", "name: Release")
    }

    fn run_summary() -> RunSummary {
        RunSummary {
            name: "Build".into(),
            job_summaries: vec![JobSummary {
                job_id: "build".into(),
                name: Some("Build job".into()),
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
                        name: "Compile".into(),
                        step_type: StepType::Run,
                        exit_code: Some(0),
                        continue_on_error: false,
                        duration: Duration::ZERO,
                        stdout: String::new(),
                        stderr: String::new(),
                    },
                ],
                success: true,
            }],
            success: true,
            duration: Duration::ZERO,
        }
    }

    #[test]
    fn run_command_reports_workflow_jobs_and_step_statuses() {
        let repository = repository();
        let output = cli(run_summary())
            .run_with_terminal(
                ["ephact", "run", &repository.path_argument()],
                &FixedTerminal,
            )
            .unwrap();

        assert!(output.contains("Summary"));
        assert!(output.contains("Workflow: Build"));
        assert!(output.contains("[ok] build (Build job)"));
        assert!(output.contains("[ok] Step 'Checkout'"));
        assert!(output.contains("[ok] Step 'Compile'"));
    }

    #[test]
    fn list_workflows_command_reports_fixture_workflow_names() {
        let repository = repository();
        let output = cli(run_summary())
            .run_with_terminal(
                ["ephact", "list-workflows", &repository.path_argument()],
                &FixedTerminal,
            )
            .unwrap();

        assert!(output.contains("Workflows"));
        assert!(output.contains("Build"));
        assert!(output.contains("Release"));
        assert!(!output.contains("build.yml"));
    }

    #[test]
    fn list_actions_command_reports_custom_fixture_actions() {
        let repository = repository();
        let output = cli(run_summary())
            .run_with_terminal(
                ["ephact", "list-actions", &repository.path_argument()],
                &FixedTerminal,
            )
            .unwrap();
        assert!(output.contains("Actions"));
        assert!(output.contains("build-action"));
        assert!(output.contains("release-action"));
        assert!(!output.contains("custom/build-action"));
    }
}
