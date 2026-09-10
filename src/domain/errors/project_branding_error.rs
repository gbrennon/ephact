use std::{error::Error, fmt};

/// Validation errors that can occur when constructing project branding domain objects.
///
/// Each variant represents a violation of the domain invariant that all project branding
/// attributes must have non-empty string values.
///
/// # Examples
///
/// ```
/// # use ephact::domain::errors::ProjectBrandingError;
/// let error: ProjectBrandingError = ProjectBrandingError::EmptyName;
/// assert_eq!(error.to_string(), "project name must not be empty");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectBrandingError {
    /// The project name was empty.
    EmptyName,

    /// The project description was empty.
    EmptyDescription,

    /// The project version was empty.
    EmptyVersion,

    /// The project emblem was empty.
    EmptyEmblem,
}

impl fmt::Display for ProjectBrandingError {
    /// Formats the project branding error into a descriptive message.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "project name must not be empty"),
            Self::EmptyDescription => write!(f, "project description must not be empty"),
            Self::EmptyVersion => write!(f, "project version must not be empty"),
            Self::EmptyEmblem => write!(f, "project emblem must not be empty"),
        }
    }
}

impl Error for ProjectBrandingError {}

#[cfg(test)]
#[test]
fn display_empty_name_formats_expected_message() {
    let error = ProjectBrandingError::EmptyName;
    assert_eq!(error.to_string(), "project name must not be empty");
}

#[cfg(test)]
#[test]
fn display_empty_description_formats_expected_message() {
    let error = ProjectBrandingError::EmptyDescription;
    assert_eq!(error.to_string(), "project description must not be empty");
}

#[cfg(test)]
#[test]
fn display_empty_version_formats_expected_message() {
    let error = ProjectBrandingError::EmptyVersion;
    assert_eq!(error.to_string(), "project version must not be empty");
}

#[cfg(test)]
#[test]
fn display_empty_emblem_formats_expected_message() {
    let error = ProjectBrandingError::EmptyEmblem;
    assert_eq!(error.to_string(), "project emblem must not be empty");
}
