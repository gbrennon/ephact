use std::path::PathBuf;

use uuid::Uuid;

use crate::{
    application::dtos::requests::RunWorkflowRequest,
    application::dtos::responses::RunSummaryResponse,
    application::ports::inbound::run_workflow_port::RunWorkflowPort,
    domain::{RepoPath, Repository, RepositoryName},
};

/// Handles the TUI `Run workflow` action by executing the selected workflow
/// in the current repository through the application run port.
pub struct RunHandler;

impl RunHandler {
    /// Executes a single workflow (specific when named, otherwise detected)
    /// in the repository and returns the run summary.
    pub fn handle(
        run_workflow_port: &dyn RunWorkflowPort,
        repository_path: PathBuf,
        workflow: Option<String>,
    ) -> Result<RunSummaryResponse, Box<dyn std::error::Error>> {
        let repository = Self::build_repository(repository_path)?;
        let request = Self::build_request(&repository, workflow);
        let summary = run_workflow_port.execute(request)?;
        Ok(summary)
    }

    fn build_repository(
        repository_path: PathBuf,
    ) -> Result<Repository, Box<dyn std::error::Error>> {
        let repo_path = RepoPath::new(repository_path).map_err(|e| format!("{e:?}"))?;
        let repo_name = RepositoryName::from_repo_path(&repo_path).map_err(|e| format!("{e:?}"))?;
        Ok(Repository::new(repo_path, repo_name))
    }

    fn build_request(repository: &Repository, workflow: Option<String>) -> RunWorkflowRequest {
        let job = None;
        let event = None;
        let inputs = Vec::new();
        let secrets = Vec::new();
        let all_workflows = false;
        let allow_repo_writes = false;
        let allow_real_container = false;
        let allow_real_fetcher = false;
        let allow_network = false;
        let run_id = Uuid::new_v4().to_string();

        RunWorkflowRequest::new(
            repository.path().as_path().to_path_buf(),
            repository.name().as_str().to_string(),
            workflow,
            job,
            event,
            inputs,
            secrets,
            all_workflows,
            allow_repo_writes,
            allow_real_container,
            allow_real_fetcher,
            allow_network,
            run_id,
        )
    }
}
