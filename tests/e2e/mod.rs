#[path = "../common/mod.rs"]
pub mod common;

mod scenarios;
mod support;

mod cli_commands_tests;
mod continue_on_error_pipeline_tests;
mod delivery_pipeline_tests;
mod every_workflow_tests;
mod failing_pipeline_tests;
mod remote_action_pipeline_tests;
mod supported_workflow_formats_tests;
mod workflow_platform_detection_tests;
