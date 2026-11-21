use std::path::{Path, PathBuf};

use crate::domain::Settings;

pub struct SingleWorkflowRun {
    repository_path: PathBuf,
    workflow: Option<String>,
    event: Option<String>,
    inputs: Vec<(String, String)>,
    allow_repo_writes: bool,
    allow_real_container: bool,
    allow_real_fetcher: bool,
    allow_network: bool,
    run_id: String,
}

impl SingleWorkflowRun {
    pub fn new(
        repository_path: PathBuf,
        workflow: Option<String>,
        event: Option<String>,
        inputs: Vec<(String, String)>,
        run_id: &str,
    ) -> Self {
        Self {
            repository_path,
            workflow,
            event,
            inputs,
            allow_repo_writes: false,
            allow_real_container: false,
            allow_real_fetcher: false,
            allow_network: false,
            run_id: run_id.to_owned(),
        }
    }

    pub fn repository_path(&self) -> &Path {
        &self.repository_path
    }

    pub fn workflow(&self) -> Option<&str> {
        self.workflow.as_deref()
    }

    pub fn event(&self) -> Option<&str> {
        self.event.as_deref()
    }

    pub fn inputs(&self) -> &[(String, String)] {
        &self.inputs
    }

    pub fn with_settings(mut self, settings: &Settings) -> Self {
        self.allow_repo_writes = settings.allow_repo_writes();
        self.allow_real_container = settings.allow_real_container();
        self.allow_real_fetcher = settings.allow_real_fetcher();
        self.allow_network = settings.allow_network();
        self
    }

    pub fn allow_repo_writes(&self) -> bool {
        self.allow_repo_writes
    }

    pub fn allow_real_container(&self) -> bool {
        self.allow_real_container
    }

    pub fn allow_real_fetcher(&self) -> bool {
        self.allow_real_fetcher
    }

    pub fn allow_network(&self) -> bool {
        self.allow_network
    }

    pub fn run_id(&self) -> &str {
        &self.run_id
    }
}
