/// Typed flags needed before the application container is constructed.
pub struct RunInvocationFlags {
    verbose: bool,
    network_allowed: bool,
    ssh_forwarding_requested: bool,
}

impl RunInvocationFlags {
    /// Creates the preflight flags from parsed run arguments.
    pub fn new(verbose: bool, network_allowed: bool, ssh_forwarding_requested: bool) -> Self {
        Self {
            verbose,
            network_allowed,
            ssh_forwarding_requested,
        }
    }

    /// Reports whether verbose output was requested.
    pub fn is_verbose_requested(&self) -> bool {
        self.verbose
    }

    /// Reports whether network access was requested.
    pub fn is_network_allowed(&self) -> bool {
        self.network_allowed
    }

    /// Reports whether SSH agent forwarding was requested.
    pub fn is_ssh_forwarding_requested(&self) -> bool {
        self.ssh_forwarding_requested
    }
}
