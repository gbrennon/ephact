use crate::dtos::{
    requests::BuildActionInputEnvironmentRequest, responses::BuildActionInputEnvironmentResponse,
};

/// Builds the environment variables used when executing an action.
pub trait BuildActionInputEnvironmentPort: Send + Sync {
    /// Starts with the request environment, adds `GITHUB_ACTION_PATH`, and
    /// maps each input to an `INPUT_` variable.
    /// Input names are uppercased and spaces are replaced with underscores.
    fn build(
        &self,
        request: BuildActionInputEnvironmentRequest,
    ) -> BuildActionInputEnvironmentResponse;
}
