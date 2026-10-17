#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use ephact::presentation::cli::{CliParser, parse_run_test_args};

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
    fn forward_ssh_is_detected_from_parsed_run_arguments() {
        let arguments = vec![
            OsString::from("ephact"),
            OsString::from("run"),
            OsString::from("--forward-ssh"),
        ];
        let parser = CliParser::try_parse_invocation_from(&arguments).unwrap();

        assert!(parser.is_ssh_forwarding_requested());
    }

    #[test]
    fn forward_ssh_is_rejected_outside_run_arguments() {
        let arguments = vec![OsString::from("ephact"), OsString::from("tui")];
        let parser = CliParser::try_parse_invocation_from(&arguments).unwrap();

        assert!(!parser.is_ssh_forwarding_requested());
    }

    #[test]
    fn forward_ssh_after_argument_terminator_is_not_parsed_as_a_flag() {
        let arguments = vec![
            OsString::from("ephact"),
            OsString::from("run"),
            OsString::from("--"),
            OsString::from("--forward-ssh"),
        ];

        let parser = CliParser::try_parse_invocation_from(&arguments).unwrap();

        assert!(!parser.is_ssh_forwarding_requested());
    }
}
