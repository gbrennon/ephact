use std::path::PathBuf;

use super::super::components::{
    box_component::BoxComponent, component::Component, run_summary::RunSummaryComponent,
    terminal::Terminal,
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
    domain::{
        RepoPath, Repository, RepositoryName,
        value_objects::{ActEvent, ActInput, ActRunConfig, ActWorkflow},
    },
    presentation::cli::run_args::RunArgs,
};

/// Handles the `run` subcommand by dispatching parsed CLI arguments to the
/// application port.
///
/// Live progress is rendered by the event handler registered in the
/// container; this handler only prints the final GitHub-Actions-like run
/// summary and interprets the result for the process exit code.
pub struct RunHandler;

pub struct PreflightPorts<'a> {
    discover_run_inputs_port: &'a dyn RunInputsDiscovererPort,
    list_workflows_port: &'a dyn ListWorkflowsPort,
    terminal: &'a dyn Terminal,
}

impl<'a> PreflightPorts<'a> {
    pub fn new(
        discover_run_inputs_port: &'a dyn RunInputsDiscovererPort,
        list_workflows_port: &'a dyn ListWorkflowsPort,
        terminal: &'a dyn Terminal,
    ) -> Self {
        Self {
            discover_run_inputs_port,
            list_workflows_port,
            terminal,
        }
    }
}

pub struct DiagnosticStores<'a> {
    error_store: &'a crate::infrastructure::logging::FailureLogErrorStore,
    path_store: &'a crate::infrastructure::logging::FailureLogPathStore,
}

impl<'a> DiagnosticStores<'a> {
    pub fn new(
        error_store: &'a crate::infrastructure::logging::FailureLogErrorStore,
        path_store: &'a crate::infrastructure::logging::FailureLogPathStore,
    ) -> Self {
        Self {
            error_store,
            path_store,
        }
    }
}

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
        let repository = Self::build_repository(repository_path)?;
        let config = Self::single_workflow_config(workflow, event, inputs);
        let request = Self::build_run_workflow_request(&config, &repository);
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
        let repo_path = RepoPath::new(repository_path)?;
        let repo_name = RepositoryName::from_repo_path(&repo_path)?;
        Ok(Repository::new(repo_path, repo_name))
    }

    fn single_workflow_config(
        workflow: Option<String>,
        event: Option<String>,
        inputs: Vec<(String, String)>,
    ) -> ActRunConfig {
        let config = event
            .map(|name| ActRunConfig::new().with_event(ActEvent::new(name)))
            .unwrap_or_default();
        let config = match workflow {
            Some(name) => config.with_workflow(ActWorkflow::new(name)),
            None => config,
        };
        inputs.into_iter().fold(config, |config, (key, value)| {
            config.add_input(ActInput::new(key, value))
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
        let summary = Self::execute_async(
            config,
            repository,
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
            preflight_ports.discover_run_inputs_port,
            preflight_ports.list_workflows_port,
            preflight_ports.terminal,
        )?;
        let summary = Self::execute_async(
            config,
            repository,
            run_workflow_port,
            run_all_workflows_port,
        )
        .await?;
        let rendered =
            BoxComponent::new(RunSummaryComponent::new(&summary), preflight_ports.terminal)
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
            preflight_ports.discover_run_inputs_port,
            preflight_ports.list_workflows_port,
            preflight_ports.terminal,
        )?;
        let run_id = config.run_id().to_string();
        let summary = match Self::execute_async(
            config,
            repository,
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
                    diagnostics.error_store,
                    diagnostics.path_store,
                ));
            }
        };
        let diagnostic_text =
            Self::take_diagnostics(&run_id, diagnostics.error_store, diagnostics.path_store);
        let mut rendered =
            BoxComponent::new(RunSummaryComponent::new(&summary), preflight_ports.terminal)
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
    ) -> Result<(ActRunConfig, crate::domain::Repository), Box<dyn std::error::Error>> {
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
        mut config: ActRunConfig,
        repository: &crate::domain::Repository,
        list_workflows_port: &dyn ListWorkflowsPort,
        terminal: &dyn Terminal,
        collect_inputs: bool,
    ) -> Result<ActRunConfig, Box<dyn std::error::Error>> {
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
            config = config.with_workflow(ActWorkflow::new(name.to_string()));
        }
        config = config
            .with_event(ActEvent::new("pull_request".to_string()))
            .with_all_workflows(false);
        if collect_inputs {
            Self::collect_interactive_inputs(config, terminal)
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

    fn collect_interactive_inputs(
        mut config: ActRunConfig,
        terminal: &dyn Terminal,
    ) -> Result<ActRunConfig, Box<dyn std::error::Error>> {
        terminal.write_text(
            "\nAdditional inputs (optional)\nEnter KEY=VALUE, KEY=env:VARIABLE, or a blank line to continue.\nInput: ",
        )?;
        while let Some(input) = Self::read_interactive_input(terminal)? {
            config = config.add_input(input);
            terminal.write_text("Input: ")?;
        }
        Ok(config)
    }

    fn read_interactive_input(
        terminal: &dyn Terminal,
    ) -> Result<Option<ActInput>, Box<dyn std::error::Error>> {
        let line = terminal.read_line()?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return Ok(None);
        }
        let (key, source) = RunArgs::parse_input_source(trimmed)?;
        let value = source.resolve()?;
        Ok(Some(ActInput::new(key, value)))
    }

    fn preflight_inputs(
        mut config: ActRunConfig,
        repository: crate::domain::Repository,
        discover_run_inputs_port: &dyn RunInputsDiscovererPort,
        terminal: &dyn Terminal,
        interactive: bool,
    ) -> Result<ActRunConfig, Box<dyn std::error::Error>> {
        let declarations = discover_run_inputs_port
            .discover(DiscoverRunInputsRequest::new(config.clone(), repository))?;
        if Self::can_skip_prompting(&declarations, interactive) {
            return Ok(config);
        }
        Self::prompt_or_describe_inputs(&mut config, &declarations, interactive, terminal)?;
        Ok(config)
    }

    fn can_skip_prompting(
        declarations: &[crate::application::dtos::responses::RunInputDeclarationResponse],
        interactive: bool,
    ) -> bool {
        let missing: Vec<_> = declarations
            .iter()
            .filter(|input| input.required() && !input.is_resolved())
            .collect();
        declarations.is_empty() || (!interactive && missing.is_empty())
    }
    fn fail_if_missing_required_noninteractive(
        declarations: &[crate::application::dtos::responses::RunInputDeclarationResponse],
        interactive: bool,
        terminal: &dyn Terminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let missing = Self::collect_missing_required(declarations);
        if !interactive && !terminal.is_interactive() && !missing.is_empty() {
            return Err(Self::missing_inputs_error(&missing));
        }
        Ok(())
    }

    fn prompt_or_describe_inputs(
        config: &mut ActRunConfig,
        declarations: &[crate::application::dtos::responses::RunInputDeclarationResponse],
        interactive: bool,
        terminal: &dyn Terminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Self::fail_if_missing_required_noninteractive(declarations, interactive, terminal)?;
        Self::prompt_or_describe_all(config, declarations, interactive, terminal)
    }

    fn prompt_or_describe_all(
        config: &mut ActRunConfig,
        declarations: &[crate::application::dtos::responses::RunInputDeclarationResponse],
        interactive: bool,
        terminal: &dyn Terminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        terminal.write_text("\nInputs\n")?;
        Self::prompt_or_describe_each(config, declarations, interactive, terminal)
    }

    fn prompt_or_describe_each(
        config: &mut ActRunConfig,
        declarations: &[crate::application::dtos::responses::RunInputDeclarationResponse],
        interactive: bool,
        terminal: &dyn Terminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (index, declaration) in declarations.iter().enumerate() {
            if Self::should_prompt(declaration, interactive) {
                Self::prompt_and_apply(config, declaration, index, declarations.len(), terminal)?;
            } else {
                Self::describe_input(declaration, index + 1, declarations.len(), terminal)?;
            }
        }
        Ok(())
    }

    fn prompt_and_apply(
        config: &mut ActRunConfig,
        declaration: &crate::application::dtos::responses::RunInputDeclarationResponse,
        index: usize,
        total: usize,
        terminal: &dyn Terminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        *config = Self::prompt_for_input(
            std::mem::take(config),
            declaration,
            index + 1,
            total,
            terminal,
        )?;
        Ok(())
    }

    fn collect_missing_required(
        declarations: &[crate::application::dtos::responses::RunInputDeclarationResponse],
    ) -> Vec<&crate::application::dtos::responses::RunInputDeclarationResponse> {
        declarations
            .iter()
            .filter(|input| input.required() && !input.is_resolved())
            .collect()
    }

    fn missing_inputs_error(
        missing: &[&crate::application::dtos::responses::RunInputDeclarationResponse],
    ) -> Box<dyn std::error::Error> {
        format!(
            "required inputs missing: {}",
            missing
                .iter()
                .map(|input| input.name())
                .collect::<Vec<_>>()
                .join(", ")
        )
        .into()
    }

    fn should_prompt(
        declaration: &crate::application::dtos::responses::RunInputDeclarationResponse,
        interactive: bool,
    ) -> bool {
        interactive || (declaration.required() && !declaration.is_resolved())
    }
    fn describe_input(
        declaration: &crate::application::dtos::responses::RunInputDeclarationResponse,
        index: usize,
        total: usize,
        terminal: &dyn Terminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let description = declaration
            .description()
            .unwrap_or("No description provided.");
        let state = if declaration.is_resolved() {
            declaration
                .default()
                .map(|value| format!("default: {value}"))
                .unwrap_or_else(|| "already supplied".to_string())
        } else if declaration.required() {
            "required".to_string()
        } else {
            "optional".to_string()
        };
        terminal.write_text(&format!(
            "Input {index} of {total}\nName: {}\nDescription: {}\nSource: {}\nStatus: {state}\n",
            declaration.name(),
            description,
            declaration.source()
        ))?;
        Ok(())
    }

    fn prompt_for_input(
        config: ActRunConfig,
        declaration: &crate::application::dtos::responses::RunInputDeclarationResponse,
        index: usize,
        total: usize,
        terminal: &dyn Terminal,
    ) -> Result<ActRunConfig, Box<dyn std::error::Error>> {
        Self::describe_input(declaration, index, total, terminal)?;
        let value = Self::read_input_value(declaration, terminal)?;
        Self::apply_input_value(config, declaration, value)
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

    fn read_input_value(
        declaration: &crate::application::dtos::responses::RunInputDeclarationResponse,
        terminal: &dyn Terminal,
    ) -> Result<String, Box<dyn std::error::Error>> {
        terminal.write_text(&format!(
            "Value for {} (literal or env:VARIABLE; blank keeps the current/default value): ",
            declaration.name()
        ))?;
        let value = terminal.read_line()?.trim().to_owned();
        Self::validate_required_input(&value, declaration)?;
        Ok(value)
    }

    fn validate_required_input(
        value: &str,
        declaration: &crate::application::dtos::responses::RunInputDeclarationResponse,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if value.is_empty() && declaration.required() && !declaration.is_resolved() {
            return Err(format!("required input '{}' cannot be blank", declaration.name()).into());
        }
        Ok(())
    }

    fn apply_input_value(
        mut config: ActRunConfig,
        declaration: &crate::application::dtos::responses::RunInputDeclarationResponse,
        value: String,
    ) -> Result<ActRunConfig, Box<dyn std::error::Error>> {
        if !value.is_empty() {
            let (_, source) = crate::presentation::cli::RunArgs::parse_input_source(&format!(
                "{}={value}",
                declaration.name()
            ))?;
            config = config.add_input(crate::domain::value_objects::ActInput::new(
                declaration.name().to_owned(),
                source.resolve()?,
            ));
        }
        Ok(config)
    }

    async fn execute_async(
        config: ActRunConfig,
        repository: crate::domain::Repository,
        run_workflow_port: &dyn RunWorkflowPort,
        run_all_workflows_port: &dyn RunAllWorkflowsPort,
    ) -> Result<RunSummaryResponse, Box<dyn std::error::Error>> {
        if config.all_workflows() {
            let request = Self::build_run_all_request(&config, &repository);
            Ok(run_all_workflows_port.execute(request)?)
        } else {
            let request = Self::build_run_workflow_request(&config, &repository);
            Ok(run_workflow_port.execute(request).await?)
        }
    }

    fn build_run_all_request(
        config: &ActRunConfig,
        repository: &crate::domain::Repository,
    ) -> RunAllWorkflowsRequest {
        RunAllWorkflowsRequest::from_domain(repository, config)
    }

    fn build_run_workflow_request(
        config: &ActRunConfig,
        repository: &crate::domain::Repository,
    ) -> RunWorkflowRequest {
        RunWorkflowRequest::from_domain(repository, config)
    }

    pub fn render(summary: &RunSummaryResponse) -> String {
        RunSummaryComponent::new(summary).render()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::{
        application::dtos::responses::{
            JobSummaryResponse, StepSummaryDetails, StepSummaryResponse, StepSummaryResponseInput,
        },
        domain::value_objects::StepType,
    };

    fn job(job_id: &str, name: Option<&str>, success: bool) -> JobSummaryResponse {
        JobSummaryResponse::new(job_id, name.map(Into::into), vec![], success)
    }

    fn summary(
        success: bool,
        jobs: Vec<JobSummaryResponse>,
        duration: Duration,
    ) -> RunSummaryResponse {
        RunSummaryResponse::new("test", jobs, success, duration)
    }

    #[test]
    fn render_starts_with_summary_heading() {
        let rendered = Rendered::of(&summary(true, vec![], Duration::from_secs(5)));
        assert_eq!(rendered.line(0), "Summary");
    }

    #[test]
    fn render_lists_every_job_with_its_status_in_the_summary() {
        let rendered = Rendered::of(&summary(
            false,
            vec![
                job("build", Some("Build"), true),
                job("validate", None, false),
            ],
            Duration::ZERO,
        ));
        assert_eq!(rendered.line(0), "Summary");
        assert_eq!(rendered.line(1), "Workflow: test");
        assert_eq!(rendered.line(2), "  [ok] build (Build)");
        assert_eq!(rendered.line(3), "  [failed] validate");
    }
    #[test]
    fn render_includes_workflow_and_every_step_status() {
        let summary = RunSummaryResponse::new(
            "Build",
            vec![JobSummaryResponse::new(
                "compile",
                Some("Compile".to_string()),
                vec![
                    StepSummaryResponse::new(StepSummaryResponseInput::new(
                        "Checkout",
                        StepType::Run,
                        StepSummaryDetails::new(
                            Some(0),
                            false,
                            Duration::ZERO,
                            String::new(),
                            String::new(),
                        ),
                    )),
                    StepSummaryResponse::new(StepSummaryResponseInput::new(
                        "Build",
                        StepType::Run,
                        StepSummaryDetails::new(
                            Some(1),
                            false,
                            Duration::ZERO,
                            String::new(),
                            String::new(),
                        ),
                    )),
                ],
                false,
            )],
            false,
            Duration::ZERO,
        );

        let rendered = RunHandler::render(&summary);

        assert!(rendered.contains("Workflow: Build"));
        assert!(rendered.contains("[ok] Step 'Checkout'"));
        assert!(rendered.contains("[failed] Step 'Build'"));
    }

    #[test]
    fn render_reports_failed_step_status_without_output_details() {
        let summary = RunSummaryResponse::new(
            "test",
            vec![JobSummaryResponse::new(
                "lint",
                Some("Lint".to_string()),
                vec![StepSummaryResponse::new(StepSummaryResponseInput::new(
                    "Clippy",
                    StepType::Run,
                    StepSummaryDetails::new(
                        Some(101),
                        false,
                        Duration::ZERO,
                        String::new(),
                        "clippy failed".to_string(),
                    ),
                ))],
                false,
            )],
            false,
            Duration::from_secs(2),
        );

        let rendered = RunHandler::render(&summary);

        assert!(rendered.contains("[failed] Step 'Clippy'"));
        assert!(!rendered.contains("clippy failed"));
    }

    #[test]
    fn render_includes_summary_heading_without_jobs() {
        let rendered = Rendered::of(&summary(true, vec![], Duration::ZERO));
        assert_eq!(rendered.lines().count(), 2);
    }

    struct Rendered {
        text: String,
    }

    impl Rendered {
        fn of(summary: &RunSummaryResponse) -> Self {
            Self {
                text: RunHandler::render(summary),
            }
        }

        fn line(&self, index: usize) -> &str {
            self.text.lines().nth(index).unwrap()
        }

        fn lines(&self) -> std::str::Lines<'_> {
            self.text.lines()
        }
    }
}
