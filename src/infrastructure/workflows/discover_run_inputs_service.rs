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
        ports::outbound::{DiscoverRunInputsPort, WorkflowSourcePort},
    },
    domain::{
        aggregates::Workflow,
        entities::Step,
        value_objects::{ActRunConfig, ActionDefinition, ActionInput, ActionRuntime},
    },
    infrastructure::workflows::yaml::{ActionDefinitionYaml, WorkflowYaml},
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
    ) -> Result<Vec<String>, DiscoverRunInputsError> {
        if request.config().all_workflows() {
            return self
                .workflow_source
                .read_all_workflows(request.repository())
                .map_err(DiscoverRunInputsError::WorkflowSource);
        }
        self.workflow_source
            .read_workflow(
                request.repository(),
                request
                    .config()
                    .workflow()
                    .map(|workflow| workflow.as_str()),
            )
            .map(|workflow| vec![workflow])
            .map_err(DiscoverRunInputsError::WorkflowSource)
    }

    fn add_declaration(
        declarations: &mut Vec<RunInputDeclarationResponse>,
        declaration: RunInputDeclarationResponse,
    ) {
        if declarations
            .iter()
            .any(|item| item.name() == declaration.name())
        {
            return;
        }
        declarations.push(declaration);
    }

    fn add_workflow_inputs(
        workflow: &Workflow,
        declarations: &mut Vec<RunInputDeclarationResponse>,
    ) -> Result<(), DiscoverRunInputsError> {
        for trigger in workflow.trigger() {
            let Some(inputs) = trigger.inputs() else {
                continue;
            };
            for (name, input) in inputs {
                Self::add_declaration(
                    declarations,
                    RunInputDeclarationResponse::new(
                        name,
                        RunInputSourceResponse::Workflow,
                        input.description().map(str::to_owned),
                        input.required(),
                        input.default_value().map(str::to_owned),
                    )
                    .with_type(input.value_type().map(str::to_owned))
                    .with_options(input.options().to_vec()),
                );
            }
        }
        Ok(())
    }

    fn add_action_inputs(
        repository: &Path,
        action_reference: &str,
        declarations: &mut Vec<RunInputDeclarationResponse>,
    ) -> Result<(), DiscoverRunInputsError> {
        let Some(relative_path) = Self::strip_local_prefix(action_reference) else {
            return Ok(());
        };
        let action_dir = repository.join(relative_path);
        let action_path = Self::find_action_file(&action_dir)?;
        let definition = Self::load_action_definition(&action_path)?;
        Self::add_input_declarations(definition.inputs(), action_reference, declarations);
        Self::process_composite_runs(repository, definition.runs(), declarations)?;
        Ok(())
    }

    fn process_composite_runs(
        repository: &Path,
        runs: &ActionRuntime,
        declarations: &mut Vec<RunInputDeclarationResponse>,
    ) -> Result<(), DiscoverRunInputsError> {
        if let ActionRuntime::Composite { steps } = runs {
            Self::recurse_composite_steps(repository, steps, declarations)?;
        }
        Ok(())
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
        declarations: &mut Vec<RunInputDeclarationResponse>,
    ) {
        for (name, input) in inputs {
            Self::add_declaration(
                declarations,
                RunInputDeclarationResponse::new(
                    name.clone(),
                    RunInputSourceResponse::Action(action_reference.to_owned()),
                    input.description().map(str::to_owned),
                    input.required(),
                    input.default().map(str::to_owned),
                ),
            );
        }
    }

    fn recurse_composite_steps(
        repository: &Path,
        steps: &[Step],
        declarations: &mut Vec<RunInputDeclarationResponse>,
    ) -> Result<(), DiscoverRunInputsError> {
        for step in steps {
            if let Some(reference) = step.uses() {
                Self::add_action_inputs(repository, reference, declarations)?;
            }
        }
        Ok(())
    }

    fn add_actions(
        workflow: &Workflow,
        repository: &Path,
        declarations: &mut Vec<RunInputDeclarationResponse>,
        provided: &mut HashSet<String>,
    ) -> Result<(), DiscoverRunInputsError> {
        for job in workflow.jobs().values() {
            for step in job.steps() {
                provided.extend(
                    step.with()
                        .iter()
                        .filter(|(_, value)| Self::is_resolved_step_input(value))
                        .map(|(name, _)| name.clone()),
                );
                if let Some(reference) = step.uses() {
                    Self::add_action_inputs(repository, reference, declarations)?;
                }
            }
        }
        Ok(())
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

    fn collect_supplied_inputs(config: &ActRunConfig) -> HashMap<&str, &str> {
        config
            .inputs()
            .iter()
            .map(|input| (input.key(), input.value()))
            .collect()
    }

    fn process_workflows(
        contents: &[String],
        repository: &Path,
    ) -> Result<(Vec<RunInputDeclarationResponse>, HashSet<String>), DiscoverRunInputsError> {
        let mut declarations = Vec::new();
        let mut provided = HashSet::new();
        for content in contents {
            let workflow = serde_yaml::from_str::<WorkflowYaml>(content)
                .map(WorkflowYaml::into_domain)
                .map_err(|error| DiscoverRunInputsError::Discovery(error.to_string()))?;
            Self::add_workflow_inputs(&workflow, &mut declarations)?;
            Self::add_actions(&workflow, repository, &mut declarations, &mut provided)?;
        }
        Ok((declarations, provided))
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

impl DiscoverRunInputsPort for FilesystemRunInputDiscoveryService {
    fn execute(
        &self,
        request: DiscoverRunInputsRequest,
    ) -> Result<Vec<RunInputDeclarationResponse>, DiscoverRunInputsError> {
        let contents = self.workflow_contents(&request)?;
        let supplied = Self::collect_supplied_inputs(request.config());
        let repo_path = request.repository().path().as_path();
        let (declarations, provided) = Self::process_workflows(&contents, repo_path)?;
        Ok(Self::resolve_declarations(declarations, supplied, provided))
    }
}
