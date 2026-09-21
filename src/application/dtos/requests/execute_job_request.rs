use std::path::{Path, PathBuf};

pub struct ExecuteJobRequest {
    repo_path: PathBuf,
    context: Vec<(String, String)>,
    run_id: String,
    allow_repo_writes: bool,
    allow_network: bool,
}

impl ExecuteJobRequest {
    pub fn new(
        repo_path: impl Into<PathBuf>,
        context: Vec<(String, String)>,
        run_id: impl Into<String>,
        allow_repo_writes: bool,
    ) -> Self {
        Self {
            repo_path: repo_path.into(),
            context,
            run_id: run_id.into(),
            allow_repo_writes,
            allow_network: false,
        }
    }

    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }
    pub fn context(&self) -> &[(String, String)] {
        &self.context
    }
    pub fn run_id(&self) -> &str {
        &self.run_id
    }
    pub fn allow_repo_writes(&self) -> bool {
        self.allow_repo_writes
    }

    pub fn with_allow_network(mut self, allow_network: bool) -> Self {
        self.allow_network = allow_network;
        self
    }

    pub fn allow_network(&self) -> bool {
        self.allow_network
    }
}
