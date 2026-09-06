use crate::application::dtos::ContainerCleanupRequest;

/// Inbound port for handling act-run-completed events.
///
/// Implemented by application services that react to workflow completion
/// (e.g. stopping containers, sending notifications).
pub trait ContainerCleanupPort: Send + Sync {
    /// Handles the completion of a workflow run.
    ///
    /// Implementations SHOULD gracefully stop (down) each container, then
    /// force-kill any that did not exit and remove it, but MUST NOT delete
    /// cached images so the user does not have to re-download them on the
    /// next run.
    fn execute(&self, request: ContainerCleanupRequest);
}
