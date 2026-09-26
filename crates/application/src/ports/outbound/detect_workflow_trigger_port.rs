/// Reports whether a workflow's content declares a given event.
///
/// Implementations parse the workflow content in whatever format it is
/// authored in. Content that cannot be parsed declares no event, so callers
/// can treat an unparseable workflow and one without the event alike.
pub trait DetectWorkflowTriggerPort: Send + Sync {
    /// Returns `true` when `workflow_content` declares `event_name`.
    fn triggers_on_event(&self, workflow_content: &str, event_name: &str) -> bool;
}
