use std::sync::Arc;

use ephact::{
    application::ports::inbound::{
        list_actions_port::ListActionsPort, list_workflows_port::ListWorkflowsPort,
        run_all_workflows_port::RunAllWorkflowsPort, run_workflow_port::RunWorkflowPort,
    },
    infrastructure::{
        AppContainer, Container, actions::RunActionFactory, di::container::ContainerCollaborators,
        persistence::CargoProjectBrandingStore,
    },
};

use crate::common::fakes::{
    fake_action_fetcher::FakeActionFetcher, fake_runtime::FakeRuntime,
    fake_workflow_source::FakeWorkflowSource,
};

#[test]
fn build_result_contains_all_ports() {
    let runtime = Arc::new(FakeRuntime::new());
    let workflow_source = Arc::new(FakeWorkflowSource::new());
    let container: AppContainer = Container::with_collaborators_and_branding(
        ContainerCollaborators::new(
            runtime,
            Box::new(FakeActionFetcher::returning(std::path::PathBuf::new())),
            workflow_source,
        ),
        None,
        Box::new(CargoProjectBrandingStore::from_metadata(
            "ephact",
            "test description",
            "0.0.0",
            ephact::PROJECT_EMBLEM,
        )),
    );
    fn _assert_run_all_workflows(_: Box<dyn RunAllWorkflowsPort>) {}
    fn _assert_run_workflow(_: Box<dyn RunWorkflowPort>) {}
    fn _assert_run_action(_: RunActionFactory) {}
    fn _assert_list_workflows(_: Box<dyn ListWorkflowsPort>) {}
    fn _assert_list_actions(_: Box<dyn ListActionsPort>) {}
    let (_, run_all, run_workflow, run_action, _discover_run_inputs, list_workflows, list_actions) =
        container.into_parts();
    _assert_run_all_workflows(run_all);
    _assert_run_workflow(run_workflow);
    _assert_run_action(run_action);
    _assert_list_workflows(list_workflows);
    _assert_list_actions(list_actions);
}
