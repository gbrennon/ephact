use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreError {
    /// The provided repository path is invalid or does not exist.
    InvalidRepositoryPath(String),

    /// The provided path is not a git repository (no .git directory found).
    NotAGitRepository(String),

    /// A repository name was required but an empty string was provided.
    EmptyRepositoryName,

    /// An unknown container engine was specified (only "podman" or "docker" are supported).
    UnknownContainerEngine(String),
}

impl fmt::Display for CoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRepositoryPath(message) => formatter.write_str(message),
            Self::NotAGitRepository(path) => {
                write!(formatter, "'{path}' is not a Git repository")
            }
            Self::EmptyRepositoryName => formatter.write_str("repository name cannot be empty"),
            Self::UnknownContainerEngine(engine) => {
                write!(formatter, "unsupported container engine '{engine}'")
            }
        }
    }
}

impl std::error::Error for CoreError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_repository_path_displays_message() {
        let error = CoreError::InvalidRepositoryPath("bad path".into());

        assert_eq!(error.to_string(), "bad path");
    }

    #[test]
    fn not_a_git_repository_displays_path() {
        let error = CoreError::NotAGitRepository("/tmp/x".into());

        assert_eq!(error.to_string(), "'/tmp/x' is not a Git repository");
    }

    #[test]
    fn empty_repository_name_displays_message() {
        let error = CoreError::EmptyRepositoryName;

        assert_eq!(error.to_string(), "repository name cannot be empty");
    }

    #[test]
    fn unknown_container_engine_displays_engine() {
        let error = CoreError::UnknownContainerEngine("rkt".into());

        assert_eq!(error.to_string(), "unsupported container engine 'rkt'");
    }
}
