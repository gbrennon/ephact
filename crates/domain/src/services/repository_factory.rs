use std::path::PathBuf;

use crate::{RepoPath, Repository, RepositoryName, errors::core_error::CoreError};

/// Constructs repositories from primitive boundary data.
pub struct RepositoryFactory;

impl RepositoryFactory {
    /// Creates a validated repository from its path and display name.
    pub fn create(path: PathBuf, name: String) -> Result<Repository, CoreError> {
        let repository_path = RepoPath::new(path)?;
        let repository_name = RepositoryName::new(name)?;
        Ok(Repository::new(repository_path, repository_name))
    }
}

#[cfg(test)]
mod tests {
    use std::{env, path::PathBuf};

    use super::RepositoryFactory;

    fn workspace_root() -> PathBuf {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../..")
    }

    #[test]
    fn creates_a_repository_from_primitive_data() {
        let path = workspace_root();

        let repository = RepositoryFactory::create(path, "ephact".to_string()).unwrap();

        assert_eq!(repository.name().as_str(), "ephact");
    }
}
