use std::fmt;

/// Core validation errors for repository paths, names, and container engines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreError {
    /// The provided repository path value is empty.
    InvalidRepositoryPath(String),

    /// A repository name was required but an empty string was provided.
    EmptyRepositoryName,

    /// An unknown container engine was specified.
    UnknownContainerEngine(String),
}

impl fmt::Display for CoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRepositoryPath(message) => formatter.write_str(message),
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
