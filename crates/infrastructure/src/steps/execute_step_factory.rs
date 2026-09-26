use crate::application::ports::{inbound::ExecuteStepPort, outbound::ContainerPort};

pub type ExecuteStepFactory =
    Box<dyn Fn(std::sync::Arc<dyn ContainerPort>) -> Box<dyn ExecuteStepPort> + Send + Sync>;
