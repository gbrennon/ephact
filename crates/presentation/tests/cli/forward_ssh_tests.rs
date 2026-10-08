#[cfg(test)]
mod tests {
    use std::ffi::{OsStr, OsString};

    use ephact::presentation::cli::{RunArgs, parse_run_test_args};

    #[test]
    fn forward_ssh_requires_network_access() {
        let args = parse_run_test_args(&["--forward-ssh"]);

        let result = args.to_domain();
        let error = match result {
            Ok(_) => panic!("forwarding without network access must fail"),
            Err(error) => error,
        };

        assert_eq!(error.to_string(), "--forward-ssh requires --allow-network");
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

    #[test]
    fn forward_ssh_command_is_limited_to_run_options() {
        let run_args = vec![
            OsString::from("ephact"),
            OsString::from("run"),
            OsString::from("--forward-ssh"),
        ];
        let cli_run_args = vec![
            OsString::from("ephact"),
            OsString::from("cli"),
            OsString::from("run"),
            OsString::from("--forward-ssh"),
        ];
        let tui_args = vec![
            OsString::from("ephact"),
            OsString::from("tui"),
            OsString::from("--forward-ssh"),
        ];
        let escaped_args = vec![
            OsString::from("ephact"),
            OsString::from("run"),
            OsString::from("--"),
            OsString::from("--forward-ssh"),
        ];

        assert!(RunArgs::is_forward_ssh_command(&run_args));
        assert!(RunArgs::is_forward_ssh_command(&cli_run_args));
        assert!(!RunArgs::is_forward_ssh_command(&tui_args));
        assert!(!RunArgs::is_forward_ssh_command(&escaped_args));
    }
}
