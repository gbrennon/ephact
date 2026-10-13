#[cfg(test)]
mod tests {
    use ephact::{
        application::dtos::responses::WorkflowListItemResponse,
        presentation::{cli::parse_list_workflows_test_args, handlers::ListWorkflowsHandler},
    };

    use crate::common::fakes::fake_list_workflows_port::FakeListWorkflowsPort;

    #[test]
    fn cli_path_renders_workflow_names_from_parsed_args() {
        let args = parse_list_workflows_test_args(&[]);
        let workflows = vec![
            WorkflowListItemResponse::new(Some("CI".into()), Some("ci.yml".into()), vec![]),
            WorkflowListItemResponse::new(
                Some("Release".into()),
                Some("release.yml".into()),
                vec![],
            ),
        ];
        let port = FakeListWorkflowsPort::with_workflows(workflows);

        let response = ListWorkflowsHandler::handle(&port, args.path().to_path_buf())
            .expect("current repository should be valid");
        let content = ListWorkflowsHandler::render(&response);

        assert_eq!(content, "CI\nRelease");
    }

    #[test]
    fn cli_path_propagates_port_failure() {
        let args = parse_list_workflows_test_args(&[]);
        let port = FakeListWorkflowsPort::failing("workflow source failure");

        let result = ListWorkflowsHandler::handle(&port, args.path().to_path_buf());

        assert!(result.is_err());
    }
}
