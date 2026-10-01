use std::path::PathBuf;

use clap::Args;

use crate::{
    application::dtos::requests::ListWorkflowsRequest, infrastructure::RepositoryResolver,
};

/// CLI arguments for the `list-workflows` command.
#[derive(Args)]
pub struct ListWorkflowsArgs {
    #[arg(default_value = ".")]
    path: PathBuf,
}

impl ListWorkflowsArgs {
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    pub fn to_domain(&self) -> Result<ListWorkflowsRequest, Box<dyn std::error::Error>> {
        let repository = RepositoryResolver::resolve_from_path(self.path.clone())?;
        Ok(ListWorkflowsRequest::new(
            repository.path().as_path().to_path_buf(),
            repository.name().as_str().to_string(),
        ))
    }
}
