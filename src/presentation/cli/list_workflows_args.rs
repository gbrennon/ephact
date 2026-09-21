use std::path::PathBuf;

use clap::Args;

use crate::{
    application::dtos::requests::ListWorkflowsRequest,
    domain::{RepoPath, Repository, RepositoryName},
};

/// CLI arguments for the `list-workflows` command.
///
/// Maps directly to `ListWorkflowsArgs` from `clap::Args`.
#[derive(Args)]
pub struct ListWorkflowsArgs {
    #[arg(default_value = ".")]
    path: PathBuf,
}

impl ListWorkflowsArgs {
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    /// Converts CLI arguments into the domain model: a [`ListWorkflowsRequest`].
    ///
    /// This translation keeps the application layer agnostic to filesystem
    /// details by constructing a domain [`Repository`] from the path.
    pub fn to_domain(&self) -> Result<ListWorkflowsRequest, Box<dyn std::error::Error>> {
        let repo_path = RepoPath::new(self.path.clone()).map_err(|e| format!("{:?}", e))?;
        let repo_name =
            RepositoryName::from_repo_path(&repo_path).map_err(|e| format!("{:?}", e))?;
        let repository = Repository::new(repo_path, repo_name);
        Ok(ListWorkflowsRequest::new(
            repository.path().as_path().to_path_buf(),
            repository.name().as_str().to_string(),
        ))
    }
}
