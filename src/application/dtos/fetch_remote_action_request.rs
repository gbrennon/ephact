use crate::domain::value_objects::RemoteActionReference;

/// Request DTO for the
/// [`FetchRemoteActionPort`](crate::application::ports::inbound::fetch_remote_action_port::FetchRemoteActionPort)
/// inbound port.
pub struct FetchRemoteActionRequest<'a> {
    /// Reference naming the action to retrieve.
    pub reference: &'a RemoteActionReference,
}

impl<'a> FetchRemoteActionRequest<'a> {
    /// Creates a new request.
    pub fn new(reference: &'a RemoteActionReference) -> Self {
        Self { reference }
    }

    /// Reference naming the action to retrieve.
    pub fn reference(&self) -> &'a RemoteActionReference {
        self.reference
    }
}
