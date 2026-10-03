use std::path::PathBuf;

use crate::{
    aggregates::Workflow, entities::Job, messages::Message, value_objects::EvaluationContext,
};

/// Data describing the intention to execute one job.
#[derive(Debug, Clone)]
pub struct ExecuteJobPayload {
    job: Job,
    job_id: String,
    workflow: Workflow,
    repo_path: PathBuf,
    context: EvaluationContext,
    run_id: String,
    allow_repo_writes: bool,
    allow_network: bool,
}

impl ExecuteJobPayload {
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

    pub fn with_run_id(self, run_id: String) -> Self {
        Self { run_id, ..self }
    }

    pub fn with_allow_repo_writes(self, allow_repo_writes: bool) -> Self {
        Self {
            allow_repo_writes,
            ..self
        }
    }

    pub fn with_allow_network(self, allow_network: bool) -> Self {
        Self {
            allow_network,
            ..self
        }
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

impl Message for ExecuteJobPayload {}

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

    fn payload_for_test() -> ExecuteJobPayload {
        ExecuteJobPayload::new(
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
        .with_run_id("run-1".into())
        .with_allow_repo_writes(true)
        .with_allow_network(true)
    }

    #[test]
    fn new_exposes_every_field() {
        let payload = payload_for_test();

        assert_eq!(payload.job_id(), "build");
        assert_eq!(payload.workflow(), &workflow_for_test());
        assert_eq!(payload.repo_path(), &PathBuf::from("/repo"));
        assert!(payload.context().get("source").is_none());
        assert_eq!(payload.run_id(), "run-1");
        assert!(payload.allow_repo_writes());
        assert!(payload.allow_network());
        assert_eq!(payload.job().runs_on(), Some("ubuntu"));
    }

    #[test]
    fn into_parts_returns_owned_fields() {
        let (_job, job_id, workflow, repo_path, context, run_id, allow_repo_writes) =
            payload_for_test().into_parts();

        assert_eq!(job_id, "build");
        assert_eq!(workflow, workflow_for_test());
        assert_eq!(repo_path, PathBuf::from("/repo"));
        assert!(context.get("source").is_none());
        assert_eq!(run_id, "run-1");
        assert!(allow_repo_writes);
    }
}
