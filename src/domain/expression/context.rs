/// Evaluation context for `${{ }}` expressions.
///
/// Mirrors the GitHub Actions context hierarchy. Each field is a
/// `serde_json::Value` so the evaluator can traverse property and
/// index accesses naturally. Callers populate these from workflow
/// state before evaluation.
use serde_json::Value;

/// Holds all context data available during expression evaluation.
///
/// # Example
///
/// ```rust
/// use ephact::domain::expression::context::EvalContext;
///
/// let ctx = EvalContext::new();
/// assert!(ctx.github.is_object());
/// ```
#[derive(Debug, Clone)]
pub struct EvalContext {
    github: Value,
    env: Value,
    job: Value,
    steps: Value,
    runner: Value,
    secrets: Value,
    vars: Value,
    strategy: Value,
    matrix: Value,
    needs: Value,
    inputs: Value,
}

impl Default for EvalContext {
    fn default() -> Self {
        Self::new()
    }
}

impl EvalContext {
    /// Creates a new evaluation context with empty objects for every field.
    #[must_use]
    pub fn new() -> Self {
        let empty_obj = Value::Object(serde_json::Map::new());
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
    pub fn get(&self, name: &str) -> Option<&Value> {
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
    pub fn github(&self) -> &Value {
        &self.github
    }

    #[must_use]
    pub fn env(&self) -> &Value {
        &self.env
    }

    #[must_use]
    pub fn job(&self) -> &Value {
        &self.job
    }

    #[must_use]
    pub fn steps(&self) -> &Value {
        &self.steps
    }

    #[must_use]
    pub fn runner(&self) -> &Value {
        &self.runner
    }

    #[must_use]
    pub fn secrets(&self) -> &Value {
        &self.secrets
    }

    #[must_use]
    pub fn vars(&self) -> &Value {
        &self.vars
    }

    #[must_use]
    pub fn strategy(&self) -> &Value {
        &self.strategy
    }

    #[must_use]
    pub fn matrix(&self) -> &Value {
        &self.matrix
    }

    #[must_use]
    pub fn needs(&self) -> &Value {
        &self.needs
    }

    #[must_use]
    pub fn inputs(&self) -> &Value {
        &self.inputs
    }

    pub fn with_github(mut self, github: Value) -> Self {
        self.github = github;
        self
    }

    pub fn with_env(mut self, env: Value) -> Self {
        self.env = env;
        self
    }

    pub fn with_job(mut self, job: Value) -> Self {
        self.job = job;
        self
    }

    pub fn with_steps(mut self, steps: Value) -> Self {
        self.steps = steps;
        self
    }

    pub fn with_runner(mut self, runner: Value) -> Self {
        self.runner = runner;
        self
    }

    pub fn with_secrets(mut self, secrets: Value) -> Self {
        self.secrets = secrets;
        self
    }

    pub fn with_vars(mut self, vars: Value) -> Self {
        self.vars = vars;
        self
    }

    pub fn with_strategy(mut self, strategy: Value) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn with_matrix(mut self, matrix: Value) -> Self {
        self.matrix = matrix;
        self
    }

    pub fn with_needs(mut self, needs: Value) -> Self {
        self.needs = needs;
        self
    }

    pub fn with_inputs(mut self, inputs: Value) -> Self {
        self.inputs = inputs;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_empty_objects() {
        let ctx = EvalContext::new();
        assert!(ctx.github().is_object());
        assert!(ctx.env().is_object());
    }

    #[test]
    fn get_known_context() {
        let ctx = EvalContext::new();
        assert!(ctx.get("github").is_some());
        assert!(ctx.get("env").is_some());
    }

    #[test]
    fn get_unknown_context() {
        let ctx = EvalContext::new();
        assert!(ctx.get("nonexistent").is_none());
    }

    #[test]
    fn default_equals_new() {
        let ctx1 = EvalContext::new();
        let ctx2 = EvalContext::default();
        assert_eq!(ctx1.github, ctx2.github);
    }
}
