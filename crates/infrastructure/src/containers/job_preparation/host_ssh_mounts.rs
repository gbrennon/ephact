use std::collections::HashMap;

/// Generic container bind and environment values for host SSH-agent forwarding.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct HostSshMounts {
    binds: Vec<String>,
    environment: HashMap<String, String>,
}

impl HostSshMounts {
    /// Creates host SSH-agent mount values.
    pub fn new(binds: Vec<String>, environment: HashMap<String, String>) -> Self {
        Self { binds, environment }
    }

    /// Consumes the mount values into runtime configuration parts.
    pub fn into_parts(self) -> (Vec<String>, HashMap<String, String>) {
        (self.binds, self.environment)
    }
}
