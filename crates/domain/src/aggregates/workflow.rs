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
    /// Creates a workflow with its name, triggers, environment, and jobs.
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

    /// Sets the workflow's default job settings.
    pub fn with_defaults(self, defaults: Option<ExecutionDefaults>) -> Self {
        Self { defaults, ..self }
    }

    /// Sets the workflow's token permissions.
    pub fn with_permissions(self, permissions: Option<TokenPermissions>) -> Self {
        Self {
            permissions,
            ..self
        }
    }

    /// Sets the workflow's concurrency settings.
    pub fn with_concurrency(self, concurrency: Option<ConcurrencyGroup>) -> Self {
        Self {
            concurrency,
            ..self
        }
    }

    /// Returns the workflow's display name.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the workflow file name, if set.
    pub fn file(&self) -> Option<&str> {
        self.file.as_deref()
    }

    /// Sets the workflow file name.
    pub fn with_file(self, file: impl Into<String>) -> Self {
        Self {
            file: Some(file.into()),
            ..self
        }
    }

    /// Returns the triggers declared by the workflow.
    pub fn trigger(&self) -> &[WorkflowTrigger] {
        &self.trigger
    }

    /// Returns the workflow-wide environment variables.
    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    /// Returns the jobs defined by the workflow.
    pub fn jobs(&self) -> &HashMap<String, Job> {
        &self.jobs
    }

    /// Returns the workflow's default job settings, if any.
    pub fn defaults(&self) -> Option<&ExecutionDefaults> {
        self.defaults.as_ref()
    }

    /// Returns the workflow's token permissions, if any.
    pub fn permissions(&self) -> Option<&TokenPermissions> {
        self.permissions.as_ref()
    }

    /// Returns the workflow's concurrency settings, if any.
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
        let (in_degree, dependents) = Self::build_dependency_maps(&dependencies)?;
        let queue: VecDeque<&str> = in_degree
            .iter()
            .filter(|(_, degree)| **degree == 0)
            .map(|(&id, _)| id)
            .collect();

        self.plan_stages(
            dependencies.len(),
            &dependents,
            in_degree,
            queue,
            Vec::new(),
            0,
        )
    }

    fn plan_stages<'a>(
        &self,
        dependency_count: usize,
        dependents: &HashMap<&'a str, Vec<&'a str>>,
        in_degree: HashMap<&'a str, usize>,
        queue: VecDeque<&'a str>,
        stages: Vec<ExecutionStage>,
        processed: usize,
    ) -> Result<ExecutionPlan, PlanError> {
        match Self::take_next_stage(queue) {
            None if processed == dependency_count => Ok(ExecutionPlan::new(stages)),
            None => Err(PlanError::UnresolvedDependencies),
            Some(stage_jobs) => {
                let runs = self.runs_for_stage(&stage_jobs);
                let stages = stages
                    .into_iter()
                    .chain(std::iter::once(ExecutionStage::new(runs)))
                    .collect();
                let (in_degree, queue) =
                    Self::release_dependents(&stage_jobs, dependents, in_degree, VecDeque::new());
                self.plan_stages(
                    dependency_count,
                    dependents,
                    in_degree,
                    queue,
                    stages,
                    processed + stage_jobs.len(),
                )
            }
        }
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
            Self::visit_neighbors(job_id, neighbors, dependencies, visited, in_stack)?;
        }
        in_stack.remove(job_id);
        Ok(())
    }

    fn visit_neighbors<'a>(
        job_id: &'a str,
        neighbors: &[&'a str],
        dependencies: &HashMap<&'a str, Vec<&'a str>>,
        visited: &mut HashSet<&'a str>,
        in_stack: &mut HashSet<&'a str>,
    ) -> Result<(), PlanError> {
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
        Ok(())
    }

    fn take_next_stage<'a>(queue: VecDeque<&'a str>) -> Option<Vec<&'a str>> {
        let stage_jobs: Vec<_> = queue.into_iter().collect();
        (!stage_jobs.is_empty()).then_some(stage_jobs)
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
        in_degree: HashMap<&'a str, usize>,
        queue: VecDeque<&'a str>,
    ) -> (HashMap<&'a str, usize>, VecDeque<&'a str>) {
        stage_jobs
            .iter()
            .filter_map(|job_id| dependents.get(job_id))
            .flatten()
            .fold(
                (in_degree, queue),
                |(mut in_degree, mut queue), dependent_id| {
                    Self::decrement_dependent(dependent_id, &mut in_degree, &mut queue);
                    (in_degree, queue)
                },
            )
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
    #[test]
    fn plan_releases_dependencies_into_ordered_stages() {
        let workflow = Workflow::new(
            Some("CI".into()),
            Vec::new(),
            HashMap::new(),
            HashMap::from([
                ("setup".into(), Job::new(None, None, Vec::new(), Vec::new())),
                (
                    "lint".into(),
                    Job::new(None, None, Vec::new(), vec!["setup".into()]),
                ),
                (
                    "test".into(),
                    Job::new(None, None, Vec::new(), vec!["setup".into()]),
                ),
                (
                    "deploy".into(),
                    Job::new(None, None, Vec::new(), vec!["lint".into(), "test".into()]),
                ),
            ]),
        );

        let plan = workflow.plan().expect("workflow should have a valid plan");
        let stage_job_ids = |stage: &ExecutionStage| {
            stage
                .runs()
                .iter()
                .map(|run| run.job_id().to_owned())
                .collect::<HashSet<_>>()
        };

        assert_eq!(plan.stages().len(), 3);
        assert_eq!(
            stage_job_ids(&plan.stages()[0]),
            HashSet::from(["setup".to_owned()])
        );
        assert_eq!(
            stage_job_ids(&plan.stages()[1]),
            HashSet::from(["lint".to_owned(), "test".to_owned()])
        );
        assert_eq!(
            stage_job_ids(&plan.stages()[2]),
            HashSet::from(["deploy".to_owned()])
        );
    }

    #[test]
    fn plan_reports_missing_dependencies() {
        let workflow = Workflow::new(
            Some("CI".into()),
            Vec::new(),
            HashMap::new(),
            HashMap::from([(
                "build".into(),
                Job::new(None, None, Vec::new(), vec!["setup".into()]),
            )]),
        );

        assert_eq!(
            workflow.plan(),
            Err(PlanError::MissingDependency {
                job: "build".into(),
                dependency: "setup".into(),
            })
        );
    }

    #[test]
    fn plan_reports_cycles() {
        let workflow = Workflow::new(
            Some("CI".into()),
            Vec::new(),
            HashMap::new(),
            HashMap::from([(
                "build".into(),
                Job::new(None, None, Vec::new(), vec!["build".into()]),
            )]),
        );

        assert_eq!(
            workflow.plan(),
            Err(PlanError::CycleDetected {
                job: "build".into(),
                dependency: "build".into(),
            })
        );
    }
}
