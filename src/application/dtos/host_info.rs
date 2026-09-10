/// Information about the host container runtime.
#[derive(Debug, Clone)]
pub struct HostInfo {
    os: String,
    arch: String,
    engine_version: String,
}

impl HostInfo {
    pub fn new(
        os: impl Into<String>,
        arch: impl Into<String>,
        engine_version: impl Into<String>,
    ) -> Self {
        Self {
            os: os.into(),
            arch: arch.into(),
            engine_version: engine_version.into(),
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
