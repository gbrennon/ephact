use super::WorkflowListItemResponse;

/// Result of listing the workflow files of a repository.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ListWorkflowsResponse {
    /// Workflows found in the repository, in discovery order.
    workflows: Vec<WorkflowListItemResponse>,
}

impl ListWorkflowsResponse {
    /// Creates a response carrying the discovered workflows.
    pub fn new(workflows: Vec<WorkflowListItemResponse>) -> Self {
        Self { workflows }
    }

    /// Workflows found in the repository, in discovery order.
    pub fn workflows(&self) -> &[WorkflowListItemResponse] {
        &self.workflows
    }

    /// Consumes the response and returns the workflows.
    pub fn into_workflows(self) -> Vec<WorkflowListItemResponse> {
        self.workflows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_response_empty_by_default() {
        let response = ListWorkflowsResponse::new(vec![]);
        assert!(response.workflows().is_empty());
    }

    #[test]
    fn new_response_keeps_the_given_workflows() {
        let response = ListWorkflowsResponse::new(vec![WorkflowListItemResponse::new(
            None,
            Some("ci.yml".into()),
            vec![],
        )]);

        assert_eq!(
            response.workflows(),
            &[WorkflowListItemResponse::new(
                None,
                Some("ci.yml".into()),
                vec![]
            )]
        );
    }
}
