#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;
use std::{env, fs, path::PathBuf};

/// Controls optional host SSH-agent forwarding for infrastructure container creation.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HostSshForwardingConfig {
    enabled: bool,
    socket_override: Option<PathBuf>,
}

impl HostSshForwardingConfig {
    /// Creates a configuration that does not forward host credentials.
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            socket_override: None,
        }
    }

    /// Creates a configuration that forwards the host SSH agent when available.
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            socket_override: None,
        }
    }

    /// Creates a configuration using an explicitly supplied host agent socket.
    pub fn enabled_with_socket(socket: PathBuf) -> Result<Self, String> {
        let config = Self {
            enabled: true,
            socket_override: Some(socket),
        };
        config.socket_path().map(|_| config)
    }

    /// Returns whether host SSH-agent forwarding is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Resolves and validates the host SSH-agent socket path when forwarding is enabled.
    pub fn socket_path(&self) -> Result<Option<PathBuf>, String> {
        if !self.enabled {
            return Ok(None);
        }
        let socket = self
            .socket_override
            .clone()
            .or_else(|| env::var_os("SSH_AUTH_SOCK").map(PathBuf::from))
            .ok_or_else(|| "SSH_AUTH_SOCK is not set".to_string())?;
        let metadata = fs::metadata(&socket)
            .map_err(|error| format!("cannot inspect SSH agent socket: {error}"))?;
        if !Self::is_socket(&metadata) {
            return Err(format!(
                "SSH_AUTH_SOCK is not a Unix socket: {}",
                socket.display()
            ));
        }
        Ok(Some(socket))
    }

    #[cfg(unix)]
    fn is_socket(metadata: &fs::Metadata) -> bool {
        metadata.file_type().is_socket()
    }

    #[cfg(not(unix))]
    fn is_socket(_metadata: &fs::Metadata) -> bool {
        false
    }
}
