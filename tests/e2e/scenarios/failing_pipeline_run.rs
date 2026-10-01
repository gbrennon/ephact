use std::sync::Arc;

#[path = "failing_pipeline_action_fetcher.rs"]
mod failing_pipeline_action_fetcher;
#[path = "failing_pipeline_container.rs"]
mod failing_pipeline_container;
#[path = "failing_pipeline_runtime.rs"]
mod failing_pipeline_runtime;

use self::{
    failing_pipeline_action_fetcher::FailingPipelineActionFetcher,
    failing_pipeline_runtime::FailingPipelineRuntime,
};
use crate::support::{
    container_activity::ContainerActivity, ephact_application::EphactApplication,
    workflow_repository::WorkflowRepository,
};

const RELEASE_WORKFLOW: &str = r#"
name: Release
on: pull_request
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - run: echo "running the suite"
  release:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - uses: ./.forgejo/actions/release
"#;

const RELEASE_ACTION: &str = r#"
name: Release
description: Publishes the release
runs:
  using: composite
  steps:
    - run: echo "releasing"
"#;

/// Runs a workflow inside a container where every command exits with a failure
/// status, covering how a failed shell step and a failed composite action step
/// surface to the caller.
pub struct FailingPipelineRun {
    outcome: Result<(), String>,
    activity: ContainerActivity,
}

impl FailingPipelineRun {
    pub const SUITE_SCRIPT: &'static str = r#"echo "running the suite""#;
    pub const RELEASE_SCRIPT: &'static str = r#"echo "releasing""#;

    pub fn execute() -> Self {
        let repository = WorkflowRepository::named("release-pipeline")
            .with_workflow("release.yml", RELEASE_WORKFLOW)
            .with_action(".forgejo/actions/release", RELEASE_ACTION);
        let activity = ContainerActivity::new();
        let workflow_source = Arc::new(
            crate::common::fakes::fake_workflow_source::FakeWorkflowSource::new()
                .with_workflow_content(RELEASE_WORKFLOW),
        );
        let application = EphactApplication::compose(
            Arc::new(FailingPipelineRuntime::recording(activity.clone())),
            Box::new(FailingPipelineActionFetcher::mirroring(repository.path())),
            workflow_source,
        );

        let outcome = application
            .run([
                "ephact",
                "run",
                &repository.path_argument(),
                "--event",
                "pull_request",
                "--workflow",
                "release.yml",
            ])
            .map_err(|error| error.to_string());

        Self { outcome, activity }
    }

    pub fn outcome(&self) -> &Result<(), String> {
        &self.outcome
    }

    pub fn activity(&self) -> &ContainerActivity {
        &self.activity
    }
}
