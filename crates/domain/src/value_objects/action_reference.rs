pub mod remote_reference_parts;

use remote_reference_parts::RemoteReferenceParts;

use crate::{errors::ActionError, value_objects::RemoteActionReference};

/// A parsed `uses:` value, classified by how the action must be resolved.
///
/// Parsing is platform-agnostic: shorthand references resolve against
/// `github.com`, while any forge can be addressed with an explicit URL, so
/// `actions/cache@v4` and `https://data.forgejo.org/actions/cache@v4` are both
/// understood.
///
/// # Examples
///
/// ```
/// # use ephact_domain::value_objects::ActionReference;
/// let reference = ActionReference::parse("https://data.forgejo.org/actions/cache@v4").unwrap();
/// let ActionReference::Remote(remote) = reference else { panic!("expected a remote action") };
/// assert_eq!(remote.clone_url(), "https://data.forgejo.org/actions/cache");
/// assert_eq!(remote.git_ref(), "v4");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionReference {
    /// An action stored in the repository under test, addressed by a relative
    /// path (`./.forgejo/actions/publish`).
    Local(String),

    /// An action published in a git repository on some forge.
    Remote(RemoteActionReference),

    /// An action delivered as a container image (`docker://image:tag`).
    Docker(String),
}

impl ActionReference {
    /// Classifies a raw `uses:` value.
    ///
    /// # Errors
    ///
    /// Returns [`ActionError::InvalidReference`] when the value names neither a
    /// relative path, a container image, nor an `owner/repo` pair.
    pub fn parse(raw: &str) -> Result<Self, ActionError> {
        let reference = raw.trim();
        if reference.is_empty() {
            return Err(ActionError::InvalidReference(raw.to_string()));
        }
        if let Some(image) = reference.strip_prefix("docker://") {
            return Ok(Self::Docker(image.to_string()));
        }
        if reference.starts_with("./") || reference.starts_with("../") {
            return Ok(Self::Local(reference.to_string()));
        }
        Self::parse_remote(reference, raw)
    }

    fn parse_remote(reference: &str, raw: &str) -> Result<Self, ActionError> {
        RemoteReferenceParts::parse(reference)
            .map(|parts| Self::Remote(parts.into_remote()))
            .ok_or_else(|| ActionError::InvalidReference(raw.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_objects::RemoteActionReference;

    impl ActionReference {
        fn remote_for_test(raw: &str) -> RemoteActionReference {
            match Self::parse(raw).unwrap() {
                Self::Remote(remote) => remote,
                other => panic!("expected a remote action, got {other:?}"),
            }
        }
    }

    #[test]
    fn parse_relative_path_is_local() {
        assert_eq!(
            ActionReference::parse("./.forgejo/actions/publish").unwrap(),
            ActionReference::Local("./.forgejo/actions/publish".into())
        );
    }

    #[test]
    fn parse_parent_relative_path_is_local() {
        assert_eq!(
            ActionReference::parse("../shared/action").unwrap(),
            ActionReference::Local("../shared/action".into())
        );
    }

    #[test]
    fn parse_docker_image_is_docker() {
        assert_eq!(
            ActionReference::parse("docker://alpine:3.20").unwrap(),
            ActionReference::Docker("alpine:3.20".into())
        );
    }

    #[test]
    fn parse_shorthand_defaults_to_github() {
        let reference = ActionReference::remote_for_test("actions/checkout@v4");

        assert_eq!(reference.clone_url(), "https://github.com/actions/checkout");
        assert_eq!(reference.git_ref(), "v4");
        assert_eq!(reference.directory(), None);
    }

    #[test]
    fn parse_shorthand_without_ref_defaults_to_main() {
        assert_eq!(
            ActionReference::remote_for_test("actions/checkout").git_ref(),
            "main"
        );
    }

    #[test]
    fn parse_full_url_keeps_host() {
        let reference =
            ActionReference::remote_for_test("https://data.forgejo.org/actions/cache@v4");

        assert_eq!(reference.host(), "data.forgejo.org");
        assert_eq!(reference.owner(), "actions");
        assert_eq!(reference.repo(), "cache");
        assert_eq!(reference.git_ref(), "v4");
    }

    #[test]
    fn parse_keeps_action_subdirectory() {
        let reference =
            ActionReference::remote_for_test("https://gitlab.com/group/tools/deploy/action@main");

        assert_eq!(reference.clone_url(), "https://gitlab.com/group/tools");
        assert_eq!(reference.directory(), Some("deploy/action"));
    }

    #[test]
    fn parse_commit_sha_ref() {
        assert_eq!(
            ActionReference::remote_for_test("actions/cache@a1b2c3d4e5f6").git_ref(),
            "a1b2c3d4e5f6"
        );
    }

    #[test]
    fn parse_rejects_empty_reference() {
        assert_eq!(
            ActionReference::parse("   "),
            Err(ActionError::InvalidReference("   ".into()))
        );
    }

    #[test]
    fn parse_rejects_reference_without_repository() {
        assert_eq!(
            ActionReference::parse("checkout@v4"),
            Err(ActionError::InvalidReference("checkout@v4".into()))
        );
    }

    #[test]
    fn parse_rejects_reference_with_empty_ref() {
        assert_eq!(
            ActionReference::parse("actions/cache@"),
            Err(ActionError::InvalidReference("actions/cache@".into()))
        );
    }

    #[test]
    fn parse_rejects_url_without_path() {
        assert_eq!(
            ActionReference::parse("https://data.forgejo.org@v4"),
            Err(ActionError::InvalidReference(
                "https://data.forgejo.org@v4".into()
            ))
        );
    }
}
