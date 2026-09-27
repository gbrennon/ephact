use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn plan_detects_cycle() {
        let wf = ExecutionPlanner::make_workflow_for_test(&[("a", &["b"]), ("b", &["a"])]);

        let result = ExecutionPlanner.plan(&wf);

        assert!(result.is_err());

        assert!(matches!(
            result.unwrap_err(),
            PlanError::CycleDetected { .. }
        ));
    }

    #[test]

    fn plan_detects_missing_dependency() {
        let wf = ExecutionPlanner::make_workflow_for_test(&[("build", &["nonexistent"])]);

        let result = ExecutionPlanner.plan(&wf);

        assert!(result.is_err());

        assert!(matches!(
            result.unwrap_err(),
            PlanError::MissingDependency { .. }
        ));
    }

    #[test]

    fn topological_sort_reports_unresolved_dependencies_on_cycle() {
        let wf = ExecutionPlanner::make_workflow_for_test(&[("a", &["b"]), ("b", &["a"])]);

        let mut deps = HashMap::new();

        deps.insert("a", vec!["b"]);

        deps.insert("b", vec!["a"]);

        let result = ExecutionPlanner.topological_sort(&deps, &wf);

        assert!(matches!(result, Err(PlanError::UnresolvedDependencies)));
    }
}
