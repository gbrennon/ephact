use ephact_domain::value_objects::StepNetworkPolicy;

#[test]
fn apt_install_allows_package_manager_network_access() {
    assert_eq!(
        StepNetworkPolicy::new("apt-get install curl").network_access_reason(),
        None
    );
}

#[test]
fn npm_install_allows_package_manager_network_access() {
    assert_eq!(
        StepNetworkPolicy::new("npm install requests").network_access_reason(),
        None
    );
}

#[test]
fn pip_install_allows_package_manager_network_access() {
    assert_eq!(
        StepNetworkPolicy::new("pip install requests").network_access_reason(),
        None
    );
}

#[test]
fn cargo_fetch_allows_package_manager_network_access() {
    assert_eq!(
        StepNetworkPolicy::new("cargo fetch").network_access_reason(),
        None
    );
}

#[test]
fn blocks_http_requests() {
    assert_eq!(
        StepNetworkPolicy::new("curl https://example.com").network_access_reason(),
        Some("network access is disabled; the step would send an HTTP request")
    );
}

#[test]
fn blocks_network_commands() {
    assert_eq!(
        StepNetworkPolicy::new("git clone example/repo").network_access_reason(),
        Some("network access is disabled; the step would use a network command")
    );
}

#[test]
fn blocks_remote_mutations() {
    assert_eq!(
        StepNetworkPolicy::new("git push origin main").network_policy_violation(),
        Some("network operation blocked; the step would modify a remote environment")
    );
}
