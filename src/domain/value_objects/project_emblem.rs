use std::fmt;

use crate::domain::errors::project_branding_error::ProjectBrandingError;

/// Value object representing a validated non-empty project emblem.
///
/// Encapsulates the project emblem identifier while maintaining the invariant that
/// the contained emblem string cannot be empty.
///
/// # Examples
///
/// ```
/// # use ephact::domain::value_objects::ProjectEmblem;
/// let emblem: ProjectEmblem = ProjectEmblem::new("shield".to_string()).unwrap();
/// assert_eq!(emblem.as_str(), "shield");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectEmblem {
    value: String,
}

impl ProjectEmblem {
    /// Creates a validated project emblem from a string.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectBrandingError::EmptyEmblem`] if the provided string is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ephact::domain::value_objects::ProjectEmblem;
    /// # use ephact::domain::errors::ProjectBrandingError;
    /// let valid: Result<ProjectEmblem, ProjectBrandingError> =
    ///     ProjectEmblem::new("shield".to_string());
    /// assert!(valid.is_ok());
    ///
    /// let invalid: Result<ProjectEmblem, ProjectBrandingError> =
    ///     ProjectEmblem::new("".to_string());
    /// assert_eq!(invalid, Err(ProjectBrandingError::EmptyEmblem));
    /// ```
    pub fn new(value: String) -> Result<Self, ProjectBrandingError> {
        match value.is_empty() {
            true => Err(ProjectBrandingError::EmptyEmblem),
            false => Ok(Self { value }),
        }
    }

    /// Returns the project emblem as an immutable string slice.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for ProjectEmblem {
    /// Formats the project emblem using its inner string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
#[test]
fn new_with_valid_emblem_succeeds() {
    let emblem = ProjectEmblem::new("shield".to_string()).unwrap();
    assert_eq!(emblem.as_str(), "shield");
}

#[cfg(test)]
#[test]
fn new_with_empty_emblem_returns_empty_emblem_error() {
    let result = ProjectEmblem::new("".to_string());
    assert_eq!(result, Err(ProjectBrandingError::EmptyEmblem));
}

#[cfg(test)]
#[test]
fn display_formats_inner_string() {
    let emblem = ProjectEmblem::new("shield".to_string()).unwrap();
    assert_eq!(format!("{emblem}"), "shield");
}
