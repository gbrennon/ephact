#[cfg(test)]
mod tests {
    use ephact::presentation::{cli::parse_list_actions_test_args, handlers::ListActionsHandler};

    use crate::common::fakes::fake_list_actions_port::FakeListActionsPort;

    #[test]
    fn cli_path_renders_action_names_from_parsed_args() {
        let args = parse_list_actions_test_args(&[]);
        let port = FakeListActionsPort::with_actions(vec![
            "actions/checkout@v4".to_string(),
            "./.forgejo/actions/build".to_string(),
        ]);

        let response = ListActionsHandler::handle(&port, args.path().to_path_buf())
            .expect("current repository should be valid");
        let content = ListActionsHandler::render(&response);

        assert_eq!(content, "checkout@v4\nbuild");
    }

    #[test]
    fn cli_path_propagates_port_failure() {
        let args = parse_list_actions_test_args(&[]);
        let port = FakeListActionsPort::failing("workflow source failure");

        let result = ListActionsHandler::handle(&port, args.path().to_path_buf());

        assert!(result.is_err());
    }
}
