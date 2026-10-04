use std::path::{Path, PathBuf};

pub struct SingleWorkflowRun {
    repository_path: PathBuf,
    workflow: Option<String>,
    event: Option<String>,
    inputs: Vec<(String, String)>,
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

    pub fn run_id(&self) -> &str {
        &self.run_id
    }
}
