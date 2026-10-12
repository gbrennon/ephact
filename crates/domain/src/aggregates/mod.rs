/// Settings controlling workflow execution, permissions, and output behavior.
pub mod settings;
/// Parsed workflow data and dependency planning.
pub mod workflow;

/// Re-exports the aggregate types for convenient access.
pub use self::{settings::Settings, workflow::Workflow};
