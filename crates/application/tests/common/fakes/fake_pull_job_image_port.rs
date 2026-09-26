use ephact::{
    application::dtos::requests::PullJobImageRequest, infrastructure::containers::PullJobImagePort,
};
use parking_lot::Mutex;

/// Returns a prepared image, recording the runner labels it was asked about.
pub struct FakePullJobImagePort {
    result: Result<String, String>,
    pub requested_labels: Mutex<Vec<Option<String>>>,
}

impl FakePullJobImagePort {
    pub fn returning(image: &str) -> Self {
        Self {
            result: Ok(image.to_string()),
            requested_labels: Mutex::new(Vec::new()),
        }
    }

    pub fn failing(message: &str) -> Self {
        Self {
            result: Err(message.to_string()),
            requested_labels: Mutex::new(Vec::new()),
        }
    }
}

impl PullJobImagePort for FakePullJobImagePort {
    fn execute(&self, request: PullJobImageRequest) -> Result<String, Box<dyn std::error::Error>> {
        self.requested_labels
            .lock()
            .push(request.runs_on().map(str::to_string));
        self.result.clone().map_err(Into::into)
    }
}
