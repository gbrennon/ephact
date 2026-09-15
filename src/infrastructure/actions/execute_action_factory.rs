use crate::application::ports::inbound::ExecuteActionPort;
use crate::application::ports::outbound::ContainerPort;

pub type ExecuteActionFactory = Box<
    dyn for<'container> Fn(&'container dyn ContainerPort) -> Box<dyn ExecuteActionPort + 'container>
        + Send
        + Sync,
>;
