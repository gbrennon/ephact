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
    infrastructure::{FilesystemWorkflowSource, actions::GitActionFetcher},
};

use crate::support::{
    container_activity::ContainerActivity, ephact_application::EphactApplication,
    workflow_repository::WorkflowRepository,
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

#[derive(Clone)]
struct WoodpeckerScenarioFake {
    activity: ContainerActivity,
}

impl WoodpeckerScenarioFake {
    fn new(activity: ContainerActivity) -> Self {
        Self { activity }
    }
}

impl ContainerRuntimePort for WoodpeckerScenarioFake {
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
        Ok(HostInfoResponse::new("linux", "x86_64", "woodpecker"))
    }
}

impl ContainerPort for WoodpeckerScenarioFake {
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
            Arc::new(WoodpeckerScenarioFake::new(activity.clone())),
            Box::new(GitActionFetcher::with_default_cache_root()),
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
