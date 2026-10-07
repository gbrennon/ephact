use std::{collections::HashMap, path::PathBuf, sync::Arc};

use ephact::{
    application::{
        dtos::responses::{
            ContainerConfigResponse, ExecResultResponse, HostInfoResponse, RunnerContextResponse,
        },
        ports::outbound::{
            ActionFetcherPort, ContainerRuntimePort,
            container_port::{ContainerPort, ExecOptions},
        },
    },
    domain::{
        entities::FileEntry,
        errors::{ActionError, ContainerError},
        messages::events::OutputStream,
        value_objects::RemoteActionReference,
    },
};

use crate::support::{
    container_activity::ContainerActivity, ephact_application::EphactApplication,
    workflow_repository::WorkflowRepository,
};

const AUDIT_WORKFLOW: &str = r#"
name: Audit
on: pull_request
jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      - run: echo "auditing dependencies"
        continue-on-error: true
      - run: echo "auditing licenses"
        continue-on-error: true
"#;

pub struct ContinueOnErrorFetcherFake {
    action_directory: PathBuf,
}

impl ContinueOnErrorFetcherFake {
    fn mirroring(action_directory: PathBuf) -> Self {
        Self { action_directory }
    }
}

impl ActionFetcherPort for ContinueOnErrorFetcherFake {
    fn fetch(&self, _reference: &RemoteActionReference) -> Result<PathBuf, ActionError> {
        Ok(self.action_directory.clone())
    }

    fn clone_box(&self) -> Box<dyn ActionFetcherPort> {
        Box::new(Self::mirroring(self.action_directory.clone()))
    }
}

#[derive(Clone)]
struct ContinueOnErrorScenarioFake {
    activity: ContainerActivity,
}

impl ContinueOnErrorScenarioFake {
    fn new(activity: ContainerActivity) -> Self {
        Self { activity }
    }
}

impl ContainerRuntimePort for ContinueOnErrorScenarioFake {
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
        Ok(HostInfoResponse::new(
            "linux",
            "x86_64",
            "continue-on-error",
        ))
    }
}

impl ContainerPort for ContinueOnErrorScenarioFake {
    fn exec(
        &self,
        cmd: &[String],
        _workdir: Option<&str>,
        env: &HashMap<String, String>,
    ) -> Result<ExecResultResponse, ContainerError> {
        self.activity.record_command(cmd, env);
        if cmd.len() == 3
            && cmd[0] == "sh"
            && cmd[1] == "-c"
            && cmd[2].starts_with("mkdir -p /tmp && touch ")
        {
            return Ok(ExecResultResponse::new(0, String::new(), String::new()));
        }
        Ok(ExecResultResponse::new(1, String::new(), String::new()))
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

/// Runs a workflow whose failing steps are all marked `continue-on-error`
/// inside a container where every command fails, covering that a tolerated
/// failure keeps the run successful.
pub struct ContinueOnErrorPipelineRun {
    outcome: Result<(), String>,
    activity: ContainerActivity,
}

impl ContinueOnErrorPipelineRun {
    pub const DEPENDENCY_SCRIPT: &'static str = r#"echo "auditing dependencies""#;
    pub const LICENSE_SCRIPT: &'static str = r#"echo "auditing licenses""#;

    pub fn execute() -> Self {
        let repository =
            WorkflowRepository::named("audit-pipeline").with_workflow("audit.yml", AUDIT_WORKFLOW);
        let activity = ContainerActivity::new();
        let workflow_source = Arc::new(
            crate::common::fakes::fake_workflow_source::FakeWorkflowSource::new()
                .with_workflow_content(AUDIT_WORKFLOW),
        );
        let application = EphactApplication::compose(
            Arc::new(ContinueOnErrorScenarioFake::new(activity.clone())),
            Box::new(ContinueOnErrorFetcherFake::mirroring(repository.path())),
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
                "audit.yml",
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
