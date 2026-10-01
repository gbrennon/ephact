use crate::dtos::requests::ResolveNodeBinaryRequest;

pub trait ResolveNodeBinaryPort: Send + Sync {
    fn resolve(&self, request: ResolveNodeBinaryRequest) -> String;
}
