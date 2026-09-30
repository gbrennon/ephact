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
name: Woodpecker Build
when:
  - event: push
steps:
  - name: build
    image: alpine
    commands:
      - echo woodpecker
"#;

pub struct WoodpeckerWorkflowRun {
    outcome: Result<(), String>,
    activity: ContainerActivity,
}

impl WoodpeckerWorkflowRun {
    pub const SCRIPT: &'static str = "echo woodpecker";

    pub fn execute() -> Self {
        let repository = WorkflowRepository::named("woodpecker-e2e").with_workflow_in(
            ".woodpecker",
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
