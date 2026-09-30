use std::sync::Arc;

use ephact::infrastructure::FilesystemWorkflowSource;

use crate::{
    e2e_mirrored_action_fetcher::MirroredActionFetcher,
    e2e_succeeding_runtime::SucceedingRuntime,
    support::{
        container_activity::ContainerActivity, ephact_application::EphactApplication,
        workflow_repository::WorkflowRepository,
    },
};

const WORKFLOW: &str = r#"
name: GitHub Build
on: push
jobs:
  build:
    runs-on: alpine
    steps:
      - run: echo github-actions
"#;

pub struct GithubActionsWorkflowRun {
    outcome: Result<(), String>,
    activity: ContainerActivity,
}

impl GithubActionsWorkflowRun {
    pub const SCRIPT: &'static str = "echo github-actions";

    pub fn execute() -> Self {
        let repository = WorkflowRepository::named("github-actions-e2e").with_workflow_in(
            ".github/workflows",
            "build.yml",
            WORKFLOW,
        );
        let activity = ContainerActivity::new();
        let application = EphactApplication::compose(
            Arc::new(SucceedingRuntime::recording(activity.clone())),
            Box::new(MirroredActionFetcher::mirroring(repository.path())),
            Arc::new(FilesystemWorkflowSource::default()),
        );

        let outcome = application
            .run([
                "ephact",
                "run",
                &repository.path_argument(),
                "--event",
                "push",
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
