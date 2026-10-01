use crate::{
    errors::ActionError,
    value_objects::{
        RemoteActionReference, RemoteReferenceDefaults,
        remote_reference_parts::RemoteReferenceParts,
    },
};

/// A parsed `uses:` value, classified by how the action must be resolved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionReference {
    Local(String),
    Remote(RemoteActionReference),
    Docker(String),
}

impl ActionReference {
    pub fn parse(raw: &str, defaults: &RemoteReferenceDefaults) -> Result<Self, ActionError> {
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
        Self::parse_remote(reference, raw, defaults)
    }

    pub fn as_remote(&self) -> Option<&RemoteActionReference> {
        match self {
            Self::Remote(remote) => Some(remote),
            Self::Local(_) | Self::Docker(_) => None,
        }
    }

    fn parse_remote(
        reference: &str,
        raw: &str,
        defaults: &RemoteReferenceDefaults,
    ) -> Result<Self, ActionError> {
        RemoteReferenceParts::parse(reference, defaults)
            .map(|parts| Self::Remote(parts.into_remote()))
            .ok_or_else(|| ActionError::InvalidReference(raw.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn defaults() -> RemoteReferenceDefaults {
        RemoteReferenceDefaults::new("https".into(), "forge.example".into(), "main".into())
    }

    fn remote_for_test(raw: &str) -> RemoteActionReference {
        ActionReference::parse(raw, &defaults())
            .unwrap()
            .as_remote()
            .cloned()
            .expect("expected a remote action reference")
    }

    #[test]
    fn as_remote_is_none_for_local_and_docker_references() {
        assert!(
            ActionReference::parse("./action", &defaults())
                .unwrap()
                .as_remote()
                .is_none()
        );
        assert!(
            ActionReference::parse("docker://alpine:3.20", &defaults())
                .unwrap()
                .as_remote()
                .is_none()
        );
    }

    #[test]
    fn parse_relative_path_is_local() {
        assert_eq!(
            ActionReference::parse("./.ci/actions/publish", &defaults()).unwrap(),
            ActionReference::Local("./.ci/actions/publish".into())
        );
    }

    #[test]
    fn parse_parent_relative_path_is_local() {
        assert_eq!(
            ActionReference::parse("../shared/action", &defaults()).unwrap(),
            ActionReference::Local("../shared/action".into())
        );
    }

    #[test]
    fn parse_docker_image_is_docker() {
        assert_eq!(
            ActionReference::parse("docker://alpine:3.20", &defaults()).unwrap(),
            ActionReference::Docker("alpine:3.20".into())
        );
    }

    #[test]
    fn parse_shorthand_keeps_revision_and_directory() {
        let reference = remote_for_test("actions/checkout@v4");

        assert_eq!(reference.revision(), "v4");
        assert_eq!(reference.directory(), None);
    }

    #[test]
    fn parse_shorthand_without_ref_defaults_to_main() {
        assert_eq!(remote_for_test("actions/checkout").revision(), "main");
    }

    #[test]
    fn parse_full_url_keeps_host_and_revision() {
        let reference = remote_for_test("https://forge.example/actions/cache@v4");

        assert_eq!(reference.host(), "forge.example");
        assert_eq!(reference.revision(), "v4");
    }

    #[test]
    fn parse_keeps_action_subdirectory() {
        let reference = remote_for_test("https://forge.example/group/tools/deploy/action@main");

        assert_eq!(reference.directory(), Some("deploy/action"));
    }

    #[test]
    fn parse_commit_revision() {
        assert_eq!(
            remote_for_test("actions/cache@a1b2c3d4e5f6").revision(),
            "a1b2c3d4e5f6"
        );
    }

    #[test]
    fn parse_rejects_empty_reference() {
        assert_eq!(
            ActionReference::parse("   ", &defaults()),
            Err(ActionError::InvalidReference("   ".into()))
        );
    }

    #[test]
    fn parse_rejects_reference_without_repository() {
        assert_eq!(
            ActionReference::parse("checkout@v4", &defaults()),
            Err(ActionError::InvalidReference("checkout@v4".into()))
        );
    }

    #[test]
    fn parse_rejects_reference_with_empty_revision() {
        assert_eq!(
            ActionReference::parse("actions/cache@", &defaults()),
            Err(ActionError::InvalidReference("actions/cache@".into()))
        );
    }

    #[test]
    fn parse_rejects_url_without_path() {
        assert_eq!(
            ActionReference::parse("https://forge.example@v4", &defaults()),
            Err(ActionError::InvalidReference(
                "https://forge.example@v4".into()
            ))
        );
    }
}
