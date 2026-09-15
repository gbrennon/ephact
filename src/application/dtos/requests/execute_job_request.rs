use std::path::{Path, PathBuf};

pub struct ExecuteJobRequest {
    repo_path: PathBuf,
    context: Vec<(String, String)>,
    run_id: String,
    allow_repo_writes: bool,
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
}
