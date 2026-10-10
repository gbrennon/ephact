use std::{
    collections::HashMap,
    ffi::OsString,
    os::unix::net::UnixListener,
    path::PathBuf,
    sync::{Arc, Mutex},
};

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
    infrastructure::{actions::GitActionFetcher, containers::HostSshForwardingConfig},
    presentation::cli::RunArgs,
};

use crate::{
    common::fakes::fake_workflow_source::FakeWorkflowSource,
    support::{ephact_application::EphactApplication, workflow_repository::WorkflowRepository},
};

const WORKFLOW: &str = r#"
name: SSH Forwarding
on: push
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - run: echo "ssh forwarding"
"#;
const CONTAINER_SOCKET: &str = "/tmp/ephact-ssh-agent.sock";

#[derive(Clone)]
pub struct SshForwardingRun {
    outcome: Result<(), String>,
    configurations: Arc<Mutex<Vec<ContainerConfigResponse>>>,
    socket_path: PathBuf,
}

impl SshForwardingRun {
    pub fn execute() -> Self {
        let repository = WorkflowRepository::named("ssh-forwarding")
            .with_workflow("ssh-forwarding.yml", WORKFLOW);
        let socket_directory = tempfile::tempdir().unwrap();
        let socket_path = socket_directory.path().join("agent.sock");
        let _socket_listener = UnixListener::bind(&socket_path).unwrap();
        let arguments = vec![
            OsString::from("ephact"),
            OsString::from("cli"),
            OsString::from("run"),
            OsString::from(repository.path_argument()),
            OsString::from("--event"),
            OsString::from("push"),
            OsString::from("--workflow"),
            OsString::from("ssh-forwarding.yml"),
            OsString::from("--allow-network"),
            OsString::from("--forward-ssh"),
        ];
        let ssh_forwarding = if RunArgs::is_forward_ssh_command(&arguments) {
            HostSshForwardingConfig::enabled_with_socket(socket_path.clone()).unwrap()
        } else {
            HostSshForwardingConfig::disabled()
        };
        let runtime = Self::recording(socket_path.clone());
        let configurations = runtime.configurations.clone();
        let workflow_source = Arc::new(FakeWorkflowSource::new().with_workflow_content(WORKFLOW));
        let application = EphactApplication::compose_with_ssh_forwarding(
            Arc::new(runtime),
            Box::new(GitActionFetcher::with_default_cache_root()),
            workflow_source,
            ssh_forwarding,
        );

        let outcome = application
            .run(arguments)
            .map_err(|error| error.to_string());

        Self {
            outcome,
            configurations,
            socket_path,
        }
    }

    pub fn outcome(&self) -> &Result<(), String> {
        &self.outcome
    }

    pub fn created_agent_mount(&self) -> bool {
        let expected_bind = format!("{}:{CONTAINER_SOCKET}", self.socket_path.display());
        self.configurations
            .lock()
            .unwrap()
            .iter()
            .any(|configuration| {
                configuration
                    .binds()
                    .iter()
                    .any(|bind| bind == &expected_bind)
            })
    }

    pub fn created_agent_environment(&self) -> bool {
        self.configurations
            .lock()
            .unwrap()
            .iter()
            .any(|configuration| {
                configuration.env().get("SSH_AUTH_SOCK") == Some(&CONTAINER_SOCKET.to_string())
            })
    }

    fn recording(socket_path: PathBuf) -> Self {
        Self {
            outcome: Ok(()),
            configurations: Arc::new(Mutex::new(Vec::new())),
            socket_path,
        }
    }
}

impl ContainerRuntimePort for SshForwardingRun {
    fn pull_image(&self, _image: &str, _platform: Option<&str>) -> Result<(), ContainerError> {
        Ok(())
    }

    fn create_container(
        &self,
        configuration: &ContainerConfigResponse,
    ) -> Result<Box<dyn ContainerPort>, ContainerError> {
        self.configurations
            .lock()
            .unwrap()
            .push(configuration.clone());
        Ok(Box::new(self.clone()))
    }

    fn remove_container(&self, _name: &str) -> Result<(), ContainerError> {
        Ok(())
    }

    fn stop_container(&self, _name: &str) -> Result<(), ContainerError> {
        Ok(())
    }

    fn kill_container(&self, _name: &str) -> Result<(), ContainerError> {
        Ok(())
    }

    fn get_host_info(&self) -> Result<HostInfoResponse, ContainerError> {
        Ok(HostInfoResponse::new("linux", "x86_64", "ssh-forwarding"))
    }
}

impl ContainerPort for SshForwardingRun {
    fn exec(
        &self,
        _cmd: &[String],
        _workdir: Option<&str>,
        _env: &HashMap<String, String>,
    ) -> Result<ExecResultResponse, ContainerError> {
        Ok(ExecResultResponse::new(0, String::new(), String::new()))
    }

    fn exec_streaming(
        &self,
        options: ExecOptions<'_>,
        _on_output: &mut dyn FnMut(OutputStream, &str),
    ) -> Result<ExecResultResponse, ContainerError> {
        self.exec(options.cmd(), options.workdir(), options.env())
    }

    fn copy_to(&self, _container_path: &str, _entries: &[FileEntry]) -> Result<(), ContainerError> {
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
