use super::{input_declaration_context::InputDeclarationContext, run_args::RunArgs};
use crate::{
    components::terminal::Terminal,
    domain::value_objects::{WorkflowInput, WorkflowRunConfig},
};

pub struct InputCollector<'a> {
    terminal: &'a dyn Terminal,
}

impl<'a> InputCollector<'a> {
    pub fn new(terminal: &'a dyn Terminal) -> Self {
        Self { terminal }
    }

    pub fn collect_inputs(
        &self,
        mut config: WorkflowRunConfig,
    ) -> Result<WorkflowRunConfig, Box<dyn std::error::Error>> {
        self.terminal.write_text(
            "\nAdditional inputs (optional)\nEnter KEY=VALUE, KEY=env:VARIABLE, or a blank line to continue.\nInput: ",
        )?;
        while let Some(input) = self.read_interactive_input()? {
            config = config.add_input(input);
            self.terminal.write_text("Input: ")?;
        }
        Ok(config)
    }

    pub fn collect_declared_inputs(
        &self,
        config: &mut WorkflowRunConfig,
        declarations: &[crate::application::dtos::responses::RunInputDeclarationResponse],
        interactive: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.fail_if_missing_required_noninteractive(declarations, interactive)?;
        self.terminal.write_text("\nInputs\n")?;
        for (index, declaration) in declarations.iter().enumerate() {
            let context = InputDeclarationContext::new(
                declaration,
                index + 1,
                declarations.len(),
                interactive,
            );
            *config = self.collect_declared_input(config.clone(), context)?;
        }
        Ok(())
    }

    fn collect_declared_input(
        &self,
        config: WorkflowRunConfig,
        context: InputDeclarationContext<'_>,
    ) -> Result<WorkflowRunConfig, Box<dyn std::error::Error>> {
        if self.should_prompt(context.declaration(), context.interactive()) {
            self.prompt_for_input(
                config,
                context.declaration(),
                context.index(),
                context.total(),
            )
        } else {
            self.describe_input(context.declaration(), context.index(), context.total())?;
            Ok(config)
        }
    }

    pub fn should_skip_prompting(
        &self,
        declarations: &[crate::application::dtos::responses::RunInputDeclarationResponse],
        interactive: bool,
    ) -> bool {
        declarations.is_empty()
            || (!interactive && self.collect_missing_required(declarations).is_empty())
    }

    fn read_interactive_input(&self) -> Result<Option<WorkflowInput>, Box<dyn std::error::Error>> {
        let line = self.terminal.read_line()?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return Ok(None);
        }
        let (key, source) = RunArgs::parse_input_source(trimmed)?;
        let value = source.resolve()?;
        Ok(Some(WorkflowInput::new(key, value)))
    }

    fn fail_if_missing_required_noninteractive(
        &self,
        declarations: &[crate::application::dtos::responses::RunInputDeclarationResponse],
        interactive: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let missing = self.collect_missing_required(declarations);
        if !interactive && !self.terminal.is_interactive() && !missing.is_empty() {
            return Err(self.missing_inputs_error(&missing));
        }
        Ok(())
    }

    fn collect_missing_required<'b>(
        &self,
        declarations: &'b [crate::application::dtos::responses::RunInputDeclarationResponse],
    ) -> Vec<&'b crate::application::dtos::responses::RunInputDeclarationResponse> {
        declarations
            .iter()
            .filter(|input| input.required() && !input.is_resolved())
            .collect()
    }

    fn missing_inputs_error(
        &self,
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
        &self,
        declaration: &crate::application::dtos::responses::RunInputDeclarationResponse,
        interactive: bool,
    ) -> bool {
        interactive || (declaration.required() && !declaration.is_resolved())
    }

    fn describe_input(
        &self,
        declaration: &crate::application::dtos::responses::RunInputDeclarationResponse,
        index: usize,
        total: usize,
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
        self.terminal.write_text(&format!(
            "Input {index} of {total}\nName: {}\nDescription: {}\nSource: {}\nStatus: {state}\n",
            declaration.name(),
            description,
            declaration.source()
        ))?;
        Ok(())
    }

    fn prompt_for_input(
        &self,
        config: WorkflowRunConfig,
        declaration: &crate::application::dtos::responses::RunInputDeclarationResponse,
        index: usize,
        total: usize,
    ) -> Result<WorkflowRunConfig, Box<dyn std::error::Error>> {
        self.describe_input(declaration, index, total)?;
        let value = self.read_input_value(declaration)?;
        self.apply_input_value(config, declaration, value)
    }

    fn read_input_value(
        &self,
        declaration: &crate::application::dtos::responses::RunInputDeclarationResponse,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.terminal.write_text(&format!(
            "Value for {} (literal or env:VARIABLE; blank keeps the current/default value): ",
            declaration.name()
        ))?;
        let value = self.terminal.read_line()?.trim().to_owned();
        self.validate_required_input(&value, declaration)?;
        Ok(value)
    }

    fn validate_required_input(
        &self,
        value: &str,
        declaration: &crate::application::dtos::responses::RunInputDeclarationResponse,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if value.is_empty() && declaration.required() && !declaration.is_resolved() {
            return Err(format!("required input '{}' cannot be blank", declaration.name()).into());
        }
        Ok(())
    }

    fn apply_input_value(
        &self,
        mut config: WorkflowRunConfig,
        declaration: &crate::application::dtos::responses::RunInputDeclarationResponse,
        value: String,
    ) -> Result<WorkflowRunConfig, Box<dyn std::error::Error>> {
        if !value.is_empty() {
            let (_, source) =
                RunArgs::parse_input_source(&format!("{}={value}", declaration.name()))?;
            config = config.add_input(WorkflowInput::new(
                declaration.name().to_owned(),
                source.resolve()?,
            ));
        }
        Ok(config)
    }
}
