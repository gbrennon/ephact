#[cfg(test)]
mod tests {
    use std::{
        fs,
        os::unix::net::UnixListener,
        path::PathBuf,
        process,
        sync::atomic::{AtomicUsize, Ordering},
    };

    use ephact::infrastructure::containers::HostSshForwardingConfig;

    static NEXT_FIXTURE_ID: AtomicUsize = AtomicUsize::new(0);

    struct SocketFixture {
        path: PathBuf,
        listener: Option<UnixListener>,
    }

    impl SocketFixture {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "ephact-ssh-agent-{}-{}",
                process::id(),
                NEXT_FIXTURE_ID.fetch_add(1, Ordering::Relaxed)
            ));
            let _ = fs::remove_file(&path);
            let listener = UnixListener::bind(&path).unwrap();
            Self {
                path,
                listener: Some(listener),
            }
        }

        fn path(&self) -> PathBuf {
            self.path.clone()
        }
    }

    impl Drop for SocketFixture {
        fn drop(&mut self) {
            self.listener.take();
            let _ = fs::remove_file(&self.path);
        }
    }

    #[test]
    fn enabled_configuration_accepts_a_unix_socket() {
        let fixture = SocketFixture::new();

        let config = HostSshForwardingConfig::enabled_with_socket(fixture.path()).unwrap();

        assert!(config.is_enabled());
    }

    #[test]
    fn enabled_configuration_rejects_a_regular_file() {
        let path = std::env::temp_dir().join(format!("ephact-ssh-agent-file-{}", process::id()));
        fs::write(&path, "not a socket").unwrap();

        let result = HostSshForwardingConfig::enabled_with_socket(path.clone());

        fs::remove_file(path).unwrap();
        assert!(result.is_err());
    }

    #[test]
    fn enabled_configuration_rejects_a_directory() {
        let path =
            std::env::temp_dir().join(format!("ephact-ssh-agent-directory-{}", process::id()));
        fs::create_dir(&path).unwrap();

        let result = HostSshForwardingConfig::enabled_with_socket(path.clone());

        fs::remove_dir(path).unwrap();
        assert!(result.is_err());
    }
}
