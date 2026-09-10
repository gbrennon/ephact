use serde::Deserialize;

use crate::domain::value_objects::TokenPermissions;

/// The `permissions:` entry of a workflow or job as authored in YAML.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct TokenPermissionsYaml {
    actions: Option<String>,
    checks: Option<String>,
    contents: Option<String>,
    deployments: Option<String>,
    issues: Option<String>,
    packages: Option<String>,
    pages: Option<String>,
    #[serde(rename = "pull-requests")]
    pull_requests: Option<String>,
    #[serde(rename = "repository-projects")]
    repository_projects: Option<String>,
    #[serde(rename = "security-events")]
    security_events: Option<String>,
    statuses: Option<String>,
}

impl TokenPermissionsYaml {
    /// Builds the domain token permissions this YAML describes.
    #[must_use]
    pub fn into_domain(self) -> TokenPermissions {
        TokenPermissions::new(
            self.actions,
            self.checks,
            self.contents,
            self.deployments,
            self.issues,
            self.packages,
            self.pages,
            self.pull_requests,
            self.repository_projects,
            self.security_events,
            self.statuses,
        )
    }
}
