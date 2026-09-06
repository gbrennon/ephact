use std::collections::HashMap;

use ephact::application::dtos::ContainerConfig;
use ephact::application::ports::outbound::ContainerRuntimePort;
use ephact::infrastructure::containers::container_runtime_adapter::ContainerRuntimeAdapter;

use crate::common::fakes::{
    spy_container_runtime::SpyContainerRuntime,
    stub_docker_erroring_runtime::StubDockerErroringRuntime,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn make_config(name: &str) -> ContainerConfig {
        ContainerConfig {
            image: "alpine:latest".into(),
            platform: None,
            env: HashMap::new(),
            binds: vec![],
            workdir: None,
            cmd: Some(vec!["sleep".into(), "infinity".into()]),
            entrypoint: None,
            network: None,
            name: Some(name.into()),
            runner_context: Default::default(),
        }
    }

    #[test]
    fn pull_image_delegates_to_the_injected_runtime() {
        let spy = SpyContainerRuntime::new();
        let adapter = ContainerRuntimeAdapter::new(spy.clone(), "Docker");

        adapter.pull_image("alpine:latest", None).unwrap();

        assert_eq!(spy.pulled_images(), vec!["alpine:latest"]);
    }

    #[test]
    fn create_container_delegates_to_the_injected_runtime() {
        let spy = SpyContainerRuntime::new();
        let adapter = ContainerRuntimeAdapter::new(spy.clone(), "Docker");

        adapter.create_container(&make_config("job-1")).unwrap();

        assert_eq!(spy.created_containers(), vec!["job-1"]);
    }

    #[test]
    fn stop_container_delegates_to_the_injected_runtime() {
        let spy = SpyContainerRuntime::new();
        let adapter = ContainerRuntimeAdapter::new(spy.clone(), "Docker");

        adapter.stop_container("job-1").unwrap();

        assert_eq!(spy.stopped_containers(), vec!["job-1"]);
    }

    #[test]
    fn kill_container_delegates_to_the_injected_runtime() {
        let spy = SpyContainerRuntime::new();
        let adapter = ContainerRuntimeAdapter::new(spy.clone(), "Docker");

        adapter.kill_container("job-1").unwrap();

        assert_eq!(spy.killed_containers(), vec!["job-1"]);
    }

    #[test]
    fn remove_container_delegates_to_the_injected_runtime() {
        let spy = SpyContainerRuntime::new();
        let adapter = ContainerRuntimeAdapter::new(spy.clone(), "Docker");

        adapter.remove_container("job-1").unwrap();

        assert_eq!(spy.removed_containers(), vec!["job-1"]);
    }

    #[test]
    fn get_host_info_delegates_to_the_injected_runtime() {
        let spy = SpyContainerRuntime::new();
        let adapter = ContainerRuntimeAdapter::new(spy, "Docker");

        let info = adapter.get_host_info().unwrap();

        assert_eq!(info.os, "linux");
        assert_eq!(info.arch, "amd64");
    }

    #[test]
    fn docker_named_adapter_leaves_the_error_untouched() {
        let adapter = ContainerRuntimeAdapter::new(StubDockerErroringRuntime, "Docker");

        let err = adapter.pull_image("alpine:latest", None).unwrap_err();

        let text = format!("{err:?}");
        assert!(
            text.contains("Docker"),
            "Docker adapter should preserve 'Docker' in error: {text}"
        );
    }

    #[test]
    fn podman_named_adapter_rewrites_docker_to_podman_in_errors() {
        let adapter = ContainerRuntimeAdapter::new(StubDockerErroringRuntime, "Podman");

        let err = adapter.pull_image("alpine:latest", None).unwrap_err();

        let text = format!("{err:?}");
        assert!(
            !text.contains("Docker"),
            "Podman adapter should not contain 'Docker' in error: {text}"
        );
        assert!(
            text.contains("Podman"),
            "Podman adapter should contain 'Podman' in error: {text}"
        );
    }

    #[test]
    fn podman_named_adapter_rewrites_errors_for_every_operation() {
        let adapter = ContainerRuntimeAdapter::new(StubDockerErroringRuntime, "Podman");

        let create_err = match adapter.create_container(&make_config("x")) {
            Ok(_) => panic!("expected create_container to fail"),
            Err(e) => e,
        };
        let errors = [
            format!("{create_err:?}"),
            format!("{:?}", adapter.remove_container("x").unwrap_err()),
            format!("{:?}", adapter.stop_container("x").unwrap_err()),
            format!("{:?}", adapter.kill_container("x").unwrap_err()),
        ];

        for text in errors {
            assert!(
                !text.contains("Docker") && text.contains("Podman"),
                "expected 'Docker' rewritten to 'Podman': {text}"
            );
        }
    }

    #[test]
    fn runtime_name_reports_the_injected_label() {
        let adapter = ContainerRuntimeAdapter::new(SpyContainerRuntime::new(), "Podman");

        assert_eq!(adapter.runtime_name(), "Podman");
    }
}
