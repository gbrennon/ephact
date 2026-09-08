use super::list_actions_args::ListActionsArgs;
use crate::application::{
    dtos::ListActionsResponse, ports::inbound::list_actions_port::ListActionsPort,
};

pub struct ListActionsHandler;

impl ListActionsHandler {
    pub fn handle(
        args: ListActionsArgs,
        port: &dyn ListActionsPort,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let request = args.to_domain()?;
        let response = port.execute(request)?;
        Ok(Self::render(&response))
    }

    fn render(response: &ListActionsResponse) -> String {
        response
            .actions
            .iter()
            .map(|action| action.rsplit('/').next().unwrap_or(action).to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
