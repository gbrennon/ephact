use std::path::PathBuf;

use clap::Args;

use super::input_parser::InputParser;
use crate::{
    cli::run_invocation_flags::RunInvocationFlags,
    domain::{
        Repository, WorkflowRunConfig,
        value_objects::{EventPath, JobName, Secret, WorkflowEvent, WorkflowInput, WorkflowPath},
    },
    infrastructure::RepositoryResolver,
};

/// CLI arguments for the `run` subcommand.
///
/// Parses all user-supplied options (workflow, job, event, inputs, secrets,
/// etc.) and maps them into the domain model via
/// [`to_domain`](Self::to_domain). The container runtime is auto-detected
/// (Docker or Podman) at execution time.
#[derive(Args)]
pub struct RunArgs {
    /// Path to the repository (defaults to the current directory).
    #[arg(default_value = ".")]
    path: PathBuf,
    /// Top-level `name` of the workflow to execute (e.g. CI).
    #[arg(long)]
    workflow: Option<String>,
    /// Specific job name to run from the workflow.
    #[arg(long)]
    job: Option<String>,

    /// Event name that triggers the workflow (e.g. `push`, `pull_request`).
    #[arg(long)]
    event: Option<String>,
    /// Path to a JSON event payload used to populate the github context.
    #[arg(long = "event-path", value_name = "FILE")]
    event_path: Option<PathBuf>,

    /// Workflow inputs in `KEY=VALUE` format (repeatable).
    #[arg(long = "input", value_name = "KEY=VALUE")]
    inputs: Vec<String>,

    /// Secrets in `KEY=VALUE` format, or `KEY` alone to read the value from
    /// the environment (repeatable).
    #[arg(long = "secret", value_name = "KEY[=VALUE]")]
    secrets: Vec<String>,

    #[arg(long)]
    interactive: bool,

    /// Force running every workflow found in the repository. Running all
    /// workflows is already the default; passing `--workflow` narrows the run
    /// to a single named workflow.
    #[arg(long = "all-workflows")]
    all_workflows: bool,

    /// Preserve the ephemeral repository after execution instead of cleaning
    #[arg(long)]
    preserve: bool,

    #[arg(long = "allow-repo-writes")]
    allow_repo_writes: bool,

    #[arg(long = "allow-real-container")]
    allow_real_container: bool,

    /// Use the real action fetcher that contacts the forge instead of a local
    /// mirror.
    #[arg(long = "allow-real-fetcher")]
    allow_real_fetcher: bool,

    /// Allow containers to make outbound network requests.
    #[arg(long = "allow-network")]
    allow_network: bool,

    /// Forward the host SSH agent into job containers.
    #[arg(long = "forward-ssh")]
    forward_ssh: bool,

    /// Show real-time step details (running steps and their output) in
    /// addition to the final status of each step.
    #[arg(long)]
    verbose: bool,
    /// Retain failure diagnostics for this many hours for the current run.
    #[arg(
        long = "failure-log-retention-hours",
        value_name = "HOURS",
        value_parser = RunArgs::parse_positive_hours
    )]
    failure_log_retention_hours: Option<u64>,
}

impl RunArgs {
    fn parse_positive_hours(value: &str) -> Result<u64, String> {
        let hours = value
            .parse::<u64>()
            .map_err(|_| "failure-log-retention-hours must be a positive integer".to_string())?;
        if hours == 0 {
            return Err("failure-log-retention-hours must be greater than zero".to_string());
        }
        Ok(hours)
    }

    /// Converts CLI arguments into the domain model: an [`WorkflowRunConfig`] and a
    /// [`Repository`].
    ///
    /// # Errors
    ///
    /// Returns an error if the repository path is not a valid git repository.
    pub fn to_domain(&self) -> Result<(WorkflowRunConfig, Repository), Box<dyn std::error::Error>> {
        let repository = self.build_repository()?;
        let config = self.build_config()?;
        Ok((config, repository))
    }

    fn build_repository(&self) -> Result<Repository, Box<dyn std::error::Error>> {
        Ok(RepositoryResolver::resolve_from_path(self.path.clone())?)
    }

    fn build_config(&self) -> Result<WorkflowRunConfig, Box<dyn std::error::Error>> {
        self.validate_flags()?;
        let config = self.apply_targets(WorkflowRunConfig::new());
        let config = self.apply_event_path(config)?;
        let config = self.apply_flags(config);
        let config = self.apply_inputs(config)?;
        self.apply_secrets(config)
    }

    fn validate_flags(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.forward_ssh && !self.allow_network {
            return Err("--forward-ssh requires --allow-network".into());
        }
        Ok(())
    }

    fn apply_flags(&self, config: WorkflowRunConfig) -> WorkflowRunConfig {
        config
            .with_all_workflows(self.all_workflows || self.workflow.is_none())
            .with_allow_repo_writes(self.allow_repo_writes)
            .with_allow_real_container(self.allow_real_container)
            .with_allow_real_fetcher(self.allow_real_fetcher)
            .with_allow_network(self.allow_network)
    }

    fn apply_targets(&self, mut config: WorkflowRunConfig) -> WorkflowRunConfig {
        if let Some(wf) = &self.workflow {
            config = config.with_workflow(WorkflowPath::new(wf.clone()));
        }
        if let Some(job) = &self.job {
            config = config.with_job(JobName::new(job.clone()));
        }
        if let Some(event) = self.event.clone() {
            config.with_event(WorkflowEvent::new(event))
        } else {
            config
        }
    }
    fn apply_event_path(
        &self,
        config: WorkflowRunConfig,
    ) -> Result<WorkflowRunConfig, Box<dyn std::error::Error>> {
        match &self.event_path {
            Some(path) => Ok(config.with_event_path(EventPath::new(path.clone())?)),
            None => Ok(config),
        }
    }

    fn apply_inputs(
        &self,
        mut config: WorkflowRunConfig,
    ) -> Result<WorkflowRunConfig, Box<dyn std::error::Error>> {
        for input_str in &self.inputs {
            let (k, v) = InputParser::parse_key_value(input_str)?;
            config = config.add_input(WorkflowInput::new(k, v));
        }
        Ok(config)
    }

    fn apply_secrets(
        &self,
        mut config: WorkflowRunConfig,
    ) -> Result<WorkflowRunConfig, Box<dyn std::error::Error>> {
        for secret_str in &self.secrets {
            let (name, value) = InputParser::parse_secret(secret_str)?;
            config = config.add_secret(Secret::new(name, value));
        }
        Ok(config)
    }

    pub fn interactive(&self) -> bool {
        self.interactive
    }

    /// Reports whether verbose output was requested.
    pub fn verbose(&self) -> bool {
        self.verbose
    }
    /// Returns the flags needed before the application container is constructed.
    pub fn invocation_flags(&self) -> RunInvocationFlags {
        RunInvocationFlags::new(self.verbose, self.allow_network, self.forward_ssh)
    }

    /// Returns the explicit failure-log retention override for this run.
    pub fn failure_log_retention_hours(&self) -> Option<u64> {
        self.failure_log_retention_hours
    }

    /// Returns these arguments with persisted run defaults applied.
    pub fn with_settings(mut self, settings: &crate::domain::Settings) -> Self {
        self.interactive |= settings.interactive();
        self.all_workflows |= settings.all_workflows();
        self.preserve |= settings.preserve();
        self.allow_repo_writes |= settings.allow_repo_writes();
        self.allow_real_container |= settings.allow_real_container();
        self.allow_real_fetcher |= settings.allow_real_fetcher();
        self.allow_network |= settings.allow_network();
        self.verbose |= settings.verbose();
        self
    }
}
