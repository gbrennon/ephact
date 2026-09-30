//! End-to-end detection tests across the supported CI platforms.

use std::time::Duration;

use ephact::{
    application::{
        dtos::{
            requests::{ListActionsRequest, RunAllWorkflowsRequest, RunWorkflowRequest},
            responses::{ListActionsResponse, RunSummaryResponse, ShowProjectBrandingInfoResponse},
        },
        errors::DiscoverRunInputsError,
        ports::inbound::{
            ListActionsPort, RunAllWorkflowsPort, RunWorkflowPort, ShowProjectBrandingInfoPort,
        },
        services::ListWorkflowsService,
    },
    infrastructure::FilesystemWorkflowSource,
    presentation::{
        cli::{Cli, CliDependencies},
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

struct ActionListFake;

impl ListActionsPort for ActionListFake {
    fn execute(
        &self,
        _request: ListActionsRequest,
    ) -> Result<ListActionsResponse, ephact::application::errors::ListActionsError> {
        Ok(ListActionsResponse::new(Vec::new()))
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

struct RunFake;

impl RunWorkflowPort for RunFake {
    fn execute(
        &self,
        _request: RunWorkflowRequest,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = Result<
                        RunSummaryResponse,
                        ephact::application::errors::ApplicationError,
                    >,
                > + Send
                + '_,
        >,
    > {
        Box::pin(async { Ok(Self::noop_summary()) })
    }
}

impl RunAllWorkflowsPort for RunFake {
    fn execute(
        &self,
        _request: RunAllWorkflowsRequest,
    ) -> Result<RunSummaryResponse, ephact::application::errors::ApplicationError> {
        Ok(Self::noop_summary())
    }
}

impl RunFake {
    fn noop_summary() -> RunSummaryResponse {
        RunSummaryResponse::new("noop", Vec::new(), true, Duration::ZERO)
    }
}

struct WorkflowDetectionTests;

impl WorkflowDetectionTests {
    /// Builds a `Cli` whose `list-workflows` path uses the real detection stack.
    fn cli_with_real_detection() -> Cli {
        let list_workflows =
            ListWorkflowsService::new(Box::new(FilesystemWorkflowSource::default()));
        Cli::new(CliDependencies::new(
            (
                Box::new(RunFake),
                Box::new(RunFake),
                Box::new(InputDiscoveryFake),
            ),
            (
                Box::new(list_workflows),
                Box::new(ActionListFake),
                Box::new(BrandingFake),
            ),
        ))
    }

    /// Runs `ephact list-workflows <repo>` and returns the rendered output.
    fn list_workflows(repository: &WorkflowRepository) -> String {
        Self::cli_with_real_detection()
            .run_with_terminal(
                ["ephact", "list-workflows", &repository.path_argument()],
                &FixedTerminal,
            )
            .unwrap()
    }

    /// Detection contract shared by every platform: an ephemeral repo with a
    /// single named workflow in `platform_dir` must have that workflow listed.
    fn assert_platform_workflow_is_detected(platform_dir: &str) {
        let repository = WorkflowRepository::named("detect-e2e").with_workflow_in(
            platform_dir,
            "ci.yml",
            "name: Continuous Integration",
        );

        let output = Self::list_workflows(&repository);

        assert!(
            output.contains("Continuous Integration"),
            "expected workflow from {platform_dir} to be detected, got:\n{output}"
        );
    }

    fn detects_every_platform_in_a_single_repository() {
        let repository = WorkflowRepository::named("multi-platform-e2e")
            .with_workflow_in(".github/workflows", "gh.yml", "name: GitHub Build")
            .with_workflow_in(".forgejo/workflows", "fj.yml", "name: Forgejo Build")
            .with_workflow_in(".woodpecker", "wp.yml", "name: Woodpecker Build");

        let output = Self::list_workflows(&repository);

        assert!(output.contains("GitHub Build"), "missing GitHub: {output}");
        assert!(
            output.contains("Forgejo Build"),
            "missing Forgejo: {output}"
        );
        assert!(
            output.contains("Woodpecker Build"),
            "missing Woodpecker: {output}"
        );
    }
}

#[test]
fn detects_github_actions_workflows() {
    WorkflowDetectionTests::assert_platform_workflow_is_detected(".github/workflows");
}

#[test]
fn detects_forgejo_workflows() {
    WorkflowDetectionTests::assert_platform_workflow_is_detected(".forgejo/workflows");
}

#[test]
fn detects_woodpecker_pipelines() {
    WorkflowDetectionTests::assert_platform_workflow_is_detected(".woodpecker");
}

#[test]
fn detects_every_platform_in_a_single_repository() {
    WorkflowDetectionTests::detects_every_platform_in_a_single_repository();
}
