use std::fmt;

use crate::domain::errors::project_branding_error::ProjectBrandingError;

/// Value object representing a validated non-empty project description.
///
/// Encapsulates the project description string while maintaining the invariant that
/// the contained description cannot be empty.
///
/// # Examples
///
/// ```
/// # use ephact::domain::value_objects::ProjectDescription;
/// let description: ProjectDescription =
///     ProjectDescription::new("A fast runner".to_string()).unwrap();
/// assert_eq!(description.as_str(), "A fast runner");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectDescription {
    value: String,
}

impl ProjectDescription {
    /// Creates a validated project description from a string.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectBrandingError::EmptyDescription`] if the provided string is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ephact::domain::value_objects::ProjectDescription;
    /// # use ephact::domain::errors::ProjectBrandingError;
    /// let valid: Result<ProjectDescription, ProjectBrandingError> =
    ///     ProjectDescription::new("A fast runner".to_string());
    /// assert!(valid.is_ok());
    ///
    /// let invalid: Result<ProjectDescription, ProjectBrandingError> =
    ///     ProjectDescription::new("".to_string());
    /// assert_eq!(invalid, Err(ProjectBrandingError::EmptyDescription));
    /// ```
    pub fn new(value: String) -> Result<Self, ProjectBrandingError> {
        match value.is_empty() {
            true => Err(ProjectBrandingError::EmptyDescription),
            false => Ok(Self { value }),
        }
    }

    /// Returns the project description as an immutable string slice.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for ProjectDescription {
    /// Formats the project description using its inner string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
#[test]
fn new_with_valid_description_succeeds() {
    let description = ProjectDescription::new("A fast runner".to_string()).unwrap();
    assert_eq!(description.as_str(), "A fast runner");
}

#[cfg(test)]
#[test]
fn new_with_empty_description_returns_empty_description_error() {
    let result = ProjectDescription::new("".to_string());
    assert_eq!(result, Err(ProjectBrandingError::EmptyDescription));
}

#[cfg(test)]
#[test]
fn display_formats_inner_string() {
    let description = ProjectDescription::new("A fast runner".to_string()).unwrap();
    assert_eq!(format!("{description}"), "A fast runner");
}
