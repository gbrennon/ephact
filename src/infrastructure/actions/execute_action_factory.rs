use std::sync::Arc;

use crate::application::ports::{inbound::ExecuteActionPort, outbound::ContainerPort};

pub type ExecuteActionFactory =
    Box<dyn Fn(Arc<dyn ContainerPort>) -> Box<dyn ExecuteActionPort> + Send + Sync>;
