pub mod eval_error;
pub mod lexer_error;
pub mod parse_error;

pub use eval_error::EvalError;
pub use lexer_error::LexerError;
pub use parse_error::ParseError;

macro_rules! impl_application_error {
    ($error_type:ty, $display:expr $(,)?) => {
        impl std::fmt::Display for $error_type {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "{}", ($display)(self))
            }
        }
    };
}

mod application_error;
mod copy_repository_to_container_error;
mod discover_run_inputs_error;
mod execute_action_error;
mod execute_job_error;
mod execute_nested_action_error;
mod execute_step_error;
mod execute_workflow_error;
mod list_actions_error;
mod list_workflows_error;
mod load_workflow_error;
mod prepare_job_container_error;
mod project_branding_store_error;
mod run_action_error;
mod settings_store_error;
mod show_project_branding_info_error;
mod workflow_source_error;

pub use application_error::ApplicationError;
pub use copy_repository_to_container_error::CopyRepositoryToContainerError;
pub use discover_run_inputs_error::DiscoverRunInputsError;
pub use execute_action_error::ExecuteActionError;
pub use execute_job_error::ExecuteJobError;
pub use execute_nested_action_error::ExecuteNestedActionError;
pub use execute_step_error::ExecuteStepError;
pub use execute_workflow_error::ExecuteWorkflowError;
pub use list_actions_error::ListActionsError;
pub use list_workflows_error::ListWorkflowsError;
pub use load_workflow_error::LoadWorkflowError;
pub use prepare_job_container_error::PrepareJobContainerError;
pub use project_branding_store_error::ProjectBrandingStoreError;
pub use run_action_error::RunActionError;
pub use settings_store_error::SettingsStoreError;
pub use show_project_branding_info_error::ShowProjectBrandingInfoError;
pub use workflow_source_error::WorkflowSourceError;
