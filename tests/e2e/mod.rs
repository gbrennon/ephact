#[path = "../common/mod.rs"]
mod common;

#[path = "../common/fakes/e2e_failing_container.rs"]
mod e2e_failing_container;
#[path = "../common/fakes/e2e_failing_runtime.rs"]
mod e2e_failing_runtime;
#[path = "../common/fakes/e2e_fixed_image_mapper.rs"]
mod e2e_fixed_image_mapper;
#[path = "../common/fakes/e2e_mirrored_action_fetcher.rs"]
mod e2e_mirrored_action_fetcher;
#[path = "../common/fakes/e2e_succeeding_container.rs"]
mod e2e_succeeding_container;
#[path = "../common/fakes/e2e_succeeding_runtime.rs"]
mod e2e_succeeding_runtime;
mod scenarios;
mod support;

mod cli_commands_tests;
mod continue_on_error_pipeline_tests;
mod delivery_pipeline_tests;
mod every_workflow_tests;
mod failing_pipeline_tests;
mod remote_action_pipeline_tests;
