use std::{error::Error, sync::Arc};

use crate::application::{
    dtos::requests::PullJobImageRequest,
    ports::outbound::{ContainerRuntimePort, PullJobImagePort},
};

/// Pulls exactly the image requested by generic container preparation.
pub struct PullJobImageService {
    runtime: Arc<dyn ContainerRuntimePort>,
}

impl PullJobImageService {
    /// Creates a service that performs no platform translation or fallback.
    pub fn new(runtime: Arc<dyn ContainerRuntimePort>) -> Self {
        Self { runtime }
    }
}

impl PullJobImagePort for PullJobImageService {
    fn pull(&self, request: PullJobImageRequest) -> Result<String, Box<dyn Error>> {
        self.runtime
            .pull_image(request.image(), None)
            .map_err(|error| format!("{error:?}"))?;
        Ok(request.image().to_string())
    }
}
