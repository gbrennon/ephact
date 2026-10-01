use std::path::PathBuf;

use crate::{
    aggregates::Workflow, entities::Job, messages::commands::command::Command,
    value_objects::EvaluationContext,
};

/// Command representing the intention to execute one job of a workflow.
///
/// Published by the workflow coordination service once the execution plan is
/// known, and handled by the job command handler.
#[derive(Debug, Clone)]
pub struct ExecuteJobCommand {
    job: Job,
    job_id: String,
    workflow: Workflow,
    repo_path: PathBuf,
    context: EvaluationContext,
    run_id: String,
    allow_repo_writes: bool,
    allow_network: bool,
}

impl ExecuteJobCommand {
    pub fn new(
        job: Job,
        job_id: String,
        workflow: Workflow,
        repo_path: PathBuf,
        context: EvaluationContext,
    ) -> Self {
        Self {
            job,
            job_id,
            workflow,
            repo_path,
            context,
            run_id: String::new(),
            allow_repo_writes: false,
            allow_network: false,
        }
    }

    pub fn with_run_id(mut self, run_id: String) -> Self {
        self.run_id = run_id;
        self
    }

    pub fn with_allow_repo_writes(mut self, allow_repo_writes: bool) -> Self {
        self.allow_repo_writes = allow_repo_writes;
        self
    }

    pub fn with_allow_network(mut self, allow_network: bool) -> Self {
        self.allow_network = allow_network;
        self
    }

    pub fn job(&self) -> &Job {
        &self.job
    }

    pub fn job_id(&self) -> &str {
        &self.job_id
    }

    pub fn workflow(&self) -> &Workflow {
        &self.workflow
    }

    pub fn repo_path(&self) -> &PathBuf {
        &self.repo_path
    }

    pub fn context(&self) -> &EvaluationContext {
        &self.context
    }

    pub fn run_id(&self) -> &str {
        &self.run_id
    }
    pub fn allow_network(&self) -> bool {
        self.allow_network
    }

    pub fn allow_repo_writes(&self) -> bool {
        self.allow_repo_writes
    }

    pub fn into_parts(
        self,
    ) -> (
        Job,
        String,
        Workflow,
        PathBuf,
        EvaluationContext,
        String,
        bool,
    ) {
        (
            self.job,
            self.job_id,
            self.workflow,
            self.repo_path,
            self.context,
            self.run_id,
            self.allow_repo_writes,
        )
    }
}

impl Command for ExecuteJobCommand {}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::value_objects::WorkflowTrigger;

    fn workflow_for_test() -> Workflow {
        Workflow::new(
            Some("CI".into()),
            vec![WorkflowTrigger::Push(None)],
            HashMap::new(),
            HashMap::new(),
        )
    }

    fn command_for_test() -> ExecuteJobCommand {
        ExecuteJobCommand::new(
            Job::new(
                Some("Build".into()),
                Some("ubuntu".into()),
                Vec::new(),
                Vec::new(),
            ),
            "build".into(),
            workflow_for_test(),
            PathBuf::from("/repo"),
            EvaluationContext::new(),
        )
    }

    #[test]
    fn new_defaults_optional_fields() {
        let command = command_for_test();

        assert_eq!(command.job_id(), "build");
        assert_eq!(command.workflow(), &workflow_for_test());
        assert_eq!(command.repo_path(), &PathBuf::from("/repo"));
        assert!(command.context().get("source").is_none());
        assert_eq!(command.run_id(), "");
        assert!(!command.allow_repo_writes());
        assert!(!command.allow_network());
        assert_eq!(command.job().runs_on(), Some("ubuntu"));
    }

    #[test]
    fn builders_set_optional_fields() {
        let command = command_for_test()
            .with_run_id("run-1".into())
            .with_allow_repo_writes(true)
            .with_allow_network(true);

        assert_eq!(command.run_id(), "run-1");
        assert!(command.allow_repo_writes());
        assert!(command.allow_network());
    }

    #[test]
    fn into_parts_returns_owned_fields() {
        let (_job, job_id, workflow, repo_path, context, run_id, allow_repo_writes) =
            command_for_test().with_run_id("run-1".into()).into_parts();

        assert_eq!(job_id, "build");
        assert_eq!(workflow, workflow_for_test());
        assert_eq!(repo_path, PathBuf::from("/repo"));
        assert!(context.get("source").is_none());
        assert_eq!(run_id, "run-1");
        assert!(!allow_repo_writes);
    }

    #[test]
    fn clone_preserves_fields() {
        let command = command_for_test().clone();

        assert_eq!(command.job_id(), "build");
    }
}
