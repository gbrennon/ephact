#![allow(dead_code)]
use ephact::application::ports::outbound::read_step_exports_port::ReadStepExportsPort;
use parking_lot::Mutex;
use std::{collections::HashMap, sync::Arc};

use ephact::application::dtos::requests::ReadStepExportsRequest;
use ephact::application::dtos::responses::StepExportsResponse;

type QueuedStepExports = (Vec<String>, HashMap<String, String>);

/// Hands out the next queued set of exports, or nothing once drained.
#[derive(Clone, Default)]
pub struct FakeReadStepExportsPort {
    queued: Arc<Mutex<Vec<QueuedStepExports>>>,
    calls: Arc<Mutex<usize>>,
}

impl FakeReadStepExportsPort {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn queueing(exports: Vec<QueuedStepExports>) -> Self {
        Self {
            queued: Arc::new(Mutex::new(exports)),
            calls: Arc::new(Mutex::new(0)),
        }
    }

    pub fn calls(&self) -> usize {
        *self.calls.lock()
    }
}

impl ReadStepExportsPort for FakeReadStepExportsPort {
    fn execute(&self, _request: ReadStepExportsRequest<'_>) -> StepExportsResponse {
        *self.calls.lock() += 1;
        let mut queued = self.queued.lock();
        if queued.is_empty() {
            return StepExportsResponse::new(Vec::new(), HashMap::new());
        }
        let (path_additions, env) = queued.remove(0);
        StepExportsResponse::new(path_additions, env)
    }
}
