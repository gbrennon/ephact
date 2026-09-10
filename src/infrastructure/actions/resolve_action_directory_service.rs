use crate::application::dtos::requests::FetchRemoteActionRequest;
use crate::application::dtos::requests::ResolveActionDirectoryRequest;
use crate::application::dtos::responses::ExecuteActionResponse;
use crate::application::dtos::responses::ResolvedActionDirectoryResponse;
use crate::domain::errors::ActionError;
use crate::domain::errors::StepError;
use crate::domain::value_objects::ActionReference;
use crate::{
    application::ports::outbound::resolve_action_directory_port::ResolveActionDirectoryPort,
    infrastructure::{
        actions::fetch_remote_action_port::FetchRemoteActionPort,
        containers::workspace::CONTAINER_WORKSPACE,
    },
};

/// Repository name whose action only checks out the repository, which the
/// runner already provides by mounting the workspace.
const CHECKOUT_REPO: &str = "checkout";

/// Service that decides where the action a step references lives.
///
/// Local references resolve inside the repository under test, remote ones are
/// fetched from their forge, a checkout action needs no work because the
/// workspace is already mounted, and container actions are unsupported.
pub struct ResolveActionDirectoryService {
    remote_fetcher: Box<dyn FetchRemoteActionPort>,
}

impl ResolveActionDirectoryService {
    pub fn new(remote_fetcher: Box<dyn FetchRemoteActionPort>) -> Self {
        Self { remote_fetcher }
    }
}

impl ResolveActionDirectoryPort for ResolveActionDirectoryService {
    fn execute(
        &self,
        request: ResolveActionDirectoryRequest<'_>,
    ) -> Result<ResolvedActionDirectoryResponse, StepError> {
        let reference = ActionReference::parse(request.action_ref())
            .map_err(|error| StepError::new(error.to_string()))?;

        match &reference {
            ActionReference::Local(path) => Ok(ResolvedActionDirectoryResponse::Directory(
                request.repo_path().join(path.trim_start_matches("./")),
            )),
            ActionReference::Docker(image) => Err(StepError::new(
                ActionError::Unsupported(format!(
                    "container action '{image}' cannot be executed yet"
                ))
                .to_string(),
            )),
            ActionReference::Remote(remote) if remote.repo() == CHECKOUT_REPO => Ok(
                ResolvedActionDirectoryResponse::Skipped(ExecuteActionResponse::note(format!(
                    "[skipped] {} - the repository is already mounted at {CONTAINER_WORKSPACE}\n",
                    request.action_ref()
                ))),
            ),
            ActionReference::Remote(remote) => Ok(ResolvedActionDirectoryResponse::Directory(
                self.remote_fetcher
                    .execute(FetchRemoteActionRequest::new(remote))
                    .map_err(|error| StepError::new(error.to_string()))?,
            )),
        }
    }
}
