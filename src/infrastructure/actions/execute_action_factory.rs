use crate::application::ports::inbound::ExecuteActionPort;
use crate::application::ports::outbound::ContainerPort;
use std::sync::Arc;

pub type ExecuteActionFactory =
    Box<dyn Fn(Arc<dyn ContainerPort>) -> Box<dyn ExecuteActionPort> + Send + Sync>;
