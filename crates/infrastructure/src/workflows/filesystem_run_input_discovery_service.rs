use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use crate::{
    application::{
        dtos::{
            requests::DiscoverRunInputsRequest,
            responses::{RunInputDeclarationResponse, RunInputSourceResponse},
        },
        errors::DiscoverRunInputsError,
        ports::outbound::{RunInputsDiscovererPort, WorkflowSourcePort},
    },
    domain::{
        aggregates::Workflow,
        entities::Step,
        value_objects::{ActionDefinition, ActionInput, ActionRuntime, WorkflowRunConfig},
    },
    workflows::{actions::ActionDefinitionYaml, workflow_document::WorkflowDocument},
};

pub struct FilesystemRunInputDiscoveryService {
    workflow_source: Box<dyn WorkflowSourcePort>,
}

impl FilesystemRunInputDiscoveryService {
    pub fn new(workflow_source: Box<dyn WorkflowSourcePort>) -> Self {
        Self { workflow_source }
    }

    fn workflow_contents(
        &self,
        request: &DiscoverRunInputsRequest,
    ) -> Result<
        Vec<crate::application::dtos::responses::WorkflowSourceFileResponse>,
        DiscoverRunInputsError,
    > {
        if request.config().all_workflows() {
            return self
                .workflow_source
                .read_all_workflows_in_directory(
                    request.repository(),
                    request.config().workflow_directory(),
                )
                .map_err(DiscoverRunInputsError::WorkflowSource);
        }
        self.workflow_source
            .read_workflow_in_directory(
                request.repository(),
                request
                    .config()
                    .workflow()
                    .map(|workflow| workflow.as_str()),
                request.config().workflow_directory(),
            )
            .map(|workflow| vec![workflow])
            .map_err(DiscoverRunInputsError::WorkflowSource)
    }

    fn add_declaration(
        declarations: Vec<RunInputDeclarationResponse>,
        declaration: RunInputDeclarationResponse,
    ) -> Vec<RunInputDeclarationResponse> {
        if declarations
            .iter()
            .any(|item| item.name() == declaration.name())
        {
            return declarations;
        }
        declarations
            .into_iter()
            .chain(std::iter::once(declaration))
            .collect()
    }

    fn add_workflow_inputs(
        workflow: &Workflow,
        declarations: Vec<RunInputDeclarationResponse>,
    ) -> Result<Vec<RunInputDeclarationResponse>, DiscoverRunInputsError> {
        workflow
            .trigger()
            .iter()
            .try_fold(declarations, |declarations, trigger| {
                let Some(inputs) = trigger.inputs() else {
                    return Ok(declarations);
                };
                inputs
                    .iter()
                    .try_fold(declarations, |declarations, (name, input)| {
                        let declaration = RunInputDeclarationResponse::new(
                            name.to_owned(),
                            RunInputSourceResponse::Workflow,
                            input.description().map(str::to_owned),
                            input.required(),
                            input.default_value().map(str::to_owned),
                        )
                        .with_type(input.value_type().map(str::to_owned))
                        .with_options(input.options().to_vec());
                        Ok(Self::add_declaration(declarations, declaration))
                    })
            })
    }

    fn add_action_inputs(
        repository: &Path,
        action_reference: &str,
        declarations: Vec<RunInputDeclarationResponse>,
    ) -> Result<Vec<RunInputDeclarationResponse>, DiscoverRunInputsError> {
        let Some(relative_path) = Self::strip_local_prefix(action_reference) else {
            return Ok(declarations);
        };
        let action_dir = repository.join(relative_path);
        let action_path = Self::find_action_file(&action_dir)?;
        let definition = Self::load_action_definition(&action_path)?;
        let declarations =
            Self::add_input_declarations(definition.inputs(), action_reference, declarations);
        Self::process_composite_runs(repository, definition.runs(), declarations)
    }

    fn process_composite_runs(
        repository: &Path,
        runs: &ActionRuntime,
        declarations: Vec<RunInputDeclarationResponse>,
    ) -> Result<Vec<RunInputDeclarationResponse>, DiscoverRunInputsError> {
        match runs {
            ActionRuntime::Composite { steps } => {
                Self::recurse_composite_steps(repository, steps, declarations)
            }
            _ => Ok(declarations),
        }
    }

    fn strip_local_prefix(action_reference: &str) -> Option<&str> {
        action_reference.strip_prefix("./")
    }

    fn find_action_file(action_dir: &Path) -> Result<PathBuf, DiscoverRunInputsError> {
        [
            action_dir.join("action.yml"),
            action_dir.join("action.yaml"),
        ]
        .into_iter()
        .find(|path| path.is_file())
        .ok_or_else(|| {
            DiscoverRunInputsError::Discovery(format!(
                "action definition not found in {action_dir:?}"
            ))
        })
    }

    fn load_action_definition(
        action_path: &Path,
    ) -> Result<ActionDefinition, DiscoverRunInputsError> {
        let content = fs::read_to_string(action_path)
            .map_err(|error| DiscoverRunInputsError::Discovery(error.to_string()))?;
        serde_yaml::from_str::<ActionDefinitionYaml>(&content)
            .map(ActionDefinitionYaml::into_domain)
            .map_err(|error| DiscoverRunInputsError::Discovery(error.to_string()))
    }

    fn add_input_declarations(
        inputs: &HashMap<String, ActionInput>,
        action_reference: &str,
        declarations: Vec<RunInputDeclarationResponse>,
    ) -> Vec<RunInputDeclarationResponse> {
        inputs
            .iter()
            .fold(declarations, |declarations, (name, input)| {
                Self::add_declaration(
                    declarations,
                    RunInputDeclarationResponse::new(
                        name.clone(),
                        RunInputSourceResponse::Action(action_reference.to_owned()),
                        input.description().map(str::to_owned),
                        input.required(),
                        input.default().map(str::to_owned),
                    ),
                )
            })
    }

    fn recurse_composite_steps(
        repository: &Path,
        steps: &[Step],
        declarations: Vec<RunInputDeclarationResponse>,
    ) -> Result<Vec<RunInputDeclarationResponse>, DiscoverRunInputsError> {
        steps.iter().try_fold(declarations, |declarations, step| {
            let Some(reference) = step.uses() else {
                return Ok(declarations);
            };
            Self::add_action_inputs(repository, reference, declarations)
        })
    }

    fn add_actions(
        workflow: &Workflow,
        repository: &Path,
        declarations: Vec<RunInputDeclarationResponse>,
        provided: HashSet<String>,
    ) -> Result<(Vec<RunInputDeclarationResponse>, HashSet<String>), DiscoverRunInputsError> {
        workflow
            .jobs()
            .values()
            .flat_map(|job| job.steps())
            .try_fold(
                (declarations, provided),
                |(declarations, provided), step| {
                    let provided = step
                        .with()
                        .iter()
                        .filter(|(_, value)| Self::is_resolved_step_input(value))
                        .map(|(name, _)| name.clone())
                        .fold(provided, |provided, name| {
                            provided.into_iter().chain(std::iter::once(name)).collect()
                        });
                    let declarations = match step.uses() {
                        Some(reference) => {
                            Self::add_action_inputs(repository, reference, declarations)?
                        }
                        None => declarations,
                    };
                    Ok((declarations, provided))
                },
            )
    }

    fn is_resolved_step_input(value: &str) -> bool {
        let expression = value
            .trim()
            .strip_prefix("${{")
            .and_then(|value| value.strip_suffix("}}"));
        let Some(expression) = expression.map(str::trim) else {
            return true;
        };
        let Some(variable) = expression.strip_prefix("env.") else {
            return false;
        };
        std::env::var(variable.trim()).is_ok()
    }

    fn collect_supplied_inputs(config: &WorkflowRunConfig) -> HashMap<&str, &str> {
        config
            .inputs()
            .iter()
            .map(|input| (input.key(), input.value()))
            .collect()
    }

    fn process_workflows(
        sources: &[crate::application::dtos::responses::WorkflowSourceFileResponse],
        repository: &Path,
    ) -> Result<(Vec<RunInputDeclarationResponse>, HashSet<String>), DiscoverRunInputsError> {
        sources.iter().try_fold(
            (Vec::new(), HashSet::new()),
            |(declarations, provided), source| {
                let workflow = WorkflowDocument::parse(source.content())
                    .map(WorkflowDocument::into_domain)
                    .map(|workflow| workflow.with_file(source.file_name().to_owned()))
                    .map_err(|error| DiscoverRunInputsError::Discovery(error.to_string()))?;
                let declarations = Self::add_workflow_inputs(&workflow, declarations)?;
                Self::add_actions(&workflow, repository, declarations, provided)
            },
        )
    }

    fn resolve_declarations(
        declarations: Vec<RunInputDeclarationResponse>,
        supplied: HashMap<&str, &str>,
        provided: HashSet<String>,
    ) -> Vec<RunInputDeclarationResponse> {
        declarations
            .into_iter()
            .map(|input| {
                let resolved =
                    supplied.contains_key(input.name()) || provided.contains(input.name());
                input.with_resolved(resolved)
            })
            .collect()
    }
}

impl RunInputsDiscovererPort for FilesystemRunInputDiscoveryService {
    fn discover(
        &self,
        request: DiscoverRunInputsRequest,
    ) -> Result<Vec<RunInputDeclarationResponse>, DiscoverRunInputsError> {
        let sources = self.workflow_contents(&request)?;
        let supplied = Self::collect_supplied_inputs(request.config());
        let repo_path = request.repository().path().as_path();
        let (declarations, provided) = Self::process_workflows(&sources, repo_path)?;
        Ok(Self::resolve_declarations(declarations, supplied, provided))
    }
}
