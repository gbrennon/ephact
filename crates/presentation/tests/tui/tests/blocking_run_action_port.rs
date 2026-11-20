use std::sync::{Mutex, mpsc};

use ephact::application::{
    dtos::{requests::RunActionRequest, responses::ExecuteActionResponse},
    errors::RunActionError,
    ports::inbound::RunActionPort,
};

pub struct BlockingRunActionPort {
    started: Mutex<Option<mpsc::Sender<()>>>,
    release: Mutex<mpsc::Receiver<()>>,
}

impl BlockingRunActionPort {
    pub fn new(started: mpsc::Sender<()>, release: mpsc::Receiver<()>) -> Self {
        Self {
            started: Mutex::new(Some(started)),
            release: Mutex::new(release),
        }
    }
}

impl RunActionPort for BlockingRunActionPort {
    fn execute(&self, _request: RunActionRequest) -> Result<ExecuteActionResponse, RunActionError> {
        self.started
            .lock()
            .expect("started lock")
            .take()
            .expect("started sender")
            .send(())
            .expect("started receiver");
        self.release
            .lock()
            .expect("release lock")
            .recv()
            .expect("release signal");
        Ok(ExecuteActionResponse::note("action completed".to_owned()))
    }
}
