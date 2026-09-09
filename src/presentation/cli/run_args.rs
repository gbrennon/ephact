use std::path::PathBuf;

use clap::Args;

use crate::{
    application::dtos::WorkflowInputSource,
    domain::{
        ActRunConfig, Repository,
        value_objects::{
            ActEvent, ActInput, ActJob, ActWorkflow, RepoPath, RepositoryName, Secret,
        },
    },
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
    /// Path to the workflow file to execute (e.g. `ci.yml`).
    #[arg(long)]
    workflow: Option<String>,

    /// Specific job name to run from the workflow.
    #[arg(long)]
    job: Option<String>,

    /// Event name that triggers the workflow (e.g. `push`, `pull_request`).
    #[arg(long)]
    event: Option<String>,

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
    /// it up.
    #[arg(long)]
    preserve: bool,

    /// Use the real Docker or Podman adapter instead of the default runtime
    /// selection.
    #[arg(long = "allow-real-container")]
    allow_real_container: bool,

    /// Use the real action fetcher that contacts the forge instead of a local
    /// mirror.
    #[arg(long = "allow-real-fetcher")]
    allow_real_fetcher: bool,

    /// Allow containers to make outbound network requests.
    #[arg(long = "allow-network")]
    allow_network: bool,

    /// Show real-time step details (running steps and their output) in
    /// addition to the final status of each step.
    #[arg(long)]
    verbose: bool,
}

impl RunArgs {
    /// Converts CLI arguments into the domain model: an [`ActRunConfig`] and a
    /// [`Repository`].
    ///
    /// # Errors
    ///
    /// Returns an error if the repository path is not a valid git repository.
    pub fn to_domain(&self) -> Result<(ActRunConfig, Repository), Box<dyn std::error::Error>> {
        let repository = self.build_repository()?;
        let config = self.build_config()?;
        Ok((config, repository))
    }

    fn build_repository(&self) -> Result<Repository, Box<dyn std::error::Error>> {
        let repo_path = RepoPath::new(self.path.clone()).map_err(|e| format!("{:?}", e))?;
        let repo_name =
            RepositoryName::from_repo_path(&repo_path).map_err(|e| format!("{:?}", e))?;
        Ok(Repository::new(repo_path, repo_name))
    }

    fn build_config(&self) -> Result<ActRunConfig, Box<dyn std::error::Error>> {
        let config = ActRunConfig::new();
        let config = self.apply_targets(config);
        let config = config
            .with_all_workflows(self.all_workflows || self.workflow.is_none())
            .with_allow_real_container(self.allow_real_container)
            .with_allow_real_fetcher(self.allow_real_fetcher)
            .with_allow_network(self.allow_network);
        let config = self.apply_inputs(config)?;
        self.apply_secrets(config)
    }

    fn apply_targets(&self, mut config: ActRunConfig) -> ActRunConfig {
        if let Some(wf) = &self.workflow {
            config = config.with_workflow(ActWorkflow::new(wf.clone()));
        }
        if let Some(job) = &self.job {
            config = config.with_job(ActJob::new(job.clone()));
        }
        if let Some(event) = &self.event {
            config = config.with_event(ActEvent::new(event.clone()));
        }
        config
    }

    fn apply_inputs(
        &self,
        mut config: ActRunConfig,
    ) -> Result<ActRunConfig, Box<dyn std::error::Error>> {
        for input_str in &self.inputs {
            let (k, v) = Self::parse_key_value(input_str)?;
            config = config.add_input(ActInput::new(k, v));
        }
        Ok(config)
    }

    fn apply_secrets(
        &self,
        mut config: ActRunConfig,
    ) -> Result<ActRunConfig, Box<dyn std::error::Error>> {
        for secret_str in &self.secrets {
            let (name, value) = Self::parse_secret(secret_str)?;
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

    /// Reports whether the given argument is the verbose flag.
    pub fn is_verbose_flag(arg: &std::ffi::OsStr) -> bool {
        arg == std::ffi::OsStr::new("--verbose")
    }

    pub fn parse_key_value(s: &str) -> Result<(String, String), String> {
        s.split_once('=')
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .ok_or_else(|| format!("expected KEY=VALUE, got '{}'", s))
    }

    pub fn parse_input_source(s: &str) -> Result<(String, WorkflowInputSource), String> {
        let (key, value) = Self::parse_key_value(s)?;
        let source = value
            .strip_prefix("env:")
            .map(WorkflowInputSource::environment_variable)
            .unwrap_or_else(|| WorkflowInputSource::literal(value));
        Ok((key, source))
    }

    /// Splits a secret argument into its name and value.
    ///
    /// `KEY=VALUE` supplies the value inline; a bare `KEY` reads it from the
    /// environment variable of the same name.
    pub fn parse_secret(s: &str) -> Result<(String, String), String> {
        match s.split_once('=') {
            Some((name, value)) => Ok((name.to_string(), value.to_string())),
            None => std::env::var(s)
                .map(|value| (s.to_string(), value))
                .map_err(|_| {
                    format!("secret '{s}' has no value and no environment variable is set")
                }),
        }
    }
}
