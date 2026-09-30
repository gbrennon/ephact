/// Classifies the network capabilities a shell script fragment would exercise.
///
/// The domain depends only on this contract so it stays agnostic of concrete
/// tooling. Implementations, which live in the infrastructure layer, own the
/// knowledge of specific commands and protocols.
pub trait NetworkCommandClassifier: Send + Sync {
    /// Returns whether the script accesses a package registry.
    fn accesses_package_registry(&self, script: &str) -> bool;

    /// Returns whether the script issues an outbound HTTP request.
    fn issues_http_request(&self, script: &str) -> bool;

    /// Returns whether the script uses a general network command.
    fn uses_network_command(&self, script: &str) -> bool;

    /// Returns whether the script mutates a remote environment.
    fn mutates_remote_environment(&self, script: &str) -> bool;
}
