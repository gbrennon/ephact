use std::path::PathBuf;

use crate::{domain::errors::ActionError, dtos::requests::FetchRemoteActionRequest};

pub trait FetchRemoteActionPort: Send + Sync {
    fn fetch(&self, request: FetchRemoteActionRequest) -> Result<PathBuf, ActionError>;
}
