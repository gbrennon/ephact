use std::time::{SystemTime, UNIX_EPOCH};

/// Generates unique identifiers for CLI workflow runs.
pub(super) struct RunIdGenerator;

impl RunIdGenerator {
    /// Creates a process-unique identifier using the current timestamp.
    pub(super) fn generate(&self) -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |duration| duration.as_nanos());
        format!("run-{}-{timestamp}", std::process::id())
    }
}
