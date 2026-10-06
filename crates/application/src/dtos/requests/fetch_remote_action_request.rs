use crate::domain::value_objects::RemoteActionReference;

/// Request data for fetching a remote action reference.
pub struct FetchRemoteActionRequest {
    reference: RemoteActionReference,
}

impl FetchRemoteActionRequest {
    /// Creates a request from a remote action reference.
    pub fn new(reference: RemoteActionReference) -> Self {
        Self { reference }
    }

    /// Returns the remote action reference.
    pub fn reference(&self) -> &RemoteActionReference {
        &self.reference
    }
}
