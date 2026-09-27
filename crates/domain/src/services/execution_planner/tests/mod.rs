use super::*;
use crate::{entities::Job, value_objects::WorkflowTrigger};

impl ExecutionPlanner {
    fn make_job_for_test(needs: &[&str]) -> Job {
        let needs = needs.iter().map(|need| (*need).to_owned()).collect();
        Job::new(None, None, Vec::new(), needs)
    }

    fn make_workflow_for_test(jobs: &[(&str, &[&str])]) -> Workflow {
        let jobs = jobs
            .iter()
            .map(|(id, needs)| ((*id).to_owned(), Self::make_job_for_test(needs)))
            .collect();
        Workflow::new(
            None,
            vec![WorkflowTrigger::Push(None)],
            HashMap::new(),
            jobs,
        )
    }
}

mod planner_errors;
mod planner_order;
