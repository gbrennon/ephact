use crate::dtos::requests::ResolveNodeBinaryRequest;

/// Resolves the Node.js executable available in a container.
pub trait ResolveNodeBinaryPort: Send + Sync {
    /// Returns the path reported by `command -v node`, or `node` when no
    /// executable path is available.
    fn resolve(&self, request: ResolveNodeBinaryRequest) -> String;
}
