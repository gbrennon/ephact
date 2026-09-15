use crate::application::ports::inbound::ExecuteStepPort;
use crate::application::ports::outbound::ContainerPort;

pub type ExecuteStepFactory =
    Box<dyn Fn(std::sync::Arc<dyn ContainerPort>) -> Box<dyn ExecuteStepPort> + Send + Sync>;
