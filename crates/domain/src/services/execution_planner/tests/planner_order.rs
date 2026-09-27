use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn plan_single_job() {
        let wf = ExecutionPlanner::make_workflow_for_test(&[("build", &[])]);

        let plan = ExecutionPlanner.plan(&wf).unwrap();

        assert_eq!(plan.stages().len(), 1);

        assert_eq!(plan.stages()[0].runs().len(), 1);

        assert_eq!(plan.stages()[0].runs()[0].job_id(), "build");
    }

    #[test]

    fn plan_independent_jobs_same_stage() {
        let wf = ExecutionPlanner::make_workflow_for_test(&[("build", &[]), ("lint", &[])]);

        let plan = ExecutionPlanner.plan(&wf).unwrap();

        assert_eq!(plan.stages().len(), 1);

        assert_eq!(plan.stages()[0].runs().len(), 2);
    }

    #[test]

    fn plan_sequential_jobs() {
        let wf = ExecutionPlanner::make_workflow_for_test(&[("build", &[]), ("test", &["build"])]);

        let plan = ExecutionPlanner.plan(&wf).unwrap();

        assert_eq!(plan.stages().len(), 2);

        assert_eq!(plan.stages()[0].runs()[0].job_id(), "build");

        assert_eq!(plan.stages()[1].runs()[0].job_id(), "test");
    }

    #[test]

    fn plan_diamond_dependency() {
        let wf = ExecutionPlanner::make_workflow_for_test(&[
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
}
