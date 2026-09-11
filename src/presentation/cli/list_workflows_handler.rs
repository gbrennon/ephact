use super::list_workflows_args::ListWorkflowsArgs;
use crate::application::dtos::responses::ListWorkflowsResponse;
use crate::application::ports::inbound::list_workflows_port::ListWorkflowsPort;

pub struct ListWorkflowsHandler;

impl ListWorkflowsHandler {
    pub fn handle(
        args: ListWorkflowsArgs,
        port: &dyn ListWorkflowsPort,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let request = args.to_domain()?;
        let response = port.execute(request)?;
        Ok(Self::render(&response))
    }

    fn render(response: &ListWorkflowsResponse) -> String {
        response
            .workflows()
            .iter()
            .map(|workflow| workflow.name().unwrap_or("Unnamed workflow").to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
