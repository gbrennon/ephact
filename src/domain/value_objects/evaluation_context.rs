/// Evaluation context for `${{ }}` expressions.
///
/// Mirrors the GitHub Actions context hierarchy. Each field is a
/// [`ContextValue`] so the evaluator can traverse property and
/// index accesses naturally. Callers populate these from workflow
/// state before evaluation.
use crate::domain::value_objects::ContextValue;

/// Holds all context data available during expression evaluation.
///
/// # Example
///
/// ```rust
/// use ephact::domain::value_objects::EvaluationContext;
///
/// let ctx = EvaluationContext::new();
/// assert!(ctx.github().is_mapping());
/// ```
#[derive(Debug, Clone)]
pub struct EvaluationContext {
    github: ContextValue,
    env: ContextValue,
    job: ContextValue,
    steps: ContextValue,
    runner: ContextValue,
    secrets: ContextValue,
    vars: ContextValue,
    strategy: ContextValue,
    matrix: ContextValue,
    needs: ContextValue,
    inputs: ContextValue,
}

impl Default for EvaluationContext {
    fn default() -> Self {
        Self::new()
    }
}

impl EvaluationContext {
    /// Creates a new evaluation context with empty objects for every field.
    #[must_use]
    pub fn new() -> Self {
        let empty_obj = ContextValue::empty_mapping();
        Self {
            github: empty_obj.clone(),
            env: empty_obj.clone(),
            job: empty_obj.clone(),
            steps: empty_obj.clone(),
            runner: empty_obj.clone(),
            secrets: empty_obj.clone(),
            vars: empty_obj.clone(),
            strategy: empty_obj.clone(),
            matrix: empty_obj.clone(),
            needs: empty_obj.clone(),
            inputs: empty_obj,
        }
    }

    /// Looks up a top-level context variable by name.
    ///
    /// Returns `None` if the name does not match any known context.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&ContextValue> {
        match name {
            "github" => Some(&self.github),
            "env" => Some(&self.env),
            "job" => Some(&self.job),
            "steps" => Some(&self.steps),
            "runner" => Some(&self.runner),
            "secrets" => Some(&self.secrets),
            "vars" => Some(&self.vars),
            "strategy" => Some(&self.strategy),
            "matrix" => Some(&self.matrix),
            "needs" => Some(&self.needs),
            "inputs" => Some(&self.inputs),
            _ => None,
        }
    }

    #[must_use]
    pub fn github(&self) -> &ContextValue {
        &self.github
    }

    #[must_use]
    pub fn env(&self) -> &ContextValue {
        &self.env
    }

    #[must_use]
    pub fn job(&self) -> &ContextValue {
        &self.job
    }

    #[must_use]
    pub fn steps(&self) -> &ContextValue {
        &self.steps
    }

    #[must_use]
    pub fn runner(&self) -> &ContextValue {
        &self.runner
    }

    #[must_use]
    pub fn secrets(&self) -> &ContextValue {
        &self.secrets
    }

    #[must_use]
    pub fn vars(&self) -> &ContextValue {
        &self.vars
    }

    #[must_use]
    pub fn strategy(&self) -> &ContextValue {
        &self.strategy
    }

    #[must_use]
    pub fn matrix(&self) -> &ContextValue {
        &self.matrix
    }

    #[must_use]
    pub fn needs(&self) -> &ContextValue {
        &self.needs
    }

    #[must_use]
    pub fn inputs(&self) -> &ContextValue {
        &self.inputs
    }

    pub fn with_github(mut self, github: ContextValue) -> Self {
        self.github = github;
        self
    }

    pub fn with_env(mut self, env: ContextValue) -> Self {
        self.env = env;
        self
    }

    pub fn with_job(mut self, job: ContextValue) -> Self {
        self.job = job;
        self
    }

    pub fn with_steps(mut self, steps: ContextValue) -> Self {
        self.steps = steps;
        self
    }

    pub fn with_runner(mut self, runner: ContextValue) -> Self {
        self.runner = runner;
        self
    }

    pub fn with_secrets(mut self, secrets: ContextValue) -> Self {
        self.secrets = secrets;
        self
    }

    pub fn with_vars(mut self, vars: ContextValue) -> Self {
        self.vars = vars;
        self
    }

    pub fn with_strategy(mut self, strategy: ContextValue) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn with_matrix(mut self, matrix: ContextValue) -> Self {
        self.matrix = matrix;
        self
    }

    pub fn with_needs(mut self, needs: ContextValue) -> Self {
        self.needs = needs;
        self
    }

    pub fn with_inputs(mut self, inputs: ContextValue) -> Self {
        self.inputs = inputs;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_empty_objects() {
        let ctx = EvaluationContext::new();
        assert!(ctx.github().is_mapping());
        assert!(ctx.env().is_mapping());
    }

    #[test]
    fn get_known_context() {
        let ctx = EvaluationContext::new();
        assert!(ctx.get("github").is_some());
        assert!(ctx.get("env").is_some());
    }

    #[test]
    fn get_unknown_context() {
        let ctx = EvaluationContext::new();
        assert!(ctx.get("nonexistent").is_none());
    }

    #[test]
    fn default_equals_new() {
        let ctx1 = EvaluationContext::new();
        let ctx2 = EvaluationContext::default();
        assert_eq!(ctx1.github, ctx2.github);
    }
    #[test]
    fn accessors_and_builders_preserve_values() {
        let value = ContextValue::text("value");
        let context = EvaluationContext::new()
            .with_github(value.clone())
            .with_env(value.clone())
            .with_job(value.clone())
            .with_steps(value.clone())
            .with_runner(value.clone())
            .with_secrets(value.clone())
            .with_vars(value.clone())
            .with_strategy(value.clone())
            .with_matrix(value.clone())
            .with_needs(value.clone())
            .with_inputs(value.clone());

        assert_eq!(context.github(), &value);
        assert_eq!(context.env(), &value);
        assert_eq!(context.job(), &value);
        assert_eq!(context.steps(), &value);
        assert_eq!(context.runner(), &value);
        assert_eq!(context.secrets(), &value);
        assert_eq!(context.vars(), &value);
        assert_eq!(context.strategy(), &value);
        assert_eq!(context.matrix(), &value);
        assert_eq!(context.needs(), &value);
        assert_eq!(context.inputs(), &value);
    }
}
