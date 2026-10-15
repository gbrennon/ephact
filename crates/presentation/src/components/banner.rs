use super::component::Component;
use crate::{
    application::dtos::responses::ShowProjectBrandingInfoResponse,
    components::{EmblemLayout, ProjectBranding},
};

pub struct Banner<'a> {
    response: &'a ShowProjectBrandingInfoResponse,
    hint: &'a str,
}

impl<'a> Banner<'a> {
    pub fn new(response: &'a ShowProjectBrandingInfoResponse, hint: &'a str) -> Self {
        Self { response, hint }
    }
}

impl Component for Banner<'_> {
    fn render(&self) -> String {
        let emblem = EmblemLayout::new(self.response.emblem()).lines().join("\n");
        format!(
            "{}\n\n{}\n\n{}\n\n{}",
            emblem,
            ProjectBranding::TITLE,
            ProjectBranding::SUBTITLE,
            self.hint
        )
    }
}
