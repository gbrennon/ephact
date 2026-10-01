use crate::value_objects::{RepoPath, RepositoryName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Repository {
    path: RepoPath,
    name: RepositoryName,
}

impl Repository {
    /// Creates a repository from boundary-provided location and name values.
    pub fn new(path: RepoPath, name: RepositoryName) -> Self {
        Self { path, name }
    }

    /// Returns the repository location.
    pub fn path(&self) -> &RepoPath {
        &self.path
    }

    /// Returns the repository name.
    pub fn name(&self) -> &RepositoryName {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::*;

    fn repository_for_test() -> Repository {
        Repository::new(
            RepoPath::new(PathBuf::from("workspace")).unwrap(),
            RepositoryName::new("test-repo".to_owned()).unwrap(),
        )
    }

    #[test]
    fn new_sets_provided_path_and_name() {
        let repository = repository_for_test();

        assert_eq!(repository.path().as_path(), Path::new("workspace"));
        assert_eq!(repository.name().as_str(), "test-repo");
    }
}
