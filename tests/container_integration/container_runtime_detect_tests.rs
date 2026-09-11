#[cfg(test)]
mod tests {
    use ephact::infrastructure::containers::container_runtime_adapter::ContainerRuntimeAdapter;

    #[test]
    fn detect_selects_an_available_runtime() {
        let adapter = match ContainerRuntimeAdapter::detect() {
            Ok(adapter) => adapter,
            Err(e) => {
                eprintln!("SKIP: No container runtime available: {e:?}");
                return;
            }
        };
        let name = adapter.runtime_name();
        assert!(
            name == "Docker" || name == "Podman",
            "unexpected runtime name: {name}"
        );
    }
}
