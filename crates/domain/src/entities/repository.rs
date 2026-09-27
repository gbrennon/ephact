use crate::value_objects::{RepoPath, RepositoryName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Repository {
    path: RepoPath,
    name: RepositoryName,
}

impl Repository {
    /// Creates a repository from a validated path and name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ephact_domain::value_objects::{RepoPath, RepositoryName};
    /// # use ephact_domain::Repository;
    /// # use std::{env, path::PathBuf};
    /// # let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../..");
    /// let path = RepoPath::new(dir).unwrap();
    /// let name = RepositoryName::new("my-repo".into()).unwrap();
    /// let repo = Repository::new(path, name);
    /// ```
    pub fn new(path: RepoPath, name: RepositoryName) -> Self {
        Self { path, name }
    }

    /// Returns the repository's canonical path.
    pub fn path(&self) -> &RepoPath {
        &self.path
    }

    /// Returns the repository's name.
    pub fn name(&self) -> &RepositoryName {
        &self.name
    }
    /// Returns whether this repository is a standalone checkout.
    pub fn is_standalone(&self) -> bool {
        self.path.is_standalone()
    }

    /// Returns whether this repository is a linked worktree.
    pub fn is_worktree(&self) -> bool {
        self.path.is_worktree()
    }
}

#[cfg(test)]
mod tests {
    use std::{env, path::PathBuf};

    use super::*;

    impl Repository {
        fn workspace_root_for_test() -> PathBuf {
            PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../..")
        }

        fn repo_path_for_test() -> RepoPath {
            let workspace_root = Self::workspace_root_for_test();
            RepoPath::new(workspace_root).unwrap()
        }

        fn repo_name_for_test() -> RepositoryName {
            RepositoryName::new("test-repo".into()).unwrap()
        }
    }

    #[test]
    fn new_sets_provided_path_and_name() {
        let path = Repository::repo_path_for_test();
        let name = Repository::repo_name_for_test();

        let repo = Repository::new(path.clone(), name.clone());

        assert_eq!(repo.path(), &path);
        assert_eq!(repo.name(), &name);
    }

    #[test]
    fn reports_repository_kind() {
        let repository = Repository::new(
            Repository::repo_path_for_test(),
            Repository::repo_name_for_test(),
        );

        assert_eq!(
            repository.is_standalone(),
            repository.path().is_standalone()
        );
        assert_eq!(repository.is_worktree(), repository.path().is_worktree());
    }
}
