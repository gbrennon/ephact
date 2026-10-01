use crate::application::dtos::requests::ResolveNodeBinaryRequest;

/// Inbound port for finding the node interpreter inside a container.
pub trait ResolveNodeBinaryPort: Send + Sync {
    fn execute(&self, request: ResolveNodeBinaryRequest) -> String;
}
