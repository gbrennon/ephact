#[derive(Debug, Clone, PartialEq, Default)]
pub(super) struct JobIdentity {
    /// The display name of the job.
    pub(super) name: Option<String>,

    /// Jobs that must complete successfully before this job runs.
    pub(super) needs: Vec<String>,
}
