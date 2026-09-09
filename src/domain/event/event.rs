use std::collections::HashMap;

use super::{
    create_payload::CreatePayload, delete_payload::DeletePayload, event_payload::EventPayload,
    fork_payload::ForkPayload, issue_comment_payload::IssueCommentPayload,
    issues_payload::IssuesPayload, pull_request_payload::PullRequestPayload,
    push_payload::PushPayload, release_payload::ReleasePayload,
    repository_dispatch_payload::RepositoryDispatchPayload,
    workflow_call_payload::WorkflowCallPayload, workflow_dispatch_payload::WorkflowDispatchPayload,
};

/// A GitHub Actions event that triggers a workflow.
///
/// Each variant corresponds to a webhook event that GitHub sends.
/// The event payload is serialized to JSON and made available
/// via the `github` context in expressions.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Push(Box<PushPayload>),
    PullRequest(Box<PullRequestPayload>),
    WorkflowDispatch(Box<WorkflowDispatchPayload>),
    Schedule,
    Release(Box<ReleasePayload>),
    Issues(Box<IssuesPayload>),
    IssueComment(Box<IssueCommentPayload>),
    Create(Box<CreatePayload>),
    Delete(Box<DeletePayload>),
    Fork(Box<ForkPayload>),
    Gollum,
    PageBuild,
    Public,
    RepositoryDispatch(Box<RepositoryDispatchPayload>),
    Status,
    Watch,
    WorkflowCall(Box<WorkflowCallPayload>),
    WorkflowRun,
    Custom {
        name: String,
        payload: serde_json::Value,
    },
}

impl EventPayload for Event {
    fn event_name(&self) -> &str {
        match self {
            Event::Push(_) => "push",
            Event::PullRequest(_) => "pull_request",
            Event::WorkflowDispatch(_) => "workflow_dispatch",
            Event::Schedule => "schedule",
            Event::Release(_) => "release",
            Event::Issues(_) => "issues",
            Event::IssueComment(_) => "issue_comment",
            Event::Create(_) => "create",
            Event::Delete(_) => "delete",
            Event::Fork(_) => "fork",
            Event::Gollum => "gollum",
            Event::PageBuild => "page_build",
            Event::Public => "public",
            Event::RepositoryDispatch(_) => "repository_dispatch",
            Event::Status => "status",
            Event::Watch => "watch",
            Event::WorkflowCall(_) => "workflow_call",
            Event::WorkflowRun => "workflow_run",
            Event::Custom { name, .. } => name.as_str(),
        }
    }

    fn to_payload(&self) -> serde_json::Value {
        match self {
            Event::Push(p) => serde_json::to_value(p).unwrap_or_default(),
            Event::PullRequest(p) => serde_json::to_value(p).unwrap_or_default(),
            Event::WorkflowDispatch(p) => serde_json::to_value(p).unwrap_or_default(),
            Event::Schedule => serde_json::json!({}),
            Event::Release(p) => serde_json::to_value(p).unwrap_or_default(),
            Event::Issues(p) => serde_json::to_value(p).unwrap_or_default(),
            Event::IssueComment(p) => serde_json::to_value(p).unwrap_or_default(),
            Event::Create(p) => serde_json::to_value(p).unwrap_or_default(),
            Event::Delete(p) => serde_json::to_value(p).unwrap_or_default(),
            Event::Fork(p) => serde_json::to_value(p).unwrap_or_default(),
            Event::Gollum => serde_json::json!({}),
            Event::PageBuild => serde_json::json!({}),
            Event::Public => serde_json::json!({}),
            Event::RepositoryDispatch(p) => serde_json::to_value(p).unwrap_or_default(),
            Event::Status => serde_json::json!({}),
            Event::Watch => serde_json::json!({}),
            Event::WorkflowCall(p) => serde_json::to_value(p).unwrap_or_default(),
            Event::WorkflowRun => serde_json::json!({}),
            Event::Custom { payload, .. } => payload.clone(),
        }
    }
}

impl Event {
    /// Creates a push event with sensible defaults for local execution.
    pub fn push_default(branch: &str, repo: &super::repository_info::RepositoryInfo) -> Self {
        let act_user = super::user_info::UserInfo::new(
            "act".to_owned(),
            "act@localhost".to_owned(),
            "act".to_owned(),
        );
        Event::Push(Box::new(PushPayload::new(
            format!("refs/heads/{}", branch),
            "0000000000000000000000000000000000000000".to_owned(),
            "0000000000000000000000000000000000000000".to_owned(),
            repo.clone(),
            act_user.clone(),
            act_user,
            false,
            false,
            false,
            vec![],
            None,
            String::new(),
        )))
    }

    /// Creates a pull_request event with sensible defaults for local execution.
    pub fn pull_request_default(
        number: u64,
        repo: &super::repository_info::RepositoryInfo,
    ) -> Self {
        let head = super::branch_ref::BranchRef::new(
            "refs/heads/feature".to_owned(),
            "0000000000000000000000000000000000000000".to_owned(),
            repo.clone(),
            "feature".to_owned(),
        );
        let base = super::branch_ref::BranchRef::new(
            "refs/heads/main".to_owned(),
            "0000000000000000000000000000000000000000".to_owned(),
            repo.clone(),
            "main".to_owned(),
        );
        let user = super::user_info::UserInfo::new(
            "act".to_owned(),
            "act@localhost".to_owned(),
            "act".to_owned(),
        );
        let pull_request = super::pull_request_info::PullRequestInfo::new(
            number,
            "Local PR".to_owned(),
            None,
            head,
            base,
            user.clone(),
            String::new(),
            false,
            false,
            None,
        );
        Event::PullRequest(Box::new(PullRequestPayload::new(
            "opened".to_owned(),
            number,
            pull_request,
            repo.clone(),
            user,
        )))
    }

    /// Creates a workflow_dispatch event with the given inputs.
    pub fn workflow_dispatch(
        inputs: HashMap<String, String>,
        repo: &super::repository_info::RepositoryInfo,
    ) -> Self {
        let sender = super::user_info::UserInfo::new(
            "act".to_owned(),
            "act@localhost".to_owned(),
            "act".to_owned(),
        );
        Event::WorkflowDispatch(Box::new(WorkflowDispatchPayload::new(
            inputs,
            repo.clone(),
            sender,
            String::new(),
            "refs/heads/main".to_owned(),
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{repository_info::RepositoryInfo, user_info::UserInfo},
        *,
    };

    fn test_repo() -> RepositoryInfo {
        let owner = UserInfo::new(
            "owner".to_owned(),
            "owner@example.com".to_owned(),
            "owner".to_owned(),
        );
        RepositoryInfo::new(
            "test-repo".to_owned(),
            "owner/test-repo".to_owned(),
            owner,
            false,
            "https://github.com/owner/test-repo".to_owned(),
            "main".to_owned(),
            "https://github.com/owner/test-repo.git".to_owned(),
            "git@github.com:owner/test-repo.git".to_owned(),
        )
    }

    #[test]
    fn push_event_name() {
        let event = Event::push_default("main", &test_repo());
        assert_eq!(event.event_name(), "push");
    }

    #[test]
    fn push_event_payload_is_valid_json() {
        let event = Event::push_default("main", &test_repo());
        let payload = event.to_payload();
        let json_str = serde_json::to_string(&payload).unwrap();
        assert!(json_str.contains("refs/heads/main"));
        assert!(json_str.contains("test-repo"));
    }

    #[test]
    fn pull_request_event_name() {
        let event = Event::pull_request_default(42, &test_repo());
        assert_eq!(event.event_name(), "pull_request");
    }

    #[test]
    fn workflow_dispatch_event_name() {
        let mut inputs = HashMap::new();
        inputs.insert("name".to_owned(), "world".to_owned());
        let event = Event::workflow_dispatch(inputs, &test_repo());
        assert_eq!(event.event_name(), "workflow_dispatch");
    }

    #[test]
    fn schedule_event_has_empty_payload() {
        let event = Event::Schedule;
        assert_eq!(event.event_name(), "schedule");
        assert_eq!(event.to_payload(), serde_json::json!({}));
    }

    #[test]
    fn custom_event() {
        let event = Event::Custom {
            name: "deployment".to_owned(),
            payload: serde_json::json!({"environment": "production"}),
        };
        assert_eq!(event.event_name(), "deployment");
        assert_eq!(
            event.to_payload(),
            serde_json::json!({"environment": "production"})
        );
    }

    #[test]
    fn unit_variant_event_names_and_payloads() {
        let events = [
            (Event::Schedule, "schedule"),
            (Event::Gollum, "gollum"),
            (Event::PageBuild, "page_build"),
            (Event::Public, "public"),
            (Event::Status, "status"),
            (Event::Watch, "watch"),
            (Event::WorkflowRun, "workflow_run"),
        ];
        for (event, name) in events {
            assert_eq!(event.event_name(), name);
            assert_eq!(event.to_payload(), serde_json::json!({}));
        }
    }
}
