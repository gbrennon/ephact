use std::sync::Arc;

use ephact::application::{dtos::requests::PullJobImageRequest, ports::outbound::PullJobImagePort};
use parking_lot::Mutex;

#[derive(Clone)]
pub struct FakePullJobImagePort {
    result: Result<String, String>,
    requested_images: Arc<Mutex<Vec<String>>>,
}

impl FakePullJobImagePort {
    pub fn returning(image: &str) -> Self {
        Self {
            result: Ok(image.to_string()),
            requested_images: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn failing(message: &str) -> Self {
        Self {
            result: Err(message.to_string()),
            requested_images: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn requested_images(&self) -> Vec<String> {
        self.requested_images.lock().clone()
    }
}

impl PullJobImagePort for FakePullJobImagePort {
    fn pull(&self, request: PullJobImageRequest) -> Result<String, Box<dyn std::error::Error>> {
        self.requested_images
            .lock()
            .push(request.image().to_string());
        self.result.clone().map_err(Into::into)
    }
}
