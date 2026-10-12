/// Information about the host container runtime.
#[derive(Debug, Clone)]
pub struct HostInfoResponse {
    os: String,
    arch: String,
    engine_version: String,
}

impl HostInfoResponse {
    pub fn new(os: String, arch: String, engine_version: String) -> Self {
        Self {
            os,
            arch,
            engine_version,
        }
    }

    pub fn os(&self) -> &str {
        &self.os
    }

    pub fn arch(&self) -> &str {
        &self.arch
    }

    pub fn engine_version(&self) -> &str {
        &self.engine_version
    }

    pub fn into_parts(self) -> (String, String, String) {
        (self.os, self.arch, self.engine_version)
    }
}
