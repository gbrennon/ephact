use ephact::infrastructure::containers::container_runtime_adapter::ContainerRuntimeAdapter;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_selects_an_available_runtime() {
        let adapter = ContainerRuntimeAdapter::detect().expect("a container runtime is available");
        let name = adapter.runtime_name();
        assert!(
            name == "Docker" || name == "Podman",
            "unexpected runtime name: {name}"
        );
    }
}
