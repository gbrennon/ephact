use std::collections::{HashMap, HashSet, VecDeque};

use crate::{
    entities::{Job, JobRun},
    errors::PlanError,
    value_objects::{
        ConcurrencyGroup, ExecutionDefaults, ExecutionPlan, ExecutionStage, TokenPermissions,
        TriggerKind, WorkflowTrigger,
    },
};
type DependencyMaps<'a> = (HashMap<&'a str, usize>, HashMap<&'a str, Vec<&'a str>>);

/// Represents a parsed workflow file.
///
/// Maps to the top-level structure of a workflow YAML file.
/// Supports all standard fields including `name`, `on`, `env`, `jobs`,
/// `defaults`, and `permissions`.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
///
/// use ephact_domain::{
///     aggregates::Workflow,
///     value_objects::{TriggerKind, WorkflowTrigger},
/// };
///
/// let workflow = Workflow::new(
///     Some("CI".to_owned()),
///     vec![WorkflowTrigger::Push(None)],
///     HashMap::new(),
///     HashMap::new(),
/// );
///
/// assert_eq!(workflow.name(), Some("CI"));
/// assert!(workflow.triggers_on(TriggerKind::Push));
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Workflow {
    /// The display name of the workflow.
    name: Option<String>,

    /// The name of the workflow file (set after parsing, not from YAML).
    file: Option<String>,

    /// The triggers that activate this workflow.
    trigger: Vec<WorkflowTrigger>,

    /// Environment variables available to all jobs and steps.
    env: HashMap<String, String>,

    /// The jobs that make up this workflow.
    jobs: HashMap<String, Job>,

    /// Default settings applied to all jobs in the workflow.
    defaults: Option<ExecutionDefaults>,

    /// TokenPermissions for the workflow token.
    permissions: Option<TokenPermissions>,

    /// ConcurrencyGroup group to limit parallel runs.
    concurrency: Option<ConcurrencyGroup>,
}

impl Workflow {
    pub fn new(
        name: Option<String>,
        trigger: Vec<WorkflowTrigger>,
        env: HashMap<String, String>,
        jobs: HashMap<String, Job>,
    ) -> Self {
        Self {
            name,
            file: None,
            trigger,
            env,
            jobs,
            defaults: None,
            permissions: None,
            concurrency: None,
        }
    }

    pub fn with_defaults(mut self, defaults: Option<ExecutionDefaults>) -> Self {
        self.defaults = defaults;
        self
    }

    pub fn with_permissions(mut self, permissions: Option<TokenPermissions>) -> Self {
        self.permissions = permissions;
        self
    }

    pub fn with_concurrency(mut self, concurrency: Option<ConcurrencyGroup>) -> Self {
        self.concurrency = concurrency;
        self
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn file(&self) -> Option<&str> {
        self.file.as_deref()
    }

    pub fn with_file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    pub fn trigger(&self) -> &[WorkflowTrigger] {
        &self.trigger
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn jobs(&self) -> &HashMap<String, Job> {
        &self.jobs
    }

    pub fn defaults(&self) -> Option<&ExecutionDefaults> {
        self.defaults.as_ref()
    }

    pub fn permissions(&self) -> Option<&TokenPermissions> {
        self.permissions.as_ref()
    }

    pub fn concurrency(&self) -> Option<&ConcurrencyGroup> {
        self.concurrency.as_ref()
    }
    /// Returns whether this workflow declares the given trigger event.
    pub fn triggers_on(&self, kind: TriggerKind) -> bool {
        self.trigger.iter().any(|trigger| trigger.kind() == kind)
    }

    /// Returns the job identified by `job_id`.
    pub fn job_named(&self, job_id: &str) -> Option<&Job> {
        self.jobs.get(job_id)
    }
    /// Plans this workflow's jobs into dependency-ordered execution stages.
    ///
    /// Planning is behavior of the aggregate because it only reasons about
    /// this workflow's own jobs and their declared dependencies.
    pub fn plan(&self) -> Result<ExecutionPlan, PlanError> {
        let dependencies: HashMap<&str, Vec<&str>> = self
            .jobs
            .iter()
            .map(|(id, job)| {
                (
                    id.as_str(),
                    job.needs().iter().map(String::as_str).collect(),
                )
            })
            .collect();

        Self::detect_cycles(&dependencies)?;
        let (mut in_degree, dependents) = Self::build_dependency_maps(&dependencies)?;
        let mut queue: VecDeque<&str> = in_degree
            .iter()
            .filter(|(_, degree)| **degree == 0)
            .map(|(&id, _)| id)
            .collect();
        let mut stages = Vec::new();
        let mut processed = 0;

        while let Some(stage_jobs) = Self::take_next_stage(&mut queue) {
            let runs = self.runs_for_stage(&stage_jobs);
            stages.push(ExecutionStage::new(runs));
            processed += stage_jobs.len();
            Self::release_dependents(&stage_jobs, &dependents, &mut in_degree, &mut queue);
        }

        if processed != dependencies.len() {
            return Err(PlanError::UnresolvedDependencies);
        }
        Ok(ExecutionPlan::new(stages))
    }

    fn detect_cycles(dependencies: &HashMap<&str, Vec<&str>>) -> Result<(), PlanError> {
        let mut visited = HashSet::new();
        let mut in_stack = HashSet::new();
        for &job_id in dependencies.keys() {
            if !visited.contains(job_id) {
                Self::visit_for_cycle(job_id, dependencies, &mut visited, &mut in_stack)?;
            }
        }
        Ok(())
    }

    fn visit_for_cycle<'a>(
        job_id: &'a str,
        dependencies: &HashMap<&'a str, Vec<&'a str>>,
        visited: &mut HashSet<&'a str>,
        in_stack: &mut HashSet<&'a str>,
    ) -> Result<(), PlanError> {
        visited.insert(job_id);
        in_stack.insert(job_id);
        if let Some(neighbors) = dependencies.get(job_id) {
            for &neighbor in neighbors {
                if !visited.contains(neighbor) {
                    Self::visit_for_cycle(neighbor, dependencies, visited, in_stack)?;
                } else if in_stack.contains(neighbor) {
                    return Err(PlanError::CycleDetected {
                        job: job_id.to_owned(),
                        dependency: neighbor.to_owned(),
                    });
                }
            }
        }
        in_stack.remove(job_id);
        Ok(())
    }

    fn take_next_stage<'a>(queue: &mut VecDeque<&'a str>) -> Option<Vec<&'a str>> {
        (!queue.is_empty()).then(|| queue.drain(..).collect())
    }

    fn runs_for_stage(&self, stage_jobs: &[&str]) -> Vec<JobRun> {
        stage_jobs
            .iter()
            .map(|id| {
                JobRun::new(
                    self.name.clone(),
                    (*id).to_owned(),
                    self.jobs[*id].clone(),
                    None,
                )
            })
            .collect()
    }

    fn release_dependents<'a>(
        stage_jobs: &[&'a str],
        dependents: &HashMap<&'a str, Vec<&'a str>>,
        in_degree: &mut HashMap<&'a str, usize>,
        queue: &mut VecDeque<&'a str>,
    ) {
        stage_jobs
            .iter()
            .filter_map(|job_id| dependents.get(job_id))
            .flatten()
            .for_each(|dependent_id| Self::decrement_dependent(dependent_id, in_degree, queue));
    }

    fn decrement_dependent<'a>(
        dependent_id: &'a str,
        in_degree: &mut HashMap<&'a str, usize>,
        queue: &mut VecDeque<&'a str>,
    ) {
        let remaining = in_degree
            .get_mut(dependent_id)
            .expect("every dependent job has an in-degree entry");
        *remaining -= 1;
        if *remaining == 0 {
            queue.push_back(dependent_id);
        }
    }

    fn build_dependency_maps<'a>(
        dependencies: &HashMap<&'a str, Vec<&'a str>>,
    ) -> Result<DependencyMaps<'a>, PlanError> {
        let mut in_degree = HashMap::new();
        let mut dependents = HashMap::new();
        for (&job_id, job_dependencies) in dependencies {
            in_degree.entry(job_id).or_insert(0);
            Self::register_dependencies(
                job_id,
                job_dependencies,
                dependencies,
                &mut in_degree,
                &mut dependents,
            )?;
        }
        Ok((in_degree, dependents))
    }

    fn register_dependencies<'a>(
        job_id: &'a str,
        job_dependencies: &[&'a str],
        dependencies: &HashMap<&'a str, Vec<&'a str>>,
        in_degree: &mut HashMap<&'a str, usize>,
        dependents: &mut HashMap<&'a str, Vec<&'a str>>,
    ) -> Result<(), PlanError> {
        for &dependency in job_dependencies {
            if !dependencies.contains_key(dependency) {
                return Err(PlanError::MissingDependency {
                    job: job_id.to_owned(),
                    dependency: dependency.to_owned(),
                });
            }
            *in_degree.entry(job_id).or_insert(0) += 1;
            dependents.entry(dependency).or_default().push(job_id);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_and_with_file_preserve_fields() {
        let workflow = Workflow::new(
            Some("CI".into()),
            vec![WorkflowTrigger::Push(None)],
            HashMap::from([("KEY".into(), "value".into())]),
            HashMap::new(),
        )
        .with_file("workflow.yml");

        assert_eq!(workflow.name(), Some("CI"));
        assert_eq!(workflow.file(), Some("workflow.yml"));
        assert_eq!(workflow.env()["KEY"], "value");
        assert!(workflow.jobs().is_empty());
        assert!(workflow.defaults().is_none());
        assert_eq!(workflow.trigger(), &[WorkflowTrigger::Push(None)]);
        assert!(workflow.permissions().is_none());
        assert!(workflow.concurrency().is_none());
    }
    #[test]
    fn exposes_trigger_and_named_job_behavior() {
        let job = Job::default();
        let workflow = Workflow::new(
            Some("CI".into()),
            vec![WorkflowTrigger::Push(None)],
            HashMap::new(),
            HashMap::from([("build".into(), job)]),
        );

        assert!(workflow.triggers_on(TriggerKind::Push));
        assert!(!workflow.triggers_on(TriggerKind::PullRequest));
        assert!(workflow.job_named("build").is_some());
        assert!(workflow.job_named("missing").is_none());
    }
    #[test]
    fn builders_set_optional_sections() {
        let workflow = Workflow::new(
            Some("CI".into()),
            Vec::new(),
            HashMap::new(),
            HashMap::new(),
        )
        .with_defaults(Some(ExecutionDefaults::new(None)))
        .with_permissions(Some(TokenPermissions::new()))
        .with_concurrency(Some(ConcurrencyGroup::new("ci", Some(true))));

        assert_eq!(workflow.defaults(), Some(&ExecutionDefaults::new(None)));
        assert_eq!(workflow.permissions(), Some(&TokenPermissions::new()));
        assert_eq!(
            workflow.concurrency(),
            Some(&ConcurrencyGroup::new("ci", Some(true)))
        );
    }
}
