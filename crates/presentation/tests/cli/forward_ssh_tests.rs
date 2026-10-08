#[cfg(test)]
mod tests {
    use std::ffi::OsStr;

    use ephact::presentation::cli::{RunArgs, parse_run_test_args};

    #[test]
    fn forward_ssh_requires_network_access() {
        let args = parse_run_test_args(&["--forward-ssh"]);

        let result = args.to_domain();

        assert!(result.is_err());
    }

    #[test]
    fn forward_ssh_is_accepted_with_network_access() {
        let args = parse_run_test_args(&["--forward-ssh", "--allow-network"]);

        let result = args.to_domain();

        assert!(result.is_ok());
    }

    #[test]
    fn forward_ssh_predicate_matches_only_the_exact_flag() {
        assert!(RunArgs::is_forward_ssh_flag(OsStr::new("--forward-ssh")));
        assert!(!RunArgs::is_forward_ssh_flag(OsStr::new(
            "--forward-ssh=true"
        )));
        assert!(!RunArgs::is_forward_ssh_flag(OsStr::new("--allow-network")));
    }
}
