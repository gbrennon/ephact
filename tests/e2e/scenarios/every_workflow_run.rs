use std::{collections::HashMap, sync::Arc};

use ephact::{
    application::{
        dtos::responses::{
            ContainerConfigResponse, ExecResultResponse, HostInfoResponse, RunnerContextResponse,
        },
        ports::outbound::{
            ContainerRuntimePort,
            container_port::{ContainerPort, ExecOptions},
        },
    },
    domain::{entities::FileEntry, errors::ContainerError, messages::events::OutputStream},
};

use crate::{
    e2e_mirrored_action_fetcher::MirroredActionFetcher,
    support::{
        container_activity::ContainerActivity, ephact_application::EphactApplication,
        workflow_repository::WorkflowRepository,
    },
};

const LINT_WORKFLOW: &str = r#"
name: Lint
on: pull_request
jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - run: echo "linting ${{ github.repository }}"
"#;

#[derive(Clone)]
struct EveryWorkflowScenarioFake {
    activity: ContainerActivity,
}

impl EveryWorkflowScenarioFake {
    fn new(activity: ContainerActivity) -> Self {
        Self { activity }
    }
}

impl ContainerRuntimePort for EveryWorkflowScenarioFake {
    fn pull_image(&self, image: &str, _platform: Option<&str>) -> Result<(), ContainerError> {
        self.activity.record_pulled_image(image);
        Ok(())
    }

    fn create_container(
        &self,
        _config: &ContainerConfigResponse,
    ) -> Result<Box<dyn ContainerPort>, ContainerError> {
        Ok(Box::new(self.clone()))
    }

    fn remove_container(&self, _name: &str) -> Result<(), ContainerError> {
        Ok(())
    }

    fn stop_container(&self, name: &str) -> Result<(), ContainerError> {
        self.activity.record_stopped_container(name);
        Ok(())
    }

    fn kill_container(&self, name: &str) -> Result<(), ContainerError> {
        self.activity.record_killed_container(name);
        Ok(())
    }

    fn get_host_info(&self) -> Result<HostInfoResponse, ContainerError> {
        Ok(HostInfoResponse::new("linux", "x86_64", "every-workflow"))
    }
}

impl ContainerPort for EveryWorkflowScenarioFake {
    fn exec(
        &self,
        cmd: &[String],
        _workdir: Option<&str>,
        env: &HashMap<String, String>,
    ) -> Result<ExecResultResponse, ContainerError> {
        self.activity.record_command(cmd, env);
        Ok(ExecResultResponse::new(0, String::new(), String::new()))
    }

    fn exec_streaming(
        &self,
        options: ExecOptions<'_>,
        on_output: &mut dyn FnMut(OutputStream, &str),
    ) -> Result<ExecResultResponse, ContainerError> {
        self.exec(options.cmd(), options.workdir(), options.env())
            .inspect(|result| {
                if !result.stdout().is_empty() {
                    on_output(OutputStream::StandardOutput, result.stdout());
                }
                if !result.stderr().is_empty() {
                    on_output(OutputStream::StandardError, result.stderr());
                }
            })
    }

    fn copy_to(&self, container_path: &str, _entries: &[FileEntry]) -> Result<(), ContainerError> {
        self.activity.record_copy(container_path);
        Ok(())
    }

    fn copy_from(&self, _container_path: &str) -> Result<Vec<FileEntry>, ContainerError> {
        Ok(Vec::new())
    }

    fn remove(&self) -> Result<(), ContainerError> {
        Ok(())
    }

    fn get_runner_context(&self) -> Result<RunnerContextResponse, ContainerError> {
        Ok(RunnerContextResponse::default())
    }
}

const TEST_WORKFLOW: &str = r#"
name: Test
on: pull_request
jobs:
  unit:
    runs-on: ubuntu-latest
    steps:
      - run: echo "unit tests"
  integration:
    needs: unit
    runs-on: ubuntu-latest
    steps:
      - run: echo "integration tests"
"#;

/// Runs every workflow file of a repository in one invocation, covering the
/// `--all-workflows` mode across two files and three jobs.
pub struct EveryWorkflowRun {
    outcome: Result<(), String>,
    activity: ContainerActivity,
}

impl EveryWorkflowRun {
    pub const LINT_SCRIPT: &'static str = r#"echo "linting every-workflow""#;
    pub const UNIT_SCRIPT: &'static str = r#"echo "unit tests""#;
    pub const INTEGRATION_SCRIPT: &'static str = r#"echo "integration tests""#;

    pub fn execute() -> Self {
        let repository = WorkflowRepository::named("every-workflow")
            .with_workflow("lint.yml", LINT_WORKFLOW)
            .with_workflow("test.yml", TEST_WORKFLOW);
        let activity = ContainerActivity::new();
        let workflow_source = Arc::new(
            crate::common::fakes::fake_workflow_source::FakeWorkflowSource::new()
                .with_all_workflow_contents(vec![LINT_WORKFLOW.into(), TEST_WORKFLOW.into()]),
        );
        let application = EphactApplication::compose(
            Arc::new(EveryWorkflowScenarioFake::new(activity.clone())),
            Box::new(MirroredActionFetcher::mirroring(repository.path())),
            workflow_source,
        );

        let outcome = application
            .run([
                "ephact",
                "run",
                &repository.path_argument(),
                "--event",
                "pull_request",
                "--all-workflows",
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
