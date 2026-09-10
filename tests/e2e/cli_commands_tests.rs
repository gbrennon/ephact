#[cfg(test)]
mod tests {
    use std::time::Duration;

    use ephact::application::dtos::requests::ListActionsRequest;
    use ephact::application::dtos::requests::ListWorkflowsRequest;
    use ephact::application::dtos::requests::RunAllWorkflowsRequest;
    use ephact::application::dtos::requests::RunWorkflowRequest;
    use ephact::application::dtos::responses::JobSummaryResponse;
    use ephact::application::dtos::responses::ListActionsResponse;
    use ephact::application::dtos::responses::ListWorkflowsResponse;
    use ephact::application::dtos::responses::RunSummaryResponse;
    use ephact::application::dtos::responses::ShowProjectBrandingInfoResponse;
    use ephact::application::dtos::responses::StepSummaryResponse;
    use ephact::application::dtos::responses::WorkflowListItemResponse;
    use ephact::application::ports::inbound::ListActionsPort;
    use ephact::application::ports::inbound::ListWorkflowsPort;
    use ephact::application::ports::inbound::RunAllWorkflowsPort;
    use ephact::application::ports::inbound::RunWorkflowPort;
    use ephact::application::ports::inbound::ShowProjectBrandingInfoPort;
    use ephact::domain::value_objects::StepType;
    use ephact::presentation::cli::Cli;
    use ephact::presentation::components::terminal::Terminal;

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
        fn execute(&self) -> Result<ShowProjectBrandingInfoResponse, Box<dyn std::error::Error>> {
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
        ) -> Result<ListWorkflowsResponse, Box<dyn std::error::Error>> {
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
        ) -> Result<ListActionsResponse, Box<dyn std::error::Error>> {
            Ok(ListActionsResponse::new(vec![
                "custom/build-action".into(),
                "custom/release-action".into(),
            ]))
        }
    }

    struct InputDiscoveryFake;

    impl ephact::application::ports::outbound::DiscoverRunInputsPort for InputDiscoveryFake {
        fn execute(
            &self,
            _request: ephact::application::dtos::requests::DiscoverRunInputsRequest,
        ) -> Result<
            Vec<ephact::application::dtos::responses::RunInputDeclarationResponse>,
            Box<dyn std::error::Error>,
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
        ) -> Result<RunSummaryResponse, Box<dyn std::error::Error>> {
            Ok(self.summary.clone())
        }
    }

    impl RunAllWorkflowsPort for RunFake {
        fn execute(
            &self,
            _request: RunAllWorkflowsRequest,
        ) -> Result<RunSummaryResponse, Box<dyn std::error::Error>> {
            Ok(self.summary.clone())
        }
    }

    fn cli(summary: RunSummaryResponse) -> Cli {
        Cli::new(
            Box::new(RunFake {
                summary: summary.clone(),
            }),
            Box::new(RunFake { summary }),
            Box::new(InputDiscoveryFake),
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

    fn run_summary() -> RunSummaryResponse {
        RunSummaryResponse::new(
            "Build",
            vec![JobSummaryResponse::new(
                "build",
                Some("Build job".into()),
                vec![
                    StepSummaryResponse::new(
                        "Checkout",
                        StepType::Run,
                        Some(0),
                        false,
                        Duration::ZERO,
                        String::new(),
                        String::new(),
                    ),
                    StepSummaryResponse::new(
                        "Compile",
                        StepType::Run,
                        Some(0),
                        false,
                        Duration::ZERO,
                        String::new(),
                        String::new(),
                    ),
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
