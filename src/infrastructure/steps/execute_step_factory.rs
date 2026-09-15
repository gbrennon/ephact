use crate::application::ports::inbound::ExecuteStepPort;
use crate::application::ports::outbound::ContainerPort;

pub type ExecuteStepFactory = Box<
    dyn for<'container> Fn(&'container dyn ContainerPort) -> Box<dyn ExecuteStepPort + 'container>
        + Send
        + Sync,
>;
