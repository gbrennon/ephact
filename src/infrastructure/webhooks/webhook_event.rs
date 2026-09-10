use std::collections::HashMap;

use super::{
    create_payload::CreatePayload,
    delete_payload::DeletePayload,
    fork_payload::ForkPayload,
    issue_comment_payload::IssueCommentPayload,
    issues_payload::IssuesPayload,
    pull_request_payload::PullRequestPayload,
    push_payload::{PushMetadata, PushPayload},
    release_payload::ReleasePayload,
    repository_dispatch_payload::RepositoryDispatchPayload,
    webhook_event_payload::WebhookEventPayload,
    workflow_call_payload::WorkflowCallPayload,
    workflow_dispatch_payload::WorkflowDispatchPayload,
};

/// A GitHub Actions event that triggers a workflow.
///
/// Each variant corresponds to a webhook event that GitHub sends.
/// The event payload is serialized to JSON and made available
/// via the `github` context in expressions.
#[derive(Debug, Clone, PartialEq)]
pub enum WebhookEvent {
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

impl WebhookEventPayload for WebhookEvent {
    fn event_name(&self) -> &str {
        match self {
            WebhookEvent::Push(_) => "push",
            WebhookEvent::PullRequest(_) => "pull_request",
            WebhookEvent::WorkflowDispatch(_) => "workflow_dispatch",
            WebhookEvent::Schedule => "schedule",
            WebhookEvent::Release(_) => "release",
            WebhookEvent::Issues(_) => "issues",
            WebhookEvent::IssueComment(_) => "issue_comment",
            WebhookEvent::Create(_) => "create",
            WebhookEvent::Delete(_) => "delete",
            WebhookEvent::Fork(_) => "fork",
            WebhookEvent::Gollum => "gollum",
            WebhookEvent::PageBuild => "page_build",
            WebhookEvent::Public => "public",
            WebhookEvent::RepositoryDispatch(_) => "repository_dispatch",
            WebhookEvent::Status => "status",
            WebhookEvent::Watch => "watch",
            WebhookEvent::WorkflowCall(_) => "workflow_call",
            WebhookEvent::WorkflowRun => "workflow_run",
            WebhookEvent::Custom { name, .. } => name.as_str(),
        }
    }

    fn to_payload(&self) -> serde_json::Value {
        match self {
            WebhookEvent::Push(p) => serde_json::to_value(p).unwrap_or_default(),
            WebhookEvent::PullRequest(p) => serde_json::to_value(p).unwrap_or_default(),
            WebhookEvent::WorkflowDispatch(p) => serde_json::to_value(p).unwrap_or_default(),
            WebhookEvent::Schedule => serde_json::json!({}),
            WebhookEvent::Release(p) => serde_json::to_value(p).unwrap_or_default(),
            WebhookEvent::Issues(p) => serde_json::to_value(p).unwrap_or_default(),
            WebhookEvent::IssueComment(p) => serde_json::to_value(p).unwrap_or_default(),
            WebhookEvent::Create(p) => serde_json::to_value(p).unwrap_or_default(),
            WebhookEvent::Delete(p) => serde_json::to_value(p).unwrap_or_default(),
            WebhookEvent::Fork(p) => serde_json::to_value(p).unwrap_or_default(),
            WebhookEvent::Gollum => serde_json::json!({}),
            WebhookEvent::PageBuild => serde_json::json!({}),
            WebhookEvent::Public => serde_json::json!({}),
            WebhookEvent::RepositoryDispatch(p) => serde_json::to_value(p).unwrap_or_default(),
            WebhookEvent::Status => serde_json::json!({}),
            WebhookEvent::Watch => serde_json::json!({}),
            WebhookEvent::WorkflowCall(p) => serde_json::to_value(p).unwrap_or_default(),
            WebhookEvent::WorkflowRun => serde_json::json!({}),
            WebhookEvent::Custom { payload, .. } => payload.clone(),
        }
    }
}

impl WebhookEvent {
    /// Creates a push event with sensible defaults for local execution.
    pub fn push_default(branch: &str, repo: &super::repository_info::RepositoryInfo) -> Self {
        let act_user = super::user_info::UserInfo::new(
            "act".to_owned(),
            "act@localhost".to_owned(),
            "act".to_owned(),
        );
        WebhookEvent::Push(Box::new(PushPayload::new(
            format!("refs/heads/{}", branch),
            "0000000000000000000000000000000000000000".to_owned(),
            "0000000000000000000000000000000000000000".to_owned(),
            PushMetadata::new(
                repo.clone(),
                act_user.clone(),
                act_user,
                false,
                false,
                false,
            ),
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
            super::pull_request_info::PullRequestBranches::new(head, base),
            user.clone(),
            String::new(),
            super::pull_request_info::PullRequestState::new(false, false, None),
        );
        WebhookEvent::PullRequest(Box::new(PullRequestPayload::new(
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
        WebhookEvent::WorkflowDispatch(Box::new(WorkflowDispatchPayload::new(
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
    use super::super::{
        branch_ref::BranchRef,
        comment_info::CommentInfo,
        issue_info::IssueInfo,
        pull_request_info::{PullRequestBranches, PullRequestInfo, PullRequestState},
        release_info::ReleaseInfo,
    };
    use super::{
        super::{
            repository_info::{RepositoryInfo, RepositoryLinks},
            user_info::UserInfo,
        },
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
            RepositoryLinks::new(
                "https://github.com/owner/test-repo".to_owned(),
                "main".to_owned(),
                "https://github.com/owner/test-repo.git".to_owned(),
                "git@github.com:owner/test-repo.git".to_owned(),
            ),
        )
    }

    #[test]
    fn push_event_name() {
        let event = WebhookEvent::push_default("main", &test_repo());
        assert_eq!(event.event_name(), "push");
    }

    #[test]
    fn push_event_payload_is_valid_json() {
        let event = WebhookEvent::push_default("main", &test_repo());
        let payload = event.to_payload();
        let json_str = serde_json::to_string(&payload).unwrap();
        assert!(json_str.contains("refs/heads/main"));
        assert!(json_str.contains("test-repo"));
    }

    #[test]
    fn pull_request_event_name() {
        let event = WebhookEvent::pull_request_default(42, &test_repo());
        assert_eq!(event.event_name(), "pull_request");
    }

    #[test]
    fn workflow_dispatch_event_name() {
        let mut inputs = HashMap::new();
        inputs.insert("name".to_owned(), "world".to_owned());
        let event = WebhookEvent::workflow_dispatch(inputs, &test_repo());
        assert_eq!(event.event_name(), "workflow_dispatch");
    }

    #[test]
    fn schedule_event_has_empty_payload() {
        let event = WebhookEvent::Schedule;
        assert_eq!(event.event_name(), "schedule");
        assert_eq!(event.to_payload(), serde_json::json!({}));
    }

    #[test]
    fn custom_event() {
        let event = WebhookEvent::Custom {
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
            (WebhookEvent::Schedule, "schedule"),
            (WebhookEvent::Gollum, "gollum"),
            (WebhookEvent::PageBuild, "page_build"),
            (WebhookEvent::Public, "public"),
            (WebhookEvent::Status, "status"),
            (WebhookEvent::Watch, "watch"),
            (WebhookEvent::WorkflowRun, "workflow_run"),
        ];
        for (event, name) in events {
            assert_eq!(event.event_name(), name);
            assert_eq!(event.to_payload(), serde_json::json!({}));
        }
    }
    #[test]
    fn payload_variants_have_names_and_payloads() {
        let repo = test_repo();
        let user = UserInfo::new("name".into(), "email".into(), "login".into());
        let release = ReleaseInfo::new("v1".into(), None, None, false, false, "url".into());
        let events = [
            WebhookEvent::Release(Box::new(ReleasePayload::new(
                "published".into(),
                release,
                repo.clone(),
                user.clone(),
            ))),
            WebhookEvent::RepositoryDispatch(Box::new(RepositoryDispatchPayload::new(
                "custom".into(),
                serde_json::json!({}),
                repo.clone(),
                user.clone(),
            ))),
            WebhookEvent::WorkflowCall(Box::new(WorkflowCallPayload::new(
                HashMap::new(),
                HashMap::new(),
            ))),
        ];

        assert_eq!(events[0].event_name(), "release");
        assert_eq!(events[1].event_name(), "repository_dispatch");
        assert_eq!(events[2].event_name(), "workflow_call");
        assert!(events.iter().all(|event| event.to_payload().is_object()));
        let issue = IssueInfo::new(
            1,
            "title".into(),
            None,
            "open".into(),
            user.clone(),
            Vec::new(),
            "url".into(),
        );
        let comment = CommentInfo::new(2, "body".into(), user.clone(), "url".into());
        let branch = BranchRef::new("ref".into(), "sha".into(), repo.clone(), "main".into());
        let pull_request = PullRequestInfo::new(
            1,
            "title".into(),
            None,
            PullRequestBranches::new(branch.clone(), branch),
            user.clone(),
            "url".into(),
            PullRequestState::new(false, false, None),
        );
        let extra_events = [
            WebhookEvent::Issues(Box::new(IssuesPayload::new(
                "opened".into(),
                issue.clone(),
                repo.clone(),
                user.clone(),
            ))),
            WebhookEvent::IssueComment(Box::new(IssueCommentPayload::new(
                "created".into(),
                issue,
                comment,
                repo.clone(),
                user.clone(),
            ))),
            WebhookEvent::Create(Box::new(CreatePayload::new(
                "branch".into(),
                "main".into(),
                "main".into(),
                repo.clone(),
                user.clone(),
            ))),
            WebhookEvent::Delete(Box::new(DeletePayload::new(
                "branch".into(),
                "main".into(),
                repo.clone(),
                user.clone(),
            ))),
            WebhookEvent::Fork(Box::new(ForkPayload::new(
                repo.clone(),
                repo.clone(),
                user.clone(),
            ))),
            WebhookEvent::PullRequest(Box::new(PullRequestPayload::new(
                "opened".into(),
                1,
                pull_request,
                repo,
                user,
            ))),
        ];
        assert_eq!(extra_events[0].event_name(), "issues");
        assert_eq!(extra_events[1].event_name(), "issue_comment");
        assert_eq!(extra_events[2].event_name(), "create");
        assert_eq!(extra_events[3].event_name(), "delete");
        assert_eq!(extra_events[4].event_name(), "fork");
        assert_eq!(extra_events[5].event_name(), "pull_request");
    }
}
