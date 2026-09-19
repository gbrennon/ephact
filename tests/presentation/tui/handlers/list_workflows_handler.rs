#[cfg(test)]
mod tests {
    use std::env;

    use ephact::application::dtos::responses::WorkflowListItemResponse;
    use ephact::presentation::tui::ListWorkflowsHandler;

    use crate::common::fakes::fake_list_workflows_port::FakeListWorkflowsPort;

    #[test]
    fn handler_returns_workflows_from_port() {
        let workflows = vec![WorkflowListItemResponse::new(
            Some("CI".into()),
            Some("ci.yml".into()),
            vec!["push".into()],
        )];
        let port = FakeListWorkflowsPort::with_workflows(workflows.clone());

        let response =
            ListWorkflowsHandler::handle(&port, env::current_dir().expect("current directory"))
                .expect("repository should be valid");

        assert_eq!(response.workflows(), workflows);
    }
}
