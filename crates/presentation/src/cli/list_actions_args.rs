use std::path::PathBuf;

use clap::Args;

use crate::{application::dtos::requests::ListActionsRequest, infrastructure::RepositoryResolver};

/// CLI arguments for the `list-actions` command.
#[derive(Args)]
pub struct ListActionsArgs {
    #[arg(default_value = ".")]
    path: PathBuf,
}

impl ListActionsArgs {
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    pub fn to_domain(&self) -> Result<ListActionsRequest, Box<dyn std::error::Error>> {
        let repository = RepositoryResolver::resolve_from_path(self.path.clone())?;
        Ok(ListActionsRequest::new(
            repository.path().as_path().to_path_buf(),
            repository.name().as_str().to_string(),
        ))
    }
}
