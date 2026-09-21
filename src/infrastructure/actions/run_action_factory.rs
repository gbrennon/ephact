use std::sync::Arc;

use crate::application::ports::{inbound::RunActionPort, outbound::ContainerPort};

pub type RunActionFactory =
    Box<dyn Fn(Arc<dyn ContainerPort>) -> Box<dyn RunActionPort> + Send + Sync>;
