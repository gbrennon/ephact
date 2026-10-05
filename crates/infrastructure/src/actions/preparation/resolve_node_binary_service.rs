use std::collections::HashMap;

use crate::application::{
    dtos::requests::ResolveNodeBinaryRequest, ports::outbound::ResolveNodeBinaryPort,
};

/// Interpreter used for JavaScript actions when the container exposes no
/// absolute path for it.
const NODE_COMMAND: &str = "node";

/// Resolves the command used to run a JavaScript action in a container.
///
/// Returns the absolute `node` path reported by the container when available;
/// otherwise returns `node` so eventual execution reports a missing
/// interpreter.
pub struct ResolveNodeBinaryService;

impl ResolveNodeBinaryService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ResolveNodeBinaryService {
    fn default() -> Self {
        Self::new()
    }
}

impl ResolveNodeBinaryPort for ResolveNodeBinaryService {
    fn resolve(&self, request: ResolveNodeBinaryRequest) -> String {
        request
            .container()
            .exec(
                &["bash".into(), "-lc".into(), "command -v node".into()],
                None,
                &HashMap::new(),
            )
            .ok()
            .filter(|result| result.exit_code() == 0)
            .map(|result| result.stdout().trim().to_string())
            .filter(|path| !path.is_empty())
            .unwrap_or_else(|| NODE_COMMAND.to_string())
    }
}
