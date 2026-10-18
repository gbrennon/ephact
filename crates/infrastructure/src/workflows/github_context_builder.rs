use std::{collections::BTreeMap, sync::Arc};

use crate::{
    application::ports::outbound::WorkflowSourcePort,
    containers::workspace::CONTAINER_WORKSPACE,
    domain::{
        messages::commands::ExecuteWorkflowPayload,
        value_objects::{ContextValue, EvaluationContext},
    },
    workflows::{FilesystemWorkflowSource, GithubContext},
};

/// Builds the GitHub-compatible expression context for one workflow run.
pub struct GithubContextBuilder {
    workflow_source: Arc<dyn WorkflowSourcePort>,
}

impl GithubContextBuilder {
    /// Creates a builder backed by the supplied filesystem source port.
    pub fn new(workflow_source: Arc<dyn WorkflowSourcePort>) -> Self {
        Self { workflow_source }
    }

    /// Builds an evaluation context, loading the configured event payload when present.
    pub fn build(&self, command: &ExecuteWorkflowPayload) -> Result<EvaluationContext, String> {
        self.build_with_payload(command)
            .map(|context| context.into_parts().0)
    }

    /// Builds an evaluation context and preserves its original event payload.
    pub fn build_with_payload(
        &self,
        command: &ExecuteWorkflowPayload,
    ) -> Result<GithubContext, String> {
        let inputs = Self::inputs(command);
        let (event, event_payload) = self.event(command, &inputs)?;
        let github = Self::github(command, &event);

        Ok(GithubContext::new(
            EvaluationContext::new()
                .with_root("secrets", Self::secrets(command))
                .with_root("inputs", ContextValue::Mapping(inputs.clone()))
                .with_root("github", github)
                .with_root("runner", Self::runner_context()),
            event_payload,
        ))
    }

    fn github(command: &ExecuteWorkflowPayload, event: &ContextValue) -> ContextValue {
        ContextValue::mapping([
            ("event_name".to_owned(), Self::event_name(command, event)),
            ("action".to_owned(), Self::property(event, &["action"])),
            ("ref".to_owned(), Self::property(event, &["ref"])),
            (
                "ref_name".to_owned(),
                Self::ref_name(&Self::property(event, &["ref"])),
            ),
            (
                "ref_type".to_owned(),
                Self::ref_type(&Self::property(event, &["ref"])),
            ),
            (
                "head_ref".to_owned(),
                Self::property(event, &["pull_request", "head", "ref"]),
            ),
            (
                "base_ref".to_owned(),
                Self::property(event, &["pull_request", "base", "ref"]),
            ),
            ("sha".to_owned(), Self::sha(event)),
            ("actor".to_owned(), Self::actor(event)),
            ("repository".to_owned(), Self::repository(command, event)),
            ("repository_owner".to_owned(), Self::repository_owner(event)),
            ("workflow".to_owned(), Self::workflow(command, event)),
            ("job".to_owned(), Self::job(command, event)),
            ("run_id".to_owned(), ContextValue::text(command.run_id())),
            (
                "workspace".to_owned(),
                ContextValue::text(CONTAINER_WORKSPACE),
            ),
            ("event".to_owned(), event.clone()),
        ])
    }

    fn event(
        &self,
        command: &ExecuteWorkflowPayload,
        inputs: &BTreeMap<String, ContextValue>,
    ) -> Result<(ContextValue, Option<String>), String> {
        match command.config().event_path() {
            Some(path) => {
                let text = self
                    .workflow_source
                    .read_file(path.as_path())
                    .map_err(|error| format!("failed to read event payload: {error}"))?;
                let event = ContextValue::from_json_text(&text)
                    .map_err(|error| format!("failed to parse event payload: {error}"))?;
                Ok((event, Some(text)))
            }
            None => Ok((
                ContextValue::mapping([(
                    "inputs".to_owned(),
                    ContextValue::Mapping(inputs.clone()),
                )]),
                None,
            )),
        }
    }

    fn inputs(command: &ExecuteWorkflowPayload) -> BTreeMap<String, ContextValue> {
        command
            .config()
            .inputs()
            .iter()
            .map(|input| (input.key().to_owned(), ContextValue::text(input.value())))
            .collect()
    }

    fn secrets(command: &ExecuteWorkflowPayload) -> ContextValue {
        ContextValue::mapping(
            command
                .config()
                .secrets()
                .iter()
                .map(|secret| (secret.name().to_owned(), ContextValue::text(secret.value()))),
        )
    }

    fn event_name(command: &ExecuteWorkflowPayload, event: &ContextValue) -> ContextValue {
        command
            .config()
            .event()
            .map(|value| ContextValue::text(value.as_str()))
            .unwrap_or_else(|| {
                Self::property(event, &["event_name"])
                    .as_text()
                    .map(ContextValue::text)
                    .unwrap_or_else(|| ContextValue::text("workflow_dispatch"))
            })
    }

    fn property(value: &ContextValue, path: &[&str]) -> ContextValue {
        path.iter()
            .try_fold(value, |current, name| current.property(name))
            .cloned()
            .unwrap_or(ContextValue::Null)
    }

    fn first_property(event: &ContextValue, paths: &[&[&str]]) -> ContextValue {
        paths
            .iter()
            .map(|path| Self::property(event, path))
            .find(|value| !matches!(value, ContextValue::Null))
            .unwrap_or(ContextValue::Null)
    }

    fn ref_name(reference: &ContextValue) -> ContextValue {
        let Some(reference) = reference.as_text() else {
            return ContextValue::Null;
        };
        let name = reference
            .strip_prefix("refs/heads/")
            .or_else(|| reference.strip_prefix("refs/tags/"))
            .or_else(|| reference.strip_prefix("refs/"));
        name.map(ContextValue::text).unwrap_or(ContextValue::Null)
    }

    fn ref_type(reference: &ContextValue) -> ContextValue {
        let Some(reference) = reference.as_text() else {
            return ContextValue::Null;
        };
        if reference.starts_with("refs/heads/") || reference.starts_with("refs/pull/") {
            ContextValue::text("branch")
        } else if reference.starts_with("refs/tags/") {
            ContextValue::text("tag")
        } else {
            ContextValue::Null
        }
    }

    fn sha(event: &ContextValue) -> ContextValue {
        Self::first_property(
            event,
            &[
                &["sha"],
                &["after"],
                &["pull_request", "head", "sha"],
                &["pull_request", "merge_commit_sha"],
            ],
        )
    }

    fn actor(event: &ContextValue) -> ContextValue {
        Self::first_property(event, &[&["actor"], &["sender", "login"]])
    }

    fn repository(command: &ExecuteWorkflowPayload, event: &ContextValue) -> ContextValue {
        Self::first_property(event, &[&["repository", "full_name"], &["repository"]])
            .as_text()
            .map(ContextValue::text)
            .unwrap_or_else(|| ContextValue::text(command.repository().name().as_str()))
    }

    fn repository_owner(event: &ContextValue) -> ContextValue {
        let repository = Self::repository_name(event);
        repository
            .as_text()
            .and_then(|value| value.split('/').next())
            .filter(|value| !value.is_empty())
            .map(ContextValue::text)
            .unwrap_or(ContextValue::Null)
    }

    fn repository_name(event: &ContextValue) -> ContextValue {
        Self::first_property(
            event,
            &[
                &["repository", "full_name"],
                &["repository"],
                &["repository", "name"],
            ],
        )
    }

    fn workflow(command: &ExecuteWorkflowPayload, event: &ContextValue) -> ContextValue {
        Self::first_property(event, &[&["workflow"]])
            .as_text()
            .map(ContextValue::text)
            .or_else(|| command.workflow_file_name().map(ContextValue::text))
            .or_else(|| {
                command
                    .config()
                    .workflow()
                    .map(|value| ContextValue::text(value.as_str()))
            })
            .unwrap_or(ContextValue::Null)
    }

    fn job(command: &ExecuteWorkflowPayload, event: &ContextValue) -> ContextValue {
        Self::first_property(event, &[&["job"]])
            .as_text()
            .map(ContextValue::text)
            .or_else(|| {
                command
                    .config()
                    .job()
                    .map(|value| ContextValue::text(value.as_str()))
            })
            .unwrap_or(ContextValue::Null)
    }

    fn runner_context() -> ContextValue {
        ContextValue::mapping([
            ("os".to_owned(), ContextValue::text("Linux")),
            ("arch".to_owned(), ContextValue::text("X64")),
            ("temp".to_owned(), ContextValue::text("/tmp")),
        ])
    }
}

impl Default for GithubContextBuilder {
    fn default() -> Self {
        Self::new(Arc::new(FilesystemWorkflowSource::default()))
    }
}
