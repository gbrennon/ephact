use std::collections::{HashMap, HashSet, VecDeque};

use crate::domain::{
    aggregates::Workflow,
    entities::JobRun,
    errors::PlanError,
    value_objects::{ExecutionPlan, ExecutionStage},
};

/// Plans the execution order of workflow jobs.
///
/// Builds a DAG from job `needs` dependencies and topologically sorts
/// into stages where independent jobs run in parallel.
pub struct ExecutionPlanner;
type DependencyMaps<'a> = (HashMap<&'a str, usize>, HashMap<&'a str, Vec<&'a str>>);

impl ExecutionPlanner {
    /// Plans the execution of a single workflow.
    ///
    /// Jobs with no `needs` go in the first stage. Jobs that depend on
    /// other jobs go in later stages. The planner detects cycles and
    /// returns an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// use ephact::domain::aggregates::Workflow;
    /// use ephact::domain::entities::Job;
    /// use ephact::domain::services::ExecutionPlanner;
    /// use ephact::domain::value_objects::WorkflowTrigger;
    ///
    /// fn job(needs: Vec<String>) -> Job {
    ///     Job::new(
    ///             None,
    ///             None,
    ///             Vec::new(),
    ///             needs,
    ///             None,
    ///             None,
    ///             HashMap::new(),
    ///             None,
    ///             HashMap::new(),
    ///             HashMap::new(),
    ///             None,
    ///             None,
    ///             None,
    ///             None,
    ///             None,
    ///             None,
    ///         )
    /// }
    ///
    /// let jobs = HashMap::from([
    ///     ("build".to_owned(), job(Vec::new())),
    ///     ("test".to_owned(), job(vec!["build".to_owned()])),
    /// ]);
    /// let workflow = Workflow::new(
    ///     None,
    ///     None,
    ///     WorkflowTrigger::default(),
    ///     HashMap::new(),
    ///     jobs,
    ///     None,
    ///     None,
    ///     None,
    /// );
    ///
    /// let plan = ExecutionPlanner.plan(&workflow).unwrap();
    /// assert_eq!(plan.stages().len(), 2);
    /// ```
    pub fn plan(&self, workflow: &Workflow) -> Result<ExecutionPlan, PlanError> {
        let job_ids: Vec<&String> = workflow.jobs().keys().collect();

        let mut dependencies: HashMap<&str, Vec<&str>> = HashMap::new();
        for id in &job_ids {
            let job = &workflow.jobs()[*id];
            let deps: Vec<&str> = job.needs().iter().map(|n| n.as_str()).collect();
            dependencies.insert(id.as_str(), deps);
        }

        self.detect_cycles(&dependencies)?;

        let stages = self.topological_sort(&dependencies, workflow)?;

        Ok(ExecutionPlan::new(stages))
    }

    /// Detects cycles in the dependency graph.
    fn detect_cycles(&self, deps: &HashMap<&str, Vec<&str>>) -> Result<(), PlanError> {
        let mut visited = HashSet::new();
        let mut in_stack = HashSet::new();

        for &node in deps.keys() {
            if !visited.contains(node) {
                self.detect_cycle_depth_first(node, deps, &mut visited, &mut in_stack)?;
            }
        }
        Ok(())
    }

    fn detect_cycle_depth_first<'a>(
        &self,
        node: &'a str,
        deps: &HashMap<&'a str, Vec<&'a str>>,
        visited: &mut HashSet<&'a str>,
        in_stack: &mut HashSet<&'a str>,
    ) -> Result<(), PlanError> {
        visited.insert(node);
        in_stack.insert(node);

        if let Some(neighbors) = deps.get(node) {
            for &neighbor in neighbors {
                self.validate_neighbor_dependency(node, neighbor, deps, visited, in_stack)?;
            }
        }

        in_stack.remove(node);
        Ok(())
    }

    fn validate_neighbor_dependency<'a>(
        &self,
        node: &'a str,
        neighbor: &'a str,
        deps: &HashMap<&'a str, Vec<&'a str>>,
        visited: &mut HashSet<&'a str>,
        in_stack: &mut HashSet<&'a str>,
    ) -> Result<(), PlanError> {
        if !visited.contains(neighbor) {
            self.detect_cycle_depth_first(neighbor, deps, visited, in_stack)?;
        } else if in_stack.contains(neighbor) {
            return Err(PlanError::CycleDetected {
                job: node.to_owned(),
                dependency: neighbor.to_owned(),
            });
        }
        Ok(())
    }

    /// Topologically sorts jobs into stages.
    ///
    /// Stage 0: jobs with no dependencies.
    /// Stage N: jobs whose dependencies are all in stages < N.
    fn topological_sort(
        &self,
        deps: &HashMap<&str, Vec<&str>>,
        workflow: &Workflow,
    ) -> Result<Vec<ExecutionStage>, PlanError> {
        let (mut in_degree, dependents) = Self::build_dependency_maps(deps)?;
        let mut queue: VecDeque<&str> = in_degree
            .iter()
            .filter(|(_, deg)| **deg == 0)
            .map(|(&id, _)| id)
            .collect();

        let mut stages: Vec<ExecutionStage> = Vec::new();
        let mut processed = 0usize;
        let total = deps.len();

        while !queue.is_empty() {
            let stage_jobs: Vec<&str> = queue.drain(..).collect();
            let runs = Self::build_runs_for_stage(&stage_jobs, workflow);
            stages.push(ExecutionStage::new(runs));
            processed += stage_jobs.len();
            Self::release_dependents(&stage_jobs, &dependents, &mut in_degree, &mut queue);
        }

        if processed != total {
            return Err(PlanError::UnresolvedDependencies);
        }

        Ok(stages)
    }

    fn build_dependency_maps<'a>(
        deps: &HashMap<&'a str, Vec<&'a str>>,
    ) -> Result<DependencyMaps<'a>, PlanError> {
        let mut in_degree: HashMap<&'a str, usize> = HashMap::new();
        let mut dependents: HashMap<&'a str, Vec<&'a str>> = HashMap::new();

        for (&job_id, job_deps) in deps {
            in_degree.entry(job_id).or_insert(0);
            Self::register_job_dependencies(
                job_id,
                job_deps,
                deps,
                &mut in_degree,
                &mut dependents,
            )?;
        }

        Ok((in_degree, dependents))
    }

    fn register_job_dependencies<'a>(
        job_id: &'a str,
        job_deps: &[&'a str],
        deps: &HashMap<&'a str, Vec<&'a str>>,
        in_degree: &mut HashMap<&'a str, usize>,
        dependents: &mut HashMap<&'a str, Vec<&'a str>>,
    ) -> Result<(), PlanError> {
        for &dep in job_deps {
            if !deps.contains_key(dep) {
                return Err(PlanError::MissingDependency {
                    job: job_id.to_owned(),
                    dependency: dep.to_owned(),
                });
            }
            *in_degree.entry(job_id).or_insert(0) += 1;
            dependents.entry(dep).or_default().push(job_id);
        }
        Ok(())
    }

    fn build_runs_for_stage(stage_jobs: &[&str], workflow: &Workflow) -> Vec<JobRun> {
        stage_jobs
            .iter()
            .map(|&id| {
                JobRun::new(
                    workflow.name().map(str::to_string),
                    id.to_owned(),
                    workflow.jobs()[id].clone(),
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
        for &job_id in stage_jobs {
            if let Some(deps) = dependents.get(job_id) {
                Self::decrement_dependents(deps, in_degree, queue);
            }
        }
    }

    fn decrement_dependents<'a>(
        deps: &[&'a str],
        in_degree: &mut HashMap<&'a str, usize>,
        queue: &mut VecDeque<&'a str>,
    ) {
        for &dep_id in deps {
            if let Some(deg) = in_degree.get_mut(dep_id) {
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(dep_id);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::{entities::Job, value_objects::WorkflowTrigger};

    fn make_job(needs: &[&str]) -> Job {
        let needs = needs.iter().map(|need| (*need).to_owned()).collect();
        Job::new(
            None,
            None,
            Vec::new(),
            needs,
            None,
            None,
            HashMap::new(),
            None,
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }

    fn make_workflow(jobs: &[(&str, &[&str])]) -> Workflow {
        let jobs = jobs
            .iter()
            .map(|(id, needs)| ((*id).to_owned(), make_job(needs)))
            .collect();
        Workflow::new(
            None,
            None,
            WorkflowTrigger::default(),
            HashMap::new(),
            jobs,
            None,
            None,
            None,
        )
    }

    #[test]
    fn plan_single_job() {
        let wf = make_workflow(&[("build", &[])]);
        let plan = ExecutionPlanner.plan(&wf).unwrap();
        assert_eq!(plan.stages().len(), 1);
        assert_eq!(plan.stages()[0].runs().len(), 1);
        assert_eq!(plan.stages()[0].runs()[0].job_id(), "build");
    }

    #[test]
    fn plan_independent_jobs_same_stage() {
        let wf = make_workflow(&[("build", &[]), ("lint", &[])]);
        let plan = ExecutionPlanner.plan(&wf).unwrap();
        assert_eq!(plan.stages().len(), 1);
        assert_eq!(plan.stages()[0].runs().len(), 2);
    }

    #[test]
    fn plan_sequential_jobs() {
        let wf = make_workflow(&[("build", &[]), ("test", &["build"])]);
        let plan = ExecutionPlanner.plan(&wf).unwrap();
        assert_eq!(plan.stages().len(), 2);
        assert_eq!(plan.stages()[0].runs()[0].job_id(), "build");
        assert_eq!(plan.stages()[1].runs()[0].job_id(), "test");
    }

    #[test]
    fn plan_diamond_dependency() {
        let wf = make_workflow(&[
            ("build", &[]),
            ("test", &["build"]),
            ("lint", &["build"]),
            ("deploy", &["test", "lint"]),
        ]);
        let plan = ExecutionPlanner.plan(&wf).unwrap();
        assert_eq!(plan.stages().len(), 3);
        assert_eq!(plan.stages()[0].runs()[0].job_id(), "build");
        assert_eq!(plan.stages()[1].runs().len(), 2);
        assert_eq!(plan.stages()[2].runs().len(), 1);
    }

    #[test]
    fn plan_detects_cycle() {
        let wf = make_workflow(&[("a", &["b"]), ("b", &["a"])]);
        let result = ExecutionPlanner.plan(&wf);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PlanError::CycleDetected { .. }
        ));
    }

    #[test]
    fn plan_detects_missing_dependency() {
        let wf = make_workflow(&[("build", &["nonexistent"])]);
        let result = ExecutionPlanner.plan(&wf);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PlanError::MissingDependency { .. }
        ));
    }

    #[test]
    fn topological_sort_reports_unresolved_dependencies_on_cycle() {
        let wf = make_workflow(&[("a", &["b"]), ("b", &["a"])]);
        let mut deps = HashMap::new();
        deps.insert("a", vec!["b"]);
        deps.insert("b", vec!["a"]);
        let result = ExecutionPlanner.topological_sort(&deps, &wf);
        assert!(matches!(result, Err(PlanError::UnresolvedDependencies)));
    }
}
