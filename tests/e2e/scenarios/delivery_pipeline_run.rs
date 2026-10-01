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
use parking_lot::Mutex;

use crate::support::{
    container_activity::ContainerActivity, ephact_application::EphactApplication,
    workflow_repository::WorkflowRepository,
};

const PIPELINE_WORKFLOW: &str = r#"
name: Delivery Pipeline
on: pull_request
env:
  PIPELINE: delivery
jobs:
  publish:
    needs: package
    runs-on: ubuntu-latest
    steps:
      - run: echo "publishing ${{ inputs.channel }} with ${{ secrets.REGISTRY_TOKEN }}"
  build:
    runs-on: ubuntu-latest
    env:
      STAGE: build
    steps:
      - uses: actions/checkout@v4
      - run: echo "pipeline=${{ env.PIPELINE }} stage=${{ env.STAGE }}"
      - run: echo "event=${{ github.event_name }} repo=${{ github.repository }} os=${{ runner.os }}"
  package:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: ./.forgejo/actions/package
        with:
          artifact: delivery.tar
"#;

const PACKAGE_ACTION: &str = r#"
name: Package
description: Packages the build output
inputs:
  artifact:
    description: Name of the artifact to package
    required: true
  compression:
    description: Compression algorithm to apply
    default: gzip
runs:
  using: composite
  steps:
    - run: echo "packaging ${{ inputs.artifact }} with ${{ inputs.compression }}"
    - uses: ./.forgejo/actions/checksum
      with:
        artifact: ${{ inputs.artifact }}
"#;

const CHECKSUM_ACTION: &str = r#"
name: Checksum
description: Signs the packaged artifact
inputs:
  artifact:
    description: Name of the artifact to sign
    required: true
runs:
  using: composite
  steps:
    - run: echo "checksum for ${{ inputs.artifact }} signed with ${{ secrets.REGISTRY_TOKEN }}"
"#;

#[derive(Clone)]
pub struct DeliveryPipelineFetcherFake {
    action_directory: PathBuf,
    fetched: Arc<Mutex<Vec<RemoteActionReference>>>,
}

impl DeliveryPipelineFetcherFake {
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

impl ActionFetcherPort for DeliveryPipelineFetcherFake {
    fn fetch(&self, reference: &RemoteActionReference) -> Result<PathBuf, ActionError> {
        self.fetched.lock().push(reference.clone());
        Ok(self.action_directory.clone())
    }

    fn clone_box(&self) -> Box<dyn ActionFetcherPort> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
struct DeliveryPipelineScenarioFake {
    activity: ContainerActivity,
}

impl DeliveryPipelineScenarioFake {
    fn new(activity: ContainerActivity) -> Self {
        Self { activity }
    }
}

impl ContainerRuntimePort for DeliveryPipelineScenarioFake {
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
            "delivery-pipeline",
        ))
    }
}

impl ContainerPort for DeliveryPipelineScenarioFake {
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

/// Runs a workflow whose jobs depend on each other and whose steps cover
/// workflow and job environments, the `github`, `runner`, `inputs` and
/// `secrets` contexts, a checked-out action, a local composite action with an
/// input default, and an action nested inside that composite action.
pub struct DeliveryPipelineRun {
    outcome: Result<(), String>,
    activity: ContainerActivity,
    fetcher: DeliveryPipelineFetcherFake,
}

impl DeliveryPipelineRun {
    pub const ENVIRONMENT_SCRIPT: &'static str = r#"echo "pipeline=delivery stage=build""#;
    pub const CONTEXT_SCRIPT: &'static str =
        r#"echo "event=pull_request repo=delivery-pipeline os=Linux""#;
    pub const PACKAGE_SCRIPT: &'static str = r#"echo "packaging delivery.tar with gzip""#;
    pub const CHECKSUM_SCRIPT: &'static str =
        r#"echo "checksum for delivery.tar signed with super-secret""#;
    pub const PUBLISH_SCRIPT: &'static str = r#"echo "publishing staging with super-secret""#;

    pub fn execute() -> Self {
        let repository = WorkflowRepository::named("delivery-pipeline")
            .with_workflow("pipeline.yml", PIPELINE_WORKFLOW)
            .with_action(".forgejo/actions/package", PACKAGE_ACTION)
            .with_action(".forgejo/actions/checksum", CHECKSUM_ACTION);
        let activity = ContainerActivity::new();
        let fetcher = DeliveryPipelineFetcherFake::mirroring(repository.path());
        let workflow_source = Arc::new(
            crate::common::fakes::fake_workflow_source::FakeWorkflowSource::new()
                .with_workflow_content(PIPELINE_WORKFLOW),
        );
        let application = EphactApplication::compose(
            Arc::new(DeliveryPipelineScenarioFake::new(activity.clone())),
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
                "pipeline.yml",
                "--input",
                "channel=staging",
                "--secret",
                "REGISTRY_TOKEN=super-secret",
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

    pub fn fetcher(&self) -> &DeliveryPipelineFetcherFake {
        &self.fetcher
    }
}
