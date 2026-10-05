/// Errors that can occur during workflow planning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanError {
    /// A circular dependency was detected.
    CycleDetected { job: String, dependency: String },

    /// A job depends on a job that doesn't exist.
    MissingDependency { job: String, dependency: String },

    /// One or more declared dependencies prevented all jobs from being placed
    /// in an execution stage.
    UnresolvedDependencies,
}
