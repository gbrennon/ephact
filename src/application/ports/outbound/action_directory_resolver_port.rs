use crate::{
    application::dtos::{
        requests::ResolveActionDirectoryRequest, responses::ResolvedActionDirectoryResponse,
    },
    domain::errors::StepError,
};

/// Resolves an action reference to its directory.
pub trait ActionDirectoryResolverPort: Send + Sync {
    /// Classifies the reference and resolves it to a directory.
    fn resolve(
        &self,
        request: ResolveActionDirectoryRequest,
    ) -> Result<ResolvedActionDirectoryResponse, StepError>;
}
