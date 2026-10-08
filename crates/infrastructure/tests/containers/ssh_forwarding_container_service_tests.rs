#[cfg(test)]
mod tests {
    use std::{
        os::unix::net::UnixListener,
        path::Path,
        process,
        sync::atomic::{AtomicUsize, Ordering},
    };

    use ephact::{
        application::{
            dtos::requests::CreateJobContainerRequest, ports::outbound::CreateJobContainerPort,
        },
        infrastructure::containers::{CreateJobContainerService, HostSshForwardingConfig},
    };

    use crate::common::fakes::fake_runtime::FakeRuntime;

    static NEXT_SOCKET_ID: AtomicUsize = AtomicUsize::new(0);

    struct SocketFixture {
        path: std::path::PathBuf,
        listener: UnixListener,
    }

    impl SocketFixture {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "ephact-ssh-forwarding-{}-{}",
                process::id(),
                NEXT_SOCKET_ID.fetch_add(1, Ordering::Relaxed)
            ));
            let _ = std::fs::remove_file(&path);
            let listener = UnixListener::bind(&path).unwrap();
            Self { path, listener }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for SocketFixture {
        fn drop(&mut self) {
            let _ = &self.listener;
            let _ = std::fs::remove_file(&self.path);
        }
    }

    #[test]
    fn create_maps_the_agent_socket_to_container_environment() {
        let socket = SocketFixture::new();
        let config =
            HostSshForwardingConfig::enabled_with_socket(socket.path().to_path_buf()).unwrap();
        let runtime = std::sync::Arc::new(FakeRuntime::new());
        let service = CreateJobContainerService::with_ssh_forwarding(runtime.clone(), config);
        let request = CreateJobContainerRequest::new(
            "ubuntu:latest".to_string(),
            "ephact-build-42".to_string(),
            "ephact-build".to_string(),
            "/repo".into(),
            false,
        );

        service.create(request).unwrap();

        let created = runtime.created_containers.lock();
        let container = created.first().unwrap();
        assert_eq!(
            container.binds(),
            vec![format!(
                "{}:/tmp/ephact-ssh-agent.sock",
                socket.path().display()
            )]
        );
        assert_eq!(
            container.env().get("SSH_AUTH_SOCK"),
            Some(&"/tmp/ephact-ssh-agent.sock".to_string())
        );
    }
}
