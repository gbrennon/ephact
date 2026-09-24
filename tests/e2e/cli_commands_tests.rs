#[cfg(test)]
mod tests {
    use std::time::Duration;

    use ephact::{
        application::{
            dtos::{
                requests::{
                    ListActionsRequest, ListWorkflowsRequest, RunAllWorkflowsRequest,
                    RunWorkflowRequest,
                },
                responses::{
                    JobSummaryResponse, ListActionsResponse, ListWorkflowsResponse,
                    RunSummaryResponse, ShowProjectBrandingInfoResponse, StepSummaryDetails,
                    StepSummaryResponse, StepSummaryResponseInput, WorkflowListItemResponse,
                },
            },
            errors::DiscoverRunInputsError,
            ports::inbound::{
                ListActionsPort, ListWorkflowsPort, RunAllWorkflowsPort, RunWorkflowPort,
                ShowProjectBrandingInfoPort,
            },
        },
        domain::value_objects::StepType,
        presentation::{
            cli::{Cli, cli::CliDependencies},
            components::terminal::Terminal,
        },
    };

    use super::super::support::workflow_repository::WorkflowRepository;

    struct FixedTerminal;

    impl Terminal for FixedTerminal {
        fn dimensions(&self) -> (usize, usize) {
            (100, 40)
        }

        fn write_text(&self, _text: &str) -> std::io::Result<()> {
            Ok(())
        }

        fn read_line(&self) -> std::io::Result<String> {
            Ok(String::new())
        }
    }

    struct BrandingFake;

    impl ShowProjectBrandingInfoPort for BrandingFake {
        fn execute(
            &self,
        ) -> Result<
            ShowProjectBrandingInfoResponse,
            ephact::application::errors::ShowProjectBrandingInfoError,
        > {
            Ok(ShowProjectBrandingInfoResponse::new(
                "ephact".into(),
                "test runner".into(),
                "test".into(),
                "*".into(),
            ))
        }
    }

    struct WorkflowListFake;

    impl ListWorkflowsPort for WorkflowListFake {
        fn execute(
            &self,
            _request: ListWorkflowsRequest,
        ) -> Result<ListWorkflowsResponse, ephact::application::errors::ListWorkflowsError>
        {
            Ok(ListWorkflowsResponse::new(vec![
                WorkflowListItemResponse::new(
                    Some("Build".into()),
                    Some("build.yml".into()),
                    vec!["pull_request".into()],
                ),
                WorkflowListItemResponse::new(
                    Some("Release".into()),
                    Some("release.yml".into()),
                    vec!["push".into()],
                ),
            ]))
        }
    }

    struct ActionListFake;

    impl ListActionsPort for ActionListFake {
        fn execute(
            &self,
            _request: ListActionsRequest,
        ) -> Result<ListActionsResponse, ephact::application::errors::ListActionsError> {
            Ok(ListActionsResponse::new(vec![
                "custom/build-action".into(),
                "custom/release-action".into(),
            ]))
        }
    }

    struct InputDiscoveryFake;

    impl ephact::application::ports::outbound::RunInputsDiscovererPort for InputDiscoveryFake {
        fn discover(
            &self,
            _request: ephact::application::dtos::requests::DiscoverRunInputsRequest,
        ) -> Result<
            Vec<ephact::application::dtos::responses::RunInputDeclarationResponse>,
            DiscoverRunInputsError,
        > {
            Ok(Vec::new())
        }
    }
    struct RunFake {
        summary: RunSummaryResponse,
    }

    impl RunWorkflowPort for RunFake {
        fn execute(
            &self,
            _request: RunWorkflowRequest,
        ) -> std::pin::Pin<
            Box<
                dyn std::future::Future<
                        Output = Result<
                            RunSummaryResponse,
                            ephact::application::errors::RunWorkflowError,
                        >,
                    > + Send
                    + '_,
            >,
        > {
            let summary = self.summary.clone();
            Box::pin(async move { Ok(summary) })
        }
    }

    impl RunAllWorkflowsPort for RunFake {
        fn execute(
            &self,
            _request: RunAllWorkflowsRequest,
        ) -> Result<RunSummaryResponse, ephact::application::errors::RunAllWorkflowsError> {
            Ok(self.summary.clone())
        }
    }

    fn cli(summary: RunSummaryResponse) -> Cli {
        Cli::new(CliDependencies::new(
            (
                Box::new(RunFake {
                    summary: summary.clone(),
                }),
                Box::new(RunFake { summary }),
                Box::new(InputDiscoveryFake),
            ),
            (
                Box::new(WorkflowListFake),
                Box::new(ActionListFake),
                Box::new(BrandingFake),
            ),
        ))
    }

    fn repository() -> WorkflowRepository {
        WorkflowRepository::named("cli-e2e")
            .with_workflow("build.yml", "name: Build")
            .with_workflow("release.yml", "name: Release")
            .with_action(".forgejo/actions/build", "name: Build")
            .with_action(".github/actions/release", "name: Release")
    }

    fn run_summary() -> RunSummaryResponse {
        RunSummaryResponse::new(
            "Build",
            vec![JobSummaryResponse::new(
                "build",
                Some("Build job".into()),
                vec![
                    StepSummaryResponse::new(StepSummaryResponseInput::new(
                        "Checkout",
                        StepType::Run,
                        StepSummaryDetails::new(
                            Some(0),
                            false,
                            Duration::ZERO,
                            String::new(),
                            String::new(),
                        ),
                    )),
                    StepSummaryResponse::new(StepSummaryResponseInput::new(
                        "Compile",
                        StepType::Run,
                        StepSummaryDetails::new(
                            Some(0),
                            false,
                            Duration::ZERO,
                            String::new(),
                            String::new(),
                        ),
                    )),
                ],
                true,
            )],
            true,
            Duration::ZERO,
        )
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
