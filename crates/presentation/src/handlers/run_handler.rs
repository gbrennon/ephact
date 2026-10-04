use std::path::PathBuf;

use super::{
    super::components::{
        box_component::BoxComponent, component::Component, run_summary::RunSummaryComponent,
        terminal::Terminal,
    },
    diagnostic_stores::DiagnosticStores,
    preflight_ports::PreflightPorts,
    run_id::RunIdGenerator,
    single_workflow_run::SingleWorkflowRun,
};
use crate::{
    application::{
        dtos::{
            requests::{
                DiscoverRunInputsRequest, ListWorkflowsRequest, RunAllWorkflowsRequest,
                RunWorkflowRequest,
            },
            responses::RunSummaryResponse,
        },
        ports::{
            inbound::{ListWorkflowsPort, RunAllWorkflowsPort, RunWorkflowPort},
            outbound::RunInputsDiscovererPort,
        },
    },
    cli::{InputCollector, run_args::RunArgs},
    domain::{
        Repository,
        value_objects::{WorkflowEvent, WorkflowInput, WorkflowPath, WorkflowRunConfig},
    },
    infrastructure::RepositoryResolver,
};

/// Handles the `run` subcommand by dispatching parsed CLI arguments to the
/// application port.
///
/// Live progress is rendered by the event handler registered in the
/// container; this handler only prints the final GitHub-Actions-like run
/// summary and interprets the result for the process exit code.
pub struct RunHandler;

impl RunHandler {
    /// Executes a single workflow programmatically (used by the TUI).
    ///
    /// Converts `repository_path` into a [`Repository`], builds a run request
    /// with the optional `workflow`, and returns the run
    /// summary produced by the port.
    pub async fn handle(
        run_workflow_port: &dyn RunWorkflowPort,
        repository_path: PathBuf,
        workflow: Option<String>,
    ) -> Result<RunSummaryResponse, Box<dyn std::error::Error>> {
        Self::handle_with_event_and_inputs(
            run_workflow_port,
            repository_path,
            workflow,
            None,
            Vec::new(),
        )
        .await
    }

    pub async fn handle_with_event_and_inputs(
        run_workflow_port: &dyn RunWorkflowPort,
        repository_path: PathBuf,
        workflow: Option<String>,
        event: Option<String>,
        inputs: Vec<(String, String)>,
    ) -> Result<RunSummaryResponse, Box<dyn std::error::Error>> {
        let run_id = RunIdGenerator.generate();
        let request = SingleWorkflowRun::new(repository_path, workflow, event, inputs, &run_id);
        Self::execute_single_workflow(run_workflow_port, request).await
    }

    pub async fn handle_with_event_and_inputs_and_run_id(
        run_workflow_port: &dyn RunWorkflowPort,
        repository_path: PathBuf,
        workflow: Option<String>,
        event: Option<String>,
        inputs: Vec<(String, String)>,
    ) -> Result<(RunSummaryResponse, String), Box<dyn std::error::Error>> {
        let run_id = RunIdGenerator.generate();
        let request = SingleWorkflowRun::new(repository_path, workflow, event, inputs, &run_id);
        let summary = Self::execute_single_workflow(run_workflow_port, request).await?;
        Ok((summary, run_id))
    }

    async fn execute_single_workflow(
        run_workflow_port: &dyn RunWorkflowPort,
        run: SingleWorkflowRun,
    ) -> Result<RunSummaryResponse, Box<dyn std::error::Error>> {
        let repository = Self::build_repository(run.repository_path().to_path_buf())?;
        let config = Self::single_workflow_config(
            run.workflow().map(str::to_owned),
            run.event().map(str::to_owned),
            run.inputs().to_vec(),
        );
        let request = Self::build_run_workflow_request(&config, &repository, run.run_id());
        Ok(run_workflow_port.execute(request).await?)
    }

    pub fn discover_inputs(
        discover_run_inputs_port: &dyn RunInputsDiscovererPort,
        repository_path: PathBuf,
        workflow: Option<String>,
        event: Option<String>,
    ) -> Result<
        Vec<crate::application::dtos::responses::RunInputDeclarationResponse>,
        Box<dyn std::error::Error>,
    > {
        let repository = Self::build_repository(repository_path)?;
        let config = Self::single_workflow_config(workflow, event, Vec::new());
        Ok(discover_run_inputs_port.discover(DiscoverRunInputsRequest::new(config, repository))?)
    }

    fn build_repository(
        repository_path: PathBuf,
    ) -> Result<Repository, Box<dyn std::error::Error>> {
        Ok(RepositoryResolver::resolve_from_path(repository_path)?)
    }

    fn single_workflow_config(
        workflow: Option<String>,
        event: Option<String>,
        inputs: Vec<(String, String)>,
    ) -> WorkflowRunConfig {
        let config = event
            .map(|name| WorkflowRunConfig::new().with_event(WorkflowEvent::new(name)))
            .unwrap_or_default();
        let config = match workflow {
            Some(name) => config.with_workflow(WorkflowPath::new(name)),
            None => config,
        };
        inputs.into_iter().fold(config, |config, (key, value)| {
            config.add_input(WorkflowInput::new(key, value))
        })
    }

    /// Executes the `run` subcommand: converts CLI args to domain objects,
    /// runs the workflow(s), prints the summary, and maps failure to an error.
    pub async fn handle_cli(
        args: RunArgs,
        run_workflow_port: &dyn RunWorkflowPort,
        run_all_workflows_port: &dyn RunAllWorkflowsPort,
        list_workflows_port: &dyn ListWorkflowsPort,
        terminal: &dyn Terminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (rendered, success) = Self::handle_with_output(
            args,
            run_workflow_port,
            run_all_workflows_port,
            list_workflows_port,
            terminal,
        )
        .await?;
        print!("{rendered}");
        Self::result_for(success)
    }

    pub async fn handle_with_output(
        args: RunArgs,
        run_workflow_port: &dyn RunWorkflowPort,
        run_all_workflows_port: &dyn RunAllWorkflowsPort,
        list_workflows_port: &dyn ListWorkflowsPort,
        terminal: &dyn Terminal,
    ) -> Result<(String, bool), Box<dyn std::error::Error>> {
        let (config, repository) = args.to_domain()?;
        let config = if args.interactive() {
            Self::prepare_interactive_config(
                config,
                &repository,
                list_workflows_port,
                terminal,
                true,
            )?
        } else {
            config
        };
        let run_id = RunIdGenerator.generate();
        let summary = Self::execute_async(
            config,
            repository,
            &run_id,
            run_workflow_port,
            run_all_workflows_port,
        )
        .await?;
        let rendered = BoxComponent::new(RunSummaryComponent::new(&summary), terminal).render();
        Ok((rendered, summary.success()))
    }

    pub async fn handle_with_preflight_output(
        args: RunArgs,
        run_workflow_port: &dyn RunWorkflowPort,
        run_all_workflows_port: &dyn RunAllWorkflowsPort,
        preflight_ports: PreflightPorts<'_>,
    ) -> Result<(String, bool), Box<dyn std::error::Error>> {
        let (config, repository) = Self::prepare_preflight_config(
            args,
            preflight_ports.discover_run_inputs_port(),
            preflight_ports.list_workflows_port(),
            preflight_ports.terminal(),
        )?;
        let run_id = RunIdGenerator.generate();
        let summary = Self::execute_async(
            config,
            repository,
            &run_id,
            run_workflow_port,
            run_all_workflows_port,
        )
        .await?;
        let rendered = BoxComponent::new(
            RunSummaryComponent::new(&summary),
            preflight_ports.terminal(),
        )
        .render();
        Ok((rendered, summary.success()))
    }

    pub async fn handle_with_preflight_output_and_diagnostics(
        args: RunArgs,
        run_workflow_port: &dyn RunWorkflowPort,
        run_all_workflows_port: &dyn RunAllWorkflowsPort,
        preflight_ports: PreflightPorts<'_>,
        diagnostics: DiagnosticStores<'_>,
    ) -> Result<(String, bool), Box<dyn std::error::Error>> {
        let (config, repository) = Self::prepare_preflight_config(
            args,
            preflight_ports.discover_run_inputs_port(),
            preflight_ports.list_workflows_port(),
            preflight_ports.terminal(),
        )?;
        let run_id = RunIdGenerator.generate();
        let summary = match Self::execute_async(
            config,
            repository,
            &run_id,
            run_workflow_port,
            run_all_workflows_port,
        )
        .await
        {
            Ok(summary) => summary,
            Err(error) => {
                return Err(Self::augment_execution_error(
                    error,
                    &run_id,
                    diagnostics.error_store(),
                    diagnostics.path_store(),
                ));
            }
        };
        let diagnostic_text =
            Self::take_diagnostics(&run_id, diagnostics.error_store(), diagnostics.path_store());
        let mut rendered = BoxComponent::new(
            RunSummaryComponent::new(&summary),
            preflight_ports.terminal(),
        )
        .render();
        if !summary.success() {
            rendered.push_str(&diagnostic_text);
        }
        Ok((rendered, summary.success()))
    }
    fn prepare_preflight_config(
        args: RunArgs,
        discover_run_inputs_port: &dyn RunInputsDiscovererPort,
        list_workflows_port: &dyn ListWorkflowsPort,
        terminal: &dyn Terminal,
    ) -> Result<(WorkflowRunConfig, crate::domain::Repository), Box<dyn std::error::Error>> {
        let interactive = args.interactive();
        let (config, repository) = args.to_domain()?;
        let config = if interactive {
            Self::prepare_interactive_config(
                config,
                &repository,
                list_workflows_port,
                terminal,
                false,
            )?
        } else {
            config
        };
        let config = Self::preflight_inputs(
            config,
            repository.clone(),
            discover_run_inputs_port,
            terminal,
            interactive,
        )?;
        Ok((config, repository))
    }

    fn result_for(success: bool) -> Result<(), Box<dyn std::error::Error>> {
        if success {
            Ok(())
        } else {
            Err("workflow failed; see the run summary for failed steps".into())
        }
    }

    fn prepare_interactive_config(
        mut config: WorkflowRunConfig,
        repository: &crate::domain::Repository,
        list_workflows_port: &dyn ListWorkflowsPort,
        terminal: &dyn Terminal,
        collect_inputs: bool,
    ) -> Result<WorkflowRunConfig, Box<dyn std::error::Error>> {
        let response = list_workflows_port.execute(ListWorkflowsRequest::new(
            repository.path().as_path().to_path_buf(),
            repository.name().as_str().to_string(),
        ))?;
        let workflows: Vec<_> = response
            .workflows()
            .iter()
            .filter(|workflow| workflow.has_pull_request_event())
            .collect();
        let workflow = Self::select_pull_request_workflow(&workflows, terminal)?;
        if let Some(name) = workflow.name() {
            config = config.with_workflow(WorkflowPath::new(name.to_string()));
        }
        config = config
            .with_event(WorkflowEvent::new("pull_request".to_string()))
            .with_all_workflows(false);
        if collect_inputs {
            InputCollector::new(terminal).collect_inputs(config)
        } else {
            Ok(config)
        }
    }

    fn select_pull_request_workflow<'a>(
        workflows: &[&'a crate::application::dtos::responses::WorkflowListItemResponse],
        terminal: &dyn Terminal,
    ) -> Result<
        &'a crate::application::dtos::responses::WorkflowListItemResponse,
        Box<dyn std::error::Error>,
    > {
        if workflows.is_empty() {
            return Err("no pull_request workflows are available for interactive execution".into());
        }
        terminal.write_text(&Self::workflow_selection_form(workflows))?;
        let selection = Self::read_workflow_selection(terminal)?;
        workflows
            .get(selection.saturating_sub(1))
            .copied()
            .ok_or_else(|| {
                "workflow selection is outside the available pull_request workflows".into()
            })
    }

    fn read_workflow_selection(
        terminal: &dyn Terminal,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let line = terminal.read_line()?;
        line.trim()
            .parse::<usize>()
            .map_err(|_| "workflow selection must be a number".into())
    }

    fn workflow_selection_form(
        workflows: &[&crate::application::dtos::responses::WorkflowListItemResponse],
    ) -> String {
        let options = workflows
            .iter()
            .enumerate()
            .map(|(index, workflow)| {
                format!(
                    "{}. {}",
                    index + 1,
                    workflow.name().unwrap_or("Unnamed workflow")
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "Interactive workflow run\n\nPull request workflows\n{options}\n\nSelect workflow: "
        )
    }

    fn preflight_inputs(
        mut config: WorkflowRunConfig,
        repository: crate::domain::Repository,
        discover_run_inputs_port: &dyn RunInputsDiscovererPort,
        terminal: &dyn Terminal,
        interactive: bool,
    ) -> Result<WorkflowRunConfig, Box<dyn std::error::Error>> {
        let declarations = discover_run_inputs_port
            .discover(DiscoverRunInputsRequest::new(config.clone(), repository))?;
        let collector = InputCollector::new(terminal);
        if collector.should_skip_prompting(&declarations, interactive) {
            return Ok(config);
        }
        collector.collect_declared_inputs(&mut config, &declarations, interactive)?;
        Ok(config)
    }

    fn take_diagnostics(
        run_id: &str,
        error_store: &crate::infrastructure::logging::FailureLogErrorStore,
        path_store: &crate::infrastructure::logging::FailureLogPathStore,
    ) -> String {
        let path = path_store.take(run_id);
        let errors = error_store.read_and_clear();
        let mut output = String::new();
        if let Some(path) = path {
            output.push_str(&format!("\nFailure diagnostics: {}\n", path.display()));
        }
        for error in errors {
            output.push_str(&format!("\nFailure log write error: {error}\n"));
        }
        output
    }

    fn augment_execution_error(
        error: Box<dyn std::error::Error>,
        run_id: &str,
        error_store: &crate::infrastructure::logging::FailureLogErrorStore,
        path_store: &crate::infrastructure::logging::FailureLogPathStore,
    ) -> Box<dyn std::error::Error> {
        let diagnostics = Self::take_diagnostics(run_id, error_store, path_store);
        if diagnostics.is_empty() {
            error
        } else {
            format!("{error}{diagnostics}").into()
        }
    }

    async fn execute_async(
        config: WorkflowRunConfig,
        repository: crate::domain::Repository,
        run_id: &str,
        run_workflow_port: &dyn RunWorkflowPort,
        run_all_workflows_port: &dyn RunAllWorkflowsPort,
    ) -> Result<RunSummaryResponse, Box<dyn std::error::Error>> {
        if config.all_workflows() {
            let request = Self::build_run_all_request(&config, &repository, run_id);
            Ok(run_all_workflows_port.execute(request)?)
        } else {
            let request = Self::build_run_workflow_request(&config, &repository, run_id);
            Ok(run_workflow_port.execute(request).await?)
        }
    }

    fn build_run_all_request(
        config: &WorkflowRunConfig,
        repository: &crate::domain::Repository,
        run_id: &str,
    ) -> RunAllWorkflowsRequest {
        RunAllWorkflowsRequest::from_domain(repository, config, run_id)
    }

    fn build_run_workflow_request(
        config: &WorkflowRunConfig,
        repository: &crate::domain::Repository,
        run_id: &str,
    ) -> RunWorkflowRequest {
        RunWorkflowRequest::from_domain(repository, config, run_id)
    }

    pub fn render(summary: &RunSummaryResponse) -> String {
        RunSummaryComponent::new(summary).render()
    }
}
