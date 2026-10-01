#[path = "../common/mod.rs"]
pub mod common;

#[path = "../common/fakes/e2e_fixed_image_mapper.rs"]
mod e2e_fixed_image_mapper;
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
