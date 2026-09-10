use std::{collections::HashMap, error::Error, fs, path::Path};

use crate::{
    application::{
        dtos::{DiscoverRunInputsRequest, RunInputDeclaration, RunInputSource},
        ports::outbound::{DiscoverRunInputsPort, WorkflowSourcePort},
    },
    domain::workflow::{ActionDefinition, ActionRuns, Workflow},
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
    ) -> Result<Vec<String>, Box<dyn Error>> {
        if request.config().all_workflows() {
            return self
                .workflow_source
                .read_all_workflows(request.repository());
        }
        Ok(vec![
            self.workflow_source.read_workflow(
                request.repository(),
                request
                    .config()
                    .workflow()
                    .map(|workflow| workflow.as_str()),
            )?,
        ])
    }

    fn add_declaration(
        declarations: &mut Vec<RunInputDeclaration>,
        declaration: RunInputDeclaration,
    ) -> Result<(), Box<dyn Error>> {
        if declarations
            .iter()
            .any(|item| item.name() == declaration.name())
        {
            return Ok(());
        }
        declarations.push(declaration);
        Ok(())
    }

    fn add_workflow_inputs(
        workflow: &Workflow,
        declarations: &mut Vec<RunInputDeclaration>,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(inputs) = workflow.on().workflow_dispatch_inputs() {
            for (name, input) in inputs {
                Self::add_declaration(
                    declarations,
                    RunInputDeclaration::new(
                        name,
                        RunInputSource::Workflow,
                        input.description().map(str::to_owned),
                        input.required(),
                        input.default().map(str::to_owned),
                    ),
                )?;
            }
        }
        Ok(())
    }

    fn add_action_inputs(
        repository: &Path,
        action_reference: &str,
        declarations: &mut Vec<RunInputDeclaration>,
    ) -> Result<(), Box<dyn Error>> {
        let Some(relative_path) = action_reference.strip_prefix("./") else {
            return Ok(());
        };
        let action_dir = repository.join(relative_path);
        let action_path = [
            action_dir.join("action.yml"),
            action_dir.join("action.yaml"),
        ]
        .into_iter()
        .find(|path| path.is_file())
        .ok_or_else(|| format!("action definition not found for '{action_reference}'"))?;
        let definition: ActionDefinition = serde_yaml::from_str(&fs::read_to_string(action_path)?)?;
        for (name, input) in definition.inputs() {
            Self::add_declaration(
                declarations,
                RunInputDeclaration::new(
                    name,
                    RunInputSource::Action(action_reference.to_owned()),
                    input.description().map(str::to_owned),
                    input.required(),
                    input.default().map(str::to_owned),
                ),
            )?;
        }
        if let ActionRuns::Composite { steps } = definition.runs() {
            for step in steps {
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

    fn add_actions(
        workflow: &Workflow,
        repository: &Path,
        declarations: &mut Vec<RunInputDeclaration>,
        provided: &mut std::collections::HashSet<String>,
    ) -> Result<(), Box<dyn Error>> {
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
}

impl DiscoverRunInputsPort for FilesystemRunInputDiscoveryService {
    fn execute(
        &self,
        request: DiscoverRunInputsRequest,
    ) -> Result<Vec<RunInputDeclaration>, Box<dyn Error>> {
        let contents = self.workflow_contents(&request)?;
        let supplied: HashMap<&str, &str> = request
            .config()
            .inputs()
            .iter()
            .map(|input| (input.key(), input.value()))
            .collect();
        let mut declarations = Vec::new();
        let mut provided = std::collections::HashSet::new();
        for content in contents {
            let workflow: Workflow = serde_yaml::from_str(&content)?;
            Self::add_workflow_inputs(&workflow, &mut declarations)?;
            Self::add_actions(
                &workflow,
                request.repository().path().as_path(),
                &mut declarations,
                &mut provided,
            )?;
        }
        Ok(declarations
            .into_iter()
            .map(|input| {
                let resolved =
                    supplied.contains_key(input.name()) || provided.contains(input.name());
                input.with_resolved(resolved)
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::domain::{RepoPath, Repository, RepositoryName, value_objects::ActRunConfig};

    #[test]
    fn discovers_required_workflow_and_local_action_inputs() {
        let temporary = tempfile::tempdir().unwrap();
        fs::create_dir(temporary.path().join(".git")).unwrap();
        let workflow_dir = temporary.path().join(".forgejo/workflows");
        let action_dir = temporary.path().join(".forgejo/actions/check");
        fs::create_dir_all(&workflow_dir).unwrap();
        fs::create_dir_all(&action_dir).unwrap();
        fs::write(
            workflow_dir.join("ci.yml"),
            "name: CI\non:\n  workflow_dispatch:\n    inputs:\n      workflow_name:\n        required: true\n      optional:\n        required: false\n        default: default-value\njobs:\n  test:\n    steps:\n      - uses: ./.forgejo/actions/check\n",
        )
        .unwrap();
        fs::write(
            action_dir.join("action.yml"),
            "name: Check\ninputs:\n  action_name:\n    required: true\nruns:\n  using: composite\n  steps:\n    - run: echo check\n",
        )
        .unwrap();
        let repository = Repository::new(
            RepoPath::new(temporary.path().to_path_buf()).unwrap(),
            RepositoryName::new("test".to_string()).unwrap(),
        );
        let service = FilesystemRunInputDiscoveryService::new(Box::new(
            super::super::FilesystemWorkflowSource::new(&[".forgejo/workflows"]),
        ));
        let declarations = service
            .execute(DiscoverRunInputsRequest::new(
                ActRunConfig::new(),
                repository,
            ))
            .unwrap();
        let mut names = declarations
            .iter()
            .map(RunInputDeclaration::name)
            .collect::<Vec<_>>();
        names.sort_unstable();
        assert_eq!(names, vec!["action_name", "optional", "workflow_name"]);
    }
}
