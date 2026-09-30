use std::{collections::HashMap, path::PathBuf, sync::Arc};

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
    domain::{
        entities::FileEntry,
        errors::{ActionError, ContainerError},
        messages::events::OutputStream,
        value_objects::RemoteActionReference,
    },
    infrastructure::actions::ActionFetcherPort,
};
use parking_lot::Mutex;

use crate::support::{
    container_activity::ContainerActivity, ephact_application::EphactApplication,
    remote_action_mirror::RemoteActionMirror, workflow_repository::WorkflowRepository,
};

const TOOLCHAIN_WORKFLOW: &str = r#"
name: Remote Toolchain
on: pull_request
jobs:
  setup:
    runs-on: ubuntu-latest
    steps:
      - uses: https://data.forgejo.org/actions/setup-node@v4
        with:
          node-version: "20"
      - run: echo "toolchain ready for ${{ github.repository }}"
"#;

const SETUP_NODE_ACTION: &str = r#"
name: Setup Node
description: Installs a node toolchain
inputs:
  node-version:
    description: Version of node to install
    default: "18"
runs:
  using: node20
  main: index.js
"#;

const SETUP_NODE_ENTRY_POINT: &str = "console.log('setup-node');\n";

#[derive(Clone)]
pub struct RemoteActionFetcherFake {
    action_directory: PathBuf,
    fetched: Arc<Mutex<Vec<RemoteActionReference>>>,
}

impl RemoteActionFetcherFake {
    fn mirroring(action_directory: PathBuf) -> Self {
        Self {
            action_directory,
            fetched: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn fetched(&self) -> Vec<RemoteActionReference> {
        self.fetched.lock().clone()
    }
}

impl ActionFetcherPort for RemoteActionFetcherFake {
    fn fetch(&self, reference: &RemoteActionReference) -> Result<PathBuf, ActionError> {
        self.fetched.lock().push(reference.clone());
        Ok(self.action_directory.clone())
    }

    fn clone_box(&self) -> Box<dyn ActionFetcherPort> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
struct RemoteActionScenarioFake {
    activity: ContainerActivity,
}

impl RemoteActionScenarioFake {
    fn new(activity: ContainerActivity) -> Self {
        Self { activity }
    }
}

impl ContainerRuntimePort for RemoteActionScenarioFake {
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
        Ok(HostInfoResponse::new("linux", "x86_64", "remote-action"))
    }
}

impl ContainerPort for RemoteActionScenarioFake {
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

/// Runs a workflow that references an action hosted on a forge other than
/// GitHub, so the reference has to be parsed, fetched, copied into the
/// container, and executed as a JavaScript action.
pub struct RemoteActionPipelineRun {
    outcome: Result<(), String>,
    activity: ContainerActivity,
    fetcher: RemoteActionFetcherFake,
}

impl RemoteActionPipelineRun {
    pub const TOOLCHAIN_SCRIPT: &'static str = r#"echo "toolchain ready for remote-toolchain""#;
    pub const ENTRY_POINT_FILE: &'static str = "index.js";
    pub const CONTAINER_ACTIONS_ROOT: &'static str = "ephact-actions";
    pub const INPUT_VARIABLE: &'static str = "INPUT_NODE-VERSION";

    pub fn execute() -> Self {
        let repository = WorkflowRepository::named("remote-toolchain")
            .with_workflow("toolchain.yml", TOOLCHAIN_WORKFLOW);
        let mirror = RemoteActionMirror::new()
            .with_definition(SETUP_NODE_ACTION)
            .with_file(Self::ENTRY_POINT_FILE, SETUP_NODE_ENTRY_POINT);
        let activity = ContainerActivity::new();
        let fetcher = RemoteActionFetcherFake::mirroring(mirror.path());
        let workflow_source = Arc::new(
            crate::common::fakes::fake_workflow_source::FakeWorkflowSource::new()
                .with_workflow_content(TOOLCHAIN_WORKFLOW),
        );
        let application = EphactApplication::compose(
            Arc::new(RemoteActionScenarioFake::new(activity.clone())),
            Box::new(fetcher.clone()),
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
                "toolchain.yml",
            ])
            .map_err(|error| error.to_string());

        Self {
            outcome,
            activity,
            fetcher,
        }
    }

    pub fn outcome(&self) -> &Result<(), String> {
        &self.outcome
    }

    pub fn activity(&self) -> &ContainerActivity {
        &self.activity
    }

    pub fn fetcher(&self) -> &RemoteActionFetcherFake {
        &self.fetcher
    }
}
