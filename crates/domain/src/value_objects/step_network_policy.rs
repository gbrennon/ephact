use crate::traits::NetworkCommandClassifier;

/// Evaluates a step's script against the network policy.
///
/// The policy rule lives in the domain; the knowledge of which commands touch
/// the network is provided by an injected [`NetworkCommandClassifier`], keeping
/// the domain agnostic of concrete tooling.
pub struct StepNetworkPolicy {
    script: String,
}

impl StepNetworkPolicy {
    pub fn new(script: &str) -> Self {
        Self {
            script: script.to_ascii_lowercase(),
        }
    }

    pub fn network_access_reason(
        &self,
        classifier: &dyn NetworkCommandClassifier,
    ) -> Option<&'static str> {
        if classifier.accesses_package_registry(&self.script) {
            return None;
        }
        if classifier.issues_http_request(&self.script) {
            return Some("network access is disabled; the step would send an HTTP request");
        }
        if classifier.uses_network_command(&self.script) {
            return Some("network access is disabled; the step would use a network command");
        }
        None
    }

    pub fn network_policy_violation(
        &self,
        classifier: &dyn NetworkCommandClassifier,
    ) -> Option<&'static str> {
        classifier
            .mutates_remote_environment(&self.script)
            .then_some("network operation blocked; the step would modify a remote environment")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakeClassifier {
        package_registry: bool,
        http_request: bool,
        network_command: bool,
        remote_mutation: bool,
    }

    impl NetworkCommandClassifier for FakeClassifier {
        fn accesses_package_registry(&self, _script: &str) -> bool {
            self.package_registry
        }

        fn issues_http_request(&self, _script: &str) -> bool {
            self.http_request
        }

        fn uses_network_command(&self, _script: &str) -> bool {
            self.network_command
        }

        fn mutates_remote_environment(&self, _script: &str) -> bool {
            self.remote_mutation
        }
    }

    #[test]
    fn package_registry_access_is_always_allowed() {
        let policy = StepNetworkPolicy::new("Apt Install Curl");
        let classifier = FakeClassifier {
            package_registry: true,
            http_request: true,
            network_command: true,
            ..FakeClassifier::default()
        };

        let reason = policy.network_access_reason(&classifier);

        assert_eq!(reason, None);
    }

    #[test]
    fn http_request_without_package_access_is_reported() {
        let policy = StepNetworkPolicy::new("script");
        let classifier = FakeClassifier {
            http_request: true,
            network_command: true,
            ..FakeClassifier::default()
        };

        let reason = policy.network_access_reason(&classifier);

        assert_eq!(
            reason,
            Some("network access is disabled; the step would send an HTTP request")
        );
    }

    #[test]
    fn network_command_without_http_is_reported() {
        let policy = StepNetworkPolicy::new("script");
        let classifier = FakeClassifier {
            network_command: true,
            ..FakeClassifier::default()
        };

        let reason = policy.network_access_reason(&classifier);

        assert_eq!(
            reason,
            Some("network access is disabled; the step would use a network command")
        );
    }

    #[test]
    fn no_network_usage_reports_no_reason() {
        let policy = StepNetworkPolicy::new("script");
        let classifier = FakeClassifier::default();

        let reason = policy.network_access_reason(&classifier);

        assert_eq!(reason, None);
    }

    #[test]
    fn remote_mutation_is_a_policy_violation() {
        let policy = StepNetworkPolicy::new("Git Push");
        let classifier = FakeClassifier {
            remote_mutation: true,
            ..FakeClassifier::default()
        };

        let violation = policy.network_policy_violation(&classifier);

        assert_eq!(
            violation,
            Some("network operation blocked; the step would modify a remote environment")
        );
    }

    #[test]
    fn absence_of_remote_mutation_is_no_violation() {
        let policy = StepNetworkPolicy::new("script");
        let classifier = FakeClassifier::default();

        let violation = policy.network_policy_violation(&classifier);

        assert_eq!(violation, None);
    }

    #[test]
    fn new_lowercases_the_script() {
        let policy = StepNetworkPolicy::new("HELLO");

        assert_eq!(policy.script, "hello");
    }
}
